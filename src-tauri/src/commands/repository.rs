use std::path::PathBuf;

use serde::Serialize;
use specta::Type;
use sqlx::prelude::FromRow;
use tokio::fs;

use crate::DbPool;

#[derive(Serialize, Type, FromRow)]
pub struct Repository {
    id: i64,
    name: String,
    local_path: String,
    created_at: String,
    last_opened_at: Option<String>,
    last_fetched_at: Option<String>,
    has_changes: bool,
}

#[tauri::command]
#[specta::specta]
pub async fn add_repository(
    pool: tauri::State<'_, DbPool>,
    local_path: String,
    created_at: String,
) -> Result<(), ()> {
    let local_path = PathBuf::from(local_path);

    // Check that it's a valid git repository
    fs::metadata(local_path.join(".git"))
        .await
        .ok()
        .filter(|meta| meta.is_dir())
        .ok_or(())?;

    let pool = pool.0.lock().await;
    let insert_res = sqlx::query(
        "
INSERT INTO repository (
    name, local_path, created_at
) VALUES (
    ?, ?, ?
)
    ",
    )
    .bind(local_path.file_name().map(|name| name.to_str()).ok_or(())?)
    .bind(local_path.to_str())
    .bind(created_at)
    .execute(&*pool)
    .await
    .expect("Failed to create repository");

    // Set as the open repository
    sqlx::query("UPDATE state SET open_repository = ? WHERE id = 0")
        .bind(insert_res.last_insert_rowid())
        .execute(&*pool)
        .await
        .expect("Failed to update open repository");

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_repositories(pool: tauri::State<'_, DbPool>) -> Result<Vec<Repository>, ()> {
    let pool = pool.0.lock().await;
    let repositories = sqlx::query_as("SELECT * FROM repository")
        .fetch_all(&*pool)
        .await
        .expect("Failed to fetch repositories");

    Ok(repositories)
}
