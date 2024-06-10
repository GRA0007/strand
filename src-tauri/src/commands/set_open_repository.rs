use crate::{commands::CommandResult, state::StrandState};

#[tauri::command]
#[specta::specta]
pub async fn set_open_repository(
    state: tauri::State<'_, StrandState>,
    id: Option<i64>,
) -> CommandResult<()> {
    state.set_open_repository(id).await?;

    Ok(())
}
