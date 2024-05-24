// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;

use helpers::git_command::GitCommand;
use serde::Serialize;
use specta::{collect_types, ts::ExportConfiguration, Type};
use tauri_specta::ts::Exporter;

pub mod helpers;

#[derive(Debug, Serialize, Type)]
struct GitHash(String);

impl TryFrom<String> for GitHash {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 40 {
            return Err(());
        }
        Ok(Self(value))
    }
}

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
struct LocalBranch {
    head: bool,
    /// e.g. `["feat", "implement-stuff"]`
    name: Vec<String>,
    upstream_name: Vec<String>,
    upstream_track: UpstreamTrack,
    hash: GitHash,
}

#[derive(Debug, Serialize, Type)]
struct RemoteBranch {
    /// e.g. `["origin", "feat", "implement-stuff"]`
    name: Vec<String>,
    hash: GitHash,
}

const LOCAL_BRANCH_FIELDS: &[&str] = &[
    "HEAD",
    "refname:short",
    "upstream:short",
    "upstream:track,nobracket",
    "objectname",
];

const REMOTE_BRANCH_FIELDS: &[&str] = &["refname:short", "objectname"];

#[tauri::command]
#[specta::specta]
fn local_branches() -> Vec<LocalBranch> {
    let format = GitCommand::create_format_arg(LOCAL_BRANCH_FIELDS);
    let branches = GitCommand::new("for-each-ref")
        .arg(format!("--format={format}"))
        .arg("refs/heads")
        .run();
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

#[tauri::command]
#[specta::specta]
fn remote_branches() -> Vec<RemoteBranch> {
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

fn main() {
    // Generate ts types
    #[cfg(debug_assertions)]
    Exporter::new(
        collect_types![local_branches, remote_branches],
        "../src/commands.ts",
    )
    .with_cfg(ExportConfiguration::new().bigint(specta::ts::BigIntExportBehavior::Number))
    .export()
    .unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![local_branches, remote_branches])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
