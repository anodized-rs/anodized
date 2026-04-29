use anodized::spec;

#[spec(maintains: self.a.pow(2) + self.b.pow(2) == self.c.pow(2))]
struct PythagoreanTriple {
    a: u32,
    b: u32,
    c: u32,
}

#[spec(maintains: !self.0.is_empty())]
struct NonEmptyVec<T>(Vec<T>);
