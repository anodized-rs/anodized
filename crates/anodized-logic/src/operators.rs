pub use crate::implies;

/// Material implication: `a --> b`.
///
/// Note that `implies!(a, b)` is equivalent to `!a || b`, evaluating `b` lazily. Because function
///   arguments are evaluated eagerly, `implies` must be a macro.
#[macro_export]
macro_rules! implies {
    ($p:expr, $q:expr) => {
        if $p { $q } else { true }
    };
}
