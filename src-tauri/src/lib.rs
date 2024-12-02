// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::{Manager, PhysicalPosition, Runtime};
use tauri_plugin_shell::ShellExt;

#[tauri::command]
async fn open_search(window: tauri::Window, query: &str) -> Result<(), String> {
    let url = format!("https://www.google.com/search?q={}", query);
    window.shell().open(&url, None).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            
            // Get the monitor's size
            if let Some(monitor) = window.current_monitor().unwrap() {
                let size = monitor.size();
                let position = PhysicalPosition {
                    x: (size.width as f64 / 2.0 - 340.0) as i32,
                    y: (size.height as f64 / 3.0) as i32,
                };
                window.set_position(position).unwrap();
            }
            
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![open_search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
