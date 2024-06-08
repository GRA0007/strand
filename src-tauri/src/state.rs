use std::path::PathBuf;

use chrono::NaiveDateTime;
use serde::Serialize;
use specta::Type;
use sqlx::{Pool, Sqlite};
use tokio::sync::Mutex;

#[derive(Serialize, Clone, Type, Debug)]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub local_path: String,
    pub created_at: NaiveDateTime,
    pub last_opened_at: Option<NaiveDateTime>,
    pub last_fetched_at: Option<NaiveDateTime>,
    pub has_changes: bool,
}

#[derive(Serialize, Clone, Default, Type)]
pub struct StrandData {
    pub repositories: Vec<Repository>,
    pub open_repository: Option<Repository>,
}

pub struct StrandState {
    pub pool: Pool<Sqlite>,
    pub data: Mutex<StrandData>,
}

impl StrandState {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            pool,
            data: Default::default(),
        }
    }

    pub async fn load(&self) -> Result<(), sqlx::Error> {
        let mut data = self.data.lock().await;

        data.repositories = sqlx::query_as!(Repository, "SELECT * FROM repository")
            .fetch_all(&self.pool)
            .await?;

        data.open_repository = sqlx::query_as!(Repository, "SELECT * FROM repository WHERE id IN (SELECT open_repository_id FROM state WHERE id = 0)")
            .fetch_one(&self.pool)
            .await
            .ok();

        Ok(())
    }

    pub async fn add_repository(&self, local_path: PathBuf) -> Result<Repository, sqlx::Error> {
        let mut data = self.data.lock().await;

        let name = local_path
            .file_name()
            .map(|name| name.to_str())
            .expect("Couldn't get folder name from path");
        let local_path = local_path.to_str();

        let id = sqlx::query!(
            "INSERT INTO repository (name, local_path) VALUES (?, ?)",
            name,
            local_path
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        let repository: Repository =
            sqlx::query_as!(Repository, "SELECT * FROM repository WHERE id = ?", id)
                .fetch_one(&self.pool)
                .await?;

        data.repositories.push(repository.clone());

        Ok(repository)
    }

    pub async fn set_open_repository(&self, id: Option<i64>) -> Result<(), sqlx::Error> {
        let mut data = self.data.lock().await;

        // Repository is already set
        if match data.open_repository.as_ref() {
            Some(repo) => id.is_some_and(|id| id == repo.id),
            None => id.is_none(),
        } {
            return Ok(());
        }

        sqlx::query!("UPDATE state SET open_repository_id = ? WHERE id = 0", id)
            .execute(&self.pool)
            .await?;

        // Update last_opened_at of opened repo
        if let Some(id) = id {
            sqlx::query!(
                "UPDATE repository SET last_opened_at = CURRENT_TIMESTAMP WHERE id = ?",
                id
            )
            .execute(&self.pool)
            .await?;
        }

        data.open_repository = match id {
            Some(id) => Some(
                sqlx::query_as!(Repository, "SELECT * FROM repository WHERE id = ?", id)
                    .fetch_one(&self.pool)
                    .await?,
            ),
            None => None,
        };

        Ok(())
    }
}
