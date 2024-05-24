// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use specta::{collect_types, ts::ExportConfiguration, Type};
use tauri_specta::ts::Exporter;

use crate::commands::*;

pub mod commands;
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

fn main() {
    // Generate ts types
    #[cfg(debug_assertions)]
    Exporter::new(
        collect_types![
            local_branches::local_branches,
            remote_branches::remote_branches
        ],
        "../src/commands.ts",
    )
    .with_cfg(ExportConfiguration::new().bigint(specta::ts::BigIntExportBehavior::Number))
    .export()
    .unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            local_branches::local_branches,
            remote_branches::remote_branches
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
