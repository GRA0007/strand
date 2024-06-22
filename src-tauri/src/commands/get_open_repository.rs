use crate::{
    commands::CommandResult,
    db::{Db, Repository},
};

#[tauri::command]
#[specta::specta]
pub async fn get_open_repository(db: tauri::State<'_, Db>) -> CommandResult<Option<Repository>> {
    Ok(db.state.lock().await.open_repository.clone())
}
