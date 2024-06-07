use std::str::FromStr;

use serde::Serialize;
use specta::Type;

use crate::{helpers::git_command::GitCommand, GitHash};

/// If both are 0, it's in sync. If None, the tracked upstream is missing.
#[derive(Debug, Serialize, Type, Clone)]
struct UpstreamTrack(Option<(usize, usize)>);

impl FromStr for UpstreamTrack {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gone" => Ok(Self(None)),
            "" => Ok(Self(Some((0, 0)))),
            v => {
                // TODO: clean this mess up
                if let Some((ahead, behind)) = v.split_once(", ") {
                    Ok(Self(Some((
                        ahead.strip_prefix("ahead ").unwrap().parse().unwrap(),
                        behind.strip_prefix("behind ").unwrap().parse().unwrap(),
                    ))))
                } else if let Some(ahead) = v.strip_prefix("ahead ").and_then(|v| v.parse().ok()) {
                    Ok(Self(Some((ahead, 0))))
                } else {
                    let behind = v
                        .strip_prefix("behind ")
                        .and_then(|v| v.parse().ok())
                        .unwrap();
                    Ok(Self(Some((0, behind))))
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Type)]
pub struct LocalBranch {
    head: bool,
    /// e.g. `["feat", "implement-stuff"]`
    name: Vec<String>,
    upstream_name: Vec<String>,
    upstream_track: UpstreamTrack,
    hash: GitHash,
}

const LOCAL_BRANCH_FIELDS: &[&str] = &[
    "HEAD",
    "refname:short",
    "upstream:short",
    "upstream:track,nobracket",
    "objectname",
];

async fn local_branches(app_handle: &tauri::AppHandle) -> Vec<LocalBranch> {
    let format = GitCommand::create_format_arg(LOCAL_BRANCH_FIELDS);
    let branches = GitCommand::new("for-each-ref")
        .arg(format!("--format={format}"))
        .arg("refs/heads")
        .run(app_handle)
        .await;
    branches
        .lines()
        .map(|line| {
            let mut parts = line.split('\x00');
            LocalBranch {
                head: parts.next().unwrap() == "*",
                name: parts
                    .next()
                    .unwrap()
                    .split('/')
                    .map(|s| s.to_owned())
                    .collect(),
                upstream_name: parts
                    .next()
                    .unwrap()
                    .split('/')
                    .map(|s| s.to_owned())
                    .collect(),
                upstream_track: parts.next().unwrap().parse().unwrap(),
                hash: parts.next().unwrap().to_string().try_into().unwrap(),
            }
        })
        .collect()
}

#[derive(Debug, Serialize, Type)]
pub struct RemoteBranch {
    /// e.g. `["origin", "feat", "implement-stuff"]`
    name: Vec<String>,
    hash: GitHash,
}

const REMOTE_BRANCH_FIELDS: &[&str] = &["refname:short", "objectname"];

async fn remote_branches(app_handle: &tauri::AppHandle) -> Vec<RemoteBranch> {
    let format = GitCommand::create_format_arg(REMOTE_BRANCH_FIELDS);
    let branches = GitCommand::new("for-each-ref")
        .arg(format!("--format={format}"))
        .arg("refs/remotes")
        .run(app_handle)
        .await;
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

#[derive(Debug, Serialize, Type)]
pub struct Branches {
    local: Vec<LocalBranch>,
    remote: Vec<RemoteBranch>,
}

#[tauri::command]
#[specta::specta]
pub async fn get_branches(app_handle: tauri::AppHandle) -> Branches {
    Branches {
        local: local_branches(&app_handle).await,
        remote: remote_branches(&app_handle).await,
    }
}
