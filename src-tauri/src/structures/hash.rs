use std::str::FromStr;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct GitHash(pub String);

impl FromStr for GitHash {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 40 {
            return Err(());
        }
        Ok(Self(s.into()))
    }
}
