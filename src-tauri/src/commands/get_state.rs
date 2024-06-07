use crate::state::{StrandData, StrandState};

#[tauri::command]
#[specta::specta]
pub async fn get_state_data(state: tauri::State<'_, StrandState>) -> Result<StrandData, ()> {
    Ok(state.0.lock().await.data.clone())
}