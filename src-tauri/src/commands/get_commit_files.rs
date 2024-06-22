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
    let files = GitCommand::new("diff")
        .arg(format!("{}^ {}", commit_hash.0, commit_hash.0))
        .arg("--raw")
        .arg("--abbrev=40")
        .arg("-z")
        .run(&app_handle, GitCommandType::Query)
        .await?;

    let files = files
        .strip_prefix(':')
        .ok_or(CommandError::Parse)?
        .strip_suffix('\x00')
        .ok_or(CommandError::Parse)?
        .split("\x00:");

    files
        .map(|line| line.parse().map_err(|_err| CommandError::Parse))
        .collect()
}
