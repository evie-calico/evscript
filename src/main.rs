use std::process::exit;
use std::vec::Vec;
use std::str::Chars;

#[derive(Debug, PartialEq)]
enum Token {
	Environment,
	Identifier(String),
	LeftCurlyBrace,
	RightCurlyBrace,
	Eof,
}

#[derive(Debug)]
enum Statement {
	Block(Vec<Statement>),
	Environment(Environment),
}

#[derive(Debug)]
struct Environment {
	name: String,
	contents: Vec<Statement>
}

fn lex(input: &mut Chars) -> Result<Token, String> {
	enum Mode {
		Begin,
		Identifier,
	}

	let mut token_string = String::new();
	let mut mode = Mode::Begin;
	let mut input = input.peekable();

	loop {
		let next = match input.peek() {
			Some(character) => character,
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
						mode = Mode::Identifier
					}
					'{' => {
						input.next();
						return Ok(Token::LeftCurlyBrace);
					}
					'}' => {
						input.next();
						return Ok(Token::RightCurlyBrace);
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
						} else {
							return Ok(Token::Identifier(token_string));
						}
					}
				}
			}
		}

	}
}

fn sub_parse(input: &mut Chars, is_root: bool) -> Result<Vec<Statement>, String> {
	fn next_token(input: &mut Chars) -> Token {
		let token = lex(input).unwrap_or_else(|error| {
			eprintln!("Failed to read input: {error}");
			exit(1);
		});
		//println!("{token:?}");
		token
	}

	let mut ast = Vec::<Statement>::new();

	loop {
		let root = next_token(input);
		match root {
			Token::Environment => {
				let name = next_token(input);
				let name = match name {
					Token::Identifier(name) => name,
					_ => return Err(format!("Unexpected {name:?} after `env`."))
				};

				if next_token(input) != Token::LeftCurlyBrace {
					return Err(format!("Unexpected {name:?} after `env`."));
				}

				ast.push(Statement::Environment(Environment { name, contents: sub_parse(input, false)? }));
			}

			Token::Eof => {
				if is_root {
					break;
				} else {
					return Err(format!("Unexpected {root:?}"));
				}
			}

			Token::RightCurlyBrace => {
				if !is_root {
					break;
				} else {
					return Err(format!("Unexpected {root:?}"));
				}
			}

			_ => return Err(format!("Unexpected {root:?}"))
		}
	}

	Ok(ast)
}

fn parse(input: &mut Chars) -> Result<Vec<Statement>, String> { sub_parse(input, true) }

fn main() {
	match parse(&mut "env script { env sub {} }".chars()) {
		Ok(ast) => println!("{ast:#?}"),
		Err(err) => eprintln!("Failed to parse: {err}")
	}
}
