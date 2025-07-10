use std::sync::Arc;

use tauri::{AppHandle, Wry};
use tauri_plugin_store::{Store, StoreExt};
use tokio::sync::RwLock;

pub(crate) type AppSettings = RwLock<Settings>;

pub struct Settings {
    store: Arc<Store<Wry>>
}

impl Settings {
    pub fn new(app: &AppHandle<Wry>) -> Self 
    {
        let settings = app.store("settings.json").unwrap();
        Self { 
            store: settings 
        }
    }

    pub fn store_ollama_url(&self, url: &str)
    {
        self.store.set("ollamaUrl", url);
        self.save()
    }

    fn save(&self) {
        let _ = self.store.save().inspect_err(|e| {
            eprintln!("Cannot save settings: {e}");
        });
    }

    pub fn ollama_url(&self) -> String
    {
        self.store.get("ollamaUrl")
            .and_then(|v| v.as_str().and_then(|v| Some(v.to_owned())))
            .unwrap_or("http://localhost:11434/api/".to_string())
    }
}

pub(crate) fn build_settings(app: &AppHandle<Wry>) -> AppSettings
{
    RwLock::new(Settings::new(app))
}
