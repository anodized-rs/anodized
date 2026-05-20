pub use crate::opaque;

#[macro_export]
macro_rules! opaque {
    ($($t:tt)*) => {{
        let bottom: bool = panic!("Cannot run `opaque!` expression: {}", stringify!($($t)*));
        #[allow(unreachable_code)]
        bottom
    }}
}
