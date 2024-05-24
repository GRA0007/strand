use std::str::FromStr;

use serde::Serialize;
use specta::Type;

use crate::{helpers::git_command::GitCommand, GitHash};

#[derive(Debug, Serialize, Type, Clone)]
enum UpstreamTrack {
    Delta(usize, usize),
    InSync,
    Gone,
}

impl FromStr for UpstreamTrack {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gone" => Ok(Self::Gone),
            "" => Ok(Self::InSync),
            v => {
                // TODO: clean this mess up
                if let Some((ahead, behind)) = v.split_once(", ") {
                    Ok(Self::Delta(
                        ahead.strip_prefix("ahead ").unwrap().parse().unwrap(),
                        behind.strip_prefix("behind ").unwrap().parse().unwrap(),
                    ))
                } else if let Some(ahead) = v.strip_prefix("ahead ").and_then(|v| v.parse().ok()) {
                    Ok(Self::Delta(ahead, 0))
                } else {
                    let behind = v
                        .strip_prefix("behind ")
                        .and_then(|v| v.parse().ok())
                        .unwrap();
                    Ok(Self::Delta(0, behind))
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

#[tauri::command]
#[specta::specta]
pub fn local_branches(app_handle: tauri::AppHandle) -> Vec<LocalBranch> {
    let format = GitCommand::create_format_arg(LOCAL_BRANCH_FIELDS);
    let branches = GitCommand::new("for-each-ref")
        .arg(format!("--format={format}"))
        .arg("refs/heads")
        .run(Some(app_handle));
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
