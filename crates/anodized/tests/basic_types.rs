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

#[spec(maintains:
    (&self.0).into_iter().rev().eq((&self.0).into_iter())
)]
struct PalindromeContainer<T: Eq, C>(C)
where
    for<'a> &'a C: IntoIterator<Item = &'a T>,
    for<'a> <&'a C as IntoIterator>::IntoIter: DoubleEndedIterator;
