use std::error::Error;
use tauri::{App, Manager, Runtime};

use crate::backend::build_backend_store;

mod commands;
mod errors;
mod backend;

pub fn setup<R>(app: &mut App<R>) -> Result<(), Box<dyn Error>>
where R: Runtime
{
    let backends = build_backend_store();
    app.manage(backends);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup)
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(commands::init!())
        .run(tauri::generate_context!())
        .expect("Error starting Whisper2");
}
