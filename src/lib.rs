// Copyright © 2025 Stephan Kunz
#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

#[doc(hidden)]
extern crate alloc;

pub mod compilation;
pub mod environment;
pub mod error;
pub mod execution;
pub mod runtime;
pub mod scripting_value;

// flatten
pub use environment::{DefaultEnvironment, Environment};
pub use error::{Error, Result};
pub use execution::Chunk;
pub use runtime::{Runtime, SharedRuntime};
pub use scripting_value::ScriptingValue;

// reexport
pub use tinyscript_derive::ScriptEnum;

// region:		--- modules
use alloc::{sync::Arc, vec::Vec};
// endregion:	--- modules

// region		--- types
/// An immutable thread safe `String` type.
/// see: [Logan Smith](https://www.youtube.com/watch?v=A4cKi7PTJSs).
type ConstString = Arc<str>;
// endregion:   --- types

// region		--- ScriptEnum
/// The trait for script enums.
pub trait ScriptEnum {
	/// Function to get key-value tuples for registering.
	fn key_value_tuples<'a>() -> Vec<(&'a str, i8)>;
}
// endregion:   --- ScriptEnum
