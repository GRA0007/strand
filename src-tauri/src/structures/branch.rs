use std::str::FromStr;

use serde::Serialize;
use specta::Type;

use super::{hash::GitHash, upstream_track::UpstreamTrack};

#[derive(Debug, Serialize, Type)]
pub struct LocalBranch {
    pub head: bool,
    /// e.g. `["feat", "implement-stuff"]`
    pub name: Vec<String>,
    pub upstream_name: Vec<String>,
    pub upstream_track: UpstreamTrack,
    pub hash: GitHash,
}

#[derive(Debug, Serialize, Type)]
pub struct RemoteBranch {
    /// e.g. `["origin", "feat", "implement-stuff"]`
    pub name: Vec<String>,
    pub hash: GitHash,
}

impl FromStr for LocalBranch {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('\x00');
        Ok(Self {
            head: parts.next().ok_or(())? == "*",
            name: parts
                .next()
                .ok_or(())?
                .split('/')
                .map(|s| s.to_owned())
                .collect(),
            upstream_name: parts
                .next()
                .ok_or(())?
                .split('/')
                .map(|s| s.to_owned())
                .collect(),
            upstream_track: parts.next().ok_or(())?.parse().map_err(|_err| ())?,
            hash: parts.next().ok_or(())?.parse().map_err(|_err| ())?,
        })
    }
}

impl FromStr for RemoteBranch {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('\x00');
        Ok(Self {
            name: parts
                .next()
                .ok_or(())?
                .split('/')
                .map(|s| s.to_owned())
                .collect(),
            hash: parts.next().ok_or(())?.parse().map_err(|_err| ())?,
        })
    }
}
