use std::error::Error;
use tauri::{App, Manager, Wry};

use crate::{backend::{build_backend_store, shutdown_backends}, settings::build_settings};

mod commands;
mod errors;
mod backend;
mod settings;

pub fn setup(app: &mut App<Wry>) -> Result<(), Box<dyn Error>>
{
    let settings = build_settings(app.app_handle());
    app.manage(build_backend_store(&*settings.blocking_read()));
    app.manage(settings);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(setup)
        .invoke_handler(commands::init!())
        .build(tauri::generate_context!())
        .expect("Error starting Whisper2")
        .run(|app, event| match event {
            tauri::RunEvent::ExitRequested { .. } => {
                // Shutting down backends
                // For some reason, Tauri doesn't drop managed state
                // so we have to do that ourselves.
                shutdown_backends(&app);
            }
            _ => {}
        });
}
