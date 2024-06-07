use std::path::PathBuf;

use tokio::fs;

use crate::state::StrandState;

#[tauri::command]
#[specta::specta]
pub async fn add_repository_from_path(
    state: tauri::State<'_, StrandState>,
    local_path: String,
) -> Result<(), ()> {
    let local_path = PathBuf::from(local_path);

    // Check that it's a valid git repository
    fs::metadata(local_path.join(".git"))
        .await
        .ok()
        .filter(|meta| meta.is_dir())
        .ok_or(())?;

    let mut state = state.0.lock().await;
    let repository = state.add_repository(local_path).await.unwrap();
    state
        .set_open_repository(Some(repository.id))
        .await
        .unwrap();

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn set_open_repository(
    state: tauri::State<'_, StrandState>,
    id: Option<i64>,
) -> Result<(), ()> {
    let mut state = state.0.lock().await;

    state.set_open_repository(id).await.unwrap();

    Ok(())
}
