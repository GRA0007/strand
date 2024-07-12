use std::{ops::Range, time::Duration};

use serde::Serialize;
use specta::Type;

use crate::{
    structures::file_diff_meta::HunkSection,
    utils::highlight::{HighlightLanguage, Highlights, SyntaxHighlighter},
};

use super::{diff_status::DiffStatus, file_diff_meta::FileDiffMeta};

#[derive(Debug, Serialize, Type, Clone, Eq, PartialEq)]
pub struct Fragment {
    pub text: String,
    pub status: DiffStatus,
    pub class: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Type, Clone)]
pub struct LineDiff {
    pub fragments: Vec<Fragment>,
    pub status: DiffStatus,
    /// None if status is Added
    pub src_line_number: Option<usize>,
    /// None if status is Removed
    pub dst_line_number: Option<usize>,
}

#[derive(Debug, Serialize, Type)]
pub struct DiffHunk {
    /// Raw header text, or None if the whole file was requested
    pub header: Option<String>,
    pub lines: Vec<LineDiff>,
}

#[derive(Debug, Serialize, Type)]
pub struct FileDiff(pub Vec<DiffHunk>);

impl FileDiff {
    pub fn from(
        meta: FileDiffMeta,
        src_file: Option<String>,
        dst_file: Option<String>,
    ) -> Result<Self, String> {
        let src_lines: Option<Vec<&str>> = src_file.as_ref().map(|file| file.lines().collect());
        let dst_lines: Option<Vec<&str>> = dst_file.as_ref().map(|file| file.lines().collect());

        let mut highlighter = SyntaxHighlighter::new();

        let src_highlight = src_file
            .as_ref()
            .map(|file| highlighter.highlight(HighlightLanguage::from_path(&meta.src_path), file));
        let dst_highlight = dst_file
            .as_ref()
            .map(|file| highlighter.highlight(HighlightLanguage::from_path(&meta.dst_path), file));

        Ok(Self(
            meta.hunks
                .into_iter()
                .map(|hunk| {
                    let lines: Vec<LineDiff> = hunk
                        .sections
                        .into_iter()
                        .flat_map(|section| {
                            match (section, &src_lines, &dst_lines) {
                                (HunkSection::Unmodified(line_numbers), Some(_), Some(lines)) => {
                                    line_numbers
                                        .into_iter()
                                        .map(|(src_line, dst_line)| LineDiff {
                                            fragments: Fragment::from_highlighted(
                                                line_number_range(
                                                    dst_line,
                                                    dst_file.as_ref().unwrap(),
                                                ),
                                                lines[dst_line - 1].into(),
                                                DiffStatus::Unmodified,
                                                dst_highlight.as_ref().unwrap(),
                                            ),
                                            status: DiffStatus::Unmodified,
                                            src_line_number: Some(src_line),
                                            dst_line_number: Some(dst_line),
                                        })
                                        .collect::<Vec<_>>()
                                }
                                (HunkSection::Added(line_numbers), Some(_) | None, Some(lines)) => {
                                    line_numbers
                                        .into_iter()
                                        .map(|line_number| LineDiff {
                                            fragments: Fragment::from_highlighted(
                                                line_number_range(
                                                    line_number,
                                                    dst_file.as_ref().unwrap(),
                                                ),
                                                lines[line_number - 1].into(),
                                                DiffStatus::Unmodified,
                                                dst_highlight.as_ref().unwrap(),
                                            ),
                                            status: DiffStatus::Added,
                                            src_line_number: None,
                                            dst_line_number: Some(line_number),
                                        })
                                        .collect::<Vec<_>>()
                                }
                                (
                                    HunkSection::Removed(line_numbers),
                                    Some(lines),
                                    Some(_) | None,
                                ) => line_numbers
                                    .into_iter()
                                    .map(|line_number| LineDiff {
                                        fragments: Fragment::from_highlighted(
                                            line_number_range(
                                                line_number,
                                                src_file.as_ref().unwrap(),
                                            ),
                                            lines[line_number - 1].into(),
                                            DiffStatus::Unmodified,
                                            src_highlight.as_ref().unwrap(),
                                        ),
                                        status: DiffStatus::Removed,
                                        src_line_number: Some(line_number),
                                        dst_line_number: None,
                                    })
                                    .collect::<Vec<_>>(),
                                (
                                    HunkSection::RemovedAdded(
                                        removed_line_numbers,
                                        added_line_numbers,
                                    ),
                                    Some(removed_lines),
                                    Some(added_lines),
                                ) => {
                                    let removed_text = removed_line_numbers
                                        .iter()
                                        .map(|i| removed_lines[*i - 1])
                                        .collect::<Vec<_>>()
                                        .join("\n");
                                    let added_text = added_line_numbers
                                        .iter()
                                        .map(|i| added_lines[*i - 1])
                                        .collect::<Vec<_>>()
                                        .join("\n");

                                    // Calculate word diff
                                    let removed_tokenized = tokenize_code(&removed_text);
                                    let added_tokenized = tokenize_code(&added_text);
                                    let diff = similar::TextDiff::configure()
                                        .algorithm(similar::Algorithm::Patience)
                                        .timeout(Duration::from_millis(500))
                                        .diff_slices(&removed_tokenized, &added_tokenized);
                                    let remapper = similar::utils::TextDiffRemapper::from_text_diff(
                                        &diff,
                                        &removed_text,
                                        &added_text,
                                    );
                                    let (src_pos, dst_pos) = (
                                        line_number_range(
                                            *removed_line_numbers.first().unwrap(),
                                            src_file.as_ref().unwrap(),
                                        )
                                        .start,
                                        line_number_range(
                                            *added_line_numbers.first().unwrap(),
                                            dst_file.as_ref().unwrap(),
                                        )
                                        .start,
                                    );
                                    let (_, _, diff) = diff
                                        .ops()
                                        .iter()
                                        .flat_map(move |x| remapper.iter_slices(x))
                                        .fold(
                                            (src_pos, dst_pos, Vec::new()),
                                            |(mut src_pos, mut dst_pos, mut diff), (tag, value)| {
                                                diff.extend(value.split_inclusive('\n').flat_map(
                                                    |text| {
                                                        Fragment::from_highlighted(
                                                            match tag {
                                                                similar::ChangeTag::Equal => {
                                                                    let pos = dst_pos
                                                                        ..dst_pos + text.len();
                                                                    src_pos += text.len();
                                                                    dst_pos += text.len();
                                                                    pos
                                                                }
                                                                similar::ChangeTag::Insert => {
                                                                    let pos = dst_pos
                                                                        ..dst_pos + text.len();
                                                                    dst_pos += text.len();
                                                                    pos
                                                                }
                                                                similar::ChangeTag::Delete => {
                                                                    let pos = src_pos
                                                                        ..src_pos + text.len();
                                                                    src_pos += text.len();
                                                                    pos
                                                                }
                                                            },
                                                            text.into(),
                                                            tag.into(),
                                                            match tag {
                                                                similar::ChangeTag::Equal
                                                                | similar::ChangeTag::Insert => {
                                                                    dst_highlight.as_ref()
                                                                }
                                                                similar::ChangeTag::Delete => {
                                                                    src_highlight.as_ref()
                                                                }
                                                            }
                                                            .unwrap(),
                                                        )
                                                    },
                                                ));

                                                (src_pos, dst_pos, diff)
                                            },
                                        );

                                    let removed_lines: Vec<_> = split_fragments_into_lines(
                                        diff.iter()
                                            .filter(|f| !matches!(f.status, DiffStatus::Added))
                                            .collect(),
                                    )
                                    .into_iter()
                                    .enumerate()
                                    .map(|(i, line)| LineDiff {
                                        fragments: line,
                                        status: DiffStatus::Removed,
                                        src_line_number: Some(removed_line_numbers[i]),
                                        dst_line_number: None,
                                    })
                                    .collect();

                                    let added_lines: Vec<_> = split_fragments_into_lines(
                                        diff.iter()
                                            .filter(|f| !matches!(f.status, DiffStatus::Removed))
                                            .collect(),
                                    )
                                    .into_iter()
                                    .enumerate()
                                    .map(|(i, line)| LineDiff {
                                        fragments: line,
                                        status: DiffStatus::Added,
                                        src_line_number: None,
                                        dst_line_number: Some(added_line_numbers[i]),
                                    })
                                    .collect();

                                    [removed_lines, added_lines].concat()
                                }
                                _ => panic!("Invalid hunk section"),
                            }
                        })
                        .collect();

                    DiffHunk {
                        header: Some(hunk.header),
                        lines,
                    }
                })
                .collect(),
        ))
    }
}

impl Fragment {
    /// Takes a string with a status and applies highlights to it.
    ///
    /// - `source_range`: The range of the `text` from it's file
    /// - `text`: The raw text
    /// - `status`: The diff status, passed through
    /// - `highlights`: The full highlights for the file that `text` belongs to
    fn from_highlighted(
        source_range: Range<usize>,
        text: String,
        status: DiffStatus,
        highlights: &Highlights,
    ) -> Vec<Self> {
        let start = source_range.start;

        highlights
            .get_ranges(source_range)
            .into_iter()
            .map(|h| Self {
                text: text
                    .get(h.0.start - start..h.0.end - start)
                    .expect("Range not within text bounds")
                    .into(),
                status: status.clone(),
                class: h.1,
            })
            .collect()
    }
}

/// Take fragments where some may be terminated by a newline, and split on that.
/// Removes all newlines.
fn split_fragments_into_lines(fragments: Vec<&Fragment>) -> Vec<Vec<Fragment>> {
    let len = fragments.len();
    let mut lines = vec![Vec::new()];
    for (i, fragment) in fragments.into_iter().enumerate() {
        lines.last_mut().unwrap().push(Fragment {
            text: fragment.text.trim_end_matches('\n').into(),
            ..fragment.clone()
        });
        if fragment.text.ends_with('\n') && i != len - 1 {
            lines.push(Vec::new());
        }
    }
    lines
}

/// Get the start..end range of a single line in a file
fn line_number_range(line_number: usize, file: &str) -> Range<usize> {
    let mut lines = file.split_inclusive('\n');
    let start = lines
        .clone()
        .take(line_number - 1)
        .flat_map(|s| s.chars())
        .count();
    let end = lines
        .nth(line_number - 1)
        .unwrap()
        .trim_end_matches('\n')
        .len();
    start..start + end
}

/// Tokenize into chars, but keep runs of ascii letters and numbers in single tokens
fn tokenize_code(s: &str) -> Vec<&str> {
    let mut rv = vec![];
    let mut iter = s.char_indices().peekable();

    while let Some((idx, c)) = iter.next() {
        let start = idx;
        let mut end = idx + c.len_utf8();
        while let Some(&(_, next_char)) = iter.peek() {
            if !(((next_char.is_ascii_alphanumeric() || next_char == '_')
                && (c.is_ascii_alphanumeric() || c == '_'))
                || (next_char.is_whitespace()
                    && next_char != '\n'
                    && c.is_whitespace()
                    && c != '\n'))
            {
                break;
            }
            iter.next();
            end += next_char.len_utf8();
        }
        rv.push(&s[start..end]);
    }

    rv
}

#[cfg(test)]
mod test {
    use crate::structures::{diff_status::DiffStatus, file_diff::line_number_range};

    use super::{split_fragments_into_lines, Fragment, Highlights};

    #[test]
    fn calculates_line_number_range() {
        let file = "First line
Second line
Third line
Fourth and final line";
        assert_eq!(file.get(line_number_range(1, file)), Some("First line"));
        assert_eq!(file.get(line_number_range(2, file)), Some("Second line"));
        assert_eq!(file.get(line_number_range(3, file)), Some("Third line"));
        assert_eq!(
            file.get(line_number_range(4, file)),
            Some("Fourth and final line")
        );
    }

    #[test]
    fn fills_in_range_gaps() {
        let file = "line 1\n  second line";
        let highlights = Highlights(vec![
            (0..4, vec!["variable".into()]),
            (5..6, vec!["constant".into()]),
            (9..15, vec!["variable".into()]),
            (16..20, vec!["constant".into()]),
        ]);
        let ranges = highlights.get_ranges(line_number_range(2, file));

        assert_eq!(
            ranges,
            vec![
                (7..9, None),
                (9..15, Some(vec!["variable".into()])),
                (15..16, None),
                (16..20, Some(vec!["constant".into()])),
            ]
        );
    }

    #[test]
    fn creates_fragments_from_highlights() {
        let file = "// My cool string\nlet my_variable = String::from(\"Something\");";
        let highlights = Highlights(vec![
            (0..17, vec!["comment".into()]),
            (18..21, vec!["keyword".into()]),
            (22..33, vec!["variable".into()]),
            (34..35, vec!["operator".into()]),
            (36..42, vec!["constructor".into()]),
            (42..44, vec!["punctuation".into()]),
            (44..48, vec!["function".into()]),
            (48..61, vec!["string".into()]),
            (61..62, vec!["punctuation".into()]),
        ]);
        let fragments = Fragment::from_highlighted(
            line_number_range(2, file),
            file.get(line_number_range(2, file)).unwrap().into(),
            DiffStatus::Unmodified,
            &highlights,
        );

        assert_eq!(
            fragments,
            vec![
                Fragment {
                    text: "let".into(),
                    status: DiffStatus::Unmodified,
                    class: Some(vec!["keyword".into()])
                },
                Fragment {
                    text: " ".into(),
                    status: DiffStatus::Unmodified,
                    class: None,
                },
                Fragment {
                    text: "my_variable".into(),
                    status: DiffStatus::Unmodified,
                    class: Some(vec!["variable".into()])
                },
                Fragment {
                    text: " ".into(),
                    status: DiffStatus::Unmodified,
                    class: None,
                },
                Fragment {
                    text: "=".into(),
                    status: DiffStatus::Unmodified,
                    class: Some(vec!["operator".into()])
                },
                Fragment {
                    text: " ".into(),
                    status: DiffStatus::Unmodified,
                    class: None,
                },
                Fragment {
                    text: "String".into(),
                    status: DiffStatus::Unmodified,
                    class: Some(vec!["constructor".into()])
                },
                Fragment {
                    text: "::".into(),
                    status: DiffStatus::Unmodified,
                    class: Some(vec!["punctuation".into()])
                },
                Fragment {
                    text: "from".into(),
                    status: DiffStatus::Unmodified,
                    class: Some(vec!["function".into()])
                },
                Fragment {
                    text: "(\"Something\")".into(),
                    status: DiffStatus::Unmodified,
                    class: Some(vec!["string".into()])
                },
                Fragment {
                    text: ";".into(),
                    status: DiffStatus::Unmodified,
                    class: Some(vec!["punctuation".into()])
                },
            ]
        );
    }

    #[test]
    fn splits_fragments_into_lines() {
        let fragment_1 = Fragment {
            text: "one".into(),
            status: DiffStatus::Unmodified,
            class: None,
        };
        let fragment_2 = Fragment {
            text: "two\n".into(),
            status: DiffStatus::Added,
            class: None,
        };
        let fragment_3 = Fragment {
            text: "three".into(),
            status: DiffStatus::Removed,
            class: None,
        };
        let fragments = vec![&fragment_1, &fragment_2, &fragment_3];
        let lines = split_fragments_into_lines(fragments);

        assert_eq!(
            lines,
            vec![
                vec![
                    fragment_1,
                    Fragment {
                        text: "two".into(),
                        status: DiffStatus::Added,
                        class: None,
                    }
                ],
                vec![fragment_3]
            ]
        );
    }
}
