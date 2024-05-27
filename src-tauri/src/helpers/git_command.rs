use std::{ffi::OsStr, fs::canonicalize, process::Command};

use tauri_specta::Event;

use crate::GitCommandEvent;

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

    pub fn run(&self, app_handle: &tauri::AppHandle) -> String {
        let mut cmd = Command::new("git");
        cmd.arg(&self.command);
        for arg in self.args.iter() {
            cmd.arg(arg);
        }
        cmd.current_dir(canonicalize("../../stevent").unwrap());
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
