use tauri::Window;

#[tauri::command]
fn debug_print(name: &str) -> String {
    format!("Debugger Print: {}!", name)
}

mod app_functions;
mod window_functions;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            debug_print,
            app_functions::open_file,
            app_functions::save_file,
            window_functions::minimize_window,
            window_functions::maximize_window,
            window_functions::close_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}