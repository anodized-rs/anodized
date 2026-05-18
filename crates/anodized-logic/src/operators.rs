/// Material implication.
///
/// Note that `a --> b` is equivalent to `!a || b`, which evaluates `b` lazily. Because function
///   arguments are evaluated eagerly, `implies` must be a macro.
#[macro_export]
macro_rules! implies {
    ($p:expr, $q:expr) => {
        if $p { $q } else { true }
    };
}
