use std::{
    str::{FromStr, Lines},
    sync::OnceLock,
};

use serde::Serialize;
use similar::{utils::TextDiffRemapper, ChangeTag, TextDiff};
use specta::Type;
use syntect::{
    easy::HighlightLines,
    highlighting::{FontStyle, Style, ThemeSet},
    parsing::SyntaxSet,
};

#[derive(Debug, Serialize, Type, Clone)]
pub enum DiffStatus {
    Added,
    Removed,
    Unmodified,
}

#[derive(Debug, Serialize, Type, Clone)]
pub struct FragmentFontStyle {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

#[derive(Debug, Serialize, Type, Clone)]
pub struct FragmentStyle {
    pub foreground: (u8, u8, u8, u8),
    pub background: (u8, u8, u8, u8),
    pub font_style: FragmentFontStyle,
}

impl From<Style> for FragmentStyle {
    fn from(value: Style) -> Self {
        Self {
            foreground: (
                value.foreground.r,
                value.foreground.g,
                value.foreground.b,
                value.foreground.a,
            ),
            background: (
                value.background.r,
                value.background.g,
                value.background.b,
                value.background.a,
            ),
            font_style: FragmentFontStyle {
                bold: value.font_style.contains(FontStyle::BOLD),
                italic: value.font_style.contains(FontStyle::ITALIC),
                underline: value.font_style.contains(FontStyle::UNDERLINE),
            },
        }
    }
}

#[derive(Debug, Serialize, Type, Clone)]
pub struct Fragment {
    pub text: String,
    pub status: DiffStatus,
    pub style: Option<FragmentStyle>,
}

#[derive(Debug, Serialize, Type)]
pub struct LineDiff {
    pub fragments: Vec<Fragment>,
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
    /// Word diffs only need to be calculated for this variant
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

impl DiffHunk {
    fn from(s: &str, extension: &str) -> Result<Self, String> {
        let mut lines = s.lines();

        let header: String = format!("@@{}", lines.next().ok_or("Failed to get header")?);
        let (mut src_line_number, mut dst_line_number) = parse_line_numbers(&header)?;

        // Group diffs into hunk "segments"
        let segments = group_diff_into_segments(lines)?;

        // TODO: Reduce duplication of this match
        // Calculate word diffs and turn segments back into lines
        let mut lines: Vec<LineDiff> = segments
            .into_iter()
            .flat_map(|segment| match segment {
                HunkSegment::Unmodified(text) => text
                    .split('\n')
                    .map(|line| {
                        let line = LineDiff {
                            fragments: vec![Fragment {
                                text: line.into(),
                                status: DiffStatus::Unmodified,
                                style: None,
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
                            fragments: vec![Fragment {
                                text: line.into(),
                                status: DiffStatus::Unmodified,
                                style: None,
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
                            fragments: vec![Fragment {
                                text: line.into(),
                                status: DiffStatus::Unmodified,
                                style: None,
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
                    let old_tokenized = tokenize_code(&removed);
                    let new_tokenized = tokenize_code(&added);
                    let diff = TextDiff::from_slices(&old_tokenized, &new_tokenized);
                    let remapper = TextDiffRemapper::from_text_diff(&diff, &removed, &added);
                    let diff: Vec<_> = diff
                        .ops()
                        .iter()
                        .flat_map(move |x| remapper.iter_slices(x))
                        .flat_map(|(tag, value)| {
                            value.split_inclusive('\n').map(move |text| Fragment {
                                text: text.into(),
                                status: tag.into(),
                                style: None,
                            })
                        })
                        .collect();

                    let mut lines = Vec::new();

                    let mut current_line: Vec<Fragment> = Vec::new();
                    let removed_words: Vec<_> = diff
                        .iter()
                        .filter(|word| !matches!(word.status, DiffStatus::Added))
                        .collect();
                    for (i, word) in removed_words.iter().enumerate() {
                        current_line.push(Fragment {
                            text: word.text.trim_end_matches('\n').into(),
                            status: word.status.clone(),
                            style: None,
                        });
                        if word.text.ends_with('\n') || i == removed_words.len() - 1 {
                            lines.push(LineDiff {
                                fragments: current_line.to_vec(),
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
                        current_line.push(Fragment {
                            text: word.text.trim_end_matches('\n').into(),
                            status: word.status.clone(),
                            style: None,
                        });
                        if word.text.ends_with('\n') || i == added_words.len() - 1 {
                            lines.push(LineDiff {
                                fragments: current_line.to_vec(),
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

        // Highlight lines
        lines.iter_mut().for_each(|line| line.highlight(extension));

        Ok(Self { header, lines })
    }
}

impl FromStr for FileDiff {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split("\n@@");
        let diff_header = sections.next().ok_or("Failed to get diff header")?;
        // TODO: This method of extracting the filename is possibly quite flaky
        let (removed_filename, added_filename) = diff_header
            .split_once("--- ")
            .ok_or("Failed to extract file names")?
            .1
            .split_once("\n+++ ")
            .ok_or("Failed to split file names")?;
        let extension = if added_filename == "/dev/null" {
            removed_filename
        } else {
            added_filename
        }
        .rsplit_once('.')
        .ok_or("Failed to get file extension")?
        .1;

        let hunks = sections
            .map(|hunk| DiffHunk::from(hunk, extension))
            .collect::<Result<_, _>>()?;

        Ok(Self(hunks))
    }
}

static PS: OnceLock<SyntaxSet> = OnceLock::new();
static TS: OnceLock<ThemeSet> = OnceLock::new();

impl LineDiff {
    fn highlight(&mut self, extension: &str) {
        let line = self
            .fragments
            .iter()
            .map(|f| f.text.as_str())
            .collect::<Vec<_>>()
            .join("");

        let ps = PS.get_or_init(|| SyntaxSet::load_defaults_newlines());
        let ts = TS.get_or_init(|| ThemeSet::load_defaults());

        // TODO: don't load themes/syntax for every line
        let syntax = ps
            .find_syntax_by_extension(extension)
            .unwrap_or_else(|| ps.find_syntax_plain_text());
        let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.light"]);
        self.fragments = h
            .highlight_line(&line, &ps)
            .expect("Failed to highlight line")
            .into_iter()
            .flat_map(|(style, text)| {
                vec![Fragment {
                    text: text.into(),
                    status: DiffStatus::Unmodified,
                    style: Some(style.into()),
                }]
            })
            .collect();
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
fn group_diff_into_segments(lines: Lines) -> Result<Vec<HunkSegment>, String> {
    let mut segments: Vec<HunkSegment> = Vec::new();
    for line in lines {
        let (status, text) = if line.is_empty() { (" ", "") } else { line.split_at(1) };

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

/// Tokenize into chars, but keep runs of ascii letters and numbers in single tokens
fn tokenize_code(s: &str) -> Vec<&str> {
    let mut rv = vec![];
    let mut iter = s.char_indices().peekable();

    while let Some((idx, c)) = iter.next() {
        let start = idx;
        let mut end = idx + c.len_utf8();
        while let Some(&(_, next_char)) = iter.peek() {
            if !next_char.is_ascii_alphanumeric() || !c.is_ascii_alphanumeric() {
                break;
            }
            iter.next();
            end += next_char.len_utf8();
        }
        rv.push(&s[start..end]);
    }

    rv
}
