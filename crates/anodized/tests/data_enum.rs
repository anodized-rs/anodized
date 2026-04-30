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
