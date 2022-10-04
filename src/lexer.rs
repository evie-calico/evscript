use std::collections::HashMap;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
	// Keywords
	Environment,
	Function,
	Include,
	Asm,
	Use,
	Def,
	Mac,
	Pool,
	Terminator,
	// Simple tokens
	Semicolon,
	LeftCurlyBrace,
	RightCurlyBrace,
	LeftParenthesis,
	RightParenthesis,
	// Complex tokens
	Identifier(String),
	String(String),
	InlineAssembly(String),

	Eof,
}

pub struct Location {
	file: String,
	line: u32,
	column: u32,
	// For highlighting ranges (an entire token)
	column_end: Option<u32>,
}

impl Location {
	pub fn new(file: &str) -> Location {
		Location {
			file: String::from(file),
			line: 1,
			column: 1,
			column_end: None,
		}
	}
}

impl fmt::Display for Location {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}:{}:{}{}",
			self.file,
			self.line,
			self.column,
			if let Some(end) = self.column_end {
				format!("-{end}")
			} else {
				String::from("")
			}
		)
	}
}

pub fn lex(input: &mut Peekable<Chars>, location: &mut Location) -> Result<Token, String> {
	// The lexing mode determines what characters are allowed in the current token.
	// For example, a token beginning with a letter or underscore uses "Identifier" mode,
	// which forces all following characters to be letters, numbers, or underscores.
	enum Mode {
		Begin,
		Identifier,
		String,
		InlineAssembly,
	}

	let mut token_string = String::new();

	let mut get_next = |input: &mut Peekable<Chars>| -> Result<char, String> {
		let next = match input.next() {
			Some(c) => c,
			None => return Err(format!("Unterminated token"))
		};

		if next == '\n' {
			location.line += 1;
			location.column = 1;
		} else {
			location.column += 1;
		}
		Ok(next)
	};

	let mut mode = Mode::Begin;

	loop {
		let next = match input.peek() {
			Some(character) => *character,
			None => {
				if token_string.len() == 0 {
					return Ok(Token::Eof);
				} else {
					return Err(format!("Unterminated token: {token_string}"));
				}
			}
		};

		match mode {
			Mode::Begin => {
				match next {
					' ' | '\t' | '\n' | '\r' => {
						get_next(input)?;
						continue;
					}
					'A'..='Z' | 'a'..='z' | '_' => {
						mode = Mode::Identifier;
					}
					'\"' => {
						get_next(input)?;
						mode = Mode::String;
					}
					'{' => {
						get_next(input)?;
						return Ok(Token::LeftCurlyBrace);
					}
					'}' => {
						get_next(input)?;
						return Ok(Token::RightCurlyBrace);
					}
					'(' => {
						get_next(input)?;
						return Ok(Token::LeftParenthesis);
					}
					')' => {
						get_next(input)?;
						return Ok(Token::RightParenthesis);
					}
					';' => {
						get_next(input)?;
						return Ok(Token::Semicolon);
					}
					'#' => {
						get_next(input)?;
						if get_next(input)? != 'a'
							|| get_next(input)? != 's'
							|| get_next(input)? != 'm'
						{
							return Err(String::from("Invalid hash character; expected #asm"));
						}
						mode = Mode::InlineAssembly
					}
					_ => {
						return Err(format!("Invalid character: {next}"));
					}
				}

				token_string.push(get_next(input)?);
			}

			Mode::Identifier => {
				match next {
					'A'..='Z' | 'a'..='z' | '_' | '0'..='9' => {
						token_string.push(get_next(input)?);
					},
					_ => {
						// Identifier terminated, check for keywords
						if token_string == "env" {
							return Ok(Token::Environment);
						} else if token_string == "fn" {
							return Ok(Token::Function);
						} else if token_string == "include" {
							return Ok(Token::Include);
						} else if token_string == "asm" {
							return Ok(Token::Asm);
						} else if token_string == "use" {
							return Ok(Token::Use);
						} else if token_string == "def" {
							return Ok(Token::Def);
						} else if token_string == "mac" {
							return Ok(Token::Mac);
						} else if token_string == "pool" {
							return Ok(Token::Pool);
						} else if token_string == "terminator" {
							return Ok(Token::Terminator);
						} else {
							return Ok(Token::Identifier(token_string));
						}
					}
				}
			}

			Mode::String => {
				let next = get_next(input)?;

				if next == '\\' {
					let escape_chars = HashMap::from([
						('\\', '\\'),
						('n', '\n'),
						('t', '\t'),
						('r', '\r'),
						('0', '\0'),
					]);

					let escaped = match get_next(input) {
						Ok(character) => character,
						Err(..) => {
							return Err(format!("Unterminated string"));
						}
					};

					match escape_chars.get(&escaped) {
						Some(c) => token_string.push(*c),
						None => return Err(format!("Invalid escape character: {escaped}"))
					}
				} else if next == '\"' {
					return Ok(Token::String(token_string));
				} else {
					token_string.push(next)
				}
			}

			Mode::InlineAssembly => {
				if next == '#' {
					get_next(input)?;

					let end_string = ['e', 'n', 'd'];

					for i in 0..3 {
						let c = get_next(input)?;

						if c != end_string[i] {
							token_string.push('#');
							for i in 0..i {
								token_string.push(end_string[i]);
							}
							token_string.push(c);
							continue;
						}
					}

					return Ok(Token::InlineAssembly(token_string));
				} else {
					token_string.push(get_next(input)?);
				}
			}
		}

	}
}
