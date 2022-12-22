#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Token {
	ILLEGAL(String),
	EOF,
	Keyword(Keyword),

	// user-defined
	Identifier(String),
	Integer(usize),

	// Operators
	Assign,
	Plus,
	Minus,
	Bang,
	Asterisk,
	Slash,
	LT,
	GT,
	Eq,
	NotEq,

	// Delimiters
	Comma,
	Semicolon,
	LParen,
	RParen,
	LBrace,
	RBrace,
}

impl std::str::FromStr for Token {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Ok(keyword) = s.parse::<Keyword>() {
			return Ok(Self::Keyword(keyword));
		}

		if s == "\0" {
			return Ok(Self::EOF);
		}

		match s {
			"=" => Ok(Self::Assign),
			"+" => Ok(Self::Plus),
			"-" => Ok(Self::Minus),
			"!" => Ok(Self::Bang),
			"*" => Ok(Self::Asterisk),
			"/" => Ok(Self::Slash),
			"<" => Ok(Self::LT),
			">" => Ok(Self::GT),
			"==" => Ok(Self::Eq),
			"!=" => Ok(Self::NotEq),
			"," => Ok(Self::Comma),
			";" => Ok(Self::Semicolon),
			"(" => Ok(Self::LParen),
			")" => Ok(Self::RParen),
			"{" => Ok(Self::LBrace),
			"}" => Ok(Self::RBrace),
			unknown => Err(format!("Unexpected Token: `{}`", unknown)),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
	Function,
	Let,
	True,
	False,
	If,
	Else,
	Return,
}

impl std::str::FromStr for Keyword {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"fn" => Ok(Self::Function),
			"let" => Ok(Self::Let),
			"true" => Ok(Self::True),
			"false" => Ok(Self::False),
			"if" => Ok(Self::If),
			"else" => Ok(Self::Else),
			"return" => Ok(Self::Return),
			unknown => Err(format!("Unexpected Keyword: `{}`", unknown)),
		}
	}
}
