use std::sync::OnceLock;
use std::collections::HashMap;

pub(crate) mod llm;
pub(crate) mod chat;
pub(crate) mod ollama;
pub(crate) mod reader;

use tauri::{AppHandle, Manager};
use url::Url;

use crate::backend::ollama::SharedOllamaBackend;
use crate::backend::llm::SharedBackend;
use crate::settings::Settings;

pub type BackendStore = OnceLock<HashMap<String, SharedBackend>>;

pub fn build_backend_store(settings: &Settings) -> BackendStore {
    let mut backends: HashMap<String, SharedBackend> = HashMap::new();

    // Ollama backend
    let ollama_url = settings.ollama_url();
    let ollama_url = Url::parse(&ollama_url).unwrap();
    let ollama = SharedOllamaBackend::new(ollama_url);
    let backend_name: String;
    {
        let backend = ollama.blocking_read();
        backend_name = backend.name().to_owned();
    }
    backends.insert(backend_name, ollama);

    let lock: BackendStore = OnceLock::new();
    let _ = lock.set(backends);
    return lock;
}

pub fn shutdown_backends(app: &AppHandle) {
    let store = app.state::<BackendStore>();
    let backends = store.get().unwrap();
    let rt = tokio::runtime::Builder::new_current_thread()
        .worker_threads(1)
        .build()
        .unwrap();
    rt.block_on(async {
        for backend in backends.values() {
            let mut guard = backend.write().await;
            let _ = guard.shutdown().await;
        }
    });
}
