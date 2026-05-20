use anodized::spec;

//////////////////////////
// Test runtime checks. //
//////////////////////////

#[spec]
trait MinFinder {
    #[spec(
        requires: [
            input.len() > 0,
        ],
        ensures: [
            input.iter().all(|item| output <= item),
            input.iter().any(|item| output == item) || input.len() == 0,
        ],
    )]
    fn find_min(input: &[f32]) -> f32;
}

pub struct ValidNarrowing;

#[spec]
impl MinFinder for ValidNarrowing {
    #[spec(
        // Weaker than trait precondition: allows `input` to be empty.
        requires: [],
        // Stronger than trait postcondition: clarifies what to output when `input` is empty.
        ensures: [
            input.iter().all(|item| output <= item),
            input.iter().any(|item| output == item)
                || (input.len() == 0 && *output == f32::INFINITY),
        ],
    )]
    #[warn(unused_comparisons)]
    fn find_min(input: &[f32]) -> f32 {
        let mut min = f32::INFINITY;
        for item in input.iter().copied() {
            if item < min {
                min = item;
            }
        }
        min
    }
}

pub struct StrongerImplPre;

#[spec]
impl MinFinder for StrongerImplPre {
    #[spec(
        // INVALID - Stronger than trait precondition: requires sorted `input`.
        requires: [
            input.len() > 0,
            input.is_sorted(),
        ],
        ensures: [
            input.iter().all(|item| output <= item),
            input.iter().any(|item| output == item) || input.len() == 0,
        ],
    )]
    fn find_min(input: &[f32]) -> f32 {
        input[0]
    }
}

pub struct WeakerImplPost;

#[spec]
impl MinFinder for WeakerImplPost {
    #[spec(
        requires: [
            input.len() > 0,
        ],
        // INVALID - Weaker than trait postcondition: `input` may be ignored completely.
        ensures: [
            input.iter().all(|item| output <= item),
        ],
    )]
    fn find_min(input: &[f32]) -> f32 {
        let _ = input;
        f32::NEG_INFINITY
    }
}

const TEST_INPUT: [f32; 3] = [5.0, -42.0, std::f32::consts::PI];

#[test]
fn runtime_allows_valid_narrowing() {
    // NOTE: The trait's runtime checks are active even when the concrete type is statically known.
    assert_eq!(ValidNarrowing::find_min(&TEST_INPUT), -42.0);
}

#[cfg(anodized_panic)]
#[test]
#[should_panic(expected = "Precondition failed: input.is_sorted()")]
fn runtime_rejects_stronger_impl_precondition() {
    // NOTE: The trait's runtime checks are active even when the concrete type is statically known.
    assert_eq!(StrongerImplPre::find_min(&TEST_INPUT), -42.0);
}

#[cfg(anodized_panic)]
#[test]
#[should_panic(expected = "\
Postcondition failed: | output | input.iter().any(| item | output == item) || input.len() == 0")]
fn runtime_rejects_weaker_impl_postcondition() {
    // NOTE: The trait's runtime checks are active even when the concrete type is statically known.
    assert_eq!(WeakerImplPost::find_min(&TEST_INPUT), -42.0);
}

//////////////////////////////////////////////////////
// Smoke-test instrumentation on more complex code. //
//////////////////////////////////////////////////////

#[spec]
pub trait Matrix<T> {
    fn count_rows(&self) -> usize;
    fn count_cols(&self) -> usize;

    #[spec(
        requires: [
            input.count_rows() == self.count_cols(),
        ],
        ensures: [
            output.count_rows() == self.count_rows(),
            output.count_cols() == input.count_cols(),
        ],
    )]
    fn mul<Input: Matrix<T>, Output: Matrix<T>>(&self, input: &Input) -> Output;
}

pub struct DiagonalMatrix<T>(Vec<T>);

#[spec]
impl<T> Matrix<T> for DiagonalMatrix<T> {
    #[spec(ensures: *output == self.count_cols())]
    fn count_rows(&self) -> usize {
        self.0.len()
    }

    #[spec(ensures: *output == self.count_rows())]
    fn count_cols(&self) -> usize {
        self.0.len()
    }

    #[spec(
        requires: [
            input.count_rows() == self.count_cols(),
        ],
        ensures: [
            output.count_rows() == input.count_rows(),
            output.count_cols() == input.count_cols(),
        ],
    )]
    fn mul<Input: Matrix<T>, Output: Matrix<T>>(&self, input: &Input) -> Output {
        let _ = input;
        todo!()
    }
}
