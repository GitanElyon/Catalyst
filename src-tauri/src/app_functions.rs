use std::fs;

#[tauri::command]
pub async fn open_file() -> Result<String, String> {
    match rfd::FileDialog::new().pick_file() {
        Some(path) => fs::read_to_string(path).map_err(|e| e.to_string()),
        None => {
            println!("No file was selected.");
            Ok(String::new())
        },
    }
}

#[tauri::command]
pub async fn save_file(content: &str) -> Result<(), String> {
    match rfd::FileDialog::new().save_file() {
        Some(path) => fs::write(path, content).map_err(|e| e.to_string()),
        None => {
            println!("No save location was selected.");
            Ok(())
        },
    }
}