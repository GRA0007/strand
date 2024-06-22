use crate::{
    commands::CommandResult,
    db::{Db, Repository},
};

#[tauri::command]
#[specta::specta]
pub async fn get_repositories(db: tauri::State<'_, Db>) -> CommandResult<Vec<Repository>> {
    Ok(db.get_repositories().await?)
}
