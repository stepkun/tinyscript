// Copyright © 2025 Stephan Kunz
//! [`GroupingParselet`] handles parentheses.

use crate::{
	compilation::{
		Lexer, Parser,
		error::CompilationResult,
		token::{Token, TokenKind},
	},
	execution::Chunk,
};

use super::PrefixParselet;

pub struct GroupingParselet;

impl PrefixParselet for GroupingParselet {
	fn parse(&self, lexer: &mut Lexer, parser: &mut Parser, chunk: &mut Chunk, _token: Token) -> CompilationResult<()> {
		parser.expression(lexer, chunk)?;
		parser.consume(lexer, TokenKind::RightParen)
	}
}
