use crate::{
    cli::GitCommand,
    db::GitCommandType,
    structures::{file_diff::FileDiff, file_diff_meta::FileDiffMeta, hash::GitHash},
};

use super::{CommandError, CommandResult};

#[tauri::command]
#[specta::specta]
pub async fn get_file_diff(
    app_handle: tauri::AppHandle,
    commit_hash: GitHash,
    path: String,
) -> CommandResult<FileDiff> {
    let diff_meta: FileDiffMeta = GitCommand::new("diff")
        .arg("--abbrev=40")
        .arg(format!("{}^", commit_hash.0))
        .arg(commit_hash.0)
        .arg("--")
        .arg(path)
        .run(&app_handle, GitCommandType::Query)
        .await?
        .parse()
        .map_err(CommandError::Parse)?;

    let src_file = match &diff_meta.src_hash {
        Some(hash) => Some(
            GitCommand::new("cat-file")
                .arg("blob")
                .arg(hash.0.clone())
                .run(&app_handle, GitCommandType::Query)
                .await?,
        ),
        None => None,
    };
    let dst_file = match &diff_meta.dst_hash {
        Some(hash) => Some(
            GitCommand::new("cat-file")
                .arg("blob")
                .arg(hash.0.clone())
                .run(&app_handle, GitCommandType::Query)
                .await?,
        ),
        None => None,
    };

    FileDiff::from(diff_meta, src_file, dst_file).map_err(CommandError::Parse)
}
