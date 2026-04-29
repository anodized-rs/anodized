use anodized::spec;

#[spec(maintains: self.a * self.a + self.b * self.b == self.c * self.c)]
struct PythagoreanTriple {
    a: u32,
    b: u32,
    c: u32,
}
