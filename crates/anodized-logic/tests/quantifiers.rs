use anodized_logic::quantifiers::{exists, forall};

#[test]
#[should_panic(expected = "Runtime checks are not supported for quantifiers.")]
fn exists_with_1_var() {
    let _ = exists(|i: i32| i < 42);
}

#[test]
#[should_panic(expected = "Runtime checks are not supported for quantifiers.")]
fn forall_with_1_var() {
    let _ = forall(|i: i32| i < 42);
}

#[test]
#[should_panic(expected = "Runtime checks are not supported for quantifiers.")]
fn exists_with_5_var() {
    let _ = exists(|i: i32, j: usize, k: bool, x: f32, y: f32| (i as usize) < j && k == (x != y));
}

#[test]
#[should_panic(expected = "Runtime checks are not supported for quantifiers.")]
fn forall_with_5_var() {
    let _ = forall(|i: i32, j: usize, k: bool, x: f32, y: f32| (i as usize) < j && k == (x != y));
}
