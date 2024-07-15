use crate::{
    cli::GitCommand,
    db::GitCommandType,
    structures::{file::File, hash::GitHash},
};

use super::{CommandError, CommandResult};

#[tauri::command]
#[specta::specta]
pub async fn get_commit_files(
    app_handle: tauri::AppHandle,
    commit_hash: GitHash,
) -> CommandResult<Vec<File>> {
    let files = GitCommand::new("show")
        .arg(commit_hash.0)
        .arg("--raw") // Show a summary of changes without the full diffs
        .arg("--abbrev=40") // Show full git hashes
        .arg("-z") // Separate commits with NULs for parsing
        .arg("--diff-merges=1") // Show merge commits in the same format as regular commits
        .arg("--format=") // Disable the commit info so we only get the files
        .run(&app_handle, GitCommandType::Query)
        .await?;

    let files = files
        .strip_prefix(':')
        .ok_or(CommandError::Parse("Failed to strip files prefix".into()))?
        .strip_suffix('\x00')
        .ok_or(CommandError::Parse("Failed to strip files suffix".into()))?
        .split("\x00:");

    files
        .map(|line| line.parse().map_err(CommandError::Parse))
        .collect()
}
