use std::sync::Mutex;
use tauri::Runtime;
use tauri::AppHandle;
use tauri::State;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use std::time::Duration;
use std::thread;
use serde_json;

const SIDECAR_PORT: u16 = 1421;

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

async fn try_graceful_shutdown() -> Result<(), reqwest::Error> {
    let client = Client::new();
    // Trigger FastAPI's shutdown event
    let response = client
        .post(format!("http://127.0.0.1:{}/shutdown", SIDECAR_PORT))
        .timeout(Duration::from_secs(2))
        .send()
        .await?;
    
    // Wait for response to ensure shutdown was received
    let _ = response.json::<serde_json::Value>().await?;
    Ok(())
}

#[tauri::command]
pub fn stop_python_server(
    sidecar: State<'_, PythonSidecar>,
) -> Result<(), String> {
    let mut sidecar = sidecar.0.lock().map_err(|e| e.to_string())?;
    
    if let Some(child) = sidecar.take() {
        // Try graceful shutdown through FastAPI/Uvicorn
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _ = rt.block_on(try_graceful_shutdown());
        
        // Get PID before we move child with kill()
        #[cfg(unix)]
        let pid = child.pid();
        
        // Send termination signal
        let _ = child.kill();
        thread::sleep(Duration::from_millis(500));
        
        // Try direct signal as last resort
        #[cfg(unix)]
        unsafe {
            libc::kill(pid as i32, libc::SIGKILL);
        }
    }
    
    Ok(())
}

#[tauri::command]
pub async fn process_text(text: String) -> Result<String, String> {
    let client = Client::new();
    let response = client
        .post(format!("http://127.0.0.1:{}/process", SIDECAR_PORT))
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