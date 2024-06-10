use std::{ffi::OsStr, io, process::Command};

use specta::Type;
use tauri::Manager;
use tauri_specta::Event;
use thiserror::Error;

use crate::{state::StrandState, GitCommandEvent};

pub mod local_branches;
pub mod remote_branches;

#[derive(Error, Debug, Type)]
pub enum GitError {
    #[error(transparent)]
    Io(
        #[serde(skip)]
        #[from]
        io::Error,
    ),
    #[error("git command returned an error: {0}")]
    Unsuccessful(String),
    #[error("no repository open")]
    NoRepoOpen,
    #[error("not a valid repository")]
    NotARepository,
}

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

    pub async fn run(&self, app_handle: &tauri::AppHandle) -> Result<String, GitError> {
        let state = app_handle.state::<StrandState>();
        let local_path = state
            .data
            .lock()
            .await
            .open_repository
            .as_ref()
            .map(|repo| repo.local_path.clone())
            .ok_or(GitError::NoRepoOpen)?;

        let mut cmd = Command::new("git");
        cmd.arg(&self.command);
        for arg in self.args.iter() {
            cmd.arg(arg);
        }
        cmd.current_dir(local_path);
        let output = cmd.output()?;
        if !output.status.success() {
            return Err(GitError::Unsuccessful(
                String::from_utf8(output.stderr).expect("Failed to parse error as utf8"),
            ));
        }

        // Emit event
        let program = cmd.get_program();
        let args = cmd
            .get_args()
            .collect::<Vec<&OsStr>>()
            .join(OsStr::new(" "));
        GitCommandEvent(format!(
            "{} {}",
            program.to_string_lossy(),
            args.to_string_lossy()
        ))
        .emit(app_handle)
        .expect("Failed to emit event");

        Ok(String::from_utf8(output.stdout).expect("Failed to parse as utf8"))
    }
}
