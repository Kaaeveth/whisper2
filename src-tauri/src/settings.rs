use std::{path::{Path, PathBuf}, sync::Arc};

use tauri::{AppHandle, Wry};
use tauri_plugin_store::{Store, StoreExt};
use tokio::sync::RwLock;
use url::Url;

pub(crate) type AppSettings = RwLock<Settings>;

const OLLAMA_URL_KEY: &'static str = "ollamaUrl";
const OLLAMA_MODELS_PATH_KEY: &'static str = "ollamaModelsPath";

pub struct Settings {
    store: Arc<Store<Wry>>
}

impl Settings {
    pub fn new(app: &AppHandle<Wry>) -> Self {
        let settings = app.store("settings.json").unwrap();
        Self {
            store: settings
        }
    }

    pub fn store_ollama_url(&self, url: &str) {
        self.store.set(OLLAMA_URL_KEY, url);
        self.save()
    }

    pub fn store_ollama_models_path(&self, path: &Path) {
        self.store.set(OLLAMA_MODELS_PATH_KEY, path.to_str().unwrap());
        self.save();
    }

    fn save(&self) {
        let _ = self.store.save().inspect_err(|e| {
            eprintln!("Cannot save settings: {e}");
        });
    }

    pub fn ollama_url(&self) -> Url {
        self.store.get(OLLAMA_URL_KEY)
            .and_then(|v| v.as_str().and_then(|v| Some(v.to_owned())))
            .and_then(|url| Url::parse(&url).ok())
            .unwrap_or(Url::parse("http://localhost:11434/api/").unwrap())
    }

    pub fn ollama_models_path(&self) -> Option<PathBuf> {
        self.store.get(OLLAMA_MODELS_PATH_KEY)
            .and_then(|path| path.as_str().and_then(|v| Some(v.to_owned())))
            .and_then(|path| Some(PathBuf::from(path)))
    }
}

pub(crate) fn build_settings(app: &AppHandle<Wry>) -> AppSettings
{
    RwLock::new(Settings::new(app))
}
