// Copyright © 2025 Stephan Kunz
//! Execution errors, only internaly used.

use crate::ConstString;

/// Shortcut for tinyscript's execution Result<T, E> type
pub type ExecutionResult<T> = core::result::Result<T, ExecutionError>;

/// Things that may go wrong during execution of a compiled script.
#[non_exhaustive]
pub enum ExecutionError {
	/// No arithemetic with boolean for now.
	BoolNoArithmetic,
	/// Passthrough environment errors.
	Environment {
		/// The original error.
		source: crate::environment::Error,
	},
	/// Nil does not allow anything.
	NilValue,
	/// Expected Boolean, got something else.
	NoBoolean {
		/// The faulty value.
		value: ConstString,
	},
	/// Comparisons (greater, less) only with numeric values.
	NoComparison,
	/// Expected Integer, got something else.
	NoInteger {
		/// The faulty value.
		value: ConstString,
	},
	/// Expected a numerical value, got something else.
	NoNumber {
		/// The faulty value.
		value: ConstString,
	},
	/// Stack overflow.
	StackOverflow,
	/// Strings only allow additions.
	OnlyAdd,

	/// This code line never should have been reached.
	Unreachable {
		/// The faulty file.
		file: ConstString,
		/// The faulty line.
		line: u32,
	},
}

/// Currently the default implementation is sufficient.
impl core::error::Error for ExecutionError {
	// fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
	// 	None
	// }

	// fn cause(&self) -> Option<&dyn core::error::Error> {
	// 	self.source()
	// }

	// fn provide<'a>(&'a self, request: &mut core::error::Request<'a>) {}
}

impl core::fmt::Debug for ExecutionError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::BoolNoArithmetic => write!(f, "BoolNoArithmetic"),
			Self::Environment { source } => write!(f, "Environment({source:?})"),
			Self::NilValue => write!(f, "NilValue"),
			Self::NoBoolean { value } => write!(f, "NoBoolean({value})"),
			Self::NoComparison => write!(f, "NoComparison"),
			Self::NoInteger { value } => write!(f, "NoInteger({value})"),
			Self::NoNumber { value } => write!(f, "NoNumber({value})"),
			Self::StackOverflow => write!(f, "StackOverflow"),
			Self::OnlyAdd => write!(f, "OnlyAdd"),
			Self::Unreachable { file, line } => write!(f, "Unreachable(file: {file}, line: {line})"),
		}
	}
}

impl core::fmt::Display for ExecutionError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::BoolNoArithmetic => write!(f, "boolean values do not allow arithmetic operations"),
			Self::Environment { source } => write!(f, "environment error: {source}"),
			Self::NilValue => write!(f, "value is 'Nil' which does not allow any operation"),
			Self::NoBoolean { value } => write!(f, "expected boolean ('true'/'false'), got {value}"),
			Self::NoComparison => write!(f, "comparing values needs two numeric types"),
			Self::NoInteger { value } => write!(f, "expected integer value, got {value}"),
			Self::NoNumber { value } => write!(f, "expected numerical value, got {value}"),
			Self::StackOverflow => write!(f, "stack overflow, to many variables/values"),
			Self::OnlyAdd => write!(f, "to Strings you can only 'ADD' something"),
			Self::Unreachable { file, line } => write!(f, "{file} at line {line} should be unreachable"),
		}
	}
}

impl From<crate::environment::Error> for ExecutionError {
	fn from(error: crate::environment::Error) -> Self {
		Self::Environment { source: error }
	}
}
