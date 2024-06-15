use crate::{
    commands::CommandResult,
    state::{GitCommandLog, StrandState},
};

#[tauri::command]
#[specta::specta]
pub async fn get_git_command_log(
    state: tauri::State<'_, StrandState>,
) -> CommandResult<Vec<GitCommandLog>> {
    Ok(state.get_git_command_log().await?)
}
