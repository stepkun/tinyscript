//! Test wrong usage of `tinyscript` enum derive macro `ScriptEnum`
// Copyright © 2025 Stephan Kunz

#[tinyscript_derive::ScriptEnum]
enum TestEnum {
	CaseA,
	CaseB,
}

// dummy main
fn main() {}
