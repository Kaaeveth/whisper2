use tauri::State;

use crate::{backend::{ollama::{OllamaBackendInner, OLLAMA_NAME}, BackendStore}, commands::backend_commands::get_backend, errors, settings::AppSettings, with_llm};

fn not_ollama() -> errors::Error {
    errors::internal("Backend is not Ollama")
}

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
            .downcast_mut::<OllamaBackendInner>()
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
            .downcast_ref::<OllamaBackendInner>()
            .ok_or(not_ollama())?;
        Ok(ollama.get_api_url().to_string())
    })
}
