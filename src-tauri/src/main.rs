// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::create_dir_all, str::FromStr};

use cli::GitCommandEvent;
use db::Db;
use specta::ts::ExportConfig;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tauri::{Manager, RunEvent};
use tauri_specta::{collect_commands, collect_events, ts};

pub mod cli;
pub mod commands;
pub mod db;
pub mod structures;
pub mod utils;

fn main() {
    let (invoke_handler, register_events) = {
        let builder = ts::builder()
            .commands(collect_commands!(
                commands::get_branches::get_branches,
                commands::add_repository_from_path::add_repository_from_path,
                commands::set_open_repository::set_open_repository,
                commands::get_open_repository::get_open_repository,
                commands::get_repositories::get_repositories,
                commands::git_fetch::git_fetch,
                commands::get_git_command_log::get_git_command_log,
                commands::get_graph::get_graph,
                commands::get_commit_files::get_commit_files,
                commands::get_file_diff::get_file_diff,
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
                let pool = SqlitePool::connect_with(
                    SqliteConnectOptions::from_str(&db_url)
                        .unwrap()
                        .create_if_missing(true),
                )
                .await?;

                // Run migrations
                sqlx::migrate!().run(&pool).await?;

                // Load into app state and manage with Tauri
                let db = Db::new(pool);
                db.load().await?;
                app.manage(db);

                Ok(())
            })
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            if let RunEvent::Exit = event {
                // Close database connection
                tauri::async_runtime::block_on(async move {
                    let db: tauri::State<Db> = app.state();
                    db.pool.close().await;
                });
            }
        });
}
