use crate::{cli::GitCommand, state::GitCommandType};

use super::CommandResult;

#[tauri::command]
#[specta::specta]
pub async fn git_fetch(app_handle: tauri::AppHandle) -> CommandResult<()> {
    GitCommand::new("fetch")
        .arg("--all")
        .run(&app_handle, GitCommandType::Mutation)
        .await?;

    Ok(())
}
