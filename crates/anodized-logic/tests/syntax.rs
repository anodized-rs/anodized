use anodized_logic::opaque;

#[test]
#[should_panic = "Cannot run `opaque!` expression: some_backend(a --> b)"]
fn execution_fails() {
    let _ = opaque!(some_backend(a --> b));
}
