use crate::app_state::AppState;
use crate::db::models::{Conversation, Message};
use tauri::State;
use uuid::Uuid;
use reqwest::Client;

#[derive(serde::Deserialize)]
pub struct AskQuestionRequest {
    question: String,
    conversation_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct AskQuestionResponse {
    success: bool,
    answer: String,
    sources: Vec<String>,
    conversation_id: String,
}

/// RAG 问答
#[tauri::command]
pub async fn ask_question(
    request: AskQuestionRequest,
    state: State<'_, AppState>,
) -> Result<AskQuestionResponse, String> {
    // 检查 RAG 服务是否已初始化
    if !state.is_rag_initialized() {
        return Err("请先配置 API Key".to_string());
    }
    
    // 1. 将问题向量化
    let question_embedding = {
        let service_arc = state.embedding_service.clone();
        let question_clone = request.question.clone();
        
        // 在独立的代码块中获取服务和调用方法
        let (api_key, model) = {
            let guard = service_arc.lock().unwrap();
            let service = guard.as_ref().ok_or("Embedding 服务未初始化")?;
            (service.api_key().clone(), service.model().clone())
        }; // guard 在这里释放
        
        // 调用 API
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;
        
        let request_body = serde_json::json!({
            "model": model,
            "input": {
                "texts": [question_clone]
            }
        });
        
        let response = client
            .post("https://dashscope.aliyuncs.com/api/v1/services/embeddings/text-embedding/text-embedding")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| format!("Embedding API网络请求失败: {}. 请检查网络连接", e))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.map_err(|e| e.to_string())?;
            return Err(format!("Embedding API 请求失败: {} - {}", status, text));
        }
        
        let result: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
        
        if let Some(embeddings) = result["output"]["embeddings"].as_array() {
            if let Some(first) = embeddings.first() {
                if let Some(embedding) = first["embedding"].as_array() {
                    embedding.iter()
                        .filter_map(|v| v.as_f64().map(|f| f as f32))
                        .collect::<Vec<f32>>()
                } else {
                    return Err("Embedding API 返回空向量".to_string());
                }
            } else {
                return Err("Embedding API 返回空向量".to_string());
            }
        } else {
            return Err("Embedding API 返回空向量".to_string());
        }
    };
    
    // 2. 检索相关文档
    let top_k = {
        let config = state.rag_config.lock().unwrap();
        config.top_k
    }; // config 的 MutexGuard 在这里释放
    
    let search_results = state.vector_store.lock().unwrap()
        .search(&question_embedding, top_k);
    
    // 3. 构建上下文
    let context: Vec<String> = search_results.iter()
        .map(|r| r.document.content.clone())
        .collect();
    
    let context_text = context.join("\n\n");
    
    // 4. 获取来源文档名称
    let sources: Vec<String> = search_results.iter()
        .filter_map(|r| {
            r.document.metadata.get("document_name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .collect();
    
    // 5. 调用 LLM 生成答案
    let answer = {
        let service_arc = state.llm_service.clone();
        let question_clone = request.question.clone();
        let context_clone = context_text.clone();
        
        // 在独立的代码块中获取服务和调用方法
        let (api_key, model) = {
            let guard = service_arc.lock().unwrap();
            let service = guard.as_ref().ok_or("LLM 服务未初始化")?;
            (service.api_key().clone(), service.model().clone())
        }; // guard 在这里释放
        
        // 调用 API
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;
        
        let messages = vec![
            serde_json::json!({
                "role": "system",
                "content": "你是一个专业的知识库助手。请基于提供的文档内容回答用户问题。如果文档中没有相关信息，请诚实告知。"
            }),
            serde_json::json!({
                "role": "user",
                "content": format!("参考文档：\n\n{}\n\n问题：{}", context_clone, question_clone)
            })
        ];
        
        let request_body = serde_json::json!({
            "model": model,
            "input": {
                "messages": messages
            },
            "parameters": {
                "temperature": 0.7,
                "top_p": 0.9
            }
        });
        
        let response = client
            .post("https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| format!("LLM API网络请求失败: {}. 请检查网络连接", e))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.map_err(|e| e.to_string())?;
            return Err(format!("LLM API 请求失败: {} - {}", status, text));
        }
        
        let result: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
        
        // 处理不同的响应格式
        if let Some(text) = result["output"]["text"].as_str() {
            text.to_string()
        } else if let Some(choices) = result["output"]["choices"].as_array() {
            if let Some(first) = choices.first() {
                if let Some(message) = first["message"].as_object() {
                    if let Some(content) = message["content"].as_str() {
                        content.to_string()
                    } else {
                        return Err("LLM API 返回空内容".to_string());
                    }
                } else {
                    return Err("LLM API 返回空内容".to_string());
                }
            } else {
                return Err("LLM API 返回空内容".to_string());
            }
        } else {
            return Err("LLM API 返回空内容".to_string());
        }
    };
    
    // 6. 保存对话历史
    let conversation_id = request.conversation_id
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    
    let timestamp = chrono::Utc::now().timestamp();
    
    // 确保对话存在
    let conv_exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM conversations WHERE id = ?"
    )
    .bind(&conversation_id)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| e.to_string())?;
    
    if conv_exists == 0 {
        // 创建新对话
        let title = if request.question.chars().count() > 20 {
            format!("{}...", request.question.chars().take(20).collect::<String>())
        } else {
            request.question.clone()
        };
        
        sqlx::query(
            "INSERT INTO conversations (id, title, created_at, updated_at) VALUES (?, ?, ?, ?)"
        )
        .bind(&conversation_id)
        .bind(&title)
        .bind(timestamp)
        .bind(timestamp)
        .execute(state.db.pool())
        .await
        .map_err(|e| e.to_string())?;
    }
    
    // 保存用户消息
    let user_msg_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO messages (id, conversation_id, role, content, sources, created_at) 
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&user_msg_id)
    .bind(&conversation_id)
    .bind("user")
    .bind(&request.question)
    .bind::<Option<String>>(None)
    .bind(timestamp)
    .execute(state.db.pool())
    .await
    .map_err(|e| e.to_string())?;
    
    // 保存 AI 回复
    let ai_msg_id = Uuid::new_v4().to_string();
    let sources_json = serde_json::to_string(&sources).ok();
    
    sqlx::query(
        "INSERT INTO messages (id, conversation_id, role, content, sources, created_at) 
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&ai_msg_id)
    .bind(&conversation_id)
    .bind("assistant")
    .bind(&answer)
    .bind(&sources_json)
    .bind(timestamp)
    .execute(state.db.pool())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(AskQuestionResponse {
        success: true,
        answer,
        sources,
        conversation_id,
    })
}

/// 获取对话历史
#[tauri::command]
pub async fn get_conversations(
    state: State<'_, AppState>,
) -> Result<Vec<Conversation>, String> {
    let conversations = sqlx::query_as::<_, Conversation>(
        "SELECT * FROM conversations ORDER BY updated_at DESC LIMIT 50"
    )
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(conversations)
}

/// 获取对话消息
#[tauri::command]
pub async fn get_messages(
    conversation_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<Message>, String> {
    let messages = sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE conversation_id = ? ORDER BY created_at ASC"
    )
    .bind(&conversation_id)
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(messages)
}

/// 删除对话
#[tauri::command]
pub async fn delete_conversation(
    conversation_id: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    sqlx::query("DELETE FROM conversations WHERE id = ?")
        .bind(&conversation_id)
        .execute(state.db.pool())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(true)
}

