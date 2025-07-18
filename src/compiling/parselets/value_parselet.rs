// Copyright © 2025 Stephan Kunz

//! `ValueParselet` for `tinyscript` analyzes and handles value tokens like numbers
//!

// region:   	--- modules
use crate::{
    Error,
    compiling::{
        Lexer, Parser,
        token::{Token, TokenKind},
    },
    execution::{Chunk, ScriptingValue, op_code::OpCode},
};

use super::PrefixParselet;
// endregion:  	--- modules

pub struct ValueParselet;

impl PrefixParselet for ValueParselet {
    fn parse(
        &self,
        lexer: &mut Lexer,
        parser: &mut Parser,
        chunk: &mut Chunk,
        token: Token,
    ) -> Result<(), Error> {
        match token.kind {
            TokenKind::Enum => {
                let Some(value) = lexer.enums().get(&token.origin) else {
                    return Err(Error::EnumValNotFound(token.origin.into(), token.line));
                };
                let offset = chunk.add_constant(ScriptingValue::Int64(i64::from(*value)))?;
                parser.emit_bytes(OpCode::Constant as u8, offset, chunk);
                Ok(())
            }
            TokenKind::FloatNumber => {
                let double: f64 = match token.origin.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        return Err(Error::ParseNumber(token.origin.into(), token.line));
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
                    return Err(Error::ParseHex(literal.into(), token.line));
                };
                let offset = chunk.add_constant(ScriptingValue::Int64(value))?;
                parser.emit_bytes(OpCode::Constant as u8, offset, chunk);
                Ok(())
            }
            TokenKind::IntNumber => {
                let Ok(value) = token.origin.parse::<i64>() else {
                    return Err(Error::ParseInt(token.origin.into(), token.line));
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
            _ => Err(Error::Unreachable(file!().into(), line!())),
        }
    }
}
