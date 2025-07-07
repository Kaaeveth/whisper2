use std::sync::Arc;
use tauri::{ipc::Channel, Manager, Resource, ResourceId, State};

use crate::{backend::{chat::ChatMessage, llm::{ModelInfo, PromptEvent, PromptResponse, RuntimeInfo, SharedBackend, SharedModel}, BackendStore}, errors::Error};

fn get_backend(backend_name: &str, store: &BackendStore)
-> Result<SharedBackend, Error>
{
    store.get().unwrap()
        .get(backend_name)
        .and_then(|b| Some(b.clone()))
        .ok_or(Error::BackendNotFound(backend_name.to_owned()))
}

async fn get_model(backend_name: &str, model_name: &str, store: &BackendStore)
-> Result<SharedModel, Error>
{
    let backend_ = get_backend(backend_name, &store)?;
    let backend = backend_.read().await;

    let mut model: Option<SharedModel> = None;
    for m in backend.models() {
        if m.read().await.info().name == model_name {
            model = Some(m.clone());
        }
    }

    match model {
        Some(m) => Ok(m),
        None => Err(Error::ModelNotFound { model: model_name.to_owned(), backend: backend_name.to_owned() })
    }
}

// === Backend ===

#[tauri::command]
pub async fn is_backend_running(backend_name: &str, store: State<'_, BackendStore>) 
-> Result<bool, Error>
{
    let backend_ = get_backend(backend_name, &store)?;
    let backend = backend_.read().await;
    Ok(backend.running().await)
}

#[tauri::command]
pub async fn boot_backend(backend_name: &str, store: State<'_, BackendStore>)
-> Result<(), Error>
{
    let backend_ = get_backend(backend_name, &store)?;
    let backend = backend_.read().await;
    println!("Booting backend {backend_name}");
    backend.boot().await?;
    Ok(())
}

#[tauri::command]
pub async fn shutdown_backend(backend_name: &str, store: State<'_, BackendStore>)
-> Result<(), Error>
{
    let backend_ = get_backend(backend_name, &store)?;
    let backend = backend_.read().await;
    backend.shutdown().await?;
    Ok(())
}

#[tauri::command]
pub async fn update_models_in_backend(backend_name: &str, store: State<'_, BackendStore>)
-> Result<(), Error>
{
    let backend_ = get_backend(backend_name, &store)?;
    let mut backend = backend_.write().await;
    backend.update_models().await
}

#[tauri::command]
pub async fn get_models_for_backend(backend_name: &str, store: State<'_, BackendStore>)
-> Result<Vec<ModelInfo>, Error>
{
    let backend_ = get_backend(backend_name, &store)?;
    let backend = backend_.read().await;

    let mut models = Vec::new();
    for m in backend.models() {
        let model = m.read().await;
        models.push(model.info().clone());
    }

    Ok(models)
}

#[tauri::command]
pub async fn get_running_models_in_backend(backend_name: &str, store: State<'_, BackendStore>)
-> Result<Vec<RuntimeInfo>, Error>
{
    let backend_ = get_backend(backend_name, &store)?;
    let backend = backend_.read().await;
    backend.get_running_models().await
}

// === Models ===

#[tauri::command]
pub async fn is_model_loaded(backend_name: &str, model_name: &str, store: State<'_, BackendStore>)
-> Result<bool, Error>
{
    let model_ = get_model(backend_name, model_name, &store).await?;
    let model = model_.read().await;
    Ok(model.loaded().await?)
}

#[tauri::command]
pub async fn get_model_loaded_size(backend_name: &str, model_name: &str, store: State<'_, BackendStore>)
-> Result<i32, Error>
{
    let model_ = get_model(backend_name, model_name, &store).await?;
    let model = model_.read().await;
    Ok(model.get_loaded_size().await?)
}

#[tauri::command]
pub async fn get_model_runtime_info(backend_name: &str, model_name: &str, store: State<'_, BackendStore>)
-> Result<RuntimeInfo, Error>
{
    let model_ = get_model(backend_name, model_name, &store).await?;
    let model = model_.read().await;
    model.get_runtime_info().await?.ok_or(Error::Internal("Model is not running".into()))
}

#[tauri::command]
pub async fn load_model(backend_name: &str, model_name: &str, store: State<'_, BackendStore>)
-> Result<(), Error>
{
    let model_ = get_model(backend_name, model_name, &store).await?;
    let mut model = model_.write().await;
    Ok(model.load().await?)
}

#[tauri::command]
pub async fn unload_model(backend_name: &str, model_name: &str, store: State<'_, BackendStore>)
-> Result<(), Error>
{
    let model_ = get_model(backend_name, model_name, &store).await?;
    let mut model = model_.write().await;
    Ok(model.unload().await?)
}

#[tauri::command]
pub async fn prompt_model(
    backend_name: &str, model_name: &str, store: State<'_, BackendStore>,
    content: ChatMessage, history: Vec<ChatMessage>, think: bool,
    response_channel: Channel<PromptEvent>,
    app_handle: tauri::AppHandle
) -> Result<ResourceId, Error>
{
    let model_ = get_model(backend_name, model_name, &store).await?;
    let model = model_.read().await;

    let mut res = model.prompt(content, &history, Some(think)).await?;
    drop(model); // Don't need to lock the model anymore
    let mut prompts = res.get_prompts();
    let rid = app_handle.resources_table().add_arc(Arc::new(PromptResponseResource(res)));

    tokio::spawn(async move {
        loop {
            if let Some(prompt) = prompts.recv().await {
                let stop = prompt.is_stop();
                if response_channel.send(prompt).is_err() || stop {
                    break;
                }
            } else {
                let _ = response_channel.send(PromptEvent::Stop);
                break;
            }
        }
        prompts.close();
        stop_prompt(rid, app_handle).await;
    });
    Ok(rid)
}

#[tauri::command]
pub async fn stop_prompt(rid: ResourceId, app_handle: tauri::AppHandle)
{
    if let Ok(r) = app_handle.resources_table().take::<PromptResponseResource>(rid) {
        r.0.abort();
    }
}

struct PromptResponseResource(Box<dyn PromptResponse>);
impl Resource for PromptResponseResource {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        "PromptResponseResource".into()
    }

    fn close(self: Arc<Self>) {
        // Backup stopping of prompt generation
        self.0.abort();
    }
}
