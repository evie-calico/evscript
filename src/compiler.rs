use crate::types;
use crate::types::Rpn;

use std::collections::HashMap;
use std::io::Write;

#[derive(Debug)]
struct Environment {
	definitions: HashMap<String, types::Definition>,
	pool: u16,
}

impl Environment {
	fn std() -> Environment {
		Environment {
			definitions: HashMap::from([
				(
					String::from("return"),
					types::Definition::Def(types::Def { args: vec![], bytecode: 0 } )
				),
			]),
			pool: 0,
		}
	}
}

struct Variable {
	name: Option<String>,
	index: u8,
	size: u8,
}

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

	fn alloc(&mut self, size: u8) -> Result<u8, String> {
		for mut i in 0..256 {
			match &self.variables[i] {
				Some(var) => i += var.size as usize,
				None => {
					let mut new_var = Variable {
						name: None,
						index: i as u8,
						size,
					};
					self.variables[i] = Some(new_var);
					return Ok(i as u8);
				}
			}
		}

		Err(String::from("Out of variable space; a single function is limited to 256 bytes"))
	}

	fn lookup(&self, name: &str) -> Result<u8, String> {
		for mut i in 0..256 {
			if let Some(variable) = &self.variables[i] {
				if let Some(variable_name) = &variable.name {
					if variable_name == name {
						return Ok(i as u8);
					}
				}
				i += variable.size as usize;
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

	fn size_of(&mut self, i: u8) -> u8 {
		match &self.variables[i as usize] {
			Some(var) => var.size,
			None => panic!("Variable index {i} does not exist"),
		}
	}
}

fn compile_environment(
	name: &str,
	env: types::Environment,
	environment_table: &HashMap<String, Environment>
) -> Result<Environment, String> {
	let mut compiled_env = Environment {
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
				for (def_name, def) in &other_env.definitions {
					if compiled_env.definitions.get(def_name).is_some() {
						eprintln!("WARN: duplicate definition of {def_name} inside `use` statement.");
					}
					
					let mut new_def = def.clone();

					match new_def {
						types::Definition::Def(ref mut sub_def) => {
							sub_def.bytecode = bytecode_index;
							bytecode_index = bytecode_index.checked_add(1)
								.ok_or(format!("Hit bytecode limit in environment {name}"))?;
						}
						_ => {}
					}

					compiled_env.definitions.insert(def_name.clone(), new_def);
				}
			}
			types::Statement::Definition(name, mut def) => {
				if compiled_env.definitions.get(&name).is_some() {
					eprintln!("WARN: duplicate definition of {name}");
				}
					match def {
						types::Definition::Def(ref mut sub_def) => {
							sub_def.bytecode = bytecode_index;
							bytecode_index = bytecode_index.checked_add(1)
								.ok_or(format!("Hit bytecode limit in environment {name}"))?;
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
	vtable: & mut VariableTable,
	output: &mut W
) -> Result<u8, String> {
	fn binary_operation<W: Write>(
		l: Box<Rpn>,
		op: &str,
		r: Box<Rpn>,
		vtable: & mut VariableTable,
		output: &mut W
	) -> Result<u8, String> {
		let l = compile_expression(*l, vtable, output)?;
		let r = compile_expression(*r, vtable, output)?;

		let l_size = vtable.size_of(l);
		let r_size = vtable.size_of(r);
		let operation_size = if l_size > r_size { l_size } else { r_size };

		let result = vtable.alloc(operation_size)?;
		// TODO: make opcodes consider operation size.

		writeln!(output, "\tstd@{op}_u8 {result}, {l}, {r}").map_err(|err| err.to_string())?;
		Ok(result)
	}

	match rpn {
		Rpn::Variable(name) => vtable.lookup(&name),
		Rpn::Signed(value) => {
			// The "default" type of an integer is i8 (think C's int)
			// This is because most projects will probably only have the 8-bit bytecode installed.
			// TODO: make the default integer type configurable per-environment
			let result = vtable.alloc(1)?;
			// put (result), value
			writeln!(output, "\tstd@put_u8 {result}, {value}").map_err(|err| err.to_string())?;
			Ok(result)
		}
		Rpn::String(..) => todo!(),
		Rpn::Call(..) => todo!(),
		Rpn::Negate(i) => {
			let operand = compile_expression(*i, vtable, output)?;
			let operand_size = vtable.size_of(operand);
			let zero = vtable.alloc(operand_size)?;
			let result = vtable.alloc(operand_size)?;
			// TODO: make opcodes consider operand size.
			writeln!(output, "\tstd@put_u8 {zero}, 0").map_err(|err| err.to_string())?;
			writeln!(output, "\tstd@sub_u8 {result}, {zero}, {operand}").map_err(|err| err.to_string())?;
			Ok(result)
		}
		Rpn::Not(i) => {
			let operand = compile_expression(*i, vtable, output)?;
			let operand_size = vtable.size_of(operand);
			// TODO: make the default integer type configurable per-environment
			let ff = vtable.alloc(operand_size)?;
			let result = vtable.alloc(operand_size)?;
			writeln!(output, "\tstd@put_u8 {ff}, $FF").map_err(|err| err.to_string())?;
			writeln!(output, "\tstd@xor_u8 {result}, {operand}, {ff}").map_err(|err| err.to_string())?;
			Ok(result)
		}
		Rpn::Deref(..) => todo!(),
		Rpn::Address(..) => todo!(),
		Rpn::Mul(l, r) => binary_operation(l, "mul", r, vtable, output),
		Rpn::Div(l, r) => binary_operation(l, "div", r, vtable, output),
		Rpn::Mod(l, r) => binary_operation(l, "mod", r, vtable, output),
		Rpn::Add(l, r) => binary_operation(l, "add", r, vtable, output),
		Rpn::Sub(l, r) => binary_operation(l, "sub", r, vtable, output),
		Rpn::ShiftLeft(l, r) => binary_operation(l, "shl", r, vtable, output),
		Rpn::ShiftRight(l, r) => binary_operation(l, "shr", r, vtable, output),
		Rpn::BinaryAnd(l, r) => binary_operation(l, "band", r, vtable, output),
		Rpn::BinaryXor(l, r) => binary_operation(l, "bxor", r, vtable, output),
		Rpn::BinaryOr(l, r) => binary_operation(l, "bor", r, vtable, output),
		Rpn::Equ(l, r) => binary_operation(l, "equ", r, vtable, output),
		Rpn::NotEqu(l, r) => binary_operation(l, "nequ", r, vtable, output),
		Rpn::LessThan(l, r) => binary_operation(l, "lt", r, vtable, output),
		Rpn::GreaterThan(l, r) => binary_operation(l, "gt", r, vtable, output),
		Rpn::LessThanEqu(l, r) => binary_operation(l, "lte", r, vtable, output),
		Rpn::GreaterThanEqu(l, r) => binary_operation(l, "gte", r, vtable, output),
		Rpn::LogicalAnd(l, r) => binary_operation(l, "land", r, vtable, output),
		Rpn::LogicalOr(l, r) => binary_operation(l, "lor", r, vtable, output),
		Rpn::Set(name, i) => {
			// A plain Set may only assign to existing variables.
			let dest = vtable.lookup(&name)?;
			// TODO: make this directly take ownership of i if it is not an Rpn::Variable.
			let source = compile_expression(*i, vtable, output)?;
			writeln!(output, "\tstd@mov_u8 {dest}, {source}").map_err(|err| err.to_string())?;
			Ok(dest)
		}
	}
}

fn compile_statement<W: Write>(
	statement: types::Statement,
	vtable: &mut VariableTable,
	output: &mut W
) -> Result<(), String> {
	match statement {
		types::Statement::Expression(rpn) => {
			compile_expression(rpn, vtable, output)?;
		}
		types::Statement::Declaration(t, name) => {
			eprintln!("WARN: type currently defaults to u8");
			let new_var = vtable.alloc(1)?;
			*vtable.name_of(new_var) = Some(name);
		}
		types::Statement::DeclareAssign(t, name, rpn) => {
			eprintln!("WARN: type currently defaults to u8");

			// Create a new variable
			let new_var = vtable.alloc(1)?;
			*vtable.name_of(new_var) = Some(name.clone());
			// Compile the Set.
			compile_expression(rpn, vtable, output)?;
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
	environment_table: &HashMap<String, Environment>,
	output: &mut W
) -> Result<(), String> {
	let env = match environment_table.get(&func.environment) {
		Some(env) => env,
		None => return Err(format!("Environment {} does not exist", func.environment)),
	};
	let mut vtable = VariableTable::new();

	writeln!(output, "section \"{name} evscript fn\", romx\n{name}::").map_err(|err| err.to_string())?;

	for i in func.contents {
		compile_statement(i, &mut vtable, output)?;
	}

	writeln!(output, "\tdb 0").map_err(|err| err.to_string())?;

	Ok(())
}

pub fn compile<W: Write>(ast: Vec<types::Root>, mut output: W) -> Result<(), String> {
	let mut environment_table = HashMap::<String, Environment>::from([
		(String::from("std"), Environment::std()),
	]);

	for i in ast {
		match i {
			types::Root::Environment(name, env) => {
				let new_env = compile_environment(&name, env, &environment_table)?;
				environment_table.insert(name, new_env);
			}
			types::Root::Function(name, func) => {
				let new_func = compile_function(&name, func, &environment_table, &mut output)?;
			}
			types::Root::Assembly(contents) => todo!(),
			types::Root::Include(path) => todo!(),
		}
	}

	Ok(())
}
