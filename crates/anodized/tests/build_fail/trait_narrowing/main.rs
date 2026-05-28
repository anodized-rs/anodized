use anodized::spec;

/////////////////////////
// Test static checks. //
/////////////////////////

#[spec]
trait MinFinder<T: Copy + PartialOrd> {
    #[spec(
        total,
        requires: [
            input.len() > 0,
        ],
        ensures: [
            input.iter().all(|item| output <= item),
            input.iter().any(|item| output == item) || input.is_empty(),
        ],
    )]
    fn find_min(input: &[T]) -> T;
}

struct WeakerImplQualifiers<T>(std::marker::PhantomData<T>);

#[spec]
impl<T: Copy + PartialOrd> MinFinder<T> for WeakerImplQualifiers<T> {
    #[spec(
        // INVALID - Weaker than trait qualifiers: may panic or run forever.
        deterministic,
        requires: [],
        ensures: [
            input.iter().all(|item| output <= item),
            input.iter().any(|item| output == item),
        ],
    )]
    fn find_min(input: &[T]) -> T {
        let _: u32 = const { Self::__anodized_fn_qualifiers_find_min };
        let _ = input;
        // Panic on empty input.
        let mut min = input[0];
        for item in input.iter().copied() {
            if item < min {
                min = item;
            }
        }
        min
    }
}

fn main() {
    let seq = [5, -42, 3];
    // NOTE: The `fn` must be used for the compile-time check to fire.
    assert_eq!(WeakerImplQualifiers::<i32>::find_min(&seq), -42);
}
