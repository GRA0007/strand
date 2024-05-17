// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{io::BufRead, process::Command};

use serde::Serialize;
use specta::{collect_types, Type};
use tauri_specta::ts;

#[derive(Debug, Serialize, Type)]
struct Branch {
    name: String,
    remote: Option<String>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
#[specta::specta]
fn list_branches() -> Vec<Branch> {
    let out = Command::new("git")
        .args(["branch", "-a", "--format=%(refname)"])
        .output()
        .unwrap();
    out.stdout
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (name, remote) = if line.starts_with("refs/remotes/") {
                let (remote, name) = line.split_at(13).1.split_once('/').unwrap();
                (name.to_string(), Some(remote.to_string()))
            } else {
                (line.split_at(11).1.to_string(), None)
            };
            Branch { name, remote }
        })
        .collect()
}

fn main() {
    // Generate ts types
    #[cfg(debug_assertions)]
    ts::export(collect_types![list_branches], "../src/commands.ts").unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_branches])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
