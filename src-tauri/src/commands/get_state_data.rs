use crate::{
    commands::CommandResult,
    state::{StrandData, StrandState},
};

#[tauri::command]
#[specta::specta]
pub async fn get_state_data(state: tauri::State<'_, StrandState>) -> CommandResult<StrandData> {
    Ok(state.data.lock().await.clone())
}
