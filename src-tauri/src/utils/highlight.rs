use std::{cmp, ops::Range, path::PathBuf, sync::OnceLock};

use tree_sitter_highlight::HighlightConfiguration;

pub const HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "type",
    "type.builtin",
    "type.enum",
    "type.enum.variant",
    "constructor",
    "constant",
    "constant.builtin",
    "constant.builtin.boolean",
    "constant.character",
    "constant.character.escape",
    "constant.numeric",
    "constant.numeric.integer",
    "constant.numeric.float",
    "string",
    "string.regexp",
    "string.special",
    "string.special.path",
    "string.special.url",
    "string.special.symbol",
    "escape",
    "comment",
    "comment.line",
    "comment.block",
    "comment.block.documentation",
    "variable",
    "variable.builtin",
    "variable.parameter",
    "variable.other",
    "variable.other.member",
    "label",
    "punctuation",
    "punctuation.delimiter",
    "punctuation.bracket",
    "punctuation.special",
    "keyword",
    "keyword.control",
    "keyword.control.conditional",
    "keyword.control.repeat",
    "keyword.control.import",
    "keyword.control.return",
    "keyword.control.exception",
    "keyword.operator",
    "keyword.directive",
    "keyword.function",
    "keyword.storage",
    "keyword.storage.type",
    "keyword.storage.modifier",
    "operator",
    "function",
    "function.builtin",
    "function.method",
    "function.macro",
    "function.special",
    "tag",
    "tag.builtin",
    "namespace",
    "special",
    "markup",
    "markup.heading",
    "markup.heading.marker",
    "markup.heading.1",
    "markup.heading.2",
    "markup.heading.3",
    "markup.heading.4",
    "markup.heading.5",
    "markup.heading.6",
    "markup.list",
    "markup.list.unnumbered",
    "markup.list.numbered",
    "markup.list.checked",
    "markup.list.unchecked",
    "markup.bold",
    "markup.italic",
    "markup.strikethrough",
    "markup.link",
    "markup.link.url",
    "markup.link.label",
    "markup.link.text",
    "markup.quote",
    "markup.raw",
    "markup.raw.inline",
    "markup.raw.block",
    "diff",
    "diff.plus",
    "diff.minus",
    "diff.delta",
    "diff.delta.moved",
];

#[derive(Default)]
pub struct SyntaxHighlighter {
    highlighter: tree_sitter_highlight::Highlighter,
}

pub struct Highlights(pub Vec<(Range<usize>, Vec<String>)>);

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            // TODO: the docs recommend only using once instance of the highlighter
            // per thread, investigate not creating it for every file
            highlighter: tree_sitter_highlight::Highlighter::new(),
        }
    }

    pub fn highlight(&mut self, language: HighlightLanguage, file: &str) -> Highlights {
        match language.config().as_ref() {
            Some(config) => {
                let mut highlights = Vec::new();
                let mut current_range: Option<Range<usize>> = None;
                let mut current_class: Option<Vec<String>> = None;

                for event in self
                    .highlighter
                    .highlight(config, file.as_bytes(), None, |token| {
                        HighlightLanguage::from_token(token).config()
                    })
                    .unwrap()
                {
                    match event.unwrap() {
                        tree_sitter_highlight::HighlightEvent::HighlightStart(h) => {
                            current_class =
                                Some(HIGHLIGHT_NAMES[h.0].split('.').map(|c| c.into()).collect());
                        }
                        tree_sitter_highlight::HighlightEvent::Source { start, end } => {
                            current_range = Some(start..end);
                        }
                        tree_sitter_highlight::HighlightEvent::HighlightEnd => {
                            if let Some(range) = current_range.clone() {
                                if let Some(class) = current_class.clone() {
                                    highlights.push((range, class))
                                }
                            }
                        }
                    }
                }

                Highlights(highlights)
            }
            None => Highlights(vec![]),
        }
    }
}

impl Highlights {
    /// Takes a range and returns the highlighted ranges and fills in the gaps with None ranges
    pub fn get_ranges(&self, range: Range<usize>) -> Vec<(Range<usize>, Option<Vec<String>>)> {
        let mut ranges = Vec::new();
        let mut current_start = range.start;

        // Assume the ranges are sorted
        let highlights_in_range = self
            .0
            .iter()
            .filter(|h| h.0.clone().any(|r| range.contains(&r)));

        for (r, class) in highlights_in_range {
            // Gap between the current pos and the start of the next range
            if current_start < r.start {
                ranges.push((current_start..r.start, None))
            }

            ranges.push((
                cmp::max(r.start, current_start)..cmp::min(r.end, range.end),
                Some(class.clone()),
            ));
            current_start = r.end;
        }

        // Gap after the last range
        if current_start < range.end {
            ranges.push((current_start..range.end, None))
        }

        ranges
    }
}

pub enum HighlightLanguage {
    C,
    Cpp,
    CSharp,
    Css,
    Html,
    Java,
    Javascript,
    Jsx,
    Json,
    Lua,
    Markdown,
    Php,
    Python,
    Ruby,
    Rust,
    Swift,
    Toml,
    Typescript,
    Tsx,
    Yaml,
    Plaintext,
}

static CONFIG_C: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_CPP: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_CSHARP: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_CSS: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_HTML: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_JAVA: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_JAVASCRIPT: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_JSX: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_JSON: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_LUA: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_MARKDOWN: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_PHP: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_PYTHON: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_RUBY: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_RUST: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_SWIFT: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_TOML: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_TYPESCRIPT: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_TSX: OnceLock<HighlightConfiguration> = OnceLock::new();
static CONFIG_YAML: OnceLock<HighlightConfiguration> = OnceLock::new();

impl HighlightLanguage {
    /// Use the extension from a file path
    pub fn from_path(path: &str) -> Self {
        Self::from_token(
            PathBuf::from(path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("txt"),
        )
    }

    fn from_token(token: &str) -> Self {
        match token {
            "c" | "h" => Self::C,
            "cplusplus" | "cpp" | "cc" | "cxx" | "cp" | "c++" | "hpp" | "hh" | "hxx" | "h++" => {
                Self::Cpp
            }
            "csharp" | "cs" => Self::CSharp,
            "css" => Self::Css,
            "html" | "htm" => Self::Html,
            "java" => Self::Java,
            "javascript" | "js" => Self::Javascript,
            "javascriptreact" | "jsx" => Self::Jsx,
            "json" => Self::Json,
            "lua" => Self::Lua,
            "markdown" | "md" => Self::Markdown,
            "php" => Self::Php,
            "python" | "py" => Self::Python,
            "ruby" | "rb" => Self::Ruby,
            "rust" | "rs" => Self::Rust,
            "swift" => Self::Swift,
            "toml" => Self::Toml,
            "typescript" | "ts" => Self::Typescript,
            "typescriptreact" | "tsx" => Self::Tsx,
            "yml" | "yaml" => Self::Yaml,
            _ => Self::Plaintext,
        }
    }

    fn config(&self) -> Option<&'static HighlightConfiguration> {
        match self {
            Self::C => Some(CONFIG_C.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_c::language(),
                    "c",
                    tree_sitter_c::HIGHLIGHT_QUERY,
                    "",
                    "",
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Cpp => Some(CONFIG_CPP.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_cpp::language(),
                    "c++",
                    tree_sitter_cpp::HIGHLIGHT_QUERY,
                    "",
                    "",
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::CSharp => Some(CONFIG_CSHARP.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_c_sharp::language(),
                    "c#",
                    tree_sitter_c_sharp::HIGHLIGHTS_QUERY,
                    "",
                    "",
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Css => Some(CONFIG_CSS.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_css::language(),
                    "css",
                    tree_sitter_css::HIGHLIGHTS_QUERY,
                    "",
                    "",
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Html => Some(CONFIG_HTML.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_html::language(),
                    "html",
                    tree_sitter_html::HIGHLIGHTS_QUERY,
                    tree_sitter_html::INJECTIONS_QUERY,
                    "",
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Java => Some(CONFIG_JAVA.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_java::language(),
                    "java",
                    tree_sitter_java::HIGHLIGHTS_QUERY,
                    "",
                    "",
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Javascript => Some(CONFIG_JAVASCRIPT.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_javascript::language(),
                    "javascript",
                    tree_sitter_javascript::HIGHLIGHT_QUERY,
                    tree_sitter_javascript::INJECTIONS_QUERY,
                    tree_sitter_javascript::LOCALS_QUERY,
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Jsx => Some(CONFIG_JSX.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_javascript::language(),
                    "jsx",
                    tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
                    tree_sitter_javascript::INJECTIONS_QUERY,
                    tree_sitter_javascript::LOCALS_QUERY,
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Json => Some(CONFIG_JSON.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_json::language(),
                    "json",
                    tree_sitter_json::HIGHLIGHTS_QUERY,
                    "",
                    "",
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Lua => Some(CONFIG_LUA.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_lua::language(),
                    "lua",
                    tree_sitter_lua::HIGHLIGHTS_QUERY,
                    tree_sitter_lua::INJECTIONS_QUERY,
                    tree_sitter_lua::LOCALS_QUERY,
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Markdown => Some(CONFIG_MARKDOWN.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_md::language(),
                    "markdown",
                    // TODO: should this use the inline version at all?
                    tree_sitter_md::HIGHLIGHT_QUERY_BLOCK,
                    tree_sitter_md::INJECTION_QUERY_BLOCK,
                    "",
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Php => Some(CONFIG_PHP.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_php::language_php(),
                    "php",
                    tree_sitter_php::HIGHLIGHTS_QUERY,
                    tree_sitter_php::INJECTIONS_QUERY,
                    "",
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Python => Some(CONFIG_PYTHON.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_python::language(),
                    "python",
                    tree_sitter_python::HIGHLIGHTS_QUERY,
                    "",
                    "",
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Ruby => Some(CONFIG_RUBY.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_ruby::language(),
                    "ruby",
                    tree_sitter_ruby::HIGHLIGHTS_QUERY,
                    "",
                    tree_sitter_ruby::LOCALS_QUERY,
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Rust => Some(CONFIG_RUST.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_rust::language(),
                    "rust",
                    tree_sitter_rust::HIGHLIGHTS_QUERY,
                    tree_sitter_rust::INJECTIONS_QUERY,
                    "",
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Swift => Some(CONFIG_SWIFT.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_swift::language(),
                    "swift",
                    tree_sitter_swift::HIGHLIGHTS_QUERY,
                    tree_sitter_swift::INJECTIONS_QUERY,
                    tree_sitter_swift::LOCALS_QUERY,
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Toml => Some(CONFIG_TOML.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_toml_ng::language(),
                    "toml",
                    tree_sitter_toml_ng::HIGHLIGHTS_QUERY,
                    "",
                    "",
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Typescript => Some(CONFIG_TYPESCRIPT.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_typescript::language_typescript(),
                    "typescript",
                    tree_sitter_typescript::HIGHLIGHTS_QUERY,
                    "",
                    tree_sitter_typescript::LOCALS_QUERY,
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Tsx => Some(CONFIG_TSX.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_typescript::language_tsx(),
                    "tsx",
                    tree_sitter_typescript::HIGHLIGHTS_QUERY,
                    "",
                    tree_sitter_typescript::LOCALS_QUERY,
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Yaml => Some(CONFIG_YAML.get_or_init(|| {
                let mut config = HighlightConfiguration::new(
                    tree_sitter_yaml::language(),
                    "yaml",
                    tree_sitter_yaml::HIGHLIGHTS_QUERY,
                    "",
                    "",
                )
                .unwrap();
                config.configure(HIGHLIGHT_NAMES);
                config
            })),
            Self::Plaintext => None,
        }
    }
}
