use specta::Type;
use thiserror::Error;

use crate::cli::GitError;

pub mod add_repository_from_path;
pub mod get_branches;
pub mod get_commit_files;
pub mod get_git_command_log;
pub mod get_graph;
pub mod get_open_repository;
pub mod get_repositories;
pub mod git_fetch;
pub mod set_open_repository;

#[derive(Error, Debug, Type)]
pub enum CommandError {
    #[error(transparent)]
    Git(#[from] GitError),
    #[error(transparent)]
    Sqlx(
        #[serde(skip)]
        #[from]
        sqlx::Error,
    ),
    #[error("failed to parse git output")]
    Parse,
    #[error("{0}")]
    Other(String),
}

impl serde::Serialize for CommandError {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type CommandResult<T> = Result<T, CommandError>;
