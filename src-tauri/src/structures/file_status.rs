use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type)]
#[repr(u8)]
pub enum FileStatus {
    /// Addition of a file
    Added = b'A',
    /// Copy of a file into a new one
    Copied = b'C',
    /// Deletion of a file
    Deleted = b'D',
    /// Modification of the contents or mode of a file
    Modified = b'M',
    /// Renaming of a file
    Renamed = b'R',
    /// Change in the type of the file (regular file, symbolic link or submodule)
    TypeChanged = b'T',
    /// File is unmerged (you must complete the merge before it can be committed)
    Unmerged = b'U',
    /// "Unknown" change type
    Unknown = b'X',
}

impl From<FileStatus> for char {
    fn from(value: FileStatus) -> Self {
        value as u8 as Self
    }
}

impl TryFrom<char> for FileStatus {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Added),
            'C' => Ok(Self::Copied),
            'D' => Ok(Self::Deleted),
            'M' => Ok(Self::Modified),
            'R' => Ok(Self::Renamed),
            'T' => Ok(Self::TypeChanged),
            'U' => Ok(Self::Unmerged),
            'X' => Ok(Self::Unknown),
            _ => Err(()),
        }
    }
}
