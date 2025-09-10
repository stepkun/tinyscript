// Copyright © 2025 Stephan Kunz
//! A universal [`ScriptingValue`] type implementation.

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
		if let ScriptingValue::Boolean(b) = value {
			Ok(b)
		} else {
			Err(crate::Error::TryConversion {
				value: value.to_string().into(),
				into: "bool".into(),
			})
		}
	}
}

impl From<bool> for ScriptingValue {
	fn from(value: bool) -> Self {
		Self::Boolean(value)
	}
}

impl TryFrom<ScriptingValue> for f64 {
	type Error = crate::Error;

	fn try_from(value: ScriptingValue) -> Result<Self, Self::Error> {
		match value {
			ScriptingValue::Float64(f) => Ok(f),
			_ => Err(crate::Error::TryConversion {
				value: value.to_string().into(),
				into: "f64".into(),
			}),
		}
	}
}

impl From<f64> for ScriptingValue {
	fn from(value: f64) -> Self {
		Self::Float64(value)
	}
}

impl TryFrom<ScriptingValue> for i64 {
	type Error = crate::Error;

	fn try_from(value: ScriptingValue) -> Result<Self, Self::Error> {
		match value {
			ScriptingValue::Int64(i) => Ok(i),
			_ => Err(crate::Error::TryConversion {
				value: value.to_string().into(),
				into: "i64".into(),
			}),
		}
	}
}

impl From<i64> for ScriptingValue {
	fn from(value: i64) -> Self {
		Self::Int64(value)
	}
}

impl TryFrom<ScriptingValue> for u64 {
	type Error = crate::Error;

	fn try_from(value: ScriptingValue) -> Result<Self, Self::Error> {
		if let ScriptingValue::Int64(i) = value {
			if i >= 0 {
				#[allow(clippy::cast_sign_loss)]
				return Ok(i as Self);
			}
		}

		Err(crate::Error::TryConversion {
			value: value.to_string().into(),
			into: "u64".into(),
		})
	}
}

impl TryFrom<u64> for ScriptingValue {
	type Error = crate::Error;

	fn try_from(value: u64) -> Result<Self, Self::Error> {
		if let Ok(i) = i64::try_from(value) {
			return Ok(Self::Int64(i));
		}
		Err(crate::Error::TryConversion {
			value: value.to_string().into(),
			into: "ScriptingValue".into(),
		})
	}
}

impl TryFrom<ScriptingValue> for i32 {
	type Error = crate::Error;

	fn try_from(value: ScriptingValue) -> Result<Self, Self::Error> {
		if let ScriptingValue::Int64(i) = value {
			if i >= i64::from(Self::MIN) && i <= i64::from(Self::MAX) {
				#[allow(clippy::cast_possible_truncation)]
				return Ok(i as Self);
			}
		}

		Err(crate::Error::TryConversion {
			value: value.to_string().into(),
			into: "i32".into(),
		})
	}
}

impl From<i32> for ScriptingValue {
	fn from(value: i32) -> Self {
		Self::Int64(i64::from(value))
	}
}

impl TryFrom<ScriptingValue> for u32 {
	type Error = crate::Error;

	fn try_from(value: ScriptingValue) -> Result<Self, Self::Error> {
		if let ScriptingValue::Int64(i) = value {
			if i >= 0 && i <= i64::from(Self::MAX) {
				#[allow(clippy::cast_possible_truncation)]
				#[allow(clippy::cast_sign_loss)]
				return Ok(i as Self);
			}
		}

		Err(crate::Error::TryConversion {
			value: value.to_string().into(),
			into: "u32".into(),
		})
	}
}

impl From<u32> for ScriptingValue {
	fn from(value: u32) -> Self {
		Self::Int64(i64::from(value))
	}
}

impl TryFrom<ScriptingValue> for i16 {
	type Error = crate::Error;

	fn try_from(value: ScriptingValue) -> Result<Self, Self::Error> {
		if let ScriptingValue::Int64(i) = value {
			if i >= i64::from(Self::MIN) && i <= i64::from(Self::MAX) {
				#[allow(clippy::cast_possible_truncation)]
				return Ok(i as Self);
			}
		}

		Err(crate::Error::TryConversion {
			value: value.to_string().into(),
			into: "i16".into(),
		})
	}
}

impl From<i16> for ScriptingValue {
	fn from(value: i16) -> Self {
		Self::Int64(i64::from(value))
	}
}

impl TryFrom<ScriptingValue> for u16 {
	type Error = crate::Error;

	fn try_from(value: ScriptingValue) -> Result<Self, Self::Error> {
		if let ScriptingValue::Int64(i) = value {
			if i >= 0 && i <= i64::from(Self::MAX) {
				#[allow(clippy::cast_possible_truncation)]
				#[allow(clippy::cast_sign_loss)]
				return Ok(i as Self);
			}
		}

		Err(crate::Error::TryConversion {
			value: value.to_string().into(),
			into: "u18".into(),
		})
	}
}

impl From<u16> for ScriptingValue {
	fn from(value: u16) -> Self {
		Self::Int64(i64::from(value))
	}
}

impl TryFrom<ScriptingValue> for i8 {
	type Error = crate::Error;

	fn try_from(value: ScriptingValue) -> Result<Self, Self::Error> {
		if let ScriptingValue::Int64(i) = value {
			if i >= i64::from(Self::MIN) && i <= i64::from(Self::MAX) {
				#[allow(clippy::cast_possible_truncation)]
				return Ok(i as Self);
			}
		}

		Err(crate::Error::TryConversion {
			value: value.to_string().into(),
			into: "i8".into(),
		})
	}
}

impl From<i8> for ScriptingValue {
	fn from(value: i8) -> Self {
		Self::Int64(i64::from(value))
	}
}

impl TryFrom<ScriptingValue> for u8 {
	type Error = crate::Error;

	fn try_from(value: ScriptingValue) -> Result<Self, Self::Error> {
		if let ScriptingValue::Int64(i) = value {
			if i >= 0 && i <= i64::from(Self::MAX) {
				#[allow(clippy::cast_possible_truncation)]
				#[allow(clippy::cast_sign_loss)]
				return Ok(i as Self);
			}
		}

		Err(crate::Error::TryConversion {
			value: value.to_string().into(),
			into: "u8".into(),
		})
	}
}

impl From<u8> for ScriptingValue {
	fn from(value: u8) -> Self {
		Self::Int64(i64::from(value))
	}
}

impl TryFrom<ScriptingValue> for String {
	type Error = crate::Error;

	fn try_from(value: ScriptingValue) -> Result<Self, Self::Error> {
		if let ScriptingValue::String(s) = value {
			Ok(s)
		} else {
			Err(crate::Error::TryConversion {
				value: value.to_string().into(),
				into: "String".into(),
			})
		}
	}
}

impl<'a> TryFrom<&'a ScriptingValue> for &'a str {
	type Error = crate::Error;

	fn try_from(value: &'a ScriptingValue) -> Result<Self, Self::Error> {
		if let ScriptingValue::String(s) = value {
			Ok(s)
		} else {
			Err(crate::Error::TryConversion {
				value: value.to_string().into(),
				into: "&str".into(),
			})
		}
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
