use std::str::FromStr;

use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type)]
pub enum DiffStatus {
    Added,
    Removed,
    Unmodified,
}

#[derive(Debug, Serialize, Type)]
pub struct WordDiff {
    pub text: String,
    pub status: DiffStatus,
}

#[derive(Debug, Serialize, Type)]
pub struct DiffHunk {
    /// Raw header text
    pub header: String,
    pub src_line: usize,
    pub src_count: usize,
    pub dst_line: usize,
    pub dst_count: usize,

    pub lines: Vec<Vec<WordDiff>>,
}

#[derive(Debug, Serialize, Type)]
pub struct FileDiff(pub Vec<DiffHunk>);

impl TryFrom<char> for DiffStatus {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Added),
            '-' => Ok(Self::Removed),
            ' ' => Ok(Self::Unmodified),
            _ => Err(()),
        }
    }
}

impl FromStr for WordDiff {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (status, text) = s.split_at(1);
        let status = status.chars().next().ok_or(())?.try_into()?;
        let text = text.into();

        Ok(Self { status, text })
    }
}

impl FromStr for DiffHunk {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let header: String = lines.next().ok_or(())?.into();
        let (src_line, src_count, dst_line, dst_count) = {
            let numbers = header
                .trim_start_matches("@@ ")
                .split_once(" @@")
                .ok_or(())?
                .0;
            let (src, dst) = numbers.split_once(' ').ok_or(())?;
            let (src_line, src_count) = src.trim_start_matches('-').split_once(',').ok_or(())?;
            let (dst_line, dst_count) = dst.trim_start_matches('+').split_once(',').ok_or(())?;
            (
                src_line.parse().map_err(|_err| ())?,
                src_count.parse().map_err(|_err| ())?,
                dst_line.parse().map_err(|_err| ())?,
                dst_count.parse().map_err(|_err| ())?,
            )
        };

        let mut diff_lines = vec![Vec::new()];
        for line in lines {
            match line {
                "~" => diff_lines.push(Vec::new()),
                line => diff_lines.last_mut().ok_or(())?.push(line.parse()?),
            }
        }

        Ok(Self {
            header,
            src_line,
            src_count,
            dst_line,
            dst_count,
            lines: diff_lines
                .into_iter()
                .filter(|line| !line.is_empty())
                .collect(),
        })
    }
}

impl FromStr for FileDiff {
    type Err = ();
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
