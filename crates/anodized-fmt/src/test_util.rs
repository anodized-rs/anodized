//! This module contains utilities meant only for testing.
//! Do not use outside of tests.

/// Inserts random comments into a fragment of well-formed Rust code.
///
/// Valid positions to insert a comment are:
/// - inside whitespace
/// - at token boundaries
pub fn inject_comments(source: &str) -> String {
    todo!()
}

/// Checks that each comment is indented correctly in a fragment of well-formed Rust code.
///
/// A comment is indented correctly in any of the following cases:
/// - The following token starts on the same line.
/// - The following token starts on a different line, and has the same indentation level.
/// - *???* The following token is a group terminator: `)`, `]`, or `}`, indented to level one less.
/// - *???* There's no following token.
pub fn is_comment_indentation_valid(source: &str) -> bool {
    todo!()
}

/// Breaks a fragment of well-formed Rust code into a sequence of elements.
pub fn break_into_elems(source: &str) -> Vec<Elem<&str>> {
    todo!()
}

pub struct Elem<Text> {
    pub text: Text,
    pub tag: Tag,
}

pub enum Tag {
    /// A single line break (either "\n" or "\r\n").
    Linebreak,
    /// A span of whitespace characters, without line breaks.
    Whitespace,
    /// A line (doc)comment, including its terminating line break.
    LineComment,
    /// A block (doc)comment.
    BlockComment,
    /// A span of punctuation characters.
    Punct,
    /// Other things: identifiers, literals, etc.
    Other,
}

/// Returns whether an (ordered) pair of characters is "glued".
///
/// The sequence `AB` is glued iff both characters are punctuation, and there's a valid Rust program in which `AB` appears as a punctuation sequence (e.g. inside an operator), and inserting a space between them (`A<space>B` makes the program invalid.
pub fn is_punct_pair_glued(a: char, b: char) -> bool {
    todo!()
}

const GLUED_PAIRS: [&'static str; 8] = ["==", "!=", "<=", ">=", "->", "=>", "<<", ">>"];
