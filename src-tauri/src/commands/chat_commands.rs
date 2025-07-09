use std::io::BufReader;

use crate::{backend::chat::{Chat, timestamp_to_string}, errors};
use crate::errors::Error;
use tauri::{AppHandle, Manager, WebviewWindow};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_store::StoreExt;

fn get_main_window(app: &AppHandle) -> Result<WebviewWindow, Error>
{
    app
    .get_webview_window("main")
    .ok_or(Error::Internal("Could not get main window".into()))
}

#[tauri::command]
pub async fn save_chats(store_name: &str, app: AppHandle) 
-> Result<(), Error> 
{
    // Get Chats and format them into a list together with the id
    let chat_store = app.store(&store_name).map_err(errors::internal)?;
    let mut chats: Vec<Chat> = Vec::with_capacity(chat_store.length());
    for key in chat_store.keys().iter() {
        if let Some(mut chat) = chat_store.get(&key) {
            let c = chat.as_object_mut().ok_or(Error::Internal("Wrong chat format!".into()))?;
            c.insert("uuid".into(), serde_json::json!(key.to_owned()));
            let chat: Chat = serde_json::from_value(chat)?;
            chats.push(chat);
        }
    }

    let main_window = get_main_window(&app)?;
    let Some(path) = app
        .dialog()
        .file()
        .set_title("Save Chats")
        .add_filter("Chats", &["json"])
        .set_file_name("chats.json")
        .set_parent(&main_window)
        .blocking_save_file() else {
            return Ok(());
        };

    println!("Saving chats to {path:?}");
    let file = std::fs::File::create(
        path.into_path().map_err(errors::internal)?
    )?;
    serde_json::to_writer_pretty(file, &chats).map_err(|e| Error::SerdeJson(e))
}

#[tauri::command]
pub async fn import_chats(store_name: &str, app: AppHandle)
-> Result<(), Error>
{
    let chat_store = app.store(&store_name).map_err(errors::internal)?;

    let main_window = get_main_window(&app)?;
    let Some(path) = app
        .dialog()
        .file()
        .set_title("Import Chats")
        .add_filter("Chats", &["json"])
        .set_parent(&main_window)
        .blocking_pick_file() else {
            return Ok(());
        };

    println!("Importing chats from {path:?}");
    let file = std::fs::File::open(
        path.into_path().map_err(|e| Error::Internal(e.to_string()))?
    )?;
    let reader = BufReader::new(file);

    // Map chat ids onto their chats
    let chats: Vec<Chat> = serde_json::from_reader(reader).map_err(|e| Error::SerdeJson(e))?;
    for chat in chats {
        let timestamp = timestamp_to_string(&chat.created_at)?;
        chat_store.set(chat.uuid.to_string(), serde_json::json!({
            "title": chat.title,
            "createdAt": timestamp,
            "history": chat.history
        }));
    }
    chat_store.save().map_err(errors::internal)
}
