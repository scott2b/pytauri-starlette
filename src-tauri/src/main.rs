// Prevent console window from showing on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod python_sidecar;
use python_sidecar::{PythonSidecar, start_python_server, stop_python_server, process_text};
use std::sync::Mutex;
use tauri::{WindowEvent, Manager, RunEvent};
use std::process;

fn main() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(PythonSidecar(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            start_python_server,
            stop_python_server,
            process_text
        ])
        .on_window_event(|window, event| {
            if let WindowEvent::Destroyed = event {
                // When the window is closed, stop the Python server and exit the app
                if let Some(app) = window.app_handle().try_state::<PythonSidecar>() {
                    let _ = stop_python_server(app);
                }
                // Force exit after window destruction
                process::exit(0);
            }
        });

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, event| {
        match event {
            RunEvent::ExitRequested { api, .. } => {
                // Handle cleanup before exit
                if let Some(python_sidecar) = app_handle.try_state::<PythonSidecar>() {
                    let _ = stop_python_server(python_sidecar);
                }
                // Don't prevent exit, let it happen
                api.prevent_exit();
                process::exit(0);
            }
            RunEvent::Exit => {
                // Final cleanup and force exit
                if let Some(python_sidecar) = app_handle.try_state::<PythonSidecar>() {
                    let _ = stop_python_server(python_sidecar);
                }
                process::exit(0);
            }
            _ => {}
        }
    });
}
