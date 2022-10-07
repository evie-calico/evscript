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
	name: String,
	index: u8,
	size: u8,
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
	variable_table: &[Option<Variable>; 256],
	function_table: &HashMap<String, Function>,
	output: &mut W
) -> Result<String, String> {
	match rpn {
		Rpn::Variable(..) => todo!(),
		Rpn::Signed(..) => todo!(),
		Rpn::String(..) => todo!(),
		Rpn::Call(..) => todo!(),
		Rpn::Negate(..) => todo!(),
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
	variable_table: &[Option<Variable>; 256],
	function_table: &HashMap<String, Function>,
	output: &mut W
) -> Result<(), String> {
	match statement {
		types::Statement::Expression(..) => todo!(),
		types::Statement::Declaration(..) => todo!(),
		types::Statement::DeclareAssign(..) => todo!(),
		types::Statement::If(..) => todo!(),
		types::Statement::While(..) => todo!(),
		types::Statement::Do(..) => todo!(),
		types::Statement::For(..) => todo!(),
		types::Statement::Repeat(..) => todo!(),
		types::Statement::Loop(..) => todo!(),
		_ => return Err(format!("{statement:?} not allowed in function")),
	}
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

	let variable_table: [Option<Variable>; 256] = [
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
	];

	writeln!(output, "{name}::").map_err(|err| err.to_string())?;

	for i in func.contents {
		compile_statement(i, &variable_table, &function_table, output)?;
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
