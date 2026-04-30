use anodized::spec;

#[spec(
    maintains: match self {
        Ascending(vec) => vec.is_sorted(),
        Descending(vec) => vec.iter().rev().is_sorted(),
    }
)]
#[allow(unused)]
enum MonotonicVec<T: Ord> {
    Ascending(Vec<T>),
    Descending(Vec<T>),
}

#[spec(
    maintains: match self {
        Small { count, .. } => *count <= UNBOXED_CAPACITY,
        Large(vec) => vec.len() > UNBOXED_CAPACITY,
    }
)]
#[allow(unused)]
enum SmallVec<T: Default, const UNBOXED_CAPACITY: usize = 128> {
    Small {
        count: usize,
        buffer: [T; UNBOXED_CAPACITY],
    },
    Large(Vec<T>),
}
