pub(crate) mod process_commands;
pub(crate) mod backend_commands;
pub(crate) mod chat_commands;
pub(crate) mod ollama_commands;

#[tauri::command]
pub fn is_debug() -> bool {
    return cfg!(debug_assertions);
}

#[macro_export]
macro_rules! init {
    () => {
        tauri::generate_handler![
            crate::commands::is_debug,
            crate::commands::process_commands::execute,
            crate::commands::process_commands::terminate,
            // Backend
            crate::commands::backend_commands::is_backend_running,
            crate::commands::backend_commands::boot_backend,
            crate::commands::backend_commands::shutdown_backend,
            crate::commands::backend_commands::update_models_in_backend,
            crate::commands::backend_commands::get_models_for_backend,
            crate::commands::backend_commands::get_running_models_in_backend,
            // Models
            crate::commands::backend_commands::is_model_loaded,
            crate::commands::backend_commands::get_model_loaded_size,
            crate::commands::backend_commands::get_model_runtime_info,
            crate::commands::backend_commands::load_model,
            crate::commands::backend_commands::unload_model,
            crate::commands::backend_commands::prompt_model,
            crate::commands::backend_commands::stop_prompt,
            // Chats
            crate::commands::chat_commands::save_chats,
            crate::commands::chat_commands::import_chats,
            // Ollama
            crate::commands::ollama_commands::ollama_set_api_url,
            crate::commands::ollama_commands::ollama_get_api_url,
            crate::commands::ollama_commands::ollama_set_models_path,
            crate::commands::ollama_commands::ollama_get_models_path
        ]
    };
}

pub(crate) use init;
