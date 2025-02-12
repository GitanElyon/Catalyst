use std::fs;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_file() -> Result<String, String> {
    match rfd::FileDialog::new().pick_file() {
        Some(path) => fs::read_to_string(path).map_err(|e| e.to_string()),
        None => {
            println!("No file was selected.");
            Ok(String::new())
        },
    }
}

#[tauri::command]
async fn save_file(content: &str) -> Result<(), String> {
    match rfd::FileDialog::new().save_file() {
        Some(path) => fs::write(path, content).map_err(|e| e.to_string()),
        None => {
            println!("No save location was selected.");
            Ok(())
        },
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, open_file, save_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}