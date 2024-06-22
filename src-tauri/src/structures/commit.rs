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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('\x00');
        Ok(Self {
            hash: parts.next().ok_or(())?.parse().map_err(|_err| ())?,
            parent_hashes: parts
                .next()
                .ok_or(())?
                .split_ascii_whitespace()
                .map(|hash| hash.parse::<GitHash>().map_err(|_err| ()))
                .collect::<Result<Vec<_>, Self::Err>>()?,
            author: parse_user(
                parts.next().ok_or(())?,
                parts.next().ok_or(())?,
                parts.next().ok_or(())?,
            )?,
            committer: parse_user(
                parts.next().ok_or(())?,
                parts.next().ok_or(())?,
                parts.next().ok_or(())?,
            )?,
            message: parts.next().ok_or(())?.into(),
            description: parts.next().or(Some("")).map(|s| s.into()),
        })
    }
}

fn parse_user(name: &str, email: &str, date: &str) -> Result<CommitUser, ()> {
    Ok(CommitUser {
        name: name.into(),
        email: email.into(),
        email_hash: {
            let mut email_hash = sha2::Sha256::new();
            email_hash.update(email.trim().to_lowercase());
            format!("{:x}", email_hash.finalize())
        },
        date: DateTime::from_timestamp(date.parse().map_err(|_err| ())?, 0)
            .ok_or(())?
            .naive_utc(),
    })
}
