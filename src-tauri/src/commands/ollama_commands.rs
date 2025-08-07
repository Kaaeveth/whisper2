use std::path::PathBuf;

use log::trace;
use tauri::{ipc::Channel, State};

use crate::{backend::{ollama::{not_ollama, OllamaBackend, OllamaPullProgress, OLLAMA_NAME}, BackendStore}, commands::backend_commands::get_backend, errors::{self, Error}, settings::AppSettings, with_llm};

#[tauri::command]
pub async fn ollama_set_api_url(
    url: &str,
    store: State<'_, BackendStore>,
    settings: State<'_, AppSettings>
)
-> Result<(), errors::Error>
{
    with_llm!(OLLAMA_NAME, &store, write|backend {
        let ollama = backend.to_mut::<OllamaBackend>().ok_or(not_ollama())?;
        ollama.set_api_url(url)?;
        settings.read().await.store_ollama_url(url);
        Ok(())
    })
}

#[tauri::command]
pub async fn ollama_get_api_url(store: State<'_, BackendStore>)
-> Result<String, errors::Error>
{
    with_llm!(OLLAMA_NAME, &store, read|backend {
        let ollama = backend.to::<OllamaBackend>().ok_or(not_ollama())?;
        Ok(ollama.get_api_url().to_string())
    })
}

#[tauri::command]
pub async fn ollama_get_models_path(store: State<'_, BackendStore>)
-> Result<Option<String>, errors::Error>
{
    with_llm!(OLLAMA_NAME, &store, read|backend {
        let ollama = backend.to::<OllamaBackend>().ok_or(not_ollama())?;
        Ok(ollama.get_models_path().and_then(|path| Some(path.to_str().unwrap().to_owned())))
    })
}

#[tauri::command]
pub async fn ollama_set_models_path(
    path: &str,
    store: State<'_, BackendStore>,
    settings: State<'_, AppSettings>
)
-> Result<(), errors::Error>
{
    with_llm!(OLLAMA_NAME, &store, write|backend {
        let ollama = backend.to_mut::<OllamaBackend>().ok_or(not_ollama())?;
        let path = PathBuf::from(path);
        if !path.exists() {
            return Err(Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Models directory does not exist")));
        }

        ollama.set_models_path(&path).await?;
        settings.read().await.store_ollama_models_path(&path);
        Ok(())
    })
}

#[tauri::command]
pub async fn ollama_pull_model(
    tag: String,
    progress_channel: Channel<OllamaPullProgress>,
    store: State<'_, BackendStore>,
)
-> Result<(), errors::Error>
{
    with_llm!(OLLAMA_NAME, &store, write|backend {
        let ollama = backend.to_mut::<OllamaBackend>().ok_or(not_ollama())?;
        let mut progress_receiver = ollama.pull_model(&tag).await?;
        tokio::spawn(async move {
            let mut iter: u64 = 0;
            while let Some(progress) = progress_receiver.recv().await {
                if iter % 64 == 0 || progress.status == "success" || progress.error.is_some() {
                    trace!("Pulling {tag} - {:?}", progress);
                    if let Err(e) = progress_channel.send(progress) {
                        trace!("Pulling model - Channel closed: {:?}", e);
                        break;
                    }
                }
                iter += 1;
            }
            let _ = progress_channel.send(OllamaPullProgress {
                status: "done".into(), digest: None, total: None, completed: None, error: None
            });
        });
        Ok(())
    })
}
