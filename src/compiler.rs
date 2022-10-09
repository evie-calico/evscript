use crate::types;
use crate::types::Rpn;

use std::collections::HashMap;
use std::fmt;
use std::io::Write;

#[derive(Debug)]
struct Environment {
	name: String,
	definitions: HashMap<String, types::Definition>,
	pool: u16,
}

impl Environment {
	fn std() -> Environment {
		macro_rules! define {
			($u:expr, $bytecode:expr) => {
				(
					String::from($u),
					types::Definition::Def(types::Def {
						args: vec![],
						bytecode: $bytecode,
					})
				)
			}
		}
		macro_rules! sign_alias {
			($u:expr, $i:expr) => {
				(
					String::from($i),
					types::Definition::Alias(types::Alias {
						args: vec![],
						target: String::from($u),
						target_args: vec![],
					})
				)
			}
		}

		Environment {
			name: String::from("std"),
			definitions: HashMap::from([
				define!("return", 0),
				define!("yield", 1),
				define!("put_u8", 2),
				sign_alias!("put_u8", "put_i8"),
				define!("mov_u8", 3),
				sign_alias!("mov_u8", "mov_i8"),
				define!("add_u8", 4),
				sign_alias!("add_u8", "add_i8"),
				define!("sub_u8", 5),
				sign_alias!("sub_u8", "sub_i8"),
				define!("mul_u8", 6),
				sign_alias!("mul_u8", "mul_i8"),
				define!("div_u8", 7),
				sign_alias!("div_u8", "div_i8"),
				define!("mod_u8", 8),
				sign_alias!("mod_u8", "mod_i8"),
				define!("shl_u8", 9),
				sign_alias!("shl_u8", "shl_i8"),
				define!("shr_u8", 10),
				sign_alias!("shr_u8", "shr_i8"),
				define!("band_u8", 11),
				sign_alias!("band_u8", "band_i8"),
				define!("bxor_u8", 12),
				sign_alias!("bxor_u8", "bxor_i8"),
				define!("bor_u8", 13),
				sign_alias!("bor_u8", "bor_i8"),
				define!("equ_u8", 14),
				sign_alias!("equ_u8", "equ_i8"),
				define!("nequ_u8", 15),
				sign_alias!("nequ_u8", "nequ_i8"),
				define!("lt_u8", 16),
				sign_alias!("lt_u8", "lt_i8"),
				define!("gt_u8", 17),
				sign_alias!("gt_u8", "gt_i8"),
				define!("lte_u8", 18),
				sign_alias!("lte_u8", "lte_i8"),
				define!("gte_u8", 19),
				sign_alias!("gte_u8", "gte_i8"),
				define!("land_u8", 20),
				sign_alias!("land_u8", "land_i8"),
				define!("lor_u8", 21),
				sign_alias!("lor_u8", "lor_i8"),
			]),
			pool: 0,
		}
	}

	fn expand(&self, name: &str) -> Result<String, String> {
		let def = match self.definitions.get(name) {
			Some(def) => def,
			None => return Err(format!("Definition of {name} not found in {self:#?}")),
		};

		match def {
			types::Definition::Def(def) => {
				Ok(format!("{}@{}", self.name, name))
			}
			types::Definition::Alias(alias) => {
				self.expand(&alias.target)
			}
			types::Definition::Macro(..) => Err(format!("{name} may not be a macro")),
			_ => Err(format!("{name} is not defined")),
		}
	}
}

type EnvironmentTable = HashMap<String, Environment>;

#[derive(Debug, Copy, Clone)]
struct Type {
	signed: bool,
	size: u8,
}

impl Type {
	fn from(l: Type, r: Type) -> Type {
		Type {
			signed: l.signed || r.signed,
			size: if l.size >= r.size { l.size } else { r.size },
		}
	}
}

impl fmt::Display for Type {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}", if self.signed { 'i' } else { 'u' }, self.size * 8)
	}
}

type TypeTable = HashMap<String, Type>;

#[derive(Debug)]
struct Variable {
	name: Option<String>,
	t: Type,
}

#[derive(Debug)]
struct VariableTable {
	variables: [Option<Variable>; 256]
}

impl VariableTable {
	fn new() -> VariableTable {
		VariableTable { variables: [
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
			None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
		]}
	}

	fn alloc(&mut self, t: Type) -> Result<u8, String> {
		let mut i = 0;

		while i < 256 {
			match &self.variables[i] {
				Some(var) => i += var.t.size as usize,
				None => {
					let new_var = Variable {
						name: None,
						t,
					};
					self.variables[i as usize] = Some(new_var);
					return Ok(i as u8);
				}
			}
		}

		Err(String::from("Out of variable space; a single function is limited to 256 bytes"))
	}

	fn lookup(&self, name: &str) -> Result<u8, String> {
		let mut i = 0;

		while i < 256 {
			if let Some(variable) = &self.variables[i] {
				if let Some(variable_name) = &variable.name {
					if variable_name == name {
						return Ok(i as u8);
					}
				}
				i += variable.t.size as usize;
			} else {
				i += 1;
			}
		}

		Err(format!("Variable {name} does not exist"))
	}

	fn name_of(&mut self, i: u8) -> &mut Option<String> {
		match &mut self.variables[i as usize] {
			Some(var) => &mut var.name,
			None => panic!("Variable index {i} does not exist"),
		}
	}

	fn type_of(&mut self, i: u8) -> Type {
		match &self.variables[i as usize] {
			Some(var) => var.t,
			None => panic!("Variable index {i} does not exist"),
		}
	}
}

fn compile_environment<W: Write>(
	this_name: &str,
	env: types::Environment,
	environment_table: &EnvironmentTable,
	output: &mut W,
) -> Result<Environment, String> {
	let mut compiled_env = Environment {
		name: String::from(this_name),
		definitions: HashMap::<String, types::Definition>::new(),
		pool: 0,
	};

	let mut bytecode_index: u8 = 0;

	for i in env.contents {
		match i {
			types::Statement::Use(name) => {
				let other_env = match environment_table.get(&name) {
					Some(other_env) => other_env,
					None => return Err(format!("Environment {name} does not exist")),
				};

				let mut greatest_bytecode = bytecode_index;

				for (def_name, def) in &other_env.definitions {
					if compiled_env.definitions.get(def_name).is_some() {
						eprintln!("WARN: duplicate definition of {def_name} inside `use` statement.");
					}
					
					let mut new_def = def.clone();

					match new_def {
						types::Definition::Def(ref mut sub_def) => {
							sub_def.bytecode = bytecode_index.checked_add(sub_def.bytecode)
								.ok_or(format!("Hit bytecode limit in environment {this_name}"))?;
							writeln!(output, "def {this_name}@{def_name} equ {}", sub_def.bytecode)
								.map_err(|err| err.to_string())?;
							if sub_def.bytecode > greatest_bytecode {
								greatest_bytecode = sub_def.bytecode;
							}
						}
						_ => {}
					}

					compiled_env.definitions.insert(def_name.clone(), new_def);
				}

				bytecode_index = greatest_bytecode;
			}
			types::Statement::Definition(name, mut def) => {
				if compiled_env.definitions.get(&name).is_some() {
					eprintln!("WARN: duplicate definition of {name}");
				}

					writeln!(output, "def {this_name}@{name} equ {bytecode_index}")
						.map_err(|err| err.to_string())?;

					match def {
						types::Definition::Def(ref mut sub_def) => {
							sub_def.bytecode = bytecode_index;
							bytecode_index = bytecode_index.checked_add(1)
								.ok_or(format!("Hit bytecode limit in environment {this_name}"))?;
						}
						_ => {}
					}

					compiled_env.definitions.insert(name.clone(), def);
			}
			types::Statement::Pool(expression) => {
				let pool_size = expression.eval_const()?;

				compiled_env.pool = if pool_size < 0 {
					return Err(String::from("Pool size may not be negative"));
				} else if pool_size > 256 {
					return Err(String::from("Pool size is limited to 256 bytes"));
				} else {
					pool_size as u16
				};
			}
			_ => return Err(format!("Statement {i:?} is not allowed within environments.")),
		}
	}

	Ok(compiled_env)
}

/// Compiles an Rpn tree, returning a variable containing the final result.
fn compile_expression<W: Write>(
	rpn: Rpn,
	env: &Environment,
	vtable: & mut VariableTable,
	output: &mut W
) -> Result<u8, String> {
	fn binary_operation<W: Write>(
		l: Box<Rpn>,
		op: &str,
		r: Box<Rpn>,
		env: &Environment,
		vtable: & mut VariableTable,
		output: &mut W
	) -> Result<u8, String> {
		let l = compile_expression(*l, env, vtable, output)?;
		let r = compile_expression(*r, env, vtable, output)?;

		let result_type = Type::from(vtable.type_of(l), vtable.type_of(r));
		let result = vtable.alloc(result_type)?;
		// TODO: make opcodes consider operation size.

		writeln!(output, "\tdb {}, {result}, {l}, {r}", env.expand(&format!("{op}_{result_type}"))?)
			.map_err(|err| err.to_string())?;
		Ok(result)
	}

	match rpn {
		Rpn::Variable(name) => vtable.lookup(&name),
		Rpn::Signed(value) => {
			// The "default" type of an integer is i8 (think C's int)
			// This is because most projects will probably only have the 8-bit bytecode installed.
			// TODO: make the default integer type configurable per-environment
			let result_type = Type { signed: true, size: 1 };
			let result = vtable.alloc(result_type)?;
			// put (result), value
			writeln!(output, "\tdb {}, {result}, {value}", env.expand(&format!("put_{result_type}"))?)
				.map_err(|err| err.to_string())?;
			Ok(result)
		}
		Rpn::String(..) => todo!(),
		Rpn::Call(name, args) => {
			let mut arg_ids = Vec::<u8>::new();

			for i in args {
				arg_ids.push(compile_expression(i, env, vtable, output)?);
			}
			write!(output, "\tdb {}", env.expand(&name)?)
				.map_err(|err| err.to_string())?;
			for i in arg_ids {
				write!(output, ", {i}")
					.map_err(|err| err.to_string())?;
			}
			writeln!(output, "");
			Ok(0)
		}
		Rpn::Negate(i) => {
			let operand = compile_expression(*i, env, vtable, output)?;
			let operand_type = vtable.type_of(operand);
			let zero = vtable.alloc(operand_type)?;
			let result = vtable.alloc(operand_type)?;
			// TODO: make opcodes consider operand size.
			writeln!(output, "\tdb {}, {zero}, 0", env.expand(&format!("put_{operand_type}"))?)
				.map_err(|err| err.to_string())?;
			writeln!(output, "\tdb {}, {result}, {zero}, {operand}", env.expand(&format!("sub_{operand_type}"))?)
				.map_err(|err| err.to_string())?;
			Ok(result)
		}
		Rpn::Not(i) => {
			let operand = compile_expression(*i, env, vtable, output)?;
			let operand_type = vtable.type_of(operand);
			// TODO: make the default integer type configurable per-environment
			let ff = vtable.alloc(operand_type)?;
			let result = vtable.alloc(operand_type)?;
			writeln!(output, "\tdb {}, {ff}, $FF", env.expand(&format!("put_{operand_type}"))?)
				.map_err(|err| err.to_string())?;
			writeln!(output, "\tdb {}, {result}, {operand}, {ff}", env.expand(&format!("xor_{operand_type}"))?)
				.map_err(|err| err.to_string())?;
			Ok(result)
		}
		Rpn::Deref(..) => todo!(),
		Rpn::Address(..) => todo!(),
		Rpn::Mul(l, r) => binary_operation(l, "mul", r, env, vtable, output),
		Rpn::Div(l, r) => binary_operation(l, "div", r, env, vtable, output),
		Rpn::Mod(l, r) => binary_operation(l, "mod", r, env, vtable, output),
		Rpn::Add(l, r) => binary_operation(l, "add", r, env, vtable, output),
		Rpn::Sub(l, r) => binary_operation(l, "sub", r, env, vtable, output),
		Rpn::ShiftLeft(l, r) => binary_operation(l, "shl", r, env, vtable, output),
		Rpn::ShiftRight(l, r) => binary_operation(l, "shr", r, env, vtable, output),
		Rpn::BinaryAnd(l, r) => binary_operation(l, "band", r, env, vtable, output),
		Rpn::BinaryXor(l, r) => binary_operation(l, "bxor", r, env, vtable, output),
		Rpn::BinaryOr(l, r) => binary_operation(l, "bor", r, env, vtable, output),
		Rpn::Equ(l, r) => binary_operation(l, "equ", r, env, vtable, output),
		Rpn::NotEqu(l, r) => binary_operation(l, "nequ", r, env, vtable, output),
		Rpn::LessThan(l, r) => binary_operation(l, "lt", r, env, vtable, output),
		Rpn::GreaterThan(l, r) => binary_operation(l, "gt", r, env, vtable, output),
		Rpn::LessThanEqu(l, r) => binary_operation(l, "lte", r, env, vtable, output),
		Rpn::GreaterThanEqu(l, r) => binary_operation(l, "gte", r, env, vtable, output),
		Rpn::LogicalAnd(l, r) => binary_operation(l, "land", r, env, vtable, output),
		Rpn::LogicalOr(l, r) => binary_operation(l, "lor", r, env, vtable, output),
		Rpn::Set(name, i) => {
			// A plain Set may only assign to existing variables.
			let dest = vtable.lookup(&name)?;
			let dest_type = vtable.type_of(dest);
			// TODO: make this directly take ownership of i if it is not an Rpn::Variable.
			let source = compile_expression(*i, env, vtable, output)?;
			writeln!(output, "\tdb {}, {dest}, {source}", env.expand(&format!("mov_{dest_type}"))?)
				.map_err(|err| err.to_string())?;
			Ok(dest)
		}
	}
}

fn compile_statement<W: Write>(
	statement: types::Statement,
	env: &Environment,
	vtable: &mut VariableTable,
	output: &mut W
) -> Result<(), String> {
	match statement {
		types::Statement::Expression(rpn) => {
			compile_expression(rpn, env, vtable, output)?;
		}
		types::Statement::Declaration(t, name) => {
			eprintln!("WARN: type currently defaults to u8");
			let new_var = vtable.alloc(Type { signed: false, size: 1 })?;
			*vtable.name_of(new_var) = Some(name);
		}
		types::Statement::DeclareAssign(t, name, rpn) => {
			eprintln!("WARN: type currently defaults to u8");

			// Create a new variable
			let new_var = vtable.alloc(Type { signed: false, size: 1 })?;
			*vtable.name_of(new_var) = Some(name.clone());
			// Compile the Set.
			compile_expression(rpn, env, vtable, output)?;
		},
		types::Statement::If(..) => todo!(),
		types::Statement::While(..) => todo!(),
		types::Statement::Do(..) => todo!(),
		types::Statement::For(..) => todo!(),
		types::Statement::Repeat(..) => todo!(),
		types::Statement::Loop(..) => todo!(),
		_ => return Err(format!("{statement:?} not allowed in function")),
	};

	Ok(())
}

fn compile_function<W: Write>(
	name: &str,
	func: types::Function,
	environment_table: &EnvironmentTable,
	type_table: &TypeTable,
	output: &mut W
) -> Result<(), String> {
	let env = match environment_table.get(&func.environment) {
		Some(env) => env,
		None => return Err(format!("Environment {} does not exist", func.environment)),
	};
	let mut vtable = VariableTable::new();

	writeln!(output, "\nsection \"{name} evscript fn\", romx\n{name}::")
		.map_err(|err| err.to_string())?;

	for i in func.contents {
		compile_statement(i, env, &mut vtable, output)?;
	}

	writeln!(output, "\tdb 0")
		.map_err(|err| err.to_string())?;

	Ok(())
}

pub fn compile<W: Write>(ast: Vec<types::Root>, mut output: W) -> Result<(), String> {
	let mut environment_table = EnvironmentTable::from([
		(String::from("std"), Environment::std()),
	]);

	let mut type_table = TypeTable::from([
		(String::from("u8"), Type { signed: false, size: 1} ),
	]);

	for i in ast {
		match i {
			types::Root::Environment(name, env) => {
				let new_env = compile_environment(&name, env, &environment_table, &mut output)?;
				environment_table.insert(name, new_env);
			}
			types::Root::Function(name, func) => {
				let new_func = compile_function(
					&name,
					func,
					&environment_table,
					&type_table,
					&mut output
				)?;
			}
			types::Root::Assembly(contents) => todo!(),
			types::Root::Include(path) => todo!(),
		}
	}

	Ok(())
}
