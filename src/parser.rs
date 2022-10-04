#![allow(dead_code)]

use std::vec::Vec;

#[derive(Debug)]
pub enum Statement {
	// Environment statements
	Use(String),
	Def(Def),
	Alias(Alias),
	Macro(Macro),
	Pool(Rpn),
	Terminator(String),
	// Function statements
	Expression(Rpn),
	Declaration(String, String),
	DeclareAssign(String, Box<Statement>),
	If(Rpn, Vec<Statement>, Option<Vec<Statement>>),
	While(Rpn, Vec<Statement>),
	Do(Rpn, Vec<Statement>),
	For(Box<Statement>, Rpn, Box<Statement>, Vec<Statement>),
	Repeat(Option<String>, Rpn, Vec<Statement>),
	Loop(Vec<Statement>),
}

#[derive(Debug)]
pub enum Root {
	Environment(Environment),
	Function(Function),
	Assembly(String),
	Include(String),
}

// Top-level statements.
#[derive(Debug)]
pub struct Environment {
	pub name: String,
	pub contents: Vec<Statement>
}

#[derive(Debug)]
pub struct Function {
	pub name: String,
	pub environment: String,
	pub args: Vec<String>,
	pub contents: Vec<Statement>
}

// Environment statements
#[derive(Debug)]
pub struct Def {
	pub name: String,
	pub args: Vec<String>,
}

#[derive(Debug)]
pub struct Macro {
	pub name: String,
	pub args: Vec<String>,
	pub target: String,
	pub varargs: bool,
}

#[derive(Debug)]
pub struct Alias {
	pub name: String,
	pub args: Vec<String>,
	pub target: String,
	pub target_args: Vec<String>,
}

#[derive(Debug)]
pub enum Rpn {
	// Values
	Variable(String),
	Unsigned(u64),
	Signed(i64),
	String(String),
	Call(String, Vec<Rpn>),
	// Unary
	Negate(Box<Rpn>),
	Deref(Box<Rpn>),
	Not(Box<Rpn>),
	Address(String),
	// Factors
	Mul(Box<Rpn>, Box<Rpn>),
	Div(Box<Rpn>, Box<Rpn>),
	Mod(Box<Rpn>, Box<Rpn>),
	// Addition
	Add(Box<Rpn>, Box<Rpn>),
	Sub(Box<Rpn>, Box<Rpn>),
	// Shifts
	ShiftLeft(Box<Rpn>, Box<Rpn>),
	ShiftRight(Box<Rpn>, Box<Rpn>),
	// Binaries
	BinaryAnd(Box<Rpn>, Box<Rpn>),
	BinaryXor(Box<Rpn>, Box<Rpn>),
	BinaryOr(Box<Rpn>, Box<Rpn>),
	// Compares
	Equ(Box<Rpn>, Box<Rpn>),
	NotEqu(Box<Rpn>, Box<Rpn>),
	LessThan(Box<Rpn>, Box<Rpn>),
	GreaterThan(Box<Rpn>, Box<Rpn>),
	LessThanEqu(Box<Rpn>, Box<Rpn>),
	GreaterThanEqu(Box<Rpn>, Box<Rpn>),
	// Logicals
	LogicalAnd(Box<Rpn>, Box<Rpn>),
	LogicalOr(Box<Rpn>, Box<Rpn>),
	// += is constructed using a Set(self, Add(self, <expression>))
	Set(String, Box<Rpn>),
}
