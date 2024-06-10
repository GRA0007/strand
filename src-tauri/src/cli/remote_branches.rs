use crate::{
    commands::{CommandError, CommandResult},
    structures::git::branch::RemoteBranch,
};

use super::GitCommand;

const REMOTE_BRANCH_FIELDS: &[&str] = &["refname:short", "objectname"];

pub async fn remote_branches(app_handle: &tauri::AppHandle) -> CommandResult<Vec<RemoteBranch>> {
    let format = GitCommand::create_format_arg(REMOTE_BRANCH_FIELDS);
    let branches = GitCommand::new("for-each-ref")
        .arg(format!("--format={format}"))
        .arg("refs/remotes")
        .run(app_handle)
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
