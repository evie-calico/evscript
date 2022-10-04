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

fn parse_environment(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<Vec<Statement>, String> {
	let mut ast = Vec::<Statement>::new();

	loop {
		let root = lex(input, loc)?;

		match root {
			Token::Use => {
				let environment = lex(input, loc)?;

				match environment {
					Token::Identifier(name) => ast.push(Statement::Use(name)),
					_ => return Err(format!("Unexpected {environment:?} after `use`"))
				}

				if lex(input, loc)? != Token::Semicolon {
					return Err(String::from("Expected ; after `use` statement"));
				}
			}

			Token::Def => {
				let identifier = lex(input, loc)?;

				let name = match identifier {
					Token::Identifier(identifier) => identifier,
					_ => return Err(format!("Unexpected {identifier:?} after `def`"))
				};

				if lex(input, loc)? != Token::LeftParenthesis {
					return Err(String::from("Expected argument list for bytecode definition"));
				}

				let mut args = Vec::<String>::new();

				loop {
					let next_token = lex(input, loc)?;

					match next_token {
						Token::RightParenthesis => break,
						Token::Identifier(string) => args.push(string),
						_ => return Err(format!("Unexpected {next_token:?} in parameter list")),
					}
				}

				ast.push(Statement::Def(Def { name, args }));

				if lex(input, loc)? != Token::Semicolon {
					return Err(String::from("Expected ; after `def` statement"));
				}
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
				let name = lex(input, loc)?;
				let name = match name {
					Token::Identifier(name) => name,
					_ => return Err(format!("Unexpected {name:?} after `env`."))
				};

				if lex(input, loc)? != Token::LeftCurlyBrace {
					return Err(String::from("Expected { after environment name."));
				}

				ast.push(Statement::Environment(Environment {
					name,
					contents: parse_environment(input, loc)?
				}));
			}

			Token::Include => {
				let include_token = lex(input, loc)?;
				match include_token {
					Token::Asm => {
						let path = lex(input, loc)?;
						match path {
							Token::String(string) => {
								ast.push(Statement::InlineAssembly(format!("include \"{string}\"")));
							}
							_ => {
								return Err(format!("Expected string after `include asm`, got {path:?}"));
							}
						}
					}
					_ => {
						return Err(format!("Expected `asm` after `include`, got {include_token:?}"));
					}
				}
			}

			Token::Identifier(identifier) => {
				match lex(input, loc)? {
					Token::Function => {
						let name = lex(input, loc)?;
						let name = match name {
							Token::Identifier(name) => name,
							_ => return Err(format!("Unexpected {name:?} after `fn`."))
						};

						if lex(input, loc)? != Token::LeftCurlyBrace {
							return Err(String::from("Expected { after function name."));
						}

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
