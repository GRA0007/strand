use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime};
use serde::Serialize;
use sha2::Digest;
use specta::Type;

use super::hash::GitHash;

#[derive(Debug, Serialize, Type)]
pub struct CommitUser {
    pub name: String,
    pub email: String,
    pub date: NaiveDateTime,
    pub email_hash: String,
}

#[derive(Debug, Serialize, Type)]
pub struct Commit {
    pub hash: GitHash,
    pub parent_hashes: Vec<GitHash>,
    pub author: CommitUser,
    pub committer: CommitUser,
    pub message: String,
    pub description: Option<String>,
}

impl FromStr for Commit {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('\x00');
        Ok(Self {
            hash: parts.next().ok_or("Failed to get commit hash")?.parse()?,
            parent_hashes: parts
                .next()
                .ok_or("Failed to get commit parent hashes")?
                .split_ascii_whitespace()
                .map(|hash| hash.parse::<GitHash>())
                .collect::<Result<Vec<_>, Self::Err>>()?,
            author: parse_user(
                parts.next().ok_or("Failed to get commit author name")?,
                parts.next().ok_or("Failed to get commit author email")?,
                parts.next().ok_or("Failed to get commit author date")?,
            )?,
            committer: parse_user(
                parts.next().ok_or("Failed to get commit committer name")?,
                parts.next().ok_or("Failed to get commit committer email")?,
                parts.next().ok_or("Failed to get commit committer date")?,
            )?,
            message: parts.next().ok_or("Failed to get commit message")?.into(),
            description: parts.next().or(Some("")).map(|s| s.into()),
        })
    }
}

fn parse_user(name: &str, email: &str, date: &str) -> Result<CommitUser, String> {
    Ok(CommitUser {
        name: name.into(),
        email: email.into(),
        email_hash: {
            let mut email_hash = sha2::Sha256::new();
            email_hash.update(email.trim().to_lowercase());
            format!("{:x}", email_hash.finalize())
        },
        date: DateTime::from_timestamp(
            date.parse()
                .map_err(|err| format!("Failed to parse date: {}", err))?,
            0,
        )
        .ok_or("Failed to convert date to timestamp")?
        .naive_utc(),
    })
}
