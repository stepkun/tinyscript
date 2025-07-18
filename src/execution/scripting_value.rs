// Copyright © 2025 Stephan Kunz

//! A universal `Value` type for `tinyscript`

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::string::String;
use core::{
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};

use crate::Error;
// endregion:	--- modules

// region:		--- Value
/// Value type to allow storing different kinds of values
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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // extern crate std;
        // std::dbg!(&s);
        str::parse::<i64>(s).map_or_else(
            |_| {
                str::parse::<f64>(s).map_or_else(
                    |_| {
                        str::parse::<bool>(s)
                            .map_or_else(|_| Ok(Self::String(s.into())), |b| Ok(Self::Boolean(b)))
                    },
                    |f| Ok(Self::Float64(f)),
                )
            },
            |i| Ok(Self::Int64(i)),
        )
    }
}

impl ScriptingValue {
    /// Create a `Nil` value.
    #[must_use]
    pub const fn nil() -> Self {
        Self::Nil()
    }

    /// Return the boolean value.
    /// # Errors
    /// if it is not a boolean type
    pub const fn as_bool(&self) -> Result<bool, Error> {
        match self {
            Self::Boolean(b) => Ok(*b),
            _ => Err(Error::NoBoolean),
        }
    }

    /// Check if it is a boolean value.
    #[must_use]
    pub const fn is_bool(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }
}
// endregion:	--- Value
