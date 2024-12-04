use chrono::{DateTime, Local};
use md5;
use serde::Serialize;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use tauri_plugin_sql::{Migration, MigrationKind};

#[derive(Debug, Serialize)]
struct FileInfo {
    name: String,
    path: String,
    size: u64,
    md5: String,
    mtime: Option<String>,
    birthtime: Option<String>,
}

fn calculate_md5(file_path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut hasher = md5::Context::new();
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.consume(&buffer[..bytes_read]);
    }
    Ok(format!("{:x}", hasher.compute()))
}

fn format_time(time: std::time::SystemTime) -> String {
  let datetime: DateTime<Local> = time.into();
  datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[tauri::command]
fn get_file_info(path: String, extensions: Vec<String>) -> Result<Vec<FileInfo>, String> {
    let mut file_infos = Vec::new();

    for entry in fs::read_dir(&path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if extensions
                    .iter()
                    .any(|e| e.eq_ignore_ascii_case(ext.to_str().unwrap_or("")))
                {
                    let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
                    let md5 = calculate_md5(&path).map_err(|e| e.to_string())?;
                    let mtime = metadata.modified().ok().map(format_time);
                    let birthtime = metadata.created().ok().map(format_time);

                    file_infos.push(FileInfo {
                        name: path.file_name().unwrap().to_string_lossy().into(),
                        path: path.to_string_lossy().into(),
                        size: metadata.len(),
                        md5,
                        mtime,
                        birthtime,
                    });
                }
            }
        }
    }

    Ok(file_infos)
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        // Define your migrations here
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "CREATE TABLE IF NOT EXISTS dirs (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL);
                  CREATE TABLE IF NOT EXISTS exts (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL);
                 ",
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:store.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .invoke_handler(tauri::generate_handler![greet, get_file_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
