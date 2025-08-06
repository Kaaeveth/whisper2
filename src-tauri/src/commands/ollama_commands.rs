use std::path::PathBuf;

use tauri::State;

use crate::{backend::{ollama::{not_ollama, OllamaBackend, OLLAMA_NAME}, BackendStore}, commands::backend_commands::get_backend, errors, settings::AppSettings, with_llm};

#[tauri::command]
pub async fn ollama_set_api_url(
    url: &str,
    store: State<'_, BackendStore>,
    settings: State<'_, AppSettings>
)
-> Result<(), errors::Error>
{
    with_llm!(OLLAMA_NAME, &store, write|backend {
        let ollama = backend
            .as_any_mut()
            .downcast_mut::<OllamaBackend>()
            .ok_or(not_ollama())?;

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
        let ollama = backend
            .as_any()
            .downcast_ref::<OllamaBackend>()
            .ok_or(not_ollama())?;
        Ok(ollama.get_api_url().to_string())
    })
}

#[tauri::command]
pub async fn ollama_get_models_path(store: State<'_, BackendStore>)
-> Result<Option<String>, errors::Error>
{
    with_llm!(OLLAMA_NAME, &store, read|backend {
        let ollama = backend
            .as_any()
            .downcast_ref::<OllamaBackend>()
            .ok_or(not_ollama())?;

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
        let ollama = backend
            .as_any_mut()
            .downcast_mut::<OllamaBackend>()
            .ok_or(not_ollama())?;

        let path = PathBuf::from(path);
        let _ = path.try_exists()?;

        ollama.set_models_path(&path).await?;
        settings.read().await.store_ollama_models_path(&path);
        Ok(())
    })
}
