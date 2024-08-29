use std::path::PathBuf;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::{prelude::FromRow, sqlite::SqliteRow, Pool, QueryBuilder, Row, Sqlite};
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

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
#[repr(i64)]
pub enum GitCommandType {
    Query,
    Mutation,
}

impl From<i64> for GitCommandType {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Query,
            1 => Self::Mutation,
            _ => panic!("Not a valid git command log type"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Type, Debug)]
pub struct GitCommandLog {
    pub id: i64,
    pub command: String,
    pub command_type: GitCommandType,
    pub created_at: NaiveDateTime,
}

impl<'r> FromRow<'r, SqliteRow> for GitCommandLog {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            command: row.try_get("command")?,
            command_type: row.try_get::<i64, _>("command_type")?.into(),
            created_at: row.try_get("created_at")?,
        })
    }
}

#[derive(Serialize, Clone, Default, Type)]
pub struct State {
    pub open_repository: Option<Repository>,
}

pub struct Db {
    pub pool: Pool<Sqlite>,
    /// In-memory state
    pub state: Mutex<State>,
}

impl Db {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            pool,
            state: Default::default(),
        }
    }

    /// Load in-memory state from the DB
    pub async fn load(&self) -> Result<(), sqlx::Error> {
        let mut state = self.state.lock().await;

        state.open_repository = sqlx::query_as!(Repository, "SELECT * FROM repository WHERE id IN (SELECT open_repository_id FROM state WHERE id = 0)")
            .fetch_one(&self.pool)
            .await
            .ok();

        Ok(())
    }

    pub async fn add_repository(&self, local_path: PathBuf) -> Result<Repository, sqlx::Error> {
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

        let repository = sqlx::query_as!(Repository, "SELECT * FROM repository WHERE id = ?", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(repository)
    }

    pub async fn get_repositories(&self) -> Result<Vec<Repository>, sqlx::Error> {
        sqlx::query_as!(Repository, "SELECT * FROM repository")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn set_open_repository(&self, id: Option<i64>) -> Result<(), sqlx::Error> {
        let mut state = self.state.lock().await;

        // Repository is already set
        if match state.open_repository.as_ref() {
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

        state.open_repository = match id {
            Some(id) => Some(
                sqlx::query_as!(Repository, "SELECT * FROM repository WHERE id = ?", id)
                    .fetch_one(&self.pool)
                    .await?,
            ),
            None => None,
        };

        Ok(())
    }

    /// Adds a git command log to the currently open repository
    pub async fn add_git_command_log(
        &self,
        command: String,
        command_type: GitCommandType,
    ) -> Result<GitCommandLog, sqlx::Error> {
        let state = self.state.lock().await;

        match &state.open_repository {
            Some(open_repository) => {
                let command_type = command_type as i64;
                let id = sqlx::query!(
                    "INSERT INTO git_command_log (command, command_type, repository_id) VALUES (?, ?, ?)",
                    command,
                    command_type,
                    open_repository.id
                )
                .execute(&self.pool)
                .await?
                .last_insert_rowid();

                // Limit the number of logs of each type stored in the database
                let count = sqlx::query!(
                    "SELECT COUNT(*) as count FROM git_command_log WHERE command_type = ?",
                    command_type
                )
                .fetch_one(&self.pool)
                .await?
                .count;

                // TODO: Use a configurable value for the maximum
                if count > 200 {
                    let amount_to_delete = count - 200;
                    sqlx::query!(
                        "DELETE FROM git_command_log WHERE id IN (SELECT id FROM git_command_log WHERE command_type = ? ORDER BY created_at ASC LIMIT ?)",
                        command_type,
                        amount_to_delete
                    )
                    .execute(&self.pool)
                    .await?;
                }

                sqlx::query_as!(
                    GitCommandLog,
                    "SELECT id, command, command_type, created_at FROM git_command_log WHERE id = ?",
                    id
                )
                .fetch_one(&self.pool)
                .await
            }
            None => panic!("TODO: handle no open repo"),
        }
    }

    /// Get the full git command log history for the open repository
    pub async fn get_git_command_log(
        &self,
        filter: Option<GitCommandType>,
    ) -> Result<Vec<GitCommandLog>, sqlx::Error> {
        let state = self.state.lock().await;

        match &state.open_repository.as_ref().map(|r| r.id) {
            Some(open_repository_id) => {
                // TODO: sort by date, and paginate
                let mut query = QueryBuilder::new("SELECT id, command, command_type, created_at FROM git_command_log WHERE repository_id = ");
                query.push_bind(open_repository_id);
                if let Some(command_type) = filter {
                    query.push(" AND command_type = ");
                    query.push_bind(command_type as i64);
                }
                query.build_query_as().fetch_all(&self.pool).await
            }
            None => panic!("TODO: handle no open repo"),
        }
    }
}
