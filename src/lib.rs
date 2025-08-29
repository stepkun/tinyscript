// Copyright © 2025 Stephan Kunz
#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

#[doc(hidden)]
extern crate alloc;

pub mod compiling;
pub mod environment;
pub mod error;
pub mod execution;
pub mod runtime;

// flatten
pub use environment::{DefaultEnvironment, Environment};
pub use error::Error;
pub use execution::Chunk;
pub use runtime::{Runtime, SharedRuntime};

// reexport
pub use tinyscript_derive::ScriptEnum;

// region:		--- modules
use alloc::{sync::Arc, vec::Vec};
// endregion:	--- modules

/// Global constant for expect statements that should never happen
#[doc(hidden)]
pub const SHOULD_NOT_HAPPEN: &str = "should not happen";

// region		--- types
/// An immutable thread safe `String` type
/// see: [Logan Smith](https://www.youtube.com/watch?v=A4cKi7PTJSs).
pub type ConstString = Arc<str>;
// endregion:   --- types

/// The trait for script enums.
pub trait ScriptEnum {
	/// Function to get key-value tuples for registering.
	fn key_value_tuples<'a>() -> Vec<(&'a str, i8)>;
}
