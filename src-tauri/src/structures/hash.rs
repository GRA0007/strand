use std::str::FromStr;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct GitHash(pub String);

impl FromStr for GitHash {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 40 {
            return Err("Hash is not 40 characters long".into());
        }
        Ok(Self(s.into()))
    }
}
