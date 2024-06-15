use crate::{cli::GitCommand, state::GitCommandType, structures::commit::Commit};

use super::{CommandError, CommandResult};

const LOG_FIELDS: &[&str] = &[
    "H",  // Commit hash
    "P",  // Parent hashes
    "an", // Author name
    "ae", // Author email
    "aD", // Author date
    "cn", // Committer name
    "ce", // Committer email
    "cD", // Committer date
    "s",  // Commit message
    "b",  // Commit description
];

#[tauri::command]
#[specta::specta]
pub async fn get_graph(app_handle: tauri::AppHandle) -> CommandResult<Vec<Commit>> {
    let format = GitCommand::create_format_arg(LOG_FIELDS, "%x00");
    let commits = GitCommand::new("log")
        .arg(format!("--format={format}"))
        .arg("--all")
        .run(&app_handle, GitCommandType::Query)
        .await?;
    commits
        .lines()
        .map(|line| line.parse().map_err(|_err| CommandError::Parse))
        .collect()
}