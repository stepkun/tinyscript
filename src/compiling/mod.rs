// Copyright © 2025 Stephan Kunz

//! Bytecode compiler for `tinyscript`
//!

mod lexer;
mod parselets;
mod parser;
mod precedence;
mod token;

// flatten
pub use lexer::Lexer;
pub use parser::Parser;
pub use token::TokenKind;
