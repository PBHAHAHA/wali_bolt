use crate::app_state::AppState;
use crate::db::models::Document;
use crate::rag::text_splitter::TextSplitter;
use crate::rag::vector_store::VectorDocument;
use crate::commands::file::read_file_content;
use tauri::State;
use uuid::Uuid;
use reqwest::Client;
use std::path::Path;

#[derive(serde::Deserialize)]
pub struct UploadDocumentRequest {
    name: String,
    content: String,
    file_type: Option<String>,
}

#[derive(serde::Serialize)]
pub struct UploadDocumentResponse {
    success: bool,
    message: String,
    document_id: Option<String>,
}

/// 上传文档
#[tauri::command]
pub async fn upload_document(
    request: UploadDocumentRequest,
    state: State<'_, AppState>,
) -> Result<UploadDocumentResponse, String> {
    // 检查 RAG 服务是否已初始化
    if !state.is_rag_initialized() {
        return Ok(UploadDocumentResponse {
            success: false,
            message: "请先配置 API Key".to_string(),
            document_id: None,
        });
    }
    
    let document_id = Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().timestamp();
    
    // 1. 保存文档到数据库
    let file_size = request.content.len() as i64;
    
    sqlx::query(
        "INSERT INTO documents (id, name, content, file_type, file_size, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&document_id)
    .bind(&request.name)
    .bind(&request.content)
    .bind(&request.file_type)
    .bind(file_size)
    .bind(timestamp)
    .bind(timestamp)
    .execute(state.db.pool())
    .await
    .map_err(|e| e.to_string())?;
    
    // 2. 文本分块
    let chunks = {
        let config = state.rag_config.lock().unwrap();
        let splitter = TextSplitter::new(config.chunk_size, config.chunk_overlap);
        splitter.split_smart(&request.content)
    }; // config 的 MutexGuard 在这里自动释放
    
    let start_time = std::time::Instant::now();
    println!("📦 文档分块完成: {} 个块", chunks.len());
    
    // 3. 批量生成向量（每批 25 个 - 通义千问 API 限制）
    const BATCH_SIZE: usize = 25;
    let total_chunks = chunks.len();
    let mut all_embeddings = Vec::new();
    
    // 获取 API 配置
    let (api_key, model) = {
        let guard = state.embedding_service.lock().unwrap();
        let service = guard.as_ref().ok_or("Embedding服务未初始化")?;
        (service.api_key().clone(), service.model().clone())
    };
    
    // 创建 HTTP 客户端
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;
    
    // 并发处理多个批次（最多 5 个并发）
    let mut batch_tasks = Vec::new();
    
    for (batch_idx, chunk_batch) in chunks.chunks(BATCH_SIZE).enumerate() {
        let client_clone = client.clone();
        let api_key_clone = api_key.clone();
        let model_clone = model.clone();
        let batch_texts: Vec<String> = chunk_batch.iter().map(|s| s.to_string()).collect();
        let batch_num = batch_idx + 1;
        let total_batches = (total_chunks + BATCH_SIZE - 1) / BATCH_SIZE;
        
        let task = tokio::spawn(async move {
            println!("🚀 正在处理批次 {}/{} ({} 个块)...", batch_num, total_batches, batch_texts.len());
            
            let request_body = serde_json::json!({
                "model": model_clone,
                "input": {
                    "texts": batch_texts
                }
            });
            
            let response = client_clone
                .post("https://dashscope.aliyuncs.com/api/v1/services/embeddings/text-embedding/text-embedding")
                .header("Authorization", format!("Bearer {}", api_key_clone))
                .header("Content-Type", "application/json")
                .json(&request_body)
                .send()
                .await
                .map_err(|e| format!("批次 {} Embedding API网络请求失败: {}", batch_num, e))?;
            
            if !response.status().is_success() {
                let status = response.status();
                let text = response.text().await.map_err(|e| e.to_string())?;
                return Err(format!("批次 {} Embedding API 请求失败: {} - {}", batch_num, status, text));
            }
            
            let result: serde_json::Value = response.json().await
                .map_err(|e| format!("批次 {} 解析响应失败: {}", batch_num, e))?;
            
            let embeddings_array = result["output"]["embeddings"].as_array()
                .ok_or_else(|| format!("批次 {} API 返回格式错误", batch_num))?;
            
            let mut batch_embeddings = Vec::new();
            for emb in embeddings_array {
                let embedding = emb["embedding"].as_array()
                    .ok_or_else(|| format!("批次 {} 向量数据格式错误", batch_num))?
                    .iter()
                    .filter_map(|v| v.as_f64().map(|f| f as f32))
                    .collect::<Vec<f32>>();
                
                if embedding.is_empty() {
                    return Err(format!("批次 {} 向量为空", batch_num));
                }
                
                batch_embeddings.push(embedding);
            }
            
            println!("✅ 批次 {}/{} 完成", batch_num, total_batches);
            Ok::<Vec<Vec<f32>>, String>(batch_embeddings)
        });
        
        batch_tasks.push(task);
        
        // 控制并发数：每 10 个任务等待一批完成
        if batch_tasks.len() >= 10 {
            let completed = batch_tasks.remove(0);
            let embeddings = completed.await
                .map_err(|e| format!("任务执行失败: {}", e))?
                .map_err(|e| e)?;
            all_embeddings.extend(embeddings);
        }
    }
    
    // 等待剩余任务完成
    for task in batch_tasks {
        let embeddings = task.await
            .map_err(|e| format!("任务执行失败: {}", e))?
            .map_err(|e| e)?;
        all_embeddings.extend(embeddings);
    }
    
    let embedding_time = start_time.elapsed();
    println!("🎉 所有向量生成完成: {} 个 (耗时: {:.2}秒)", all_embeddings.len(), embedding_time.as_secs_f64());
    
    // 4. 批量保存到数据库（单个事务）
    println!("💾 开始批量保存到数据库...");
    let mut tx = state.db.pool().begin().await
        .map_err(|e| format!("开始事务失败: {}", e))?;
    
    for (index, (chunk_content, embedding)) in chunks.iter().zip(all_embeddings.iter()).enumerate() {
        let chunk_id = Uuid::new_v4().to_string();
        
        // 保存 chunk 到数据库
        sqlx::query(
            "INSERT INTO chunks (id, document_id, content, chunk_index, created_at) 
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&chunk_id)
        .bind(&document_id)
        .bind(chunk_content)
        .bind(index as i64)
        .bind(timestamp)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("插入块 {} 失败: {}", index, e))?;
        
        // 添加到向量存储
        let vector_doc = VectorDocument {
            id: chunk_id.clone(),
            content: chunk_content.clone(),
            embedding: embedding.clone(),
            metadata: serde_json::json!({
                "document_id": document_id,
                "chunk_index": index,
                "document_name": request.name,
            }),
        };
        
        state.vector_store.lock().unwrap().add_document(vector_doc);
    }
    
    // 提交事务
    tx.commit().await
        .map_err(|e| format!("提交事务失败: {}", e))?;
    
    let total_time = start_time.elapsed();
    println!("✅ 数据库保存完成 (总耗时: {:.2}秒)", total_time.as_secs_f64());
    
    Ok(UploadDocumentResponse {
        success: true,
        message: format!("文档上传成功，共分为 {} 个块", chunks.len()),
        document_id: Some(document_id),
    })
}

/// 获取所有文档列表
#[tauri::command]
pub async fn get_documents(
    state: State<'_, AppState>,
) -> Result<Vec<Document>, String> {
    let documents = sqlx::query_as::<_, Document>(
        "SELECT * FROM documents ORDER BY created_at DESC"
    )
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(documents)
}

/// 删除文档
#[tauri::command]
pub async fn delete_document(
    document_id: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    // 从数据库删除（会级联删除chunks）
    sqlx::query("DELETE FROM documents WHERE id = ?")
        .bind(&document_id)
        .execute(state.db.pool())
        .await
        .map_err(|e| e.to_string())?;
    
    // 从向量存储删除
    state.vector_store.lock().unwrap().remove_by_document_id(&document_id);
    
    Ok(true)
}

/// 从文件路径上传文档（支持 PDF 等格式）
#[tauri::command]
pub async fn upload_document_from_path(
    file_path: String,
    state: State<'_, AppState>,
) -> Result<UploadDocumentResponse, String> {
    // 检查 RAG 服务是否已初始化
    if !state.is_rag_initialized() {
        return Ok(UploadDocumentResponse {
            success: false,
            message: "请先配置 API Key".to_string(),
            document_id: None,
        });
    }
    
    // 获取文件名
    let file_name = Path::new(&file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("未知文件")
        .to_string();
    
    // 检测文件类型
    let file_type = Path::new(&file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    // 读取文件内容（会自动处理 PDF）
    let content = read_file_content(file_path).await?;
    
    // 调用原有的上传逻辑
    let request = UploadDocumentRequest {
        name: file_name,
        content,
        file_type: Some(file_type),
    };
    
    upload_document(request, state).await
}

