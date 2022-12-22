use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Lexer {
	pub input: String,
	pub position: usize,
	pub read_position: usize,
	pub current_char: char,
	pub output: Vec<Token>,
}

impl Lexer {
	pub fn new(input: String) -> Self {
		let mut lexer =
			Self { input, position: 0, read_position: 0, current_char: '\0', output: Vec::new() };
		lexer.read_char();
		lexer
	}

	pub fn read_char(&mut self) {
		self.current_char = if self.read_position >= self.input.len() {
			'\0'
		} else {
			self.input.chars().nth(self.read_position).expect("fuck")
		};
		self.position = self.read_position;
		self.read_position += 1;
	}

	pub fn next_token(&mut self) -> Token {
		let Ok(token) = self.current_char.to_string().parse::<Token>() else {
			if self.current_char == '\0' {
				return Token::EOF
			}
			return Token::ILLEGAL(self.current_char.to_string());
		};
		self.output.push(token.clone());

		self.read_char();
		token
	}
}
