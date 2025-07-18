// Copyright © 2025 Stephan Kunz

//! Test wrong usage of `tinyscript` enum derive macro `ScriptEnum` 

#[tinyscript_derive::ScriptEnum]
enum TestEnum {
    CaseA,
    CaseB,
}

// dummy main
fn main(){}
