use std::sync::OnceLock;
use std::collections::HashMap;

pub(crate) mod llm;
pub(crate) mod chat;
pub(crate) mod ollama;
pub(crate) mod reader;

use url::Url;

use crate::backend::ollama::OllamaBackend;
use crate::backend::llm::SharedBackend;
use crate::settings::Settings;

pub type BackendStore = OnceLock<HashMap<String, SharedBackend>>;

pub fn build_backend_store(settings: &Settings) -> BackendStore {
    let mut backends: HashMap<String, SharedBackend> = HashMap::new();

    // Ollama backend
    let ollama_url = settings.ollama_url();
    let ollama_url = Url::parse(&ollama_url).unwrap();
    let ollama = OllamaBackend::new(ollama_url);
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
