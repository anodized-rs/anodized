use anodized::spec;

//////////////////////////
// Test runtime checks. //
//////////////////////////

#[spec]
trait Counter {
    #[spec(
        requires: x >= 0,
        ensures: *output >= x,
    )]
    fn bump(&self, x: i32) -> i32;
}

struct ValidNarrowing;

#[spec]
impl Counter for ValidNarrowing {
    #[spec(
        // Weaker than trait precondition: accepts a superset.
        requires: x >= -10,
        // Stronger than trait postcondition.
        ensures: *output > x,
    )]
    fn bump(&self, x: i32) -> i32 {
        x + 1
    }
}

struct StrongerImplPre;

#[spec]
impl Counter for StrongerImplPre {
    #[spec(
        // Stronger than trait precondition: this is not a valid narrowing.
        requires: x > 0,
        ensures: *output >= x,
    )]
    fn bump(&self, x: i32) -> i32 {
        x
    }
}

struct WeakerImplPost;

#[spec]
impl Counter for WeakerImplPost {
    #[spec(
        // This postcondition is weaker than the trait postcondition.
        ensures: true,
    )]
    fn bump(&self, x: i32) -> i32 {
        x - 1
    }
}

#[test]
fn runtime_allows_valid_narrowing() {
    let c = ValidNarrowing;
    assert_eq!(c.bump(0), 1);
    assert_eq!(c.bump(10), 11);
}

#[cfg(anodized_panic)]
#[test]
#[should_panic(expected = "Precondition failed: x > 0")]
fn runtime_rejects_stronger_impl_precondition() {
    let c = StrongerImplPre;
    // Satisfies trait precondition (`x >= 0`) but violates impl precondition (`x > 0`).
    let _ = c.bump(0);
}

#[cfg(anodized_panic)]
#[test]
#[should_panic(expected = "Postcondition failed: | output | * output >= x")]
fn runtime_rejects_weaker_impl_postcondition() {
    let c = WeakerImplPost;
    // Impl postcondition (`true`) passes, but trait postcondition fails.
    let _ = c.bump(5);
}

/////////////////////////////////////////////////////
// Test only instrumentation on more complex code. //
/////////////////////////////////////////////////////

#[spec]
trait Matrix<T> {
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

struct DiagonalMatrix<T>(Vec<T>);

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
