pub use crate::opaque;

/// Embed Rust syntax as an opaque expression.
///
/// Useful to pass through a term that cannot be rendered as a valid Rust expression.
/// The semantics of an opaque expression is defined only by the interpreting backend.
#[macro_export]
macro_rules! opaque {
    ($($t:tt)*) => {
        $crate::syntax::opaque_str(stringify!($($t)*))
    }
}

/// Embed a string as an opaque expression.
///
/// You probably want the [`opaque!`] macro instead.
pub fn opaque_str(expr: &str) -> bool {
    panic!("Cannot run `opaque!` expression: {}", expr)
}
