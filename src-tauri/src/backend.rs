use std::collections::HashMap;

pub(crate) mod llm;
pub(crate) mod chat;
pub(crate) mod ollama;
pub(crate) mod reader;

use crate::backend::ollama::SharedOllamaBackend;
use crate::backend::llm::SharedBackend;
use crate::settings::Settings;

pub type BackendStore = HashMap<String, SharedBackend>;

pub fn build_backend_store(settings: &Settings) -> BackendStore {
    let mut backends: HashMap<String, SharedBackend> = HashMap::new();

    // Ollama backend
    let ollama_url = settings.ollama_url();
    let ollama_models_path = settings.ollama_models_path();
    let ollama = SharedOllamaBackend::new(ollama_url, ollama_models_path);
    let backend_name: String;
    {
        let backend = ollama.blocking_read();
        backend_name = backend.name().to_owned();
    }
    backends.insert(backend_name, ollama);

    return backends;
}
