// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub(crate) mod process_commands;

#[macro_export]
macro_rules! init {
    () => {
        tauri::generate_handler![
            crate::commands::process_commands::execute,
            crate::commands::process_commands::terminate
        ]
    };
}

pub(crate) use init;
