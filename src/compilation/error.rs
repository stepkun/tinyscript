// Copyright © 2025 Stephan Kunz
//! Compilation errors, only internaly used.

use crate::ConstString;

/// Shortcut for tinyscript's compile Result<T, E> type
pub type CompilationResult<T> = core::result::Result<T, CompilationError>;

/// Things that may go wrong during compilation of a script.
#[non_exhaustive]
pub enum CompilationError {
	/// Enum value is not defined.
	EnumValNotFound {
		/// Name of the enum value.
		value: ConstString,
		/// Position(line) in code.
		pos: usize,
	},
	/// Whatever it is: It is not an expression.
	ExpressionExpected {
		/// The faulty token.
		token: ConstString,
		/// Position(line) in code.
		pos: usize,
	},
	/// Not a hex number.
	ParseHex {
		/// The faulty token.
		token: ConstString,
		/// Position(line) in code.
		pos: usize,
	},
	/// Not an int number.
	ParseInt {
		/// The faulty token.
		token: ConstString,
		/// Position(line) in code.
		pos: usize,
	},
	/// Not a float number.
	ParseNumber {
		/// The faulty token.
		token: ConstString,
		/// Position(line) in code.
		pos: usize,
	},
	/// Stack of values exceeded.
	ConstantStorageOverflow,
	/// Did not get the expected `Token`.
	TokenExpected {
		/// The expected token.
		expected: ConstString,
		/// The found token.
		found: ConstString,
		/// Position(line) in code.
		pos: usize,
	},
	/// This char should not be here.
	UnexpectedChar {
		/// The faulty character.
		c: char,
		/// Position(line) in code.
		pos: usize,
	},
	/// Missing string termination.
	UnterminatedString {
		/// The unterminated sequence.
		str: ConstString,
		/// Position(line) in code.
		pos: usize,
	},

	/// This code line never should have been reached.
	Unreachable {
		/// The faulty file.
		file: ConstString,
		/// The faulty line.
		line: u32,
	},
}

/// Currently the default implementation is sufficient.
impl core::error::Error for CompilationError {}

impl core::fmt::Debug for CompilationError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::ConstantStorageOverflow => write!(f, "to many constant values defined: storage overflow"),
			Self::EnumValNotFound { value, pos } => write!(f, "the ScriptEnum value {value} at line {pos} is not defined"),
			Self::ExpressionExpected { token, pos } => {
				write!(f, "expression expected at line {pos}, got {token}")
			}
			Self::ParseHex { token, pos } => write!(f, "could not parse {token} at line {pos} as Hex value"),
			Self::ParseInt { token, pos } => write!(f, "could not parse {token} at line {pos} as Integer value"),
			Self::ParseNumber { token, pos } => write!(f, "could not parse {token} at line {pos} as Double value"),
			Self::TokenExpected { expected, found, pos } => {
				write!(f, "expected token {expected}, found Token {found} at line {pos}")
			}
			Self::UnexpectedChar { c, pos } => write!(f, "unexpected character {c} at line {pos}"),
			Self::UnterminatedString { str, pos } => write!(f, "unterminated string {str} at line {pos}"),
			Self::Unreachable { file, line } => write!(f, "{file} at line {line} should be unreachable"),
		}
	}
}

impl core::fmt::Display for CompilationError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		core::fmt::Debug::fmt(self, f)
	}
}
