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

impl FromStr for WordDiff {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (status, text) = s.split_at(1);
        let status = status
            .chars()
            .next()
            .ok_or("Diff status char")?
            .try_into()?;
        let text = text.into();

        Ok(Self { status, text })
    }
}

impl FromStr for DiffHunk {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut str_lines = s.lines();

        let header: String = str_lines.next().ok_or("Failed to get header")?.into();
        let (mut src_line_number, mut dst_line_number) = parse_line_numbers(&header)?;

        // Parse lines
        let mut lines: Vec<Vec<WordDiff>> = vec![Vec::new()];
        for line in str_lines {
            match line {
                "~" => lines.push(Vec::new()),
                line => lines
                    .last_mut()
                    .ok_or("No preceding tilde")?
                    .push(line.parse()?),
            }
        }

        let lines = lines
            .into_iter()
            .flat_map(|words| {
                let (num_added, num_removed) =
                    words.iter().fold((0, 0), |(a, r), word| match word.status {
                        DiffStatus::Added => (a + 1, r),
                        DiffStatus::Removed => (a, r + 1),
                        DiffStatus::Unmodified => (a, r),
                    });

                let mut lines = Vec::new();

                // Context line or empty line
                if num_added == 0 && num_removed == 0 {
                    lines.push(LineDiff {
                        words: words.clone(),
                        status: DiffStatus::Unmodified,
                        src_line_number: Some(src_line_number),
                        dst_line_number: Some(dst_line_number),
                    });
                    src_line_number += 1;
                    dst_line_number += 1;
                }

                // Removed line
                if num_removed > 0 {
                    let removed_words: Vec<_> = words
                        .clone()
                        .into_iter()
                        .filter(|word| !matches!(word.status, DiffStatus::Added))
                        .collect();

                    if !removed_words.is_empty() {
                        lines.push(LineDiff {
                            words: removed_words,
                            status: DiffStatus::Removed,
                            src_line_number: Some(src_line_number),
                            dst_line_number: None,
                        });
                        src_line_number += 1;
                    }
                }

                // Added line
                if num_added > 0 || num_removed > 0 {
                    let added_words: Vec<_> = words
                        .into_iter()
                        .filter(|word| !matches!(word.status, DiffStatus::Removed))
                        .collect();

                    if !added_words.is_empty() {
                        lines.push(LineDiff {
                            words: added_words,
                            status: DiffStatus::Added,
                            src_line_number: None,
                            dst_line_number: Some(dst_line_number),
                        });
                        dst_line_number += 1;
                    }
                }

                lines
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
