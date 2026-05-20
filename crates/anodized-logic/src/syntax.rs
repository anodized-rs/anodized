pub use crate::opaque;

/// An opaque expression.
///
/// Useful to pass through a term that cannot be rendered as a Rust expression.
/// The semantics of an opaque expression is defined only by the interpreting backend.
#[macro_export]
macro_rules! opaque {
    ($($t:tt)*) => {{
        #[allow(clippy::diverging_sub_expression)]
        let bottom: bool = panic!("Cannot run `opaque!` expression: {}", stringify!($($t)*));
        #[allow(unreachable_code)]
        bottom
    }}
}
