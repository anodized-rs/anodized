//! This module contains utilities meant only for testing.
//! Do not use outside of tests.

use arbitrary::Arbitrary;

/// A template to generate formatting variations of a piece of code.
#[derive(Debug, Clone, Default)]
pub struct Template(Vec<Span>);

#[derive(Debug, Clone)]
pub enum Span {
    /// A fixed string.
    F(String),
    /// Zero or more whitespace characters.
    Z,
    /// One or more whitespace characters.
    P,
}

/// Describes a variation the template can generate.
#[derive(Debug, Clone, Default, Arbitrary)]
pub struct Variation(Vec<Whitespace>);

impl Template {
    /// New empty template.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a placeholder for zero or more whitespace characters.
    pub fn z(mut self) -> Self {
        self.0.push(Span::Z);
        self
    }

    /// Add a placeholder for one or more whitespace characters.
    pub fn p(mut self) -> Self {
        self.0.push(Span::P);
        self
    }

    /// Add a fixed span of text.
    pub fn fixed(mut self, text: &str) -> Self {
        self.0.push(Span::F(text.to_string()));
        self
    }

    /// Add tokens, replacing each internal span of whitespace with a `.z()`.
    pub fn tokens(mut self, text: &str) -> Self {
        for (i, non_whitespace) in text.split_whitespace().enumerate() {
            if i > 0 {
                self.0.push(Span::Z);
            }
            self.0.push(Span::F(non_whitespace.to_string()));
        }
        self
    }
}

impl Template {
    /// Generates an instantiation of the template based on variation information.
    pub fn generate(&self, variation: Variation) -> String {
        let mut output = String::new();

        let mut whitespaces = variation.0.into_iter();

        for span in &self.0 {
            match span {
                Span::F(text) => output.push_str(text),
                Span::Z => {
                    let whitespace = whitespaces.next().unwrap_or_default();
                    output.push_str(&whitespace.to_string());
                }
                Span::P => {
                    let whitespace = whitespaces.next().unwrap_or_default();
                    output.push_str(&whitespace.to_non_empty_string());
                }
            }
        }

        output
    }
}

/// A non-empty sequence of whitespace characters.
#[derive(Debug, Clone, Arbitrary)]
pub struct Whitespace(Vec<WsChar>, WsChar);

impl Default for Whitespace {
    fn default() -> Self {
        Whitespace(vec![], WsChar::Space)
    }
}

impl Whitespace {
    fn to_string(self: Whitespace) -> String {
        self.0.into_iter().map(char::from).collect()
    }

    fn to_non_empty_string(self: Whitespace) -> String {
        self.0
            .into_iter()
            .chain(std::iter::once(self.1))
            .map(char::from)
            .collect()
    }
}

/// A code point Rust recognizes as whitespace.
#[derive(Debug, Clone, Copy, Arbitrary)]
pub enum WsChar {
    Space,
    Tab,
    Newline,
    // TODO: expand
}

impl From<WsChar> for char {
    fn from(value: WsChar) -> Self {
        match value {
            WsChar::Space => ' ',
            WsChar::Tab => '\t',
            WsChar::Newline => '\n',
        }
    }
}
