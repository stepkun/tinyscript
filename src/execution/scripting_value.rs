// Copyright © 2025 Stephan Kunz
//! A universal `Value` type implementation.

#[doc(hidden)]
extern crate alloc;

use alloc::string::{String, ToString};
use core::{
	fmt::{Debug, Display, Formatter},
	str::FromStr,
};

use crate::execution::{ExecutionError, ExecutionResult};

/// Value type to allow storing different kinds of values.
#[derive(Clone, Debug)]
pub enum ScriptingValue {
	/// Nil signals the absence of a `Value`
	Nil(),
	/// Boolean type
	Boolean(bool),
	/// Float 64bit
	Float64(f64),
	/// Integer 64bit
	Int64(i64),
	/// String type
	String(String),
}

impl Display for ScriptingValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Nil() => write!(f, "nil"),
			Self::Boolean(val) => write!(f, "{val}"),
			Self::Float64(val) => write!(f, "{val}"),
			Self::Int64(val) => write!(f, "{val}"),
			Self::String(val) => write!(f, "{val}"),
		}
	}
}

impl FromStr for ScriptingValue {
	type Err = ExecutionError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// extern crate std;
		// std::dbg!(&s);
		str::parse::<i64>(s).map_or_else(
			|_| {
				str::parse::<f64>(s).map_or_else(
					|_| str::parse::<bool>(s).map_or_else(|_| Ok(Self::String(s.into())), |b| Ok(Self::Boolean(b))),
					|f| Ok(Self::Float64(f)),
				)
			},
			|i| Ok(Self::Int64(i)),
		)
	}
}

impl TryFrom<ScriptingValue> for bool {
	type Error = crate::Error;

	fn try_from(value: ScriptingValue) -> Result<Self, Self::Error> {
		let val = value.as_bool()?;
		Ok(val)
	}
}

impl ScriptingValue {
	/// Create a `Nil` value.
	#[must_use]
	pub(crate) const fn nil() -> Self {
		Self::Nil()
	}

	/// Return the boolean value.
	/// Internal use only.
	/// # Errors
	/// - if it is not a boolean type
	pub(crate) fn as_bool(&self) -> ExecutionResult<bool> {
		match self {
			Self::Boolean(b) => Ok(*b),
			_ => Err(ExecutionError::NoBoolean {
				value: self.to_string().into(),
			}),
		}
	}

	/// Check if it is a boolean value.
	#[must_use]
	pub const fn is_bool(&self) -> bool {
		matches!(self, Self::Boolean(_))
	}
}
