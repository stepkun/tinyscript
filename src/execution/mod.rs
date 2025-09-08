// Copyright © 2025 Stephan Kunz
//! Execution implementations.

mod chunk;
mod error;
pub mod op_code;
mod scripting_value;
mod vm;

// flatten
pub use chunk::Chunk;
pub use error::{ExecutionError, ExecutionResult};
pub use scripting_value::ScriptingValue;
pub use vm::VM;
