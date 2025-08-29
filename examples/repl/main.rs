//! `tinyscript` `REPL` example
//! Copyright © 2025 Stephan Kunz

use std::io::{Write, stdin, stdout};

use tinyscript::{DefaultEnvironment, Runtime};

const PROMPT: &str = "> ";

fn repl() {
	let mut env = DefaultEnvironment::default();
	let mut runtime = Runtime::default();
	let mut input = String::new();

	println!("Quit with 'Ctrl+D'");
	print!("{PROMPT}");
	let _ = stdout().flush();
	loop {
		match stdin().read_line(&mut input) {
			Ok(len) => {
				if len > 0 {
					// ignore CR/LF only input
					if input.len() > 1 {
						// print!("{}", &input);
						runtime.parse(&input).map_or_else(
							|err| {
								println!("parsing error: {err}");
							},
							|chunk| {
								//chunk.disassemble("created chunk");
								if let Err(error) = runtime.execute(&chunk, &mut env) {
									println!("execution error: {error}");
								} else {
									for c in runtime.stdout() {
										print!("{}", *c as char);
									}
									runtime.clear();
								}
							},
						);
					}
					input.clear();
					print!("{PROMPT}");
					let _ = stdout().flush();
				} else {
					println!("bye");
					break;
				}
			}
			Err(_) => eprintln!("could not read input"),
		}
	}
}

fn main() {
	// initialize tracing/logging
	//init_tracing();
	repl();
}
