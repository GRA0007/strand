use crate::{cli::GitCommand, db::GitCommandType, structures::hash::GitHash};

use super::CommandResult;

#[tauri::command]
#[specta::specta]
pub async fn get_file_diff(
    app_handle: tauri::AppHandle,
    commit_hash: GitHash,
    path: String,
) -> CommandResult<String> {
    let diff = GitCommand::new("diff")
        // .arg("--word-diff=porcelain")
        .arg(format!("{}^", commit_hash.0))
        .arg(commit_hash.0)
        .arg("--")
        .arg(path)
        .run(&app_handle, GitCommandType::Query)
        .await?;

    let diff = diff
        .lines()
        .skip_while(|line| !line.starts_with("@@"))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(diff)
}
