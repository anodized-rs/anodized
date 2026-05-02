use anodized::spec;

#[spec(
    requires: seq.is_sorted(),
    ensures: [
        *output <= seq.len(),
        seq[0..*output].iter().all(|item| item < value),
        seq[*output..].iter().all(|item| item >= value),
    ],
)]
fn find_insert_position<T: Ord>(seq: &[T], value: &T) -> usize {
    let mut i = 0;

    #[spec(
        maintains: seq[0..i].iter().all(|item| item < value),
        decreases: seq.len() - i,
    )]
    while i < seq.len() && seq[i] < *value {
        i += 1;
    }

    i
}
