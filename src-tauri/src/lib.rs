use std::fs::{self};
use std::path::Path;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// ディレクトリー内のファイル名を取得するTauriコマンド。
// 読取成功時はファイル名のリストを、失敗時はエラーメッセージを返す。
#[tauri::command]
fn translate(source_str: String, command_name: String) -> String {
    source_str
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            translate
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
