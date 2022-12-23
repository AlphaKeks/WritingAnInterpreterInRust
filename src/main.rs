mod lexer;
mod token;

fn main() {}

#[cfg(test)]
#[allow(unused)]
mod tests {
	use std::thread::current;

	use crate::{lexer::*, token::*};

	#[test]
	fn special_chars() {
		let input = String::from("=+(){},;\0");

		let mut lexer = Lexer::new(input);

		use Token::*;
		let expected_tokens = [Assign, Plus, LParen, RParen, LBrace, RBrace, Comma, Semicolon, EOF];

		for expected_token in expected_tokens {
			let current_token = lexer.next_token();
			if current_token != expected_token {
				eprintln!("[FAIL] Lexer: {:#?}", lexer);
				panic!("\nExpected `{:?}`\nGot: `{:?}`\n", expected_token, current_token);
			}
		}

		println!("[SUCCESS] Output: {:?}", lexer.output);
	}

	#[test]
	fn simple_monkey() {
		let input = std::fs::read_to_string("monkey/main.monkey")
			.expect("Expected `monkey/main.monkey` source file.");

		let mut lexer = Lexer::new(input);

		use {crate::token::Keyword::*, Token::*};
		let expected_tokens = [
			Keyword(Let),
			Identifier(String::from("five")),
			Assign,
			Integer(5),
			Semicolon,
			Keyword(Let),
			Identifier(String::from("ten")),
			Assign,
			Integer(10),
			Semicolon,
			Keyword(Let),
			Identifier(String::from("add")),
			Assign,
			Keyword(Function),
			LParen,
			Identifier(String::from("x")),
			Comma,
			Identifier(String::from("y")),
			RParen,
			LBrace,
			Identifier(String::from("x")),
			Plus,
			Identifier(String::from("y")),
			Semicolon,
			RBrace,
			Semicolon,
			Keyword(Let),
			Identifier(String::from("result")),
			Assign,
			Identifier(String::from("add")),
			LParen,
			Identifier(String::from("five")),
			Comma,
			Identifier(String::from("ten")),
			RParen,
			Semicolon,
			EOF,
		];

		for expected_token in expected_tokens {
			let current_token = lexer.next_token();
			if current_token != expected_token {
				eprintln!("[FAIL] Lexer: {:#?}", lexer);
				panic!("\nExpected `{:?}`\nGot: `{:?}`\n", expected_token, current_token);
			}
		}

		println!("[SUCCESS] Output: {:?}", lexer.output);
	}
}
