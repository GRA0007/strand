use crate::{
    commands::CommandResult,
    db::{Db, GitCommandLog},
};

#[tauri::command]
#[specta::specta]
pub async fn get_git_command_log(db: tauri::State<'_, Db>) -> CommandResult<Vec<GitCommandLog>> {
    Ok(db.get_git_command_log().await?)
}
