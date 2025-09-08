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
	pub const fn next_higher(self) -> Self {
		match self {
			Self::None => Self::Assignment,
			Self::Assignment => Self::Ternary,
			Self::Ternary => Self::Or,
			Self::Or => Self::And,
			Self::And => Self::BitOr,
			Self::BitOr => Self::BitXor,
			Self::BitXor => Self::BitAnd,
			Self::BitAnd => Self::Equality,
			Self::Equality => Self::Comparison,
			Self::Comparison => Self::Term,
			Self::Term => Self::Factor,
			Self::Factor => Self::Unary,
			Self::Unary => Self::Primary,
			Self::Primary => panic!(),
		}
	}
}
