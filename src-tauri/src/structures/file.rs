use std::str::FromStr;

use serde::Serialize;
use specta::Type;

use super::{file_status::FileStatus, hash::GitHash};

#[derive(Debug, Serialize, Type)]
pub struct File {
    /// None if status is addition or unmerged
    pub src_hash: Option<GitHash>,
    /// None if status is deletion, unmerged or "work tree out of sync with the index"
    pub dst_hash: Option<GitHash>,
    pub status: FileStatus,
    /// Optional similarity percentage (0..100) if status is copied, renamed, or modified
    pub score: Option<u8>,
    pub src_path: String,
    /// Optional destination path if status is copied or renamed
    pub dst_path: Option<String>,
}

impl FromStr for File {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ').skip(2);

        let src_hash = parse_hash(parts.next().ok_or(())?);
        let dst_hash = parse_hash(parts.next().ok_or(())?);

        let (status, paths) = parts.next().ok_or(())?.split_once('\x00').ok_or(())?;

        let (status, score) = status.split_at(1);

        let (src_path, dst_path) = match paths.split_once('\x00') {
            Some((src_path, dst_path)) => (src_path.into(), Some(dst_path.into())),
            None => (paths.into(), None),
        };

        Ok(Self {
            src_hash,
            dst_hash,
            status: status.chars().next().ok_or(())?.try_into()?,
            score: if score.is_empty() {
                None
            } else {
                Some(score.parse().map_err(|_err| ())?)
            },
            src_path,
            dst_path,
        })
    }
}

fn parse_hash(s: &str) -> Option<GitHash> {
    if s.trim_start_matches('0').is_empty() {
        None
    } else {
        Some(GitHash(s.into()))
    }
}
