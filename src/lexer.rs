use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
	Environment,
	Function,
	Include,
	Asm,
	Use,
	Def,
	Mac,
	Pool,
	Terminator,
	Identifier(String),
	String(String),
	Semicolon,
	LeftCurlyBrace,
	RightCurlyBrace,
	InlineAssembly(String),
	Eof,
}

pub fn lex(input: &mut Peekable<Chars>) -> Result<Token, String> {
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
						input.next();
						continue;
					}
					'A'..='Z' | 'a'..='z' | '_' => {
						mode = Mode::Identifier;
					}
					'\"' => {
						input.next();
						mode = Mode::String;
					}
					'{' => {
						input.next();
						return Ok(Token::LeftCurlyBrace);
					}
					'}' => {
						input.next();
						return Ok(Token::RightCurlyBrace);
					}
					';' => {
						input.next();
						return Ok(Token::Semicolon);
					}
					'#' => {
						input.next();
						let mut matched = input.next() == Some('a');
						matched &= input.next() == Some('s');
						matched &= input.next() == Some('m');
						if !matched {
							return Err(String::from("Invalid hash character; expected #asm or #end"));
						}
						mode = Mode::InlineAssembly
					}
					_ => {
						return Err(format!("Invalid character: {next}"));
					}
				}

				token_string.push(input.next().unwrap());
			}

			Mode::Identifier => {
				match next {
					'A'..='Z' | 'a'..='z' | '_' | '0'..='9' => {
						token_string.push(input.next().unwrap());
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
				let next = input.next().unwrap();

				if next == '\\' {
					match match input.next() {
						Some(character) => character,
						None => {
							return Err(format!("Unterminated string"));
						}
					} {
						'\\' => token_string.push('\\'),
						_ => return Err(format!("Invalid escape character"))
					}
				} else if next == '\"' {
					return Ok(Token::String(token_string));
				} else {
					token_string.push(next)
				}
			}

			Mode::InlineAssembly => {
				if next == '#' {
					input.next();

					let end_string = ['e', 'n', 'd'];

					for i in 0..3 {
						match input.next() {
							Some(c) => {
								if c != end_string[i] {
									token_string.push('#');
									for i in 0..i {
										token_string.push(end_string[i]);
									}
									token_string.push(c);
									continue;
								}
							}
							None => {
								return Err(String::from("Unterminated #asm block."));
							}
						}
					}

					return Ok(Token::InlineAssembly(token_string));
				} else {
					token_string.push(input.next().unwrap());
				}
			}
		}

	}
}
