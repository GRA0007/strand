use std::str::FromStr;

use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type, Clone)]
pub enum DiffStatus {
    Added,
    Removed,
    Unmodified,
}

#[derive(Debug, Serialize, Type, Clone)]
pub struct WordDiff {
    pub text: String,
    pub status: DiffStatus,
}

#[derive(Debug, Serialize, Type)]
pub struct LineDiff {
    pub words: Vec<WordDiff>,
    pub status: DiffStatus,
    /// None if status is Added
    pub src_line_number: Option<usize>,
    /// None if status is Removed
    pub dst_line_number: Option<usize>,
}

#[derive(Debug, Serialize, Type)]
pub struct DiffHunk {
    /// Raw header text
    pub header: String,
    pub lines: Vec<LineDiff>,
}

#[derive(Debug, Serialize, Type)]
pub struct FileDiff(pub Vec<DiffHunk>);

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

impl FromStr for DiffHunk {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let header: String = lines.next().ok_or("Failed to get header")?.into();
        let (mut src_line_number, mut dst_line_number) = parse_line_numbers(&header)?;

        let lines = lines
            .map(|line| {
                let (status, text) = line.split_at(1);

                let status: DiffStatus = status
                    .chars()
                    .next()
                    .ok_or("Diff status char")?
                    .try_into()?;

                let line = LineDiff {
                    words: vec![WordDiff {
                        status: DiffStatus::Unmodified,
                        text: text.into(),
                    }],
                    status: status.clone(),
                    src_line_number: match status {
                        DiffStatus::Added => None,
                        _ => Some(src_line_number),
                    },
                    dst_line_number: match status {
                        DiffStatus::Removed => None,
                        _ => Some(dst_line_number),
                    },
                };

                match status {
                    DiffStatus::Added => dst_line_number += 1,
                    DiffStatus::Removed => src_line_number += 1,
                    DiffStatus::Unmodified => {
                        src_line_number += 1;
                        dst_line_number += 1;
                    }
                };

                Ok(line)
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(Self { header, lines })
    }
}

impl FromStr for FileDiff {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hunks = s
            .split("\n@@")
            .enumerate()
            .map(|(i, hunk)| {
                if i == 0 {
                    hunk.into()
                } else {
                    format!("@@{hunk}")
                }
            })
            .map(|hunk| hunk.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self(hunks))
    }
}

/// Get the src and dst line numbers from a hunk header
fn parse_line_numbers(header: &str) -> Result<(usize, usize), String> {
    let numbers = header
        .trim_start_matches("@@ ")
        .split_once(" @@")
        .ok_or("Failed to split on @@ in diff header")?
        .0;
    let (src, dst) = numbers
        .split_once(' ')
        .ok_or("Failed to split src and dst in diff header")?;
    let src_line_number = src
        .trim_start_matches('-')
        .split_once(',')
        .ok_or("Failed to split src in diff header")?
        .0
        .parse()
        .map_err(|err| format!("Couldn't parse src_line_number {}", err))?;
    let dst_line_number = dst
        .trim_start_matches('+')
        .split_once(',')
        .ok_or("Failed to split dst in diff header")?
        .0
        .parse()
        .map_err(|err| format!("Couldn't parse dst_line_number {}", err))?;
    Ok((src_line_number, dst_line_number))
}
