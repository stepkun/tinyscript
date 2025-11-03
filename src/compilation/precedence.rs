// Copyright © 2025 Stephan Kunz
//! Precedence definitions for the Pratt-[`Parser`](crate::compilation::Parser)
//!
//! Defines the different precedence levels used by the infix parsers.
//! These determine how a series of infix expressions will be grouped.
//! For example, "a + b * c - d" will be parsed as "(a + (b * c)) - d"
//! because "*" has higher precedence than "+" and "-".
//! Inn case of same precedence the source is parsed from left to right.
//! Here a bigger enum value is higher precedence.

/// Precedence levels
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
	None = 0,
	Assignment,
	Ternary,
	Or,
	And,
	BitOr,
	BitXor,
	BitAnd,
	Equality,
	Comparison,
	Term,
	Factor,
	Unary,
	Primary,
}

impl Precedence {
	/// Get the next higher [`Precedence`]
	pub const fn next_higher(self) -> Option<Self> {
		match self {
			Self::None => Some(Self::Assignment),
			Self::Assignment => Some(Self::Ternary),
			Self::Ternary => Some(Self::Or),
			Self::Or => Some(Self::And),
			Self::And => Some(Self::BitOr),
			Self::BitOr => Some(Self::BitXor),
			Self::BitXor => Some(Self::BitAnd),
			Self::BitAnd => Some(Self::Equality),
			Self::Equality => Some(Self::Comparison),
			Self::Comparison => Some(Self::Term),
			Self::Term => Some(Self::Factor),
			Self::Factor => Some(Self::Unary),
			Self::Unary => Some(Self::Primary),
			Self::Primary => None,
		}
	}
}
