use derive_new::new;
use derive_setters::Setters;
use serde::{Deserialize, Serialize};

/// Represents a span of text in the source file.
/// Spans use half-open byte range semantics `[start, end)`.
/// `end` refers to the position after the last character.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, new, Setters, Serialize, Deserialize)]
#[setters(prefix = "with_", strip_option)]
pub struct Span {
    /// Start byte offset.
    pub start: usize,
    /// End byte offset (exclusive).
    pub end: usize,
    /// Start line number (1-based).
    pub start_line: usize,
    /// Start column number (1-based).
    pub start_col: usize,
    /// End line number (1-based).
    pub end_line: usize,
    /// End column number (1-based).
    pub end_col: usize,
}

/// Represents non-semantic content like comments and whitespace.
#[derive(Debug, Clone, PartialEq, Eq, Hash, new, Setters, Serialize, Deserialize)]
#[setters(prefix = "with_", strip_option)]
pub struct Trivia {
    /// The content of the trivia (e.g., "# comment\n" or "  ").
    pub content: String,
    /// The kind of trivia.
    pub kind: TriviaKind,
    /// The span of the trivia.
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TriviaKind {
    Whitespace,
    Comment,
    Newline,
}
