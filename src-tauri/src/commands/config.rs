use crate::app_state::AppState;
use tauri::State;

#[derive(serde::Serialize)]
pub struct ConfigResponse {
    success: bool,
    message: String,
}

/// 设置 API Key
#[tauri::command]
pub async fn set_api_key(
    api_key: String,
    state: State<'_, AppState>,
) -> Result<ConfigResponse, String> {
    if api_key.is_empty() {
        return Ok(ConfigResponse {
            success: false,
            message: "API Key 不能为空".to_string(),
        });
    }
    
    // 初始化 RAG 服务
    state.init_rag_services(api_key.clone());
    
    // 保存到数据库
    let timestamp = chrono::Utc::now().timestamp();
    let query = sqlx::query(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?, ?, ?)"
    )
    .bind("qwen_api_key")
    .bind(&api_key)
    .bind(timestamp);
    
    query.execute(state.db.pool()).await
        .map_err(|e| e.to_string())?;
    
    Ok(ConfigResponse {
        success: true,
        message: "API Key 设置成功".to_string(),
    })
}

/// 获取 API Key（用于检查是否已配置）
#[tauri::command]
pub async fn get_api_key_status(
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let result = sqlx::query_scalar::<_, String>(
        "SELECT value FROM settings WHERE key = ?"
    )
    .bind("qwen_api_key")
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(result.is_some())
}

