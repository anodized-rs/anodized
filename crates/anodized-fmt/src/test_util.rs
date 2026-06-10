//! This module contains utilities meant only for testing.
//! Do not use outside of tests.

/// A template to generate formatting variations of a piece of code.
pub struct Template(Vec<Span>);

pub enum Span {
    /// A fixed string.
    F(&'static str),
    /// Zero or more whitespace characters.
    Z,
    /// One or more whitespace characters.
    P,
}

impl Span {}

impl Template {
    /// Generates a variation of the template.
    pub fn generate(&self, random_source: &[u8]) -> String {
        let mut output = String::new();

        for span in &self.0 {
            todo!()
        }

        output
    }
}

fn example_template() -> Template {
    use Span::*;
    // requires: x > 0, ensures: *output > 0,
    #[rustfmt::skip]
    let template = Template(vec![
        Z, F("requires"), Z, F(":"), Z, F("x"), Z, F(">"), Z, F("0"), Z, F(","),
        Z, F("ensures"), Z, F(":"), Z, F("*"), Z, F("output"), Z, F(">"), Z, F("0"),
        Z,
    ]);
    template
}
