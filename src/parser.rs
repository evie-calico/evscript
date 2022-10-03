use crate::lexer::{lex, Token};

use std::iter::Peekable;
use std::vec::Vec;
use std::str::Chars;

#[derive(Debug)]
pub enum Statement {
	Block(Vec<Statement>),
	Environment(Environment),
	Function(Function),
	InlineAssembly(String),
	VariableDeclaration(VariableDeclaration),
}

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

#[derive(Debug)]
pub struct VariableDeclaration {
	name: String,
	var_type: String,
}

fn sub_parse(input: &mut Peekable<Chars>, is_root: bool) -> Result<Vec<Statement>, String> {
	let mut ast = Vec::<Statement>::new();

	loop {
		let root = lex(input)?;
		match root {
			Token::Environment => {
				let name = lex(input)?;
				let name = match name {
					Token::Identifier(name) => name,
					_ => return Err(format!("Unexpected {name:?} after `env`."))
				};

				if lex(input)? != Token::LeftCurlyBrace {
					return Err(String::from("Expected { after environment name."));
				}

				ast.push(Statement::Environment(Environment {
					name,
					contents: sub_parse(input, false)?
				}));
			}

			Token::Include => {
				let include_token = lex(input)?;
				match include_token {
					Token::Asm => {
						let path = lex(input)?;
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

			Token::InlineAssembly(assembly) => {
				ast.push(Statement::InlineAssembly(assembly));
			}

			Token::Identifier(identifier) => {
				match lex(input)? {
					Token::Function => {
						let name = lex(input)?;
						let name = match name {
							Token::Identifier(name) => name,
							_ => return Err(format!("Unexpected {name:?} after `fn`."))
						};

						if lex(input)? != Token::LeftCurlyBrace {
							return Err(String::from("Expected { after function name."));
						}

						ast.push(Statement::Function(Function {
							name,
							environment: identifier,
							contents: sub_parse(input, false)?
						}));
					}
					// "iden iden" is always a variable declaration.
					Token::Identifier(var_name) => {
						match lex(input)? {
							Token::Semicolon => {
								ast.push(Statement::VariableDeclaration(VariableDeclaration {
									name: var_name,
									var_type: identifier,
								}));
							}
							_ => {
								return Err(String::from("Expected ; after variable declaration."));
							}
						}
					}
					_ => {
						return Err(format!("Unexpected {identifier}."));
					}
				}
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

pub fn parse(input: &mut Peekable<Chars>) -> Result<Vec<Statement>, String> {
	sub_parse(input, true)
}
