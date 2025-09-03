//! Testing for `tinyscript` in embedded environment

#![no_main]
#![no_std]

extern crate alloc;

use tinyscript::*;

#[cfg(test)]
#[embedded_test::tests]
mod tests {
	#![allow(clippy::unwrap_used)]
	use super::*;

	#[test]
	async fn scripts() {
		let mut env = DefaultEnvironment::default();
		let mut runtime = Runtime::default();

		runtime
			.run("print (5 - (3 - 1)) + -1;", &mut env)
			.unwrap();
		//assert_eq!(runtime.stdout(), b"2\n");
	}
}
