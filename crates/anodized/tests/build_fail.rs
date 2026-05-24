#[test]
fn qualifier_narrowing_fails_to_compile() {
    let status = std::process::Command::new("cargo")
        .arg("build")
        .current_dir("tests/build_fail/trait_narrowing")
        .status()
        .expect("failed to run cargo");
    assert!(!status.success(), "expected compilation to fail");
}
