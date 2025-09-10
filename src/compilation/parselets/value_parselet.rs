// Copyright © 2025 Stephan Kunz
//! [`ValueParselet`] analyzes and handles value tokens like numbers.

// region:   	--- modules
use crate::{
	compilation::{
		Lexer, Parser,
		error::{CompilationError, CompilationResult},
		token::{Token, TokenKind},
	},
	execution::{Chunk, op_code::OpCode},
	scripting_value::ScriptingValue,
};

use super::PrefixParselet;
// endregion:  	--- modules

pub struct ValueParselet;

impl PrefixParselet for ValueParselet {
	fn parse(&self, lexer: &mut Lexer, parser: &mut Parser, chunk: &mut Chunk, token: Token) -> CompilationResult<()> {
		match token.kind {
			TokenKind::Enum => {
				let Some(value) = lexer.enums().get(&token.origin) else {
					return Err(CompilationError::EnumValNotFound {
						value: token.origin.into(),
						pos: token.line,
					});
				};
				let offset = chunk.add_constant(ScriptingValue::Int64(i64::from(*value)))?;
				parser.emit_bytes(OpCode::Constant as u8, offset, chunk);
				Ok(())
			}
			TokenKind::FloatNumber => {
				let double: f64 = match token.origin.parse() {
					Ok(n) => n,
					Err(_) => {
						return Err(CompilationError::ParseNumber {
							token: token.origin.into(),
							pos: token.line,
						});
					}
				};

				let offset = chunk.add_constant(ScriptingValue::Float64(double))?;
				parser.emit_bytes(OpCode::Constant as u8, offset, chunk);
				Ok(())
			}
			TokenKind::HexNumber => {
				// remove the '0x' before parsing
				let literal = token.origin.trim_start_matches("0x");
				let Ok(value) = i64::from_str_radix(literal, 16) else {
					return Err(CompilationError::ParseHex {
						token: literal.into(),
						pos: token.line,
					});
				};
				let offset = chunk.add_constant(ScriptingValue::Int64(value))?;
				parser.emit_bytes(OpCode::Constant as u8, offset, chunk);
				Ok(())
			}
			TokenKind::IntNumber => {
				let Ok(value) = token.origin.parse::<i64>() else {
					return Err(CompilationError::ParseInt {
						token: token.origin.into(),
						pos: token.line,
					});
				};
				let offset = chunk.add_constant(ScriptingValue::Int64(value))?;
				parser.emit_bytes(OpCode::Constant as u8, offset, chunk);
				Ok(())
			}
			TokenKind::String => {
				let offset = chunk.add_constant(ScriptingValue::String(token.origin))?;
				parser.emit_bytes(OpCode::Constant as u8, offset, chunk);
				Ok(())
			}
			_ => Err(CompilationError::Unreachable {
				file: file!().into(),
				line: line!(),
			}),
		}
	}
}
