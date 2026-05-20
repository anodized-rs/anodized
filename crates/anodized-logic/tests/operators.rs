use anodized_logic::implies;

#[test]
#[allow(clippy::bool_assert_comparison)]
fn truth_table() {
    assert_eq!(implies!(false, false), true);
    assert_eq!(implies!(false, true), true);
    assert_eq!(implies!(true, false), false);
    assert_eq!(implies!(true, true), true);
}

#[test]
#[allow(clippy::bool_assert_comparison)]
fn lazy_evaluation() {
    assert_eq!(implies!(false, panic!("failure")), true);
}
