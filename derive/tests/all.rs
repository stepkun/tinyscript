//! Run all tests for `tinyscript` macros
// Copyright © 2025 Stephan Kunz

#[test]
fn tests() {
	let t = trybuild::TestCases::new();
	t.pass("tests/enum/01-usage.rs");
	t.compile_fail("tests/enum/02-wrong-usage.rs");
}
