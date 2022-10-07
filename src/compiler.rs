use crate::types;

use std::collections::HashMap;

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

fn compile_environment(name: &str, env: types::Environment, environment_table: &HashMap<String, Environment>) -> Result<Environment, String> {
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

pub fn compile(ast: Vec<types::Root>) -> Result<(), String> {
	let mut environment_table = HashMap::<String, Environment>::from([
		(String::from("std"), Environment::std()),
	]);

	for i in ast {
		match i {
			types::Root::Environment(name, env) => {
				let new_env = compile_environment(&name, env, &environment_table)?;
				environment_table.insert(name, new_env);
			}
			types::Root::Function(name, func) => todo!(),
			types::Root::Assembly(contents) => todo!(),
			types::Root::Include(path) => todo!(),
		}
	}

	println!("{environment_table:#?}");

	Ok(())
}
