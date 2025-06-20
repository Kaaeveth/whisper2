use tauri::{plugin::{TauriPlugin, Builder as PluginBuilder}, Runtime};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod process_commands;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    PluginBuilder::new("commands")
        .invoke_handler(tauri::generate_handler![
            process_commands::execute,
            process_commands::terminate
        ])
        .build()
}
