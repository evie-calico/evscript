use crate::lexer::{lex, Location, Token};

use std::iter::Peekable;
use std::vec::Vec;
use std::str::Chars;

#[derive(Debug)]
pub enum Statement {
	Block(Vec<Statement>),
	// Top-level statements.
	Environment(Environment),
	Function(Function),
	InlineAssembly(String),
	// Environment statements
	Use(String),
	Def(Def),
	// Function statements
	VariableDeclaration(VariableDeclaration),
}

// Top-level statements.
#[derive(Debug)]
pub struct Environment {
	name: String,
	contents: Vec<Statement>
}

#[derive(Debug)]
pub struct Function {
	name: String,
	environment: String,
	contents: Vec<Statement>
}

// Environment statements
#[derive(Debug)]
pub struct Def {
	name: String,
	args: Vec<String>,
}

// Function statements
#[derive(Debug)]
pub struct VariableDeclaration {
	name: String,
	var_type: String,
}

fn expect_identifier(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<String, String> {
	let token = lex(input, loc)?;

	match token {
		Token::Identifier(name) => Ok(name),
		_ => Err(format!("Expected Identifier, got {token:?}"))
	}
}

fn expect_string(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<String, String> {
	let token = lex(input, loc)?;

	match token {
		Token::String(string) => Ok(string),
		_ => Err(format!("Expected String, got {token:?}"))
	}
}

fn expect_lparen(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<(), String> {
	let token = lex(input, loc)?;

	if token != Token::LeftParenthesis {
		Err(format!("Expected (, got {token:?}"))
	} else {
		Ok(())
	}
}

fn expect_lbrace(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<(), String> {
	let token = lex(input, loc)?;

	if token != Token::LeftCurlyBrace {
		Err(format!("Expected {{, got {token:?}"))
	} else {
		Ok(())
	}
}

fn expect_semicolon(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<(), String> {
	let token = lex(input, loc)?;

	if token != Token::Semicolon {
		Err(format!("Expected ;, got {token:?}"))
	} else {
		Ok(())
	}
}

fn parse_environment(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<Vec<Statement>, String> {
	let mut ast = Vec::<Statement>::new();

	loop {
		let root = lex(input, loc)?;

		match root {
			Token::Use => {
				ast.push(Statement::Use(expect_identifier(input, loc)?));
				expect_semicolon(input, loc)?;
			}

			Token::Def => {
				let name = expect_identifier(input, loc)?;
				expect_lparen(input, loc)?;

				let mut args = Vec::<String>::new();

				loop {
					let next_token = lex(input, loc)?;

					match next_token {
						Token::RightParenthesis => break,
						Token::Identifier(string) => args.push(string),
						_ => return Err(format!("Unexpected {next_token:?} in parameter list")),
					}
				}

				expect_semicolon(input, loc)?;

				ast.push(Statement::Def(Def { name, args }));
			}

			Token::RightCurlyBrace => break,
			_ => return Err(format!("Unexpected {root:?}"))
		}
	}

	Ok(ast)
}

fn parse_function(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<Vec<Statement>, String> {
	let mut ast = Vec::<Statement>::new();

	loop {
		let root = lex(input, loc)?;

		match root {
			Token::RightCurlyBrace => break,
			_ => return Err(format!("Unexpected {root:?}"))
		}
	}

	Ok(ast)
}

fn parse_root(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<Vec<Statement>, String> {
	let mut ast = Vec::<Statement>::new();

	loop {
		let root = lex(input, loc)?;
		match root {
			Token::Environment => {
				let name = expect_identifier(input, loc)?;
				expect_lbrace(input, loc)?;

				ast.push(Statement::Environment(Environment {
					name,
					contents: parse_environment(input, loc)?
				}));
			}

			Token::Include => {
				let include_token = lex(input, loc)?;
				match include_token {
					Token::String(path) => todo!(),
					Token::Asm => {
						ast.push(Statement::InlineAssembly(format!(
							"include \"{}\"",
							expect_string(input, loc)?
						)));
					}
					_ => return Err(format!("Expected `asm` after `include`, got {include_token:?}"))
				}
			}

			Token::Identifier(identifier) => {
				match lex(input, loc)? {
					Token::Function => {
						let name = expect_identifier(input, loc)?;
						expect_lbrace(input, loc)?;

						ast.push(Statement::Function(Function {
							name,
							environment: identifier,
							contents: parse_function(input, loc)?
						}));
					}
					_ => {
						return Err(format!("Unexpected {identifier}."));
					}
				}
			}

			Token::InlineAssembly(assembly) => {
				ast.push(Statement::InlineAssembly(assembly));
			}

			Token::Eof => break,
			_ => return Err(format!("Unexpected {root:?}"))
		}
	}

	Ok(ast)
}

pub fn parse(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<Vec<Statement>, String> {
	parse_root(input, loc)
}
