use anodized::spec;

#[spec(
    // Should fail: closure-form is no longer allowed.
    ensures: |x, y| x > 0,
)]
fn returns_positive() -> i32 {
    42
}

#[spec(
    // Should fail: closure-form is no longer allowed.
    ensures: || true,
)]
fn returns_something() -> i32 {
    42
}

fn main() {}
