pub(crate) mod process_commands;
pub(crate) mod backend_commands;

#[macro_export]
macro_rules! init {
    () => {
        tauri::generate_handler![
            crate::commands::process_commands::execute,
            crate::commands::process_commands::terminate,
            // Backend
            crate::commands::backend_commands::is_backend_running,
            crate::commands::backend_commands::boot_backend,
            crate::commands::backend_commands::shutdown_backend,
            crate::commands::backend_commands::update_models_in_backend,
            crate::commands::backend_commands::get_models_for_backend,
            crate::commands::backend_commands::is_model_loaded,
            crate::commands::backend_commands::get_model_loaded_size,
            crate::commands::backend_commands::get_model_runtime_info,
            crate::commands::backend_commands::load_model,
            crate::commands::backend_commands::unload_model,
            crate::commands::backend_commands::prompt_model,
            crate::commands::backend_commands::stop_prompt
        ]
    };
}

pub(crate) use init;
