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
#[derive(Debug, Clone, Default, Arbitrary)]
pub struct Whitespace(Vec<WsChar>);

impl Whitespace {
    fn to_string(self: Whitespace) -> String {
        self.0.into_iter().map(char::from).collect()
    }

    fn to_non_empty_string(self: Whitespace) -> String {
        if !self.0.is_empty() {
            self.to_string()
        } else {
            " ".into()
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_variation_produces_minimal_whitespace() {
        let template = Template::new()
            .fixed("fn")
            .z() // zero-or-more -> empty
            .fixed("foo")
            .p() // one-or-more -> single space
            .fixed("()");

        let output = template.generate(Variation::default());
        assert_eq!(output, "fnfoo ()");
    }

    #[test]
    fn custom_variation_uses_provided_whitespace() {
        let template = Template::new().fixed("x").z().fixed("=").p().fixed("1");

        let variation = Variation(vec![
            Whitespace(vec![WsChar::Tab, WsChar::Newline]), // for z()
            Whitespace(vec![]),                             // for p()
        ]);

        let output = template.generate(variation);
        assert_eq!(output, "x\t\n= 1");
    }

    #[test]
    fn tokens_splits_on_whitespace() {
        let template = Template::new().tokens("a b  c");
        let output = template.generate(Variation::default());
        // Should be: "a" + Z + "b" + Z + "c"
        // With default variation, Z becomes empty
        assert_eq!(output, "abc");
    }

    #[test]
    fn whitespace_to_string_methods() {
        let ws1 = Whitespace(vec![WsChar::Tab, WsChar::Space]);
        assert_eq!(ws1.clone().to_string(), "\t ");
        assert_eq!(ws1.to_non_empty_string(), "\t ");

        let ws2 = Whitespace(vec![]);
        assert_eq!(ws2.clone().to_string(), "");
        assert_eq!(ws2.to_non_empty_string(), " ");

        let ws3 = Whitespace(vec![WsChar::Newline]);
        assert_eq!(ws3.clone().to_string(), "\n");
        assert_eq!(ws3.to_non_empty_string(), "\n");
    }
}
