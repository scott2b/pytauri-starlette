// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::App;

pub fn run() -> tauri::Result<App> {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .build(tauri::generate_context!())
}
