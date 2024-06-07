// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::create_dir_all;

use serde::{Deserialize, Serialize};
use specta::{ts::ExportConfig, Type};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use state::StrandState;
use tauri::Manager;
use tauri_specta::{collect_commands, collect_events, ts, Event};

pub mod commands;
pub mod helpers;
pub mod state;

use crate::commands::*;

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
            .commands(collect_commands!(
                get_branches::get_branches,
                repository::add_repository_from_path,
                repository::set_open_repository,
                get_state::get_state_data,
            ))
            .events(collect_events!(GitCommandEvent))
            .config(ExportConfig::new().bigint(specta::ts::BigIntExportBehavior::Number));

        // Generate ts types
        #[cfg(debug_assertions)]
        let builder = builder.path("../src/bindings.ts");

        builder.build().unwrap()
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(invoke_handler)
        .setup(|app| {
            register_events(app);

            let app_path = app.path().app_config_dir().expect("No app path found");
            create_dir_all(&app_path).expect("Couldn't create app directory");
            let db_url = format!("sqlite:{}/data.db", app_path.to_string_lossy());

            tauri::async_runtime::block_on(async move {
                // Create DB and connect
                if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
                    Sqlite::create_database(&db_url).await?;
                }
                let pool = SqlitePool::connect(&db_url).await?;

                // Run migrations
                sqlx::migrate!().run(&pool).await?;

                // Load into app state and manage with Tauri
                let state = StrandState::new(pool);
                {
                    let mut inner_state = state.0.lock().await;
                    inner_state.load().await?;
                }
                app.manage(state);

                Ok(())
            })
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
