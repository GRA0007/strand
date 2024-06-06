// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::create_dir_all;

use serde::{Deserialize, Serialize};
use specta::{ts::ExportConfig, Type};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use tauri::{async_runtime::Mutex, Manager};
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

pub struct DbPool(Mutex<Pool<Sqlite>>);

fn main() {
    let (invoke_handler, register_events) = {
        let builder = ts::builder()
            .commands(collect_commands!(
                get_branches::get_branches,
                repository::add_repository,
                repository::get_repositories,
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
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(invoke_handler)
        .setup(|app| {
            register_events(app);

            let app_path = app.path().app_config_dir().expect("No app path found");
            create_dir_all(&app_path).expect("Couldn't create app directory");
            let db_url = format!("sqlite:{}data.db", app_path.to_string_lossy());

            tauri::async_runtime::block_on(async move {
                // Create DB and connect
                if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
                    Sqlite::create_database(&db_url).await?;
                }
                let pool = SqlitePool::connect(&db_url).await?;

                // Run migrations
                sqlx::migrate!().run(&pool).await?;

                app.manage(DbPool(Mutex::new(pool)));

                Ok(())
            })
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
