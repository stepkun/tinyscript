// Copyright © 2025 Stephan Kunz
//! `tinyscript`s external errors, passes through the internal errors.

use crate::{ConstString, compilation::CompilationError, execution::ExecutionError};

/// Shortcut for tinyscript's Result<T, E> type
pub type Result<T> = core::result::Result<T, Error>;

/// Error cases of the runtime
pub enum Error {
	/// Passthrough compilation errors.
	Compilation {
		/// The original error.
		source: CompilationError,
	},
	/// Passthrough execution errors.
	Execution {
		/// The original error.
		source: ExecutionError,
	},
	/// Tried to redefine an enum value.
	DuplicateEnumVariant {
		/// Name of the enum value.
		name: ConstString,
		/// Previously defined value.
		old: i8,
		/// Now defined value.
		new: i8,
	},
	/// Expected Boolean, got something else.
	NoBoolean {
		/// The faulty value.
		value: ConstString,
	},
}

/// Only a source implemaentation needed.
impl core::error::Error for Error {
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		None
	}

	// fn cause(&self) -> Option<&dyn core::error::Error> {
	//  	self.source()
	// }

	// fn provide<'a>(&'a self, request: &mut core::error::Request<'a>) {}
}

impl core::fmt::Debug for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Compilation { source } => write!(f, "Compilation({source:?})"),
			Self::Execution { source } => write!(f, "Execution({source:?})"),
			Self::DuplicateEnumVariant { name, old, new } => {
				write!(f, "DuplicateEnumVariant(name: {name}, old: {old}, new: {new})")
			}
			Self::NoBoolean { value } => write!(f, "NoBoolean({value})"),
		}
	}
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Compilation { source } => write!(f, "compilation error: {source}"),
			Self::Execution { source } => write!(f, "execution error:{source}"),
			Self::DuplicateEnumVariant { name, old, new } => {
				write!(f, "enum variant {name} already exists with value {old} new value: {new}")
			}
			Self::NoBoolean { value } => write!(f, "expected boolean, got {value}"),
		}
	}
}

impl From<CompilationError> for Error {
	fn from(error: CompilationError) -> Self {
		Self::Compilation { source: error }
	}
}

impl From<ExecutionError> for Error {
	fn from(error: ExecutionError) -> Self {
		Self::Execution { source: error }
	}
}
