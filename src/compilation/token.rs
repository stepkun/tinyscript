// Copyright © 2025 Stephan Kunz
//! [`Token`] implementation created by the [`Lexer`](crate::compilation::Lexer) for the [`Parser`](crate::compilation::Parser)

use core::fmt::Display;

use alloc::string::String;

/// The token kind designates the type of a [`Token`].
#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub enum TokenKind {
	/// Dummy to avoid using `Option<Token>` in many places
	None,
	/// =
	Equal,
	/// :
	Colon,
	/// :=
	ColonEqual,
	/// +
	Plus,
	/// +=
	PlusEqual,
	/// -
	Minus,
	/// -=
	MinusEqual,
	/// *
	Star,
	/// *=
	StarEqual,
	/// /
	Slash,
	/// /=
	SlashEqual,
	/// ;
	Semicolon,
	/// & -> binary and
	Ampersand,
	/// | -> binary or
	Pipe,
	/// ^ -> binary xor
	Caret,
	/// ~ -> binary not
	Tilde,
	/// && -> logic and
	And,
	/// || -> logic or
	Or,
	/// ! -> logic not
	Bang,
	/// !=
	BangEqual,
	/// ==
	EqualEqual,
	/// <
	Less,
	/// <=
	LessEqual,
	/// >
	Greater,
	/// >=
	GreaterEqual,
	/// ?
	QMark,
	/// (
	LeftParen,
	/// )
	RightParen,
	/// keyword 'nil'
	Nil,
	/// Keyword boolean 'true'
	True,
	/// Keyword boolean 'false'
	False,
	/// Keyword 'print'
	Print,
	/// An Identifier
	Ident,
	/// Any Number either f64 or i64
	FloatNumber,
	/// Any hexadecimal Number
	HexNumber,
	/// Any integer Number
	IntNumber,
	/// Any String
	String,
	/// An Enum value
	Enum,
}

impl Display for TokenKind {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::None => write!(f, "NONE"),
			Self::Equal => write!(f, "="),
			Self::Colon => write!(f, ":"),
			Self::ColonEqual => write!(f, ":="),
			Self::Plus => write!(f, "+"),
			Self::PlusEqual => write!(f, "+="),
			Self::Minus => write!(f, "-"),
			Self::MinusEqual => write!(f, "-="),
			Self::Star => write!(f, "*"),
			Self::StarEqual => write!(f, "*="),
			Self::Slash => write!(f, "/"),
			Self::SlashEqual => write!(f, "/="),
			Self::Semicolon => write!(f, ";"),
			Self::Ampersand => write!(f, "&"),
			Self::Pipe => write!(f, "|"),
			Self::Caret => write!(f, "^"),
			Self::Tilde => write!(f, "~"),
			Self::And => write!(f, "&&"),
			Self::Or => write!(f, "||"),
			Self::Bang => write!(f, "!"),
			Self::BangEqual => write!(f, "!="),
			Self::EqualEqual => write!(f, "=="),
			Self::Less => write!(f, "<"),
			Self::LessEqual => write!(f, "<="),
			Self::Greater => write!(f, ">"),
			Self::GreaterEqual => write!(f, ">="),
			Self::QMark => write!(f, "?"),
			Self::LeftParen => write!(f, "("),
			Self::RightParen => write!(f, ")"),
			Self::Nil => write!(f, "'nil'"),
			Self::True => write!(f, "'true'"),
			Self::Print => write!(f, "'print'"),
			Self::False => write!(f, "'false'"),
			Self::Ident => write!(f, "an 'Ident'"),
			Self::FloatNumber => write!(f, "a 'FloatNumber'"),
			Self::HexNumber => write!(f, "a 'HexNumber'"),
			Self::IntNumber => write!(f, "a 'IntNumber'"),
			Self::String => write!(f, "a 'String'"),
			Self::Enum => write!(f, "an 'Enum'"),
		}
	}
}

/// Token, the internals are directly visible to the crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
	/// Reference to the underlying location
	pub(crate) origin: String,
	/// Position of the token in the line.
	pub(crate) offset: usize,
	/// Line of the token.
	pub(crate) line: usize,
	/// Kind of token.
	pub(crate) kind: TokenKind,
}

impl Token {
	/// Get the token type. Needed for testing purposes.
	#[inline]
	pub const fn kind(&self) -> TokenKind {
		self.kind
	}

	pub fn none() -> Self {
		Self {
			origin: String::default(),
			offset: 0,
			line: 0,
			kind: TokenKind::None,
		}
	}
}
