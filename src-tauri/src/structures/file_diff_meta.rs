use std::{ops::Range, str::FromStr};

use super::{diff_status::DiffStatus, hash::GitHash};

type Line = (Option<usize>, Option<usize>, DiffStatus);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HunkSection {
    Unmodified(Vec<(usize, usize)>),
    Added(Vec<usize>),
    Removed(Vec<usize>),
    RemovedAdded(Vec<usize>, Vec<usize>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hunk {
    /// Raw header text
    pub header: String,
    // TODO: are these needed?
    pub src_lines: Range<usize>,
    pub dst_lines: Range<usize>,
    /// All lines in this hunk `(src_line, dst_line, status)`
    pub sections: Vec<HunkSection>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileDiffMeta {
    pub src_path: String,
    pub dst_path: String,
    pub src_hash: Option<GitHash>,
    pub dst_hash: Option<GitHash>,

    pub hunks: Vec<Hunk>,
}

impl HunkSection {
    fn append_line(&mut self, line: (Option<usize>, Option<usize>)) {
        match (self, line) {
            (Self::Unmodified(lines), (Some(src), Some(dst))) => lines.push((src, dst)),
            (Self::Added(lines), (None, Some(line)))
            | (Self::Removed(lines), (Some(line), None)) => lines.push(line),
            (Self::RemovedAdded(_, added), (None, Some(line))) => added.push(line),
            _ => panic!("Invalid line"),
        }
    }
}

impl FromStr for Hunk {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let header: String = format!("@@{}", lines.next().ok_or("Failed to get hunk header")?);
        let mut ranges = header
            .split("@@")
            .nth(1)
            .ok_or("Failed to split on @@ in hunk header")?
            .trim()
            .split(' ')
            .flat_map(|section| {
                let mut nums: String = section.split_at(1).1.into();
                // If there is no newline then the range might only have 1 number
                if !nums.contains(',') {
                    nums = format!("{nums},1");
                }
                let mut nums = nums.split(',').flat_map(|n| n.parse::<usize>().ok());

                let line_number = nums.next()?;
                Some(line_number..line_number + nums.next()?)
            });
        let src_lines = ranges.next().ok_or("Failed to get src from hunk header")?;
        let dst_lines = ranges.next().ok_or("Failed to get dst from hunk header")?;

        let (mut src_line, mut dst_line) = (src_lines.start, dst_lines.start);
        let lines: Vec<Line> = lines
            .map(|line| {
                if line == "\\ No newline at end of file" {
                    return Ok((None, None, DiffStatus::Unmodified));
                }

                let status: DiffStatus = if line.is_empty() {
                    ' '
                } else {
                    line.split_at(1)
                        .0
                        .chars()
                        .next()
                        .ok_or("Diff line status char")?
                }
                .try_into()?;

                Ok(match status {
                    DiffStatus::Added => {
                        let line = (None, Some(dst_line), DiffStatus::Added);
                        dst_line += 1;
                        line
                    }
                    DiffStatus::Removed => {
                        let line = (Some(src_line), None, DiffStatus::Removed);
                        src_line += 1;
                        line
                    }
                    DiffStatus::Unmodified => {
                        let line = (Some(src_line), Some(dst_line), DiffStatus::Unmodified);
                        src_line += 1;
                        dst_line += 1;
                        line
                    }
                })
            })
            // Filter out lines with no line numbers
            .filter(|r| {
                r.is_err()
                    || (r
                        .as_ref()
                        .is_ok_and(|line| line.0.is_some() || line.1.is_some()))
            })
            .collect::<Result<_, String>>()?;

        let sections = group_hunk_into_sections(lines)?;

        Ok(Self {
            header,
            src_lines,
            dst_lines,
            sections,
        })
    }
}

#[rustfmt::skip]
fn group_hunk_into_sections(lines: Vec<Line>) -> Result<Vec<HunkSection>, String> {
    let mut sections: Vec<HunkSection> = Vec::new();
    for (src_line, dst_line, status) in lines {
        let previous_section = sections.last_mut();
        match (&previous_section, status) {
            (None | Some(HunkSection::Unmodified(_)), DiffStatus::Added) => {
                sections.push(HunkSection::Added(vec![dst_line.unwrap()]))
            }
            (None | Some(HunkSection::Unmodified(_) | HunkSection::Added(_) | HunkSection::RemovedAdded(_, _)), DiffStatus::Removed) => {
                sections.push(HunkSection::Removed(vec![src_line.unwrap()]))
            }
            (Some(HunkSection::Unmodified(_)), DiffStatus::Unmodified) | (Some(HunkSection::Removed(_)), DiffStatus::Removed)
            | (Some(HunkSection::Added(_) | HunkSection::RemovedAdded(_, _)), DiffStatus::Added) => {
                previous_section.unwrap().append_line((src_line, dst_line))
            }
            (None | Some(HunkSection::Added(_) | HunkSection::Removed(_) | HunkSection::RemovedAdded(_, _)), DiffStatus::Unmodified) => {
                sections.push(HunkSection::Unmodified(vec![(src_line.unwrap(), dst_line.unwrap())]))
            }
            (Some(HunkSection::Removed(_)), DiffStatus::Added) => {
                if let Some(HunkSection::Removed(removed)) = sections.pop() {
                    sections.push(HunkSection::RemovedAdded(removed, vec![dst_line.unwrap()]));
                }
            }
        }
    }

    Ok(sections)
}

impl FromStr for FileDiffMeta {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split("\n@@");
        let mut diff_header = sections.next().ok_or("Failed to get diff header")?.lines();
        let (src_path, dst_path) = diff_header
            .next()
            .unwrap()
            .strip_prefix("diff --git ")
            .ok_or("Failed to get GNU diff command")?
            .split_once(' ')
            .ok_or("Failed to split diff files")?;

        let (src_hash, dst_hash) = diff_header
            .find(|line| line.starts_with("index "))
            .ok_or("Failed to get index line")?
            .split(' ')
            .nth(1)
            .unwrap()
            .split_once("..")
            .ok_or("Failed to split hashes")?;

        Ok(Self {
            src_path: src_path
                .strip_prefix("a/")
                .ok_or("Src path missing prefix")?
                .into(),
            dst_path: dst_path
                .strip_prefix("b/")
                .ok_or("Dst path missing prefix")?
                .into(),
            src_hash: GitHash::from_optional(src_hash)?,
            dst_hash: GitHash::from_optional(dst_hash)?,
            hunks: sections
                .map(|hunk| hunk.parse())
                .collect::<Result<_, _>>()?,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::structures::{
        file_diff_meta::{Hunk, HunkSection},
        hash::GitHash,
    };

    use super::FileDiffMeta;

    #[test]
    fn parses_a_changed_file() {
        let s = r#"diff --git a/src-tauri/src/commands/mod.rs b/src-tauri/src/commands/mod.rs
index 628f0e2e8d64bae420b909c688e0964f7f530306..87eab99c6eca0c299bdf0fa8185d4f0d40255b73 100644
--- a/src-tauri/src/commands/mod.rs
+++ b/src-tauri/src/commands/mod.rs
@@ -24,8 +24,8 @@ pub enum CommandError {
         #[from]
         sqlx::Error,
     ),
-    #[error("failed to parse git output")]
-    Parse,
+    #[error("failed to parse git output: {0}")]
+    Parse(String),
     #[error("{0}")]
     Other(String),
 }"#;
        assert_eq!(
            s.parse::<FileDiffMeta>().unwrap(),
            FileDiffMeta {
                src_path: "src-tauri/src/commands/mod.rs".into(),
                dst_path: "src-tauri/src/commands/mod.rs".into(),
                src_hash: Some(GitHash("628f0e2e8d64bae420b909c688e0964f7f530306".into())),
                dst_hash: Some(GitHash("87eab99c6eca0c299bdf0fa8185d4f0d40255b73".into())),
                hunks: vec![Hunk {
                    header: "@@ -24,8 +24,8 @@ pub enum CommandError {".into(),
                    src_lines: 24..32,
                    dst_lines: 24..32,
                    sections: vec![
                        HunkSection::Unmodified(vec![(24, 24), (25, 25), (26, 26)]),
                        HunkSection::RemovedAdded(vec![27, 28], vec![27, 28]),
                        HunkSection::Unmodified(vec![(29, 29), (30, 30), (31, 31)])
                    ]
                }]
            }
        );
    }
}
