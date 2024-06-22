use serde::Serialize;
use specta::Type;

use crate::{
    cli::GitCommand,
    db::GitCommandType,
    structures::branch::{LocalBranch, RemoteBranch},
};

use super::{CommandError, CommandResult};

const LOCAL_BRANCH_FIELDS: &[&str] = &[
    "(HEAD)",
    "(refname:short)",
    "(upstream:short)",
    "(upstream:track,nobracket)",
    "(objectname)",
];
const REMOTE_BRANCH_FIELDS: &[&str] = &["(refname:short)", "(objectname)"];

async fn local_branches(app_handle: &tauri::AppHandle) -> CommandResult<Vec<LocalBranch>> {
    let format = GitCommand::create_format_arg(LOCAL_BRANCH_FIELDS, "%00");
    let branches = GitCommand::new("for-each-ref")
        .arg(format!("--format={format}"))
        .arg("refs/heads")
        .run(app_handle, GitCommandType::Query)
        .await?;
    branches
        .lines()
        .map(|line| line.parse().map_err(|_err| CommandError::Parse))
        .collect()
}

async fn remote_branches(app_handle: &tauri::AppHandle) -> CommandResult<Vec<RemoteBranch>> {
    let format = GitCommand::create_format_arg(REMOTE_BRANCH_FIELDS, "%00");
    let branches = GitCommand::new("for-each-ref")
        .arg(format!("--format={format}"))
        .arg("refs/remotes")
        .run(app_handle, GitCommandType::Query)
        .await?;
    let branches: CommandResult<Vec<RemoteBranch>> = branches
        .lines()
        .map(|line| line.parse().map_err(|_err| CommandError::Parse))
        .collect();
    Ok(branches?
        .into_iter()
        .filter(|branch| branch.name.last() != Some(&"HEAD".to_owned()))
        .collect())
}

#[derive(Debug, Serialize, Type)]
pub struct Branches {
    local: Vec<LocalBranch>,
    remote: Vec<RemoteBranch>,
}

#[tauri::command]
#[specta::specta]
pub async fn get_branches(app_handle: tauri::AppHandle) -> CommandResult<Branches> {
    Ok(Branches {
        local: local_branches(&app_handle).await?,
        remote: remote_branches(&app_handle).await?,
    })
}
