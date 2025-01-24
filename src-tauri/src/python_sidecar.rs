use std::sync::Mutex;
use tauri::Runtime;
use tauri::AppHandle;
use tauri::State;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandChild;

pub struct PythonSidecar(pub Mutex<Option<CommandChild>>);

#[derive(Serialize, Deserialize)]
pub struct TextRequest {
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct TextResponse {
    processed_text: String,
}

#[tauri::command]
pub async fn start_python_server<R: Runtime>(
    app: AppHandle<R>,
    sidecar: State<'_, PythonSidecar>,
) -> Result<(), String> {
    let mut sidecar = sidecar.0.lock().map_err(|e| e.to_string())?;
    
    if sidecar.is_some() {
        return Ok(());
    }

    let (mut _rx, child) = app
        .shell()
        .sidecar("server")
        .expect("failed to create sidecar command")
        .spawn()
        .map_err(|e| e.to_string())?;

    *sidecar = Some(child);
    Ok(())
}

#[tauri::command]
pub async fn stop_python_server(
    sidecar: State<'_, PythonSidecar>,
) -> Result<(), String> {
    let mut sidecar = sidecar.0.lock().map_err(|e| e.to_string())?;
    
    if let Some(mut child) = sidecar.take() {
        child.kill().map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn process_text(text: String) -> Result<String, String> {
    let client = Client::new();
    let response = client
        .post("http://127.0.0.1:8000/process")
        .json(&TextRequest { text })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let result: TextResponse = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(result.processed_text)
} 