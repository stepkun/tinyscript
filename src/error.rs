// Copyright © 2025 Stephan Kunz
//! `tinyscript`s external errors, passes through the internal errors.

use crate::{ConstString, compilation::CompilationError, execution::ExecutionError};

/// Shortcut for tinyscript's Result<T, E> type
pub type Result<T> = core::result::Result<T, Error>;

/// Error cases of the runtime
#[non_exhaustive]
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
	DuplicateVariant {
		/// Name of the enum value.
		name: ConstString,
		/// Previously defined value.
		old: i8,
		/// Now defined value.
		new: i8,
	},
	/// Conversion failed.
	TryConversion {
		/// The faulty value.
		value: ConstString,
		/// the wanted conversion into.
		into: ConstString,
	},
}

/// Only a source implementation needed.
impl core::error::Error for Error {
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Compilation { source } => Some(source),
			Self::Execution { source } => Some(source),
			_ => None,
		}
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
			Self::DuplicateVariant { name, old, new } => {
				write!(f, "DuplicateVariant(name: {name}, old: {old}, new: {new})")
			}
			Self::TryConversion { value, into } => write!(f, "TryConversion(value: {value}, into: {into})"),
		}
	}
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Compilation { source } => write!(f, "compilation error: {source}"),
			Self::Execution { source } => write!(f, "execution error:{source}"),
			Self::DuplicateVariant { name, old, new } => {
				write!(f, "enum variant {name} already exists with value {old} new value: {new}")
			}
			Self::TryConversion { value, into } => write!(f, "conversion of value {value} into {into} is not possible"),
		}
	}
}

impl From<CompilationError> for Error {
	fn from(source: CompilationError) -> Self {
		Self::Compilation { source }
	}
}

impl From<ExecutionError> for Error {
	fn from(source: ExecutionError) -> Self {
		Self::Execution { source }
	}
}
