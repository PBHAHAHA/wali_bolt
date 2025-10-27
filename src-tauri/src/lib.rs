// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod db;
mod rag;
mod app_state;
mod commands;

use app_state::AppState;
use db::Database;

/// Tauri 命令：问候函数（示例）
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Tauri 应用程序入口点
/// 
/// 在移动平台上，这个函数会被标记为移动入口点
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 初始化 opener 插件，用于打开外部链接和文件
        .plugin(tauri_plugin_opener::init())
        // 初始化 dialog 插件，用于文件选择对话框
        .plugin(tauri_plugin_dialog::init())
        // 注册可从前端调用的命令处理器
        .invoke_handler(tauri::generate_handler![
            greet,
            // 配置相关
            commands::config::set_api_key,
            commands::config::get_api_key_status,
            // 文档相关
            commands::document::upload_document,
            commands::document::upload_document_from_path,
            commands::document::get_documents,
            commands::document::delete_document,
            // 对话相关
            commands::chat::ask_question,
            commands::chat::get_conversations,
            commands::chat::get_messages,
            commands::chat::delete_conversation,
            // 文件相关
            commands::file::read_file_content,
            commands::file::get_file_info,
        ])
        // 应用程序设置回调，在应用启动时执行
        .setup(|app| {
            // 初始化应用状态
            use tauri::Manager;
            
            // 获取应用数据目录
            let data_dir = app.path().app_data_dir()
                .expect("Failed to get app data directory");
            
            // 初始化数据库
            let db = tauri::async_runtime::block_on(async {
                Database::new(&data_dir).await
                    .expect("Failed to initialize database")
            });
            
            // 创建应用状态
            let app_state = tauri::async_runtime::block_on(async {
                AppState::new(db).await
                    .expect("Failed to create app state")
            });
            
            // 尝试从数据库加载 API Key
            tauri::async_runtime::block_on(async {
                if let Ok(Some(api_key)) = sqlx::query_scalar::<_, String>(
                    "SELECT value FROM settings WHERE key = ?"
                )
                .bind("qwen_api_key")
                .fetch_optional(app_state.db.pool())
                .await
                {
                    app_state.init_rag_services(api_key);
                }
            });
            
            // 将状态添加到应用管理器
            app.manage(app_state);
            
            // 仅在 Windows 平台上执行以下代码
            #[cfg(target_os = "windows")]
            {
                
                // 获取主窗口实例
                if let Some(window) = app.get_webview_window("main") {
                    // 启用窗口装饰（标题栏、边框等）
                    let _ = window.set_decorations(true);
                    
                    // 使用 Windows API 禁用 Windows 11 的圆角效果
                    #[cfg(windows)]
                    {
                        // 导入所需的 Windows API
                        use windows::Win32::Foundation::HWND;
                        use windows::Win32::UI::WindowsAndMessaging::{
                            SetWindowPos, 
                            SWP_FRAMECHANGED,  // 重新绘制窗口框架
                            SWP_NOMOVE,        // 不改变窗口位置
                            SWP_NOSIZE,        // 不改变窗口大小
                            SWP_NOZORDER       // 不改变窗口 Z 顺序
                        };
                        use windows::Win32::Graphics::Dwm::{
                            DwmSetWindowAttribute,           // DWM 窗口属性设置函数
                            DWMWA_WINDOW_CORNER_PREFERENCE,  // 窗口圆角偏好属性
                            DWMWCP_DONOTROUND                // 不使用圆角（完全直角）
                        };
                        
                        // 获取窗口句柄（HWND）
                        if let Ok(hwnd) = window.hwnd() {
                            unsafe {
                                // 将 Tauri 的窗口句柄转换为 Windows API 的 HWND 类型
                                let hwnd = HWND(hwnd.0 as _);
                                
                                // 设置窗口圆角偏好为"不使用圆角"
                                let preference = DWMWCP_DONOTROUND;
                                
                                // 调用 DWM API 设置窗口属性
                                let _ = DwmSetWindowAttribute(
                                    hwnd,                                      // 窗口句柄
                                    DWMWA_WINDOW_CORNER_PREFERENCE,           // 要设置的属性类型
                                    &preference as *const _ as *const _,      // 属性值的指针
                                    std::mem::size_of_val(&preference) as u32, // 属性值的大小（字节）
                                );
                                
                                // 刷新窗口使圆角设置立即生效
                                // SetWindowPos 会触发窗口重绘
                                let _ = SetWindowPos(
                                    hwnd,                                      // 窗口句柄
                                    HWND(std::ptr::null_mut()),               // 不改变 Z 顺序
                                    0, 0,                                      // 位置（被 SWP_NOMOVE 忽略）
                                    0, 0,                                      // 大小（被 SWP_NOSIZE 忽略）
                                    SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER  // 标志组合
                                );
                            }
                        }
                    }
                }
            }
            
            // 返回 Ok 表示设置成功
            Ok(())
        })
        // 运行 Tauri 应用程序
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
