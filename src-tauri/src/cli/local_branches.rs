use crate::{
    commands::{CommandError, CommandResult},
    structures::git::branch::LocalBranch,
};

use super::GitCommand;

const LOCAL_BRANCH_FIELDS: &[&str] = &[
    "HEAD",
    "refname:short",
    "upstream:short",
    "upstream:track,nobracket",
    "objectname",
];

pub async fn local_branches(app_handle: &tauri::AppHandle) -> CommandResult<Vec<LocalBranch>> {
    let format = GitCommand::create_format_arg(LOCAL_BRANCH_FIELDS);
    let branches = GitCommand::new("for-each-ref")
        .arg(format!("--format={format}"))
        .arg("refs/heads")
        .run(app_handle)
        .await?;
    branches
        .lines()
        .map(|line| line.parse().map_err(|_err| CommandError::Parse))
        .collect()
}
