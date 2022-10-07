use crate::types;
use crate::types::Rpn;

use std::collections::HashMap;
use std::io::Write;

#[derive(Debug)]
struct Environment {
	definitions: HashMap<String, types::Definition>,
	pool: u16,
	terminator: Option<u8>,
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
			terminator: None
		}
	}
}

struct Function {
	env: String,
	args: Vec<String>,
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

	fn name_of(&mut self, i: u8) -> &Option<String> {
		match &self.variables[i as usize] {
			Some(var) => &var.name,
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
		terminator: None,
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
			types::Statement::Terminator(name) => {
				eprintln!("`terminator` is unimplemented");
			}
			_ => return Err(format!("Statement {i:?} is not allowed within environments.")),
		}
	}

	Ok(compiled_env)
}

/// Compiles an Rpn tree, returning a variable containing the final result.
fn compile_expression<W: Write>(
	rpn: Rpn,
	variable_table: & mut VariableTable,
	function_table: &HashMap<String, Function>,
	output: &mut W
) -> Result<u8, String> {
	match rpn {
		Rpn::Variable(..) => todo!(),
		Rpn::Signed(value) => {
			// The "default" type of an integer is i8 (think C's int)
			// This is because most projects will probably only have the 8-bit bytecode installed.
			// TODO: make the default integer type configurable per-environment
			let container = variable_table.alloc(1)?;
			// put (container), value
			writeln!(output, "\tstd@put_u8, {container}, {value}").map_err(|err| err.to_string())?;
			Ok(container)
		}
		Rpn::String(..) => todo!(),
		Rpn::Call(..) => todo!(),
		Rpn::Negate(sub_rpn) => {
			let operand = compile_expression(*sub_rpn, variable_table, function_table, output)?;
			let operand_size = variable_table.size_of(operand);
			let zero = variable_table.alloc(operand_size)?;
			// TODO: make opcodes consider operand size.
			// put (zero), 0
			writeln!(output, "\tstd@put_u8, {zero}, 0").map_err(|err| err.to_string())?;
			// (operand) = (zero) - (operand)
			writeln!(output, "\tstd@sub_u8, {operand}, {zero}, {operand}").map_err(|err| err.to_string())?;
			Ok(operand)
		}
		Rpn::Deref(..) => todo!(),
		Rpn::Not(..) => todo!(),
		Rpn::Address(..) => todo!(),
		Rpn::Mul(..) => todo!(),
		Rpn::Div(..) => todo!(),
		Rpn::Mod(..) => todo!(),
		Rpn::Add(..) => todo!(),
		Rpn::Sub(..) => todo!(),
		Rpn::ShiftLeft(..) => todo!(),
		Rpn::ShiftRight(..) => todo!(),
		Rpn::BinaryAnd(..) => todo!(),
		Rpn::BinaryXor(..) => todo!(),
		Rpn::BinaryOr(..) => todo!(),
		Rpn::Equ(..) => todo!(),
		Rpn::NotEqu(..) => todo!(),
		Rpn::LessThan(..) => todo!(),
		Rpn::GreaterThan(..) => todo!(),
		Rpn::LessThanEqu(..) => todo!(),
		Rpn::GreaterThanEqu(..) => todo!(),
		Rpn::LogicalAnd(..) => todo!(),
		Rpn::LogicalOr(..) => todo!(),
		Rpn::Set(..) => todo!(),
	}
}

fn compile_statement<W: Write>(
	statement: types::Statement,
	variable_table: &mut VariableTable,
	function_table: &HashMap<String, Function>,
	output: &mut W
) -> Result<(), String> {
	match statement {
		types::Statement::Expression(rpn) => compile_expression(rpn, variable_table, function_table, output)?,
		types::Statement::Declaration(..) => todo!(),
		types::Statement::DeclareAssign(..) => todo!(),
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
	function_table: &HashMap<String, Function>,
	environment_table: &HashMap<String, Environment>,
	output: &mut W
) -> Result<Function, String> {
	let compiled_function = Function {
		env: func.environment,
		args: vec![],
	};

	let mut variable_table = VariableTable::new();

	writeln!(output, "section \"{name} evscript fn\", romx\n{name}::").map_err(|err| err.to_string())?;

	for i in func.contents {
		compile_statement(i, &mut variable_table, &function_table, output)?;
	}

	Ok(compiled_function)
}

pub fn compile<W: Write>(ast: Vec<types::Root>, mut output: W) -> Result<(), String> {
	let mut function_table = HashMap::<String, Function>::new();
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
				let new_func = compile_function(&name, func, &function_table, &environment_table, &mut output)?;
				function_table.insert(name, new_func);
			}
			types::Root::Assembly(contents) => todo!(),
			types::Root::Include(path) => todo!(),
		}
	}

	println!("{environment_table:#?}");

	Ok(())
}
