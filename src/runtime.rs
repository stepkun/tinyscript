// Copyright © 2025 Stephan Kunz
//! A runtime for executing tinyscript code.

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{collections::btree_map::BTreeMap, string::String, sync::Arc};
use spin::Mutex;

use crate::{
	compilation::Parser,
	environment::Environment,
	error::Error,
	execution::{Chunk, VM},
	scripting_value::ScriptingValue,
};

#[cfg(feature = "std")]
use std::vec::Vec;
// endregion:   --- modules

// region:      --- types
/// Defines a shared [`Runtime`].
pub type SharedRuntime = Arc<Mutex<Runtime>>;
// endregion:   --- types

// region:      --- Runtime
/// Runtime to execute tinyscript.
#[derive(Debug, Default)]
pub struct Runtime {
	parser: Parser,
	vm: VM,
	enums: BTreeMap<String, i8>,
	#[cfg(feature = "std")]
	stdout: Vec<u8>,
}

/// Cloning a Runtime is cloning the environment.
/// Parser, VM and stdout are created new.
impl Clone for Runtime {
	fn clone(&self) -> Self {
		Self {
			parser: Parser::default(),
			vm: VM::default(),
			enums: self.enums.clone(),
			#[cfg(feature = "std")]
			stdout: Vec::new(),
		}
	}
}

impl Runtime {
	/// Inserts an enum value.
	/// # Errors
	/// - [`Error::DuplicateEnumVariant`] if en enum definition (key) already exists.
	pub fn register_enum_tuple(&mut self, key: &str, value: i8) -> Result<(), Error> {
		if let Some(old_value) = self.enums.get(key) {
			return Err(Error::DuplicateVariant {
				name: key.into(),
				old: *old_value,
				new: value,
			});
		}
		self.enums.insert(key.into(), value);
		Ok(())
	}

	/// Get the discriminant of an enum value if it exists
	#[must_use]
	pub fn enum_discriminant(&self, name: &str) -> Option<i8> {
		self.enums.get(name).copied()
	}

	/// Parse a scripting source.
	/// # Errors
	/// - [`Error::Compilation`] if script is invalid
	pub fn parse(&mut self, script: &str) -> Result<Chunk, Error> {
		let chunk = self.parser.parse(&self.enums, script)?;
		Ok(chunk)
	}

	/// Execute a bytecode chunk.
	/// # Errors
	/// - [`Error::Execution`] if script cannot be executed.
	pub fn execute(&mut self, chunk: &Chunk, globals: &mut dyn Environment) -> Result<ScriptingValue, Error> {
		#[cfg(not(feature = "std"))]
		let res = self.vm.run(chunk, globals)?;
		#[cfg(feature = "std")]
		let res = self.vm.run(chunk, globals, &mut self.stdout)?;
		Ok(res)
	}

	/// Compiles and runs the new script without clearing stdout.
	/// # Errors
	/// - [`Error::Compilation`] if script is invalid.
	/// - [`Error::Execution`] if script cannot be executed.
	pub fn continue_run(&mut self, script: &str, globals: &mut dyn Environment) -> Result<ScriptingValue, Error> {
		let chunk = self.parser.parse(&self.enums, script)?;
		#[cfg(not(feature = "std"))]
		let res = self.vm.run(&chunk, globals)?;
		#[cfg(feature = "std")]
		let res = self.vm.run(&chunk, globals, &mut self.stdout)?;
		Ok(res)
	}

	/// Run a script.
	/// Clears stdout before execution.
	/// # Errors
	/// - [`Error::Execution`] if script cannot be executed.
	pub fn run(&mut self, script: &str, globals: &mut dyn Environment) -> Result<ScriptingValue, Error> {
		#[cfg(feature = "std")]
		self.stdout.clear();
		self.continue_run(script, globals)
	}

	/// Returns the stdout.
	#[cfg(feature = "std")]
	#[must_use]
	pub const fn stdout(&self) -> &Vec<u8> {
		&self.stdout
	}

	/// Clears runtimes stdout.
	pub fn clear(&mut self) {
		#[cfg(feature = "std")]
		self.stdout.clear();
	}
}
// endregion:   --- Runtime
