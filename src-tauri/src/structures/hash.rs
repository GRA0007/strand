use std::str::FromStr;

use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type)]
pub struct GitHash(String);

impl FromStr for GitHash {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 40 {
            return Err(());
        }
        Ok(Self(s.into()))
    }
}
