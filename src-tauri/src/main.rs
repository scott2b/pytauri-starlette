// Prevent console window from showing on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod python_sidecar;
use python_sidecar::{PythonSidecar, start_python_server, stop_python_server, process_text};
use std::sync::Mutex;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(PythonSidecar(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            start_python_server,
            stop_python_server,
            process_text
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
