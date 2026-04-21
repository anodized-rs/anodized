use anodized_logic::quantifiers::{exists, forall};

fn main() {
    // exists over naked expression
    let _ = exists(1 + 2 == 3);

    // forall over naked expression
    let _ = forall(1 + 2 == 3);

    // exists over 0-var predicate
    let _ = exists(|| 1 + 2 == 3);

    // forall over 0-var predicate
    let _ = forall(|| 1 + 2 == 3);

    // exists over non-bool expression
    let _ = exists(|i: i32| i + i);

    // forall over non-bool expression
    let _ = forall(|i: i32| i + i);
}
