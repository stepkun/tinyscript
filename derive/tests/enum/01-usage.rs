//! Test correct usage of `tinyscript` enum derive macro `ScriptEnum`
// Copyright © 2025 Stephan Kunz

#[doc(hidden)]
extern crate alloc;

#[derive(tinyscript_derive::ScriptEnum)]
enum TestEnum {
	CaseA,
	CaseB,
}

// dummy main
fn main() {}
