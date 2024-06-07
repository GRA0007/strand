use std::{ffi::OsStr, process::Command};

use sqlx::Row;
use tauri::Manager;
use tauri_specta::Event;

use crate::{DbPool, GitCommandEvent};

pub struct GitCommand {
    command: String,
    args: Vec<String>,
}

impl GitCommand {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.into(),
            args: vec![],
        }
    }

    pub fn arg(&mut self, arg: impl ToString) -> &mut Self {
        self.args.push(arg.to_string());
        self
    }

    pub fn arg_if(&mut self, arg: &str, condition: bool) -> &mut Self {
        if condition {
            self.arg(arg);
        }
        self
    }

    pub fn create_format_arg(fields: &[&str]) -> String {
        fields
            .iter()
            .map(|field| format!("%({field})"))
            .collect::<Vec<String>>()
            .join("%00")
    }

    pub async fn run(&self, app_handle: &tauri::AppHandle) -> String {
        let pool = app_handle.state::<DbPool>();
        let pool = pool.0.lock().await;
        let open_repository = sqlx::query(
            "
SELECT repository.local_path as local_path
    FROM state
    LEFT JOIN repository ON state.open_repository = repository.id
        ",
        )
        .fetch_one(&*pool)
        .await
        .unwrap();
        drop(pool);
        let local_path: &str = open_repository.try_get("local_path").unwrap();

        let mut cmd = Command::new("git");
        cmd.arg(&self.command);
        for arg in self.args.iter() {
            cmd.arg(arg);
        }
        cmd.current_dir(local_path);
        let output = cmd.output().unwrap();
        if !output.status.success() {
            panic!("Git command failed");
        }

        // Emit event
        let program = cmd.get_program();
        let args = cmd
            .get_args()
            .collect::<Vec<&OsStr>>()
            .join(OsStr::new(" "));
        GitCommandEvent(format!(
            "{} {}",
            program.to_str().unwrap(),
            args.to_str().unwrap()
        ))
        .emit(app_handle)
        .unwrap();

        String::from_utf8(output.stdout).unwrap()
    }
}
