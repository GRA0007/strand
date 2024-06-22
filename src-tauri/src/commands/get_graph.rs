use crate::{cli::GitCommand, db::GitCommandType, structures::commit::Commit};

use super::{CommandError, CommandResult};

const LOG_FIELDS: &[&str] = &[
    "H",  // Commit hash
    "P",  // Parent hashes
    "an", // Author name
    "ae", // Author email
    "at", // Author date
    "cn", // Committer name
    "ce", // Committer email
    "ct", // Committer date
    "s",  // Commit message
    "b",  // Commit description
];

#[tauri::command]
#[specta::specta]
pub async fn get_graph(app_handle: tauri::AppHandle) -> CommandResult<Vec<Commit>> {
    let format = GitCommand::create_format_arg(LOG_FIELDS, "%x00");
    // TODO: kinda sus way of delimiting commits, investigate a cleaner solution
    let commits = GitCommand::new("log")
        .arg(format!("--format={format}\x01"))
        .arg("--all")
        .run(&app_handle, GitCommandType::Query)
        .await?;
    commits
        .trim()
        .trim_end_matches('\x01')
        .split("\x01\n")
        .map(|line| line.parse().map_err(|_err| CommandError::Parse))
        .collect()
}
