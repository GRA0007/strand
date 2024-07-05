use std::str::{FromStr, Split};

use serde::Serialize;
use similar::{utils::TextDiffRemapper, ChangeTag, TextDiff};
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

#[derive(Debug)]
enum HunkSegment {
    Unmodified(String),
    Added(String),
    Removed(String),
    RemovedAdded(String, String),
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

impl From<ChangeTag> for DiffStatus {
    fn from(value: ChangeTag) -> Self {
        match value {
            ChangeTag::Equal => Self::Unmodified,
            ChangeTag::Delete => Self::Removed,
            ChangeTag::Insert => Self::Added,
        }
    }
}

impl HunkSegment {
    fn append_line(&mut self, line: &str) {
        match self {
            HunkSegment::Unmodified(text)
            | HunkSegment::Added(text)
            | HunkSegment::Removed(text) => text.push_str(format!("\n{line}").as_str()),
            HunkSegment::RemovedAdded(_, added) => added.push_str(format!("\n{line}").as_str()),
        }
    }
}

impl FromStr for DiffHunk {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n');

        let header: String = lines.next().ok_or("Failed to get header")?.into();
        let (mut src_line_number, mut dst_line_number) = parse_line_numbers(&header)?;

        // Group diffs into hunk "segments"
        let segments = group_diff_into_segments(lines)?;

        // Calculate word diffs and turn segments back into lines
        let lines = segments
            .into_iter()
            .flat_map(|segment| match segment {
                HunkSegment::Unmodified(text) => text
                    .split('\n')
                    .map(|line| {
                        let line = LineDiff {
                            words: vec![WordDiff {
                                text: line.into(),
                                status: DiffStatus::Unmodified,
                            }],
                            status: DiffStatus::Unmodified,
                            src_line_number: Some(src_line_number),
                            dst_line_number: Some(dst_line_number),
                        };
                        src_line_number += 1;
                        dst_line_number += 1;
                        line
                    })
                    .collect::<Vec<_>>(),
                HunkSegment::Added(text) => text
                    .split('\n')
                    .map(|line| {
                        let line = LineDiff {
                            words: vec![WordDiff {
                                text: line.into(),
                                status: DiffStatus::Unmodified,
                            }],
                            status: DiffStatus::Added,
                            src_line_number: None,
                            dst_line_number: Some(dst_line_number),
                        };
                        dst_line_number += 1;
                        line
                    })
                    .collect(),
                HunkSegment::Removed(text) => text
                    .split('\n')
                    .map(|line| {
                        let line = LineDiff {
                            words: vec![WordDiff {
                                text: line.into(),
                                status: DiffStatus::Unmodified,
                            }],
                            status: DiffStatus::Removed,
                            src_line_number: Some(src_line_number),
                            dst_line_number: None,
                        };
                        src_line_number += 1;
                        line
                    })
                    .collect(),
                HunkSegment::RemovedAdded(removed, added) => {
                    // Calculate word diff
                    let tokenized_removed = tokenize(&removed);
                    let tokenized_added = tokenize(&added);
                    let diff = TextDiff::from_slices(&tokenized_removed, &tokenized_added);
                    let remapper = TextDiffRemapper::from_text_diff(&diff, &removed, &added);
                    let diff: Vec<_> = diff
                        .ops()
                        .iter()
                        .flat_map(move |x| remapper.iter_slices(x))
                        .flat_map(|(tag, value)| {
                            value.split_inclusive('\n').map(move |text| WordDiff {
                                text: text.into(),
                                status: tag.into(),
                            })
                        })
                        .collect();

                    let mut lines = Vec::new();

                    let mut current_line: Vec<WordDiff> = Vec::new();
                    let removed_words: Vec<_> = diff
                        .iter()
                        .filter(|word| !matches!(word.status, DiffStatus::Added))
                        .collect();
                    for (i, word) in removed_words.iter().enumerate() {
                        current_line.push(WordDiff {
                            text: word.text.trim_end_matches('\n').into(),
                            status: word.status.clone(),
                        });
                        if word.text.ends_with('\n') || i == removed_words.len() - 1 {
                            lines.push(LineDiff {
                                words: current_line.to_vec(),
                                status: DiffStatus::Removed,
                                src_line_number: Some(src_line_number),
                                dst_line_number: None,
                            });
                            src_line_number += 1;
                            current_line.clear();
                        }
                    }

                    let added_words: Vec<_> = diff
                        .iter()
                        .filter(|word| !matches!(word.status, DiffStatus::Removed))
                        .collect();
                    for (i, word) in added_words.iter().enumerate() {
                        current_line.push(WordDiff {
                            text: word.text.trim_end_matches('\n').into(),
                            status: word.status.clone(),
                        });
                        if word.text.ends_with('\n') || i == added_words.len() - 1 {
                            lines.push(LineDiff {
                                words: current_line.to_vec(),
                                status: DiffStatus::Added,
                                src_line_number: None,
                                dst_line_number: Some(dst_line_number),
                            });
                            dst_line_number += 1;
                            current_line.clear();
                        }
                    }

                    lines
                }
            })
            .collect();

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

#[rustfmt::skip]
fn group_diff_into_segments(lines: Split<char>) -> Result<Vec<HunkSegment>, String> {
    let mut segments: Vec<HunkSegment> = Vec::new();
    for line in lines {
        let (status, text) = line.split_at(1);

        let status: DiffStatus = status
            .chars()
            .next()
            .ok_or("Diff status char")?
            .try_into()?;

        let previous_segment = segments.last_mut();
        match (&previous_segment, status) {
            (None | Some(HunkSegment::Unmodified(_)), DiffStatus::Added) => {
                segments.push(HunkSegment::Added(text.into()))
            }
            (None | Some(HunkSegment::Unmodified(_) | HunkSegment::Added(_) | HunkSegment::RemovedAdded(_, _)), DiffStatus::Removed) => {
                segments.push(HunkSegment::Removed(text.into()))
            }
            (Some(HunkSegment::Unmodified(_)), DiffStatus::Unmodified) | (Some(HunkSegment::Removed(_)), DiffStatus::Removed)
            | (Some(HunkSegment::Added(_) | HunkSegment::RemovedAdded(_, _)), DiffStatus::Added) => {
                previous_segment.unwrap().append_line(text)
            }
            (None | Some(HunkSegment::Added(_) | HunkSegment::Removed(_) | HunkSegment::RemovedAdded(_, _)), DiffStatus::Unmodified) => {
                segments.push(HunkSegment::Unmodified(text.into()))
            }
            (Some(HunkSegment::Removed(_)), DiffStatus::Added) => {
                if let Some(HunkSegment::Removed(removed)) = segments.pop() {
                    segments.push(HunkSegment::RemovedAdded(removed, text.into()));
                }
            }
        }
    }

    Ok(segments)
}

fn tokenize(s: &str) -> Vec<&str> {
    let mut rv = vec![];
    let mut iter = s.char_indices().peekable();

    while let Some((idx, c)) = iter.next() {
        let is_alphanumeric = !c.is_alphanumeric();
        let start = idx;
        let mut end = idx + c.len_utf8();
        while let Some(&(_, next_char)) = iter.peek() {
            if next_char.is_alphanumeric() == is_alphanumeric {
                break;
            }
            iter.next();
            end += next_char.len_utf8();
        }
        rv.push(&s[start..end]);
    }

    rv
}
