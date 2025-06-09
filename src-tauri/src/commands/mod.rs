use thiserror::Error;

use crate::cli::GitError;

pub mod add_repository_from_path;
pub mod get_branches;
pub mod get_changed_files;
pub mod get_commit_files;
pub mod get_file_diff;
pub mod get_git_command_log;
pub mod get_graph;
pub mod get_open_repository;
pub mod get_repositories;
pub mod git_fetch;
pub mod set_open_repository;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error(transparent)]
    Git(#[from] GitError),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("failed to parse git output: {0}")]
    Parse(String),
    #[error("{0}")]
    Other(String),
}

impl serde::Serialize for CommandError {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl specta::Type for CommandError {
    fn inline(_type_map: &mut specta::TypeMap, _generics: specta::Generics) -> specta::DataType {
        specta::DataType::Primitive(specta::datatype::PrimitiveType::String)
    }
}

pub type CommandResult<T> = Result<T, CommandError>;
