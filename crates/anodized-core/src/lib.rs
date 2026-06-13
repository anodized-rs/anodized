#![doc = include_str!("../README.md")]

use proc_macro2::Span;
use syn::{Error, Expr, ExprClosure, Meta, Pat};

use crate::qualifiers::FnQualifiers;

pub mod annotate;
pub mod instrument;
pub mod qualifiers;

#[cfg(test)]
mod test_util;

/// Specifies the intended behavior of a function or method: `fn`.
#[derive(Debug)]
// TODO: Rename to `FnSpec` to reduce ambiguity.
pub struct Spec {
    /// Qualifiers that constrain the behavior of the computation.
    pub qualifiers: FnQualifiers,
    /// Preconditions: conditions that must hold when the function is called.
    pub requires: Vec<PreCondition>,
    /// Invariants: conditions that must hold both when the function is called and when it returns.
    pub maintains: Vec<PreCondition>,
    /// Captures: expressions to snapshot at function entry for use in postconditions.
    pub captures: Vec<Capture>,
    /// Postconditions: conditions that must hold when the function returns.
    pub ensures: Vec<PostCondition>,
    /// The span in the source code, from which this spec was parsed.
    span: Span,
}

impl Spec {
    /// Empty spec that contains no elements.
    pub fn empty() -> Self {
        Self {
            qualifiers: FnQualifiers::empty(),
            requires: vec![],
            maintains: vec![],
            captures: vec![],
            ensures: vec![],
            span: Span::call_site(),
        }
    }

    /// Returns `true` if the spec is empty (specifies nothing), otherwise returns `false`.
    pub fn is_empty(&self) -> bool {
        self.qualifiers.is_empty()
            && self.requires.is_empty()
            && self.maintains.is_empty()
            && self.ensures.is_empty()
            && self.captures.is_empty()
    }

    /// Construct an error from the whole spec.
    pub fn spec_err(&self, message: &str) -> Error {
        Error::new::<&str>(self.span, message)
    }
}

/// Specifies the intended behavior of a data type: `struct` or `enum`.
#[derive(Debug)]
pub struct DataSpec {
    /// Invariants: conditions that must hold for all instances of the data type.
    pub maintains: Vec<PreCondition>,
    /// The span in the source code, from which this spec was parsed.
    span: Span,
}

impl DataSpec {
    /// Empty spec that contains no elements.
    pub fn empty() -> Self {
        Self {
            maintains: vec![],
            span: Span::call_site(),
        }
    }

    /// Returns `true` if the spec is empty (specifies nothing), otherwise returns `false`.
    pub fn is_empty(&self) -> bool {
        self.maintains.is_empty()
    }

    /// Construct an error from the whole spec.
    pub fn spec_err(&self, message: &str) -> Error {
        Error::new::<&str>(self.span, message)
    }
}

/// Specifies the intended behavior of a loop: `while` or `for`.
#[derive(Debug)]
pub struct LoopSpec {
    /// Loop invariants: conditions that must hold both before and after the loop's body runs.
    pub maintains: Vec<LoopInvariant>,
    /// Loop variant: an expression that decreases with each run of the loop's body.
    pub decreases: Option<LoopVariant>,
    /// The span in the source code, from which this spec was parsed.
    span: Span,
}

impl LoopSpec {
    /// Empty spec that contains no elements.
    pub fn empty() -> Self {
        Self {
            maintains: vec![],
            decreases: None,
            span: Span::call_site(),
        }
    }

    /// Returns `true` if the spec is empty (specifies nothing), otherwise returns `false`.
    pub fn is_empty(&self) -> bool {
        self.maintains.is_empty() && self.decreases.is_none()
    }

    /// Construct an error from the whole spec.
    pub fn spec_err(&self, message: &str) -> Error {
        Error::new::<&str>(self.span, message)
    }
}

/// A precondition represented by a `bool`-valued expression.
#[derive(Debug)]
// TODO: Rename to `Condition` for clarity.
pub struct PreCondition {
    /// The closure that validates the precondition,
    /// takes no input, e.g. `|| input.is_valid()`.
    pub closure: ExprClosure,
    /// **Static analyzers can safely ignore this field.**
    ///
    /// Build configuration filter to decide whether to add runtime checks.
    /// Passed to a `cfg!()` guard in the instrumented function.
    pub cfg: Option<Meta>,
}

/// A postcondition represented by a closure that takes the return value as a reference.
#[derive(Debug)]
pub struct PostCondition {
    /// The closure that validates the postcondition, taking the function's
    /// return value by reference, e.g. `|output| *output > 0`.
    pub closure: ExprClosure,
    /// **Static analyzers can safely ignore this field.**
    ///
    /// Build configuration filter to decide whether to add runtime checks.
    /// Passed to a `cfg!()` guard in the instrumented function.
    pub cfg: Option<Meta>,
}

/// Captures an expression's value at function entry.
#[derive(Debug)]
pub struct Capture {
    /// The expression to capture.
    pub expr: Expr,
    /// The pattern to bind/destructure the captured value.
    pub pat: Pat,
}

/// Holds true before and after each iteration of a loop's body.
#[derive(Debug)]
pub struct LoopInvariant {
    /// The closure that defines the invariant.
    ///
    /// On a `for` loop, the closure's single argument is the logical index.
    /// On a `while` loop, the closure has no arguments.
    pub closure: ExprClosure,
    /// **Static analyzers can safely ignore this field.**
    ///
    /// Build configuration filter to decide whether to add runtime checks.
    /// Passed to a `cfg!()` guard in the instrumented code.
    pub cfg: Option<Meta>,
}

/// Decreases with each run of a loop's body.
#[derive(Debug)]
pub struct LoopVariant {
    /// The expression that defines the variant.
    pub expr: Expr,
    /// **Static analyzers can safely ignore this field.**
    ///
    /// Build configuration filter to decide whether to add runtime checks.
    /// Passed to a `cfg!()` guard in the instrumented code.
    pub cfg: Option<Meta>,
}
