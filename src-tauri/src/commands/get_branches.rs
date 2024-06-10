use serde::Serialize;
use specta::Type;

use crate::{
    cli::{local_branches::local_branches, remote_branches::remote_branches},
    structures::git::branch::{LocalBranch, RemoteBranch},
};

use super::CommandResult;

#[derive(Debug, Serialize, Type)]
pub struct Branches {
    local: Vec<LocalBranch>,
    remote: Vec<RemoteBranch>,
}

#[tauri::command]
#[specta::specta]
pub async fn get_branches(app_handle: tauri::AppHandle) -> CommandResult<Branches> {
    Ok(Branches {
        local: local_branches(&app_handle).await?,
        remote: remote_branches(&app_handle).await?,
    })
}
