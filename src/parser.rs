#![allow(dead_code)]

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
	// Generic statements
	Expression(Rpn)
}

// Top-level statements.
#[derive(Debug)]
pub struct Environment {
	pub name: String,
	pub contents: Vec<Statement>
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
	pub name: String,
	pub args: Vec<String>,
}

// Function statements
#[derive(Debug)]
pub struct VariableDeclaration {
	name: String,
	var_type: String,
}

#[derive(Debug)]
pub enum Rpn {
	// Values
	Variable(String),
	Unsigned(u64),
	Signed(i64),
	// Unary Operations
	Negate(Box<Rpn>),
	Deref(Box<Rpn>),
	Address(String),
	// Binary Operations
	Add(Box<Rpn>, Box<Rpn>),
	Sub(Box<Rpn>, Box<Rpn>),
	Mul(Box<Rpn>, Box<Rpn>),
	Div(Box<Rpn>, Box<Rpn>),
	Mod(Box<Rpn>, Box<Rpn>),
	Equ(Box<Rpn>, Box<Rpn>),
	NotEqu(Box<Rpn>, Box<Rpn>),
	LogicalAnd(Box<Rpn>, Box<Rpn>),
	LogicalOr(Box<Rpn>, Box<Rpn>),
	// += is constructed using a Set(self, Add(self, <expression>))
	Set(String, Box<Rpn>),
}

impl Rpn {
	fn precedence(&self) -> u32 {
		match self {
			Rpn::Variable(..) => 0,
			Rpn::Unsigned(..) => 0,
			Rpn::Signed(..) => 0,
			Rpn::Negate(..) => 1,
			Rpn::Deref(..) => 1,
			Rpn::Address(..) => 1,
			Rpn::Mul(..) => 2,
			Rpn::Div(..) => 2,
			Rpn::Mod(..) => 2,
			Rpn::Add(..) => 3,
			Rpn::Sub(..) => 3,
			Rpn::Equ(..) => 4,
			Rpn::NotEqu(..) => 4,
			Rpn::LogicalAnd(..) => 5,
			Rpn::LogicalOr(..) => 6,
			Rpn::Set(..) => 7,
		}
	}
}

fn expect_identifier(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<String, String> {
	let token = lex(input, loc)?;

	match token {
		Token::Identifier(name) => Ok(name),
		_ => Err(format!("Expected Identifier, got {token}"))
	}
}

fn expect_string(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<String, String> {
	let token = lex(input, loc)?;

	match token {
		Token::String(string) => Ok(string),
		_ => Err(format!("Expected String, got {token}"))
	}
}

fn expect_lparen(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<(), String> {
	let token = lex(input, loc)?;

	if token != Token::LeftParenthesis {
		Err(format!("Expected (, got {token}"))
	} else {
		Ok(())
	}
}

fn expect_lbrace(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<(), String> {
	let token = lex(input, loc)?;

	if token != Token::LeftCurlyBrace {
		Err(format!("Expected {{, got {token}"))
	} else {
		Ok(())
	}
}

fn expect_semicolon(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<(), String> {
	let token = lex(input, loc)?;

	if token != Token::Semicolon {
		Err(format!("Expected ;, got {token}"))
	} else {
		Ok(())
	}
}

// For binary operations; checks for an operator and constructs an Rpn node for it.
fn parse_operation(input: &mut Peekable<Chars>, loc: &mut Location, lhs: Rpn) -> Result<Rpn, String> {
	match lex(input, loc)? {
		Token::Plus => Ok(Rpn::Add(Box::new(lhs), Box::new(parse_expression(lex(input, loc)?, input, loc)?))),
		Token::Star => Ok(Rpn::Mul(Box::new(lhs), Box::new(parse_expression(lex(input, loc)?, input, loc)?))),
		Token::Semicolon => Ok(lhs),
		token @ _ => return Err(format!("Unexpected {token}"))
	}
}

fn parse_expression(token: Token, input: &mut Peekable<Chars>, loc: &mut Location) -> Result<Rpn, String> {
	// Unary context
	let lhs = match token {
		Token::Identifier(identifier) => Rpn::Variable(identifier),
		Token::Minus => Rpn::Negate(Box::new(parse_expression(lex(input, loc)?, input, loc)?)),
		Token::Star => Rpn::Deref(Box::new(parse_expression(lex(input, loc)?, input, loc)?)),
		_ => return Err(format!("Unexpected {token}"))
	};

	parse_operation(input, loc, lhs)
}

fn parse_environment(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<Vec<Statement>, String> {
	let mut ast = Vec::<Statement>::new();

	loop {
		match lex(input, loc)? {
			Token::Use => {
				ast.push(Statement::Use(expect_identifier(input, loc)?));
				expect_semicolon(input, loc)?;
			}

			Token::Def => {
				let name = expect_identifier(input, loc)?;
				expect_lparen(input, loc)?;

				let mut args = Vec::<String>::new();

				loop {
					match lex(input, loc)? {
						Token::RightParenthesis => break,
						Token::Identifier(string) => args.push(string),
						token @ _ => return Err(format!("Unexpected {token} in parameter list")),
					}
				}

				expect_semicolon(input, loc)?;

				ast.push(Statement::Def(Def { name, args }));
			}

			Token::RightCurlyBrace => break,
			token @ _ => return Err(format!("Unexpected {token}"))
		}
	}

	Ok(ast)
}

fn parse_function(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<Vec<Statement>, String> {
	let mut ast = Vec::<Statement>::new();

	loop {
		match lex(input, loc)? {
			Token::RightCurlyBrace => break,
			token @ _ => ast.push(Statement::Expression(parse_expression(token, input, loc)?))
		}
	}

	Ok(ast)
}

fn parse_root(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<Vec<Statement>, String> {
	let mut ast = Vec::<Statement>::new();

	loop {
		match lex(input, loc)? {
			Token::Environment => {
				let name = expect_identifier(input, loc)?;
				expect_lbrace(input, loc)?;

				ast.push(Statement::Environment(Environment {
					name,
					contents: parse_environment(input, loc)?
				}));
			}

			Token::Include => {
				match lex(input, loc)? {
					Token::String(..) => todo!(),
					Token::Asm => {
						ast.push(Statement::InlineAssembly(format!(
							"include \"{}\"",
							expect_string(input, loc)?
						)));
					}
					token @ _ => return Err(format!("Expected `asm` after `include`, got {token}"))
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
						return Err(format!("Unexpected identifier \"{identifier}\""));
					}
				}
			}

			Token::InlineAssembly(assembly) => {
				ast.push(Statement::InlineAssembly(assembly));
			}

			Token::Eof => break,
			token @ _ => return Err(format!("Unexpected {token}"))
		}
	}

	Ok(ast)
}

pub fn parse(input: &mut Peekable<Chars>, loc: &mut Location) -> Result<Vec<Statement>, String> {
	parse_root(input, loc)
}
