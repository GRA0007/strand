use crate::{commands::CommandResult, db::Db};

#[tauri::command]
#[specta::specta]
pub async fn set_open_repository(db: tauri::State<'_, Db>, id: Option<i64>) -> CommandResult<()> {
    db.set_open_repository(id).await?;

    Ok(())
}
