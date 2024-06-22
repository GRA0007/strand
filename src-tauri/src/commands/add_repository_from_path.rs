use std::path::PathBuf;

use tokio::fs;

use crate::{cli::GitError, commands::CommandResult, db::Db};

#[tauri::command]
#[specta::specta]
pub async fn add_repository_from_path(
    db: tauri::State<'_, Db>,
    local_path: String,
) -> CommandResult<()> {
    let local_path = PathBuf::from(local_path);

    // Check that it's a valid git repository
    fs::metadata(local_path.join(".git"))
        .await
        .ok()
        .filter(|meta| meta.is_dir())
        .ok_or(GitError::NotARepository)?;

    let repository = db.add_repository(local_path).await?;
    db.set_open_repository(Some(repository.id)).await?;

    Ok(())
}
