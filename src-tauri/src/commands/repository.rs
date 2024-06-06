use serde::Serialize;
use specta::Type;
use sqlx::prelude::FromRow;
use tauri::State;

use crate::DbPool;

#[derive(Serialize, Type, FromRow)]
pub struct Repository {
    id: i64,
    folder_name: String,
    local_path: String,
    created_at: String,
    last_fetched_at: Option<String>,
}

#[tauri::command]
#[specta::specta]
pub async fn add_repository(
    pool: State<'_, DbPool>,
    path: String,
    created_at: String,
) -> Result<(), ()> {
    let pool = pool.0.lock().await;
    sqlx::query(
        "
INSERT INTO repositories (
    folder_name, local_path, created_at
) VALUES (
    ?, ?, ?
)
    ",
    )
    .bind(path.clone().split('/').last().unwrap())
    .bind(path)
    .bind(created_at)
    .execute(&*pool)
    .await
    .expect("Failed to create repository");

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_repositories(pool: State<'_, DbPool>) -> Result<Vec<Repository>, ()> {
    let pool = pool.0.lock().await;
    let repositories = sqlx::query_as("SELECT * FROM repositories")
        .fetch_all(&*pool)
        .await
        .expect("Failed to fetch repositories");

    Ok(repositories)
}
