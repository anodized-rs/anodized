use anodized::spec;

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
    fn mul<Input: Matrix<T>, Output: Matrix<T>>(&self, input: Input) -> Output;
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
    fn mul<Input: Matrix<T>, Output: Matrix<T>>(&self, input: Input) -> Output {
        let _ = input;
        todo!()
    }
}
