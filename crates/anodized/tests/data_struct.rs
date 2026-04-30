use anodized::spec;

#[spec(maintains: self.a.pow(2) + self.b.pow(2) == self.c.pow(2))]
struct PythagoreanTriple {
    a: u32,
    b: u32,
    c: u32,
}

#[spec(maintains: !self.0.is_empty())]
struct NonEmptyVec<T>(Vec<T>);

#[spec(maintains: self.0.iter().rev().eq(&self.0))]
struct PalindromeVec<T: Eq>(Vec<T>);

#[spec(
    maintains: (&self.0).into_iter().rev().eq((&self.0).into_iter())
)]
struct PalindromeContainer<T: Eq, C>(C)
where
    for<'a> &'a C: IntoIterator<Item = &'a T>,
    for<'a> <&'a C as IntoIterator>::IntoIter: DoubleEndedIterator;

#[spec(
    maintains: [
        self.size <= BUFFER_SIZE,
        str::from_utf8(&self.buffer[0..self.size]).is_ok(),
    ]
)]
struct SliceBackedString<const BUFFER_SIZE: usize = 1024> {
    size: usize,
    buffer: [u8; BUFFER_SIZE],
}

#[spec(
    maintains: std::mem::size_of::<T>() * DIM * 8 == SIMD_BITS
)]
struct SimdVector<const DIM: usize, T = f32, const SIMD_BITS: usize = 128>([T; DIM]);
