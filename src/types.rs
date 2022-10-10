use std::vec::Vec;

#[derive(Debug)]
pub enum Statement {
	// Environment statements
	Use(String),
	Definition(String, Definition),
	Pool(Rpn),
	// Function statements
	Expression(Rpn),
	Declaration(String, String),
	DeclareAssign(String, String, Rpn),
	If(Rpn, Vec<Statement>, Option<Vec<Statement>>),
	While(Rpn, Vec<Statement>),
	Do(Rpn, Vec<Statement>),
	For(Box<Statement>, Rpn, Box<Statement>, Vec<Statement>),
	Repeat(Option<String>, Rpn, Vec<Statement>),
	Loop(Vec<Statement>),
}

#[derive(Debug)]
pub enum Root {
	Environment(String, Environment),
	Function(String, Function),
	Assembly(String),
	Include(String),
}

// Top-level statements.
#[derive(Debug)]
pub struct Environment {
	pub contents: Vec<Statement>
}

#[derive(Debug)]
pub struct Function {
	pub environment: String,
	pub contents: Vec<Statement>
}

// Environment statements
#[derive(Debug, Clone)]
pub enum Definition {
	Def(Def),
	Alias(Alias),
	Macro(Macro),
}

#[derive(Debug, Clone)]
pub struct Def {
	/// The lookup value of this definition.
	pub bytecode: u8,
	pub args: Vec<DefinitionParam>,
}

#[derive(Debug, Clone)]
pub struct Macro {
	pub args: Vec<DefinitionParam>,
	pub target: String,
	pub varargs: bool,
}

#[derive(Debug, Clone)]
pub struct Alias {
	pub args: Vec<DefinitionParam>,
	pub target: String,
	pub target_args: Vec<AliasParam>,
}

#[derive(Debug, Clone)]
pub enum DefinitionParam {
	Return(String),
	Type(String),
}

#[derive(Debug, Clone)]
pub enum AliasParam {
	ArgId(usize),
	Expression(Rpn),
}

#[derive(Debug, Clone)]
pub enum Rpn {
	// Values
	Variable(String),
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

impl Rpn {
	pub fn eval_const(&self) -> Result<i64, String> {
		Ok(match self {
			Rpn::Variable(..) => return Err(format!("Unexpected variable, expression must be constant")),
			Rpn::String(..) => return Err(format!("Unexpected string, expression must be constant")),
			Rpn::Call(..) => return Err(format!("Unexpected call, expression must be constant")),
			Rpn::Deref(..) => return Err(format!("Unexpected dereference, expression must be constant")),
			Rpn::Address(..) => return Err(format!("Unexpected address operator, expression must be constant")),
			Rpn::Set(..) => return Err(format!("Unexpected assignment, expression must be constant")),

			Rpn::Signed(value) => *value,

			Rpn::Negate(i) => i.eval_const()?,
			Rpn::Not(i) => i.eval_const()?,

			Rpn::Mul(l, r) => l.eval_const()? * r.eval_const()?,
			Rpn::Div(l, r) => l.eval_const()? / r.eval_const()?,
			Rpn::Mod(l, r) => l.eval_const()? % r.eval_const()?,
			Rpn::Add(l, r) => l.eval_const()? + r.eval_const()?,
			Rpn::Sub(l, r) => l.eval_const()? - r.eval_const()?,
			Rpn::ShiftLeft(l, r) => l.eval_const()? << r.eval_const()?,
			Rpn::ShiftRight(l, r) => l.eval_const()? >> r.eval_const()?,
			Rpn::BinaryAnd(l, r) => l.eval_const()? & r.eval_const()?,
			Rpn::BinaryXor(l, r) => l.eval_const()? ^ r.eval_const()?,
			Rpn::BinaryOr(l, r) => l.eval_const()? | r.eval_const()?,
			Rpn::Equ(l, r) => (l.eval_const()? == r.eval_const()?) as i64,
			Rpn::NotEqu(l, r) => (l.eval_const()? != r.eval_const()?) as i64,
			Rpn::LessThan(l, r) => (l.eval_const()? < r.eval_const()?) as i64,
			Rpn::GreaterThan(l, r) => (l.eval_const()? > r.eval_const()?) as i64,
			Rpn::LessThanEqu(l, r) => (l.eval_const()? <= r.eval_const()?) as i64,
			Rpn::GreaterThanEqu(l, r) => (l.eval_const()? >= r.eval_const()?) as i64,
			Rpn::LogicalAnd(l, r) => (l.eval_const()? != 0 && r.eval_const()? != 0) as i64,
			Rpn::LogicalOr(l, r) => (l.eval_const()? != 0 || r.eval_const()? != 0) as i64,
		})
	}
}
