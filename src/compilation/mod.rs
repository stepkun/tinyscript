// Copyright © 2025 Stephan Kunz
//! Bytecode compiler implementation.

mod error;
mod lexer;
mod parselets;
mod parser;
mod precedence;
mod token;

// flatten
pub use error::{CompilationError, CompilationResult};
pub use lexer::Lexer;
pub use parser::Parser;
pub use token::TokenKind;
