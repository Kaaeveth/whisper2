use std::error::Error;
use tauri::{App, AppHandle, Manager, Wry};

use crate::{
    backend::{build_backend_store, BackendStore},
    settings::{build_settings, Settings},
};

mod backend;
mod commands;
mod errors;
mod settings;

pub fn setup(app: &mut App<Wry>) -> Result<(), Box<dyn Error>> {
    let settings = build_settings(app.app_handle());
    app.manage(build_backend_store(&*settings.blocking_read()));
    app.manage(settings);
    Ok(())
}

#[allow(deprecated)]
pub fn cleanup_appstate(app: &AppHandle) {
    let _ = app.unmanage::<BackendStore>();
    let _ = app.unmanage::<Settings>();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> i32 {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(setup)
        .invoke_handler(commands::init!())
        .build(tauri::generate_context!())
        .expect("Error starting Whisper2")
        .run_return(|app, event| match event {
            tauri::RunEvent::Exit => {
                // Shutting down backends and other app states.
                // For some reason, Tauri doesn't drop managed state
                // if we do not unmanage that state ourselve.
                // Even if we use run_return instead of run as to not use
                // std::process::exit, drop is not called for the backends.
                cleanup_appstate(&app);
            }
            _ => {}
        })
}
