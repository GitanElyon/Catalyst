use tauri::Window;

#[tauri::command]
pub async fn minimize_window(window: Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn maximize_window(window: Window) -> Result<(), String> {
    match window.is_maximized() {
        Ok(true) => window.unmaximize().map_err(|e| e.to_string()),
        Ok(false) => window.maximize().map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn close_window(window: Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}