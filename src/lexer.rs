#![allow(dead_code)]

use crate::token::{Keyword, Token};

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

	pub fn read_identifier(&mut self) -> String {
		let position = self.position;
		while is_letter(self.current_char) {
			self.read_char();
		}
		self.input[position..self.position].to_owned()
	}

	pub fn read_number(&mut self) -> usize {
		let position = self.position;
		while self.current_char.is_ascii_digit() {
			self.read_char();
		}
		let potential_int = &self.input[position..self.position];
		if let Ok(int) = potential_int.parse() {
			int
		} else {
			panic!("Expected Integer. Got `{}` instead.", potential_int);
		}
	}

	pub fn skip_whitespace(&mut self) {
		while self.current_char.is_ascii_whitespace() {
			self.read_char();
		}
	}

	pub fn peek_char(&self) -> char {
		if self.read_position >= self.input.len() {
			'\0'
		} else {
			self.input.chars().nth(self.read_position).expect("fuck")
		}
	}

	pub fn next_token(&mut self) -> Token {
		self.skip_whitespace();

		let Ok(token) = self.current_char.to_string().parse::<Token>() else {
			 if is_letter(self.current_char) {
				let ident = self.read_identifier();
				if let Ok(keyword) = ident.parse::<Keyword>() {
					let keyword = Token::Keyword(keyword);
					self.output.push(keyword.clone());
					return keyword;
				}
				let ident = Token::Identifier(ident);
				self.output.push(ident.clone());
				return ident;
			} else if self.current_char.is_ascii_digit() {
				let int = Token::Integer(self.read_number()); 
				self.output.push(int.clone());
				return int;
			} else if self.current_char == '\0' {
				let eof = Token::EOF;
				self.output.push(eof.clone());
				return eof;
			} else {
				let illegal = Token::ILLEGAL(self.current_char.to_string());
				self.output.push(illegal.clone());
				return illegal;
			}
		};

		let token = if token == Token::Assign && self.peek_char() == '=' {
			self.read_char();
			Token::Eq
		} else {
			token
		};

		let token = if token == Token::Bang && self.peek_char() == '=' {
			self.read_char();
			Token::NotEq
		} else {
			token
		};

		self.output.push(token.clone());

		self.read_char();
		token
	}
}

pub fn is_letter(char: char) -> bool {
	// allow `_` in variable names
	char.is_ascii_alphabetic() || char == '_'
}
