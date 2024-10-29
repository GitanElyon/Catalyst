use tauri::command;
use std::fs;
use std::path::{PathBuf, Path};
use serde::Serialize;

#[derive(Serialize)]
struct FileEntry {
    path: String,
    is_dir: bool,
    children: Option<Vec<FileEntry>>,
}

fn read_dir_recursive(path: &Path) -> Result<Vec<FileEntry>, String> {
    let mut entries = Vec::new();
    match fs::read_dir(path) {
        Ok(dir_entries) => {
            for entry in dir_entries {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();
                let is_dir = path.is_dir();
                let children = if is_dir {
                    Some(vec![]) // Initially empty, will be loaded on demand
                } else {
                    None
                };
                entries.push(FileEntry {
                    path: path.display().to_string(),
                    is_dir,
                    children,
                });
            }
            Ok(entries)
        }
        Err(err) => Err(err.to_string()),
    }
}

#[command]
fn read_dir(workspace_dir: String) -> Result<Vec<FileEntry>, String> {
    let path = PathBuf::from(workspace_dir);
    read_dir_recursive(&path)
}

#[command]
fn load_directory(path: String) -> Result<Vec<FileEntry>, String> {
    let path = PathBuf::from(path);
    read_dir_recursive(&path)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_dir, load_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}