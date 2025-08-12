// Copyright © 2025 Stephan Kunz

//! Tests of lexing functionality

use std::collections::BTreeMap;

use rstest::rstest;
use tinyscript::SHOULD_NOT_HAPPEN;
use tinyscript::compiling::{Lexer, TokenKind};

#[rstest]
#[case("=", TokenKind::Equal)]
#[case(":", TokenKind::Colon)]
#[case(":=", TokenKind::ColonEqual)]
#[case("+", TokenKind::Plus)]
#[case("+=", TokenKind::PlusEqual)]
#[case("-", TokenKind::Minus)]
#[case("-=", TokenKind::MinusEqual)]
#[case("*", TokenKind::Star)]
#[case("*=", TokenKind::StarEqual)]
#[case("/", TokenKind::Slash)]
#[case("/=", TokenKind::SlashEqual)]
#[case(";", TokenKind::Semicolon)]
#[case("&", TokenKind::Ampersand)]
#[case("|", TokenKind::Pipe)]
#[case("^", TokenKind::Caret)]
#[case("~", TokenKind::Tilde)]
#[case("&&", TokenKind::And)]
#[case("||", TokenKind::Or)]
#[case("!", TokenKind::Bang)]
#[case("!=", TokenKind::BangEqual)]
#[case("==", TokenKind::EqualEqual)]
#[case("<", TokenKind::Less)]
#[case("<=", TokenKind::LessEqual)]
#[case(">", TokenKind::Greater)]
#[case(">=", TokenKind::GreaterEqual)]
#[case("?", TokenKind::QMark)]
#[case("(", TokenKind::LeftParen)]
#[case(")", TokenKind::RightParen)]
#[case("nil", TokenKind::Nil)]
#[case("true", TokenKind::True)]
#[case("false", TokenKind::False)]
#[case("print", TokenKind::Print)]
#[case("A", TokenKind::Ident)]
#[case("3.14", TokenKind::FloatNumber)]
#[case("0xff", TokenKind::HexNumber)]
#[case("3", TokenKind::IntNumber)]
#[case("'test'", TokenKind::String)]
#[case("RED", TokenKind::Enum)]
fn lexing_token(#[case] input: &str, #[case] expected: TokenKind) {
	let mut enums: BTreeMap<String, i8> = BTreeMap::default();
	enums.insert("RED".to_string(), 0);
	let mut lexer = Lexer::new(&enums, input);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		expected
	);
	assert!(lexer.next().is_none());
}

#[allow(clippy::cognitive_complexity)]
#[allow(clippy::too_many_lines)]
fn lexing_tokens(tokens: &str) {
	let enums: BTreeMap<String, i8> = BTreeMap::default();
	let mut lexer = Lexer::new(&enums, tokens);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::ColonEqual
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Equal
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Plus
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Minus
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Star
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Slash
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::PlusEqual
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::MinusEqual
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::StarEqual
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::SlashEqual
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Semicolon
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Bang
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Ampersand
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Pipe
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Caret
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Tilde
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::And
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Or
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::EqualEqual
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::BangEqual
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Less
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::LessEqual
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Greater
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::GreaterEqual
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Colon
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::QMark
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::LeftParen
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::RightParen
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing() {
	let tokens = ":= = + - * / += -= *= /= ; ! & | ^ ~ && || == != < <= > >= : ? ( )";
	lexing_tokens(tokens);
	let tokens2 = ":==+-*/+=-=*=/=;!&|^~&&||==!=<<=>>=:?()";
	lexing_tokens(tokens2);
}

#[test]
fn lexing_keywords() {
	let tokens = "true false print";
	let enums: BTreeMap<String, i8> = BTreeMap::default();
	let mut lexer = Lexer::new(&enums, tokens);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::True
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::False
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Print
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_idents() {
	let tokens = "a_name _another_name _aThirdName_";
	let enums: BTreeMap<String, i8> = BTreeMap::default();
	let mut lexer = Lexer::new(&enums, tokens);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Ident
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Ident
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Ident
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_numbers() {
	let tokens = "123 123.0 123.456 0.123 0x123";
	let enums: BTreeMap<String, i8> = BTreeMap::default();
	let mut lexer = Lexer::new(&enums, tokens);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::IntNumber
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::FloatNumber
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::FloatNumber
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::FloatNumber
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::HexNumber
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_hex() {
	let tokens = "0x123 0xABC 0xabc 0xa1b2c3";
	let enums: BTreeMap<String, i8> = BTreeMap::default();
	let mut lexer = Lexer::new(&enums, tokens);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::HexNumber
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::HexNumber
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::HexNumber
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::HexNumber
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_strings() {
	let tokens = "'teststring' 'another_string'";
	let enums: BTreeMap<String, i8> = BTreeMap::default();
	let mut lexer = Lexer::new(&enums, tokens);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::String
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::String
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_enums() {
	let tokens = "First SECOND Third";
	// @TODO
	let mut enums: BTreeMap<String, i8> = BTreeMap::default();
	enums.insert("First".into(), 1);
	enums.insert("SECOND".into(), 2);
	enums.insert("Third".into(), 3);

	let mut lexer = Lexer::new(&enums, tokens);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Enum
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Enum
	);
	assert_eq!(
		lexer
			.next()
			.expect(SHOULD_NOT_HAPPEN)
			.expect(SHOULD_NOT_HAPPEN)
			.kind,
		TokenKind::Enum
	);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}
