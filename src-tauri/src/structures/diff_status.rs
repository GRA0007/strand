use serde::Serialize;
use similar::ChangeTag;
use specta::Type;

#[derive(Debug, Serialize, Type, Clone, Eq, PartialEq)]
pub enum DiffStatus {
    Added,
    Removed,
    Unmodified,
}

impl TryFrom<char> for DiffStatus {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Added),
            '-' => Ok(Self::Removed),
            ' ' => Ok(Self::Unmodified),
            _ => Err("Invalid diff status".into()),
        }
    }
}

impl From<ChangeTag> for DiffStatus {
    fn from(value: ChangeTag) -> Self {
        match value {
            ChangeTag::Equal => Self::Unmodified,
            ChangeTag::Delete => Self::Removed,
            ChangeTag::Insert => Self::Added,
        }
    }
}
