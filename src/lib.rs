// Copyright © 2025 Stephan Kunz
#![no_std]
#![doc = include_str!("../README.md")]

//! The implementation follows the pattern of clox as described in Part III of [crafting interpreters](https://craftinginterpreters.com/)

#[doc(hidden)]
extern crate alloc;

pub mod compiling;
pub mod error;
pub mod execution;
pub mod runtime;

// flatten
pub use error::Error;
pub use execution::Chunk;
pub use runtime::{Runtime, SharedRuntime};

// reexport
pub use tinyscript_derive::ScriptEnum;

// region:		--- modules
use alloc::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};

use execution::ScriptingValue;
use parking_lot::RwLock;
// endregion:	--- modules

// region		--- types
/// An immutable thread safe `String` type
/// see: [Logan Smith](https://www.youtube.com/watch?v=A4cKi7PTJSs).
pub type ConstString = Arc<str>;
// endregion:   --- types

/// The trait for script enums.
pub trait ScriptEnum {
    /// Function to get key-value tuples for registering.
    fn key_value_tuples() -> Vec<(&'static str, i8)>;
}

/// The trait for providing an [`Environment`] to a [`VM`](crate::execution::VM)
/// that stores variables persistently and externally available.
pub trait Environment: Send + Sync {
    /// Define the variable with `key` to `value`.
    /// It has to be created if it does not already exist.
    /// # Errors
    /// if the Variable exists with a different type
    fn define_env(&mut self, key: &str, value: ScriptingValue) -> Result<(), Error>;
    /// Get a variable by `key`
    /// # Errors
    /// if the variable does not exist
    fn get_env(&self, key: &str) -> Result<ScriptingValue, Error>;
    /// Set the variable with `key` to `value`.
    /// # Errors
    /// if variable does not exist.
    fn set_env(&mut self, key: &str, value: ScriptingValue) -> Result<(), Error>;
}

/// A very simple default Environment for testing purpose and the REPL
#[derive(Default)]
pub struct DefaultEnvironment {
    storage: RwLock<BTreeMap<String, ScriptingValue>>,
}

impl Environment for DefaultEnvironment {
    fn define_env(&mut self, name: &str, value: ScriptingValue) -> Result<(), Error> {
        self.storage.write().insert(name.to_string(), value);
        Ok(())
    }

    fn get_env(&self, name: &str) -> Result<ScriptingValue, Error> {
        self.storage.read().get(name).map_or_else(
            || Err(Error::GlobalNotDefined(name.into())),
            |value| Ok(value.clone()),
        )
    }

    fn set_env(&mut self, name: &str, value: ScriptingValue) -> Result<(), Error> {
        if self.storage.read().contains_key(name) {
            self.storage.write().insert(name.to_string(), value);
            Ok(())
        } else {
            Err(Error::GlobalNotDefined(name.into()))
        }
    }
}
