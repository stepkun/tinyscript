// Copyright © 2025 Stephan Kunz

//! Tests

use tinyscript::{
    Runtime,
    compiling::{Lexer, Parser},
    execution::VM,
};

// check, that the auto traits are available
const fn is_normal<T: Sized + Send + Sync>() {}

#[test]
const fn normal_types() {
    is_normal::<Lexer>();
    is_normal::<Parser>();
    is_normal::<Runtime>();
    is_normal::<VM>();
}
