use crate::{
    commands::CommandResult,
    db::{Db, GitCommandLog, GitCommandType},
};

#[tauri::command]
#[specta::specta]
pub async fn get_git_command_log(
    db: tauri::State<'_, Db>,
    filter: Option<GitCommandType>,
) -> CommandResult<Vec<GitCommandLog>> {
    Ok(db.get_git_command_log(filter).await?)
}
