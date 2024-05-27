// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use specta::{ts::ExportConfig, Type};
use tauri_specta::{collect_commands, collect_events, ts, Event};

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

#[derive(Debug, Clone, Serialize, Deserialize, Type, Event)]
struct GitCommandEvent(String);

fn main() {
    let (invoke_handler, register_events) = {
        let builder = ts::builder()
            .commands(collect_commands!(get_branches::get_branches))
            .events(collect_events!(GitCommandEvent))
            .config(ExportConfig::new().bigint(specta::ts::BigIntExportBehavior::Number));

        // Generate ts types
        #[cfg(debug_assertions)]
        let builder = builder.path("../src/bindings.ts");

        builder.build().unwrap()
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(invoke_handler)
        .setup(|app| {
            register_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
