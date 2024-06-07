use std::path::PathBuf;

use chrono::NaiveDateTime;
use serde::Serialize;
use specta::Type;
use sqlx::{prelude::FromRow, Pool, Sqlite};
use tokio::sync::Mutex;

#[derive(FromRow, Serialize, Clone, Type)]
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

pub struct InnerState {
    pool: Pool<Sqlite>,
    pub data: StrandData,
}

pub struct StrandState(pub Mutex<InnerState>);

impl StrandState {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self(Mutex::new(InnerState {
            pool,
            data: Default::default(),
        }))
    }
}

impl InnerState {
    pub async fn load(&mut self) -> Result<(), sqlx::Error> {
        self.data.repositories = sqlx::query_as("SELECT * FROM repository")
            .fetch_all(&self.pool)
            .await?;

        self.data.open_repository = sqlx::query_as(
            "
SELECT repository FROM state
LEFT JOIN repository ON state.open_repository_id = repository.id
WHERE id = 0
            ",
        )
        .fetch_one(&self.pool)
        .await
        .ok();

        Ok(())
    }

    pub async fn add_repository(&mut self, local_path: PathBuf) -> Result<Repository, sqlx::Error> {
        let insert_res = sqlx::query("INSERT INTO repository (name, local_path) VALUES (?, ?)")
            .bind(
                local_path
                    .file_name()
                    .map(|name| name.to_str())
                    .expect("Couldn't get folder name from path"),
            )
            .bind(local_path.to_str())
            .execute(&self.pool)
            .await?;

        let repository: Repository = sqlx::query_as("SELECT * FROM repository WHERE id = ?")
            .bind(insert_res.last_insert_rowid())
            .fetch_one(&self.pool)
            .await?;

        self.data.repositories.push(repository.clone());

        Ok(repository)
    }

    pub async fn set_open_repository(&mut self, id: Option<i64>) -> Result<(), sqlx::Error> {
        // Repository is already set
        if match self.data.open_repository.as_ref() {
            Some(repo) => id.is_some_and(|id| id == repo.id),
            None => id.is_none(),
        } {
            return Ok(());
        }

        sqlx::query("UPDATE state SET open_repository_id = ? WHERE id = 0")
            .bind(id)
            .execute(&self.pool)
            .await?;

        // Update last_opened_at of opened repo
        if let Some(id) = id {
            sqlx::query("UPDATE repository SET last_opened_at = CURRENT_TIMESTAMP WHERE id = ?")
                .bind(id)
                .execute(&self.pool)
                .await?;
        }

        self.data.open_repository = match id {
            Some(id) => Some(
                sqlx::query_as("SELECT * FROM repository WHERE id = ?")
                    .bind(id)
                    .fetch_one(&self.pool)
                    .await?,
            ),
            None => None,
        };

        Ok(())
    }
}
