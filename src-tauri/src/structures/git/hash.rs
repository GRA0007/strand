use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type)]
pub struct GitHash(String);

impl TryFrom<String> for GitHash {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 40 {
            return Err(());
        }
        Ok(Self(value))
    }
}
