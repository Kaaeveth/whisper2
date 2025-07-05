use std::sync::OnceLock;
use std::collections::HashMap;

pub(crate) mod llm;
pub(crate) mod chat;
pub(crate) mod ollama;
pub(crate) mod reader;

use crate::backend::ollama::OllamaBackend;
use crate::backend::llm::SharedBackend;

pub type BackendStore = OnceLock<HashMap<String, SharedBackend>>;

pub fn build_backend_store() -> BackendStore {
    let mut backends: HashMap<String, SharedBackend> = HashMap::new();

    // Ollama backend
    let ollama = OllamaBackend::new();
    let backend_name: String;
    {
        let backend = ollama.blocking_lock();
        backend_name = backend.name().to_owned();
    }
    backends.insert(backend_name, ollama);

    let lock: BackendStore = OnceLock::new();
    let _ = lock.set(backends);
    return lock;
}
