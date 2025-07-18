// Copyright © 2025 Stephan Kunz

//! Virtual Machine of `tinyscript`

mod chunk;
pub mod op_code;
mod scripting_value;
mod vm;

// flatten
pub use chunk::Chunk;
pub use scripting_value::ScriptingValue;
pub use vm::VM;
