// Copyright © 2025 Stephan Kunz
//! A tratt to work with the outside world and a default implementation.

// region:		--- modules
use alloc::{
	collections::btree_map::BTreeMap,
	string::{String, ToString},
};
use spin::RwLock;

use crate::{ConstString, scripting_value::ScriptingValue};
// endregion:	--- modules

/// The trait for providing an [`Environment`] to a [`VM`](crate::execution::VM)
/// that stores the [`ScriptingValue`]s persistent and external available.
///
/// An environment must be a key-value-store that can store [`ScriptingValue`]s.
pub trait Environment: Send + Sync {
	/// Creates or updates the [`ScriptingValue`] behind `key`.
	/// Value will be created if it does not already exist.
	/// # Errors
	/// [`Error::EnvVarWrongType`] if the variable exists with a different type.
	fn define_env(&mut self, key: &str, value: ScriptingValue) -> Result<(), Error>;

	/// Returns the [`ScriptingValue`] stored behind `key`.
	/// # Errors
	/// [`Error::EnvVarNotDefined`] if the variable does not exist
	fn get_env(&self, key: &str) -> Result<ScriptingValue, Error>;

	/// Set the variable with `key` to `value`.
	/// # Errors
	/// if variable does not exist.
	fn set_env(&mut self, key: &str, value: ScriptingValue) -> Result<(), Error>;
}

/// Errors that can happen when interacting with an [`Environment`].
#[non_exhaustive]
pub enum Error {
	/// A variable exceeds the limits of its type.
	EnvVarExceedsLimits {
		/// Name of the variable
		name: ConstString,
	},
	/// A variable has not been defined/created in the [`Environment`].
	EnvVarNotDefined {
		/// Name of the variable
		name: ConstString,
	},
	/// A variable has an unkown type.
	EnvVarUnknownType {
		/// Name of the variable
		name: ConstString,
	},
	/// A variable has a different type than in the [`Environment`].
	EnvVarWrongType {
		/// Name of the variable
		name: ConstString,
	},

	/// An external error when setting the variable.
	EnvVarSet {
		/// Name of the variable
		name: ConstString,
		/// Cause of error
		cause: ConstString,
	},

	/// An error casting the type of the variable.
	EnvVarTypeCast {
		/// Name of the variable
		name: ConstString,
		/// Expected type ofthe variable
		var_type: ConstString,
	},
}

/// Currently the default implementation is sufficient.
impl core::error::Error for Error {
	// fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
	// 	None
	// }

	// fn cause(&self) -> Option<&dyn core::error::Error> {
	// 	self.source()
	// }

	// fn provide<'a>(&'a self, request: &mut core::error::Request<'a>) {}
}

impl core::fmt::Debug for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::EnvVarExceedsLimits { name } => write!(f, "EnvVarExceedsLimits({name})"),
			Self::EnvVarNotDefined { name } => write!(f, "EnvVarNotDefined({name})"),
			Self::EnvVarUnknownType { name } => write!(f, "EnvVarlUnknownTType({name})"),
			Self::EnvVarWrongType { name } => write!(f, "EnvVarWrongType({name})"),
			Self::EnvVarSet { name, cause } => write!(f, "EnvVarSet({name}, {cause})"),
			Self::EnvVarTypeCast { name, var_type } => write!(f, "EnvVarTypeCast({name}, {var_type})"),
		}
	}
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::EnvVarExceedsLimits { name } => {
				write!(f, "the environment variable {name} exceeds the limits of its defined type")
			}
			Self::EnvVarNotDefined { name } => write!(f, "the environment variable {name} has not been defined"),
			Self::EnvVarUnknownType { name } => write!(f, "the environment variable {name} has an unknown type"),
			Self::EnvVarWrongType { name } => {
				write!(
					f,
					"the type of the environment variable {name} does not match its former definition"
				)
			}
			Self::EnvVarSet { name, cause } => {
				write!(f, "setting environment variable {name} failed: {cause}")
			}
			Self::EnvVarTypeCast { name, var_type } => write!(f, "cast of variable {name} to {var_type} failed"),
		}
	}
}

/// A very simple default Environment for testing purpose and the REPL.
#[derive(Debug, Default)]
pub struct DefaultEnvironment {
	storage: RwLock<BTreeMap<String, ScriptingValue>>,
}

impl Environment for DefaultEnvironment {
	fn define_env(&mut self, name: &str, value: ScriptingValue) -> Result<(), Error> {
		self.storage
			.write()
			.insert(name.to_string(), value);
		Ok(())
	}

	fn get_env(&self, name: &str) -> Result<ScriptingValue, Error> {
		self.storage.read().get(name).map_or_else(
			|| Err(Error::EnvVarNotDefined { name: name.into() }),
			|value| Ok(value.clone()),
		)
	}

	fn set_env(&mut self, name: &str, value: ScriptingValue) -> Result<(), Error> {
		if self.storage.read().contains_key(name) {
			self.storage
				.write()
				.insert(name.to_string(), value);
			Ok(())
		} else {
			Err(Error::EnvVarNotDefined { name: name.into() })
		}
	}
}
