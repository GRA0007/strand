use serde::Serialize;
use specta::Type;
use sqlx::prelude::FromRow;

use crate::DbPool;

#[derive(Serialize, Type, FromRow)]
pub struct State {
    id: i64,
    open_repository: Option<i64>,
}

#[tauri::command]
#[specta::specta]
pub async fn get_state(pool: tauri::State<'_, DbPool>) -> Result<State, ()> {
    let pool = pool.0.lock().await;
    let state = sqlx::query_as("SELECT * FROM state")
        .fetch_one(&*pool)
        .await
        .expect("Failed to fetch state");

    Ok(state)
}

#[tauri::command]
#[specta::specta]
pub async fn set_open_repository(
    pool: tauri::State<'_, DbPool>,
    open_repository: i64,
) -> Result<(), ()> {
    let pool = pool.0.lock().await;
    sqlx::query("UPDATE state SET open_repository = ? WHERE id = 0")
        .bind(open_repository)
        .execute(&*pool)
        .await
        .expect("Failed to update open repository");

    Ok(())
}
