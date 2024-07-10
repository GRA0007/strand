use std::str::FromStr;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type, Eq, PartialEq, Clone)]
pub struct GitHash(pub String);

impl FromStr for GitHash {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 40 {
            return Err(format!("Hash is not 40 characters long: {s}"));
        }
        Ok(Self(s.into()))
    }
}

impl GitHash {
    /// Attempt to parse a git hash that may be all zeroes, in which case,
    /// returns None
    pub fn from_optional(s: &str) -> Result<Option<Self>, String> {
        Ok(if s.trim_start_matches('0').is_empty() {
            None
        } else {
            Some(s.parse()?)
        })
    }
}
