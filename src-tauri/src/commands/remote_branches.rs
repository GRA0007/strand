use serde::Serialize;
use specta::Type;

use crate::{helpers::git_command::GitCommand, GitHash};

#[derive(Debug, Serialize, Type)]
pub struct RemoteBranch {
    /// e.g. `["origin", "feat", "implement-stuff"]`
    name: Vec<String>,
    hash: GitHash,
}

const REMOTE_BRANCH_FIELDS: &[&str] = &["refname:short", "objectname"];

#[tauri::command]
#[specta::specta]
pub fn remote_branches() -> Vec<RemoteBranch> {
    let format = GitCommand::create_format_arg(REMOTE_BRANCH_FIELDS);
    let branches = GitCommand::new("for-each-ref")
        .arg(format!("--format={format}"))
        .arg("refs/remotes")
        .run();
    branches
        .lines()
        .map(|line| {
            let mut parts = line.split('\x00');
            RemoteBranch {
                name: parts
                    .next()
                    .unwrap()
                    .split('/')
                    .map(|s| s.to_owned())
                    .collect(),
                hash: parts.next().unwrap().to_string().try_into().unwrap(),
            }
        })
        .filter(|branch| branch.name.last() != Some(&"HEAD".to_owned()))
        .collect()
}
