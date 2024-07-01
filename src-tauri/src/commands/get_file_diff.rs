use crate::{
    cli::GitCommand,
    db::GitCommandType,
    structures::{file_diff::FileDiff, hash::GitHash},
};

use super::{CommandError, CommandResult};

#[tauri::command]
#[specta::specta]
pub async fn get_file_diff(
    app_handle: tauri::AppHandle,
    commit_hash: GitHash,
    path: String,
) -> CommandResult<FileDiff> {
    let diff = GitCommand::new("diff")
        .arg(format!("{}^", commit_hash.0))
        .arg(commit_hash.0)
        .arg("--")
        .arg(path)
        .run(&app_handle, GitCommandType::Query)
        .await?;

    // Remove diff header
    let diff = diff
        .lines()
        .skip_while(|line| !line.starts_with("@@"))
        .collect::<Vec<_>>()
        .join("\n");

    diff.parse().map_err(CommandError::Parse)
}
