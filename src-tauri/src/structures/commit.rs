use std::str::FromStr;

use serde::Serialize;
use sha2::Digest;
use specta::Type;

use super::hash::GitHash;

#[derive(Debug, Serialize, Type)]
pub struct CommitUser {
    pub name: String,
    pub email: String,
    pub date: String,
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
            author: {
                let name = parts.next().ok_or(())?.into();
                let email: String = parts.next().ok_or(())?.into();
                let mut email_hash = sha2::Sha256::new();
                email_hash.update(email.clone().trim().to_lowercase());
                CommitUser {
                    name,
                    email,
                    email_hash: format!("{:x}", email_hash.finalize()),
                    date: parts.next().ok_or(())?.into(),
                }
            },
            committer: {
                let name = parts.next().ok_or(())?.into();
                let email: String = parts.next().ok_or(())?.into();
                let mut email_hash = sha2::Sha256::new();
                email_hash.update(email.clone().trim().to_lowercase());
                CommitUser {
                    name,
                    email,
                    email_hash: format!("{:x}", email_hash.finalize()),
                    date: parts.next().ok_or(())?.into(),
                }
            },
            message: parts.next().ok_or(())?.into(),
            description: parts.next().or(Some("")).map(|s| s.into()),
        })
    }
}
