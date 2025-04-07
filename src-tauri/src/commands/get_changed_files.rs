use crate::{
    cli::GitCommand,
    db::GitCommandType,
    structures::{file::File, file_status::FileStatus, hash::GitHash},
};

use super::{CommandError, CommandResult};

#[tauri::command]
#[specta::specta]
pub async fn get_changed_files(
    app_handle: tauri::AppHandle,
) -> CommandResult<(Vec<File>, Vec<File>)> {
    let items = GitCommand::new("status")
        .arg("-z") // Separate commits with NULs for parsing
        .arg("--porcelain=2") // Use stable parsable porcelain format
        .run(&app_handle, GitCommandType::Query)
        .await?;

    let items = items
        .strip_suffix('\x00')
        .ok_or(CommandError::Parse("Failed to strip files suffix".into()))?
        .split('\x00');

    let mut staged_files = Vec::new();
    let mut unstaged_files = Vec::new();
    for item in items {
        let (line_type, item) = item
            .split_once(' ')
            .ok_or(CommandError::Parse("Failed to split item".into()))?;

        match line_type {
            "1" | "2" | "u" => {
                let (status, item) = item
                    .split_once(' ')
                    .ok_or(CommandError::Parse("Failed to split item".into()))?;
                let status: Vec<Option<FileStatus>> = status
                    .chars()
                    .map(|l| match l {
                        '.' => None,
                        x => x.try_into().ok(),
                    })
                    .collect();

                let mut parts = item.split_ascii_whitespace().skip(4);
                if line_type == "u" {
                    parts.next();
                }
                let head_hash = parts
                    .next()
                    .ok_or(CommandError::Parse("Failed to get head hash".into()))?;
                let index_hash = parts
                    .next()
                    .ok_or(CommandError::Parse("Failed to get index hash".into()))?;
                let (score, dst_path, path) = match line_type {
                    "1" => (
                        None,
                        None,
                        parts
                            .next()
                            .ok_or(CommandError::Parse("Failed to get path for type 1".into()))?,
                    ),
                    "2" => (
                        parts
                            .next()
                            .ok_or(CommandError::Parse("Failed to score for type 2".into()))?
                            .split_at(1)
                            .1
                            .parse()
                            .ok(),
                        Some(parts.next().ok_or(CommandError::Parse(
                            "Failed to get dst_path for type 2".into(),
                        ))?),
                        parts
                            .next()
                            .ok_or(CommandError::Parse("Failed to get path for type 2".into()))?,
                    ),
                    "u" => (
                        None,
                        None,
                        parts
                            .nth(1)
                            .ok_or(CommandError::Parse("Failed to get path for type u".into()))?,
                    ),
                    _ => unreachable!(),
                };

                if let Some(staged_status) = &status[0] {
                    staged_files.push(File {
                        src_hash: GitHash::from_optional(head_hash).map_err(|_err| {
                            CommandError::Parse("Failed to parse head hash".into())
                        })?,
                        dst_hash: GitHash::from_optional(index_hash).map_err(|_err| {
                            CommandError::Parse("Failed to parse index hash".into())
                        })?,
                        status: *staged_status,
                        score,
                        src_path: path.into(),
                        dst_path: dst_path.map(|s| s.into()),
                    });
                }

                if let Some(unstaged_status) = &status[1] {
                    unstaged_files.push(File {
                        src_hash: GitHash::from_optional(head_hash).map_err(|_err| {
                            CommandError::Parse("Failed to parse head hash".into())
                        })?,
                        dst_hash: GitHash::from_optional(index_hash).map_err(|_err| {
                            CommandError::Parse("Failed to parse index hash".into())
                        })?,
                        status: *unstaged_status,
                        score,
                        src_path: path.into(),
                        dst_path: dst_path.map(|s| s.into()),
                    });
                }
            }
            // Untracked
            "?" => unstaged_files.push(File {
                src_hash: None,
                dst_hash: None,
                status: FileStatus::Added,
                score: None,
                src_path: item.into(),
                dst_path: None,
            }),
            // Ignore unknown line types
            _ => {}
        }
    }

    Ok((unstaged_files, staged_files))
}
