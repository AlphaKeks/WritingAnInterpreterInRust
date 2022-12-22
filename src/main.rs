mod lexer;
mod token;

fn main() {}

#[cfg(test)]
#[allow(unused)]
mod tests {
	use crate::{lexer::*, token::*};

	#[test]
	fn special_chars() {
		let input = String::from("=+(){},;\0");

		let mut lexer = Lexer::new(input);

		use Token::*;
		for char in [Assign, Plus, LParen, RParen, LBrace, RBrace, Comma, Semicolon, EOF] {
			let token = lexer.next_token();
			if char != token {
				eprintln!("Lexer: {:#?}", lexer);
				panic!("\nExpected `{:?}`\nGot: `{:?}`\n", char, token);
			}
		}
	}
}
