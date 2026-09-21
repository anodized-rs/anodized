#![cfg_attr(anodized_charon, feature(register_tool))]
#![cfg_attr(anodized_charon, register_tool(charon))]

use anodized::{spec, types::Spec};

#[spec(maintains: self.a.pow(2) + self.b.pow(2) == self.c.pow(2))]
pub struct PythagoreanTriple {
    #[unspec]
    a: u32,
    #[unspec]
    b: u32,
    #[unspec]
    c: u32,
}

#[spec(maintains: !self.0.is_empty())]
pub struct NonEmptyVec<T: Spec>(Vec<T>);

#[spec(maintains: self.0.iter().rev().eq(&self.0))]
pub struct PalindromeVec<T: Eq + Spec>(Vec<T>);

#[spec(
    maintains: (&self.0).into_iter().rev().eq((&self.0).into_iter())
)]
pub struct PalindromeContainer<T: Eq + Spec, C: Spec>(C)
where
    for<'a> &'a C: IntoIterator<Item = &'a T>,
    for<'a> <&'a C as IntoIterator>::IntoIter: DoubleEndedIterator;

#[spec(
    maintains: [
        self.size <= BUFFER_SIZE,
        str::from_utf8(&self.buffer[0..self.size]).is_ok(),
    ]
)]
pub struct SliceBackedString<const BUFFER_SIZE: usize = 1024> {
    #[unspec]
    size: usize,
    #[unspec]
    buffer: [u8; BUFFER_SIZE],
}

#[spec(
    maintains: std::mem::size_of::<T>() * DIM * 8 == SIMD_BITS
)]
pub struct SimdVector<const DIM: usize, T = f32, const SIMD_BITS: usize = 128>(#[unspec] [T; DIM]);
