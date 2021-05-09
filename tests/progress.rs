#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/no_args.rs");
    t.pass("tests/log_level_args.rs");
    t.pass("tests/unit_args.rs");
    t.pass("tests/log_and_unit_args.rs");
    t.compile_fail("tests/invalid_log_args.rs");
    t.compile_fail("tests/invalid_unit_args.rs");
}
