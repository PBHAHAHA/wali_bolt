use std::path::PathBuf;

/// 读取文件内容（支持 TXT、MD 等文本文件和 PDF）
#[tauri::command]
pub async fn read_file_content(file_path: String) -> Result<String, String> {
    use tokio::fs;
    
    let path = PathBuf::from(&file_path);
    
    // 检查文件是否存在
    if !path.exists() {
        return Err("文件不存在".to_string());
    }
    
    // 判断文件类型
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    match extension.as_str() {
        "pdf" => {
            // PDF 文件：使用 pdf-extract 解析
            extract_pdf_text(&file_path)
        }
        _ => {
            // 其他文本文件：直接读取
            fs::read_to_string(&path).await
                .map_err(|e| format!("读取文件失败: {}", e))
        }
    }
}

/// 从 PDF 文件提取文本
fn extract_pdf_text(file_path: &str) -> Result<String, String> {
    use pdf_extract::extract_text;
    
    extract_text(file_path)
        .map_err(|e| format!("PDF 解析失败: {}. 可能是扫描版 PDF 或加密文件", e))
}

/// 获取文件信息
#[tauri::command]
pub async fn get_file_info(file_path: String) -> Result<(String, usize), String> {
    use tokio::fs;
    
    let path = PathBuf::from(file_path);
    
    // 检查文件是否存在
    if !path.exists() {
        return Err("文件不存在".to_string());
    }
    
    // 获取文件名
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("未知文件")
        .to_string();
    
    // 获取文件大小
    let metadata = fs::metadata(&path).await
        .map_err(|e| format!("获取文件信息失败: {}", e))?;
    let file_size = metadata.len() as usize;
    
    Ok((file_name, file_size))
}

