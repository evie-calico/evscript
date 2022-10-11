use crate::types;
use crate::types::Rpn;

use std::collections::HashMap;
use std::fmt;
use std::fs::read_to_string;
use std::io::Write;
use std::process::exit;

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
				define!("goto", 22),
				define!("goto_if_true", 23),
				define!("goto_if_false", 24),
			]),
			pool: 0,
		}
	}

	fn expand(&self, name: &str) -> Result<String, String> {
		match self.lookup(name)? {
			types::Definition::Def(..) => {
				Ok(format!("{}@{}", self.name, name))
			}
			types::Definition::Alias(alias) => {
				self.expand(&alias.target)
			}
			types::Definition::Macro(..) => Err(format!("{name} may not be a macro")),
		}
	}

	fn lookup(&self, name: &str) -> Result<&types::Definition, String> {
		match self.definitions.get(name) {
			Some(def) => Ok(def),
			None => return Err(format!("Definition of {name} not found")),
		}
	}
}

type EnvironmentTable = HashMap<String, Environment>;

#[derive(Debug, Copy, Clone, PartialEq)]
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

struct TypeTable {
	table: HashMap<String, Type>,
}

impl TypeTable {
	fn lookup(&self, name: &str) -> Result<Type, String> {
		match self.table.get(name) {
			Some(t) => Ok(*t),
			None => return Err(format!("Type {name} not found")),
		}
	}
}

#[derive(Debug, PartialEq)]
struct Variable {
	name: Option<String>,
	t: Type,
	scope_level: u32,
}

#[derive(Debug)]
struct VariableTable {
	peak_usage: u8,
	// Used to free variables by scope.
	scope_level: u32,
	variables: [Option<Variable>; 256],
}

impl VariableTable {
	fn new() -> VariableTable {
		VariableTable {
			scope_level: 0,
			peak_usage: 0,
			variables: [
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
			],
		}
	}

	fn alloc(&mut self, t: Type) -> Result<u8, String> {
		let mut i = 0;

		while i < 256 {
			match &self.variables[i] {
				Some(var) => i += var.t.size as usize,
				None => {
					let this_peak = i as u8 + t.size;
					if self.peak_usage < this_peak {
						self.peak_usage = this_peak;
					}

					let new_var = Variable {
						name: None,
						t,
						scope_level: self.scope_level,
					};
					self.variables[i as usize] = Some(new_var);
					return Ok(i as u8);
				}
			}
		}

		Err(String::from("Out of variable space; a single function is limited to 256 bytes"))
	}

	fn free(&mut self, i: u8) {
		assert!(self.variables[i as usize] != None, "Variable does not exist");
		self.variables[i as usize] = None;
	}

	fn autofree(&mut self, i: u8) {
		if let Some(var) = &self.variables[i as usize] {
			if let None = var.name {
				self.variables[i as usize] = None;
			}
		} else {
			panic!("Variable {i} does not exist");
		}
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

		Err(format!("Variable {name} does not exist in {self:#?}"))
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

	fn push_scope(&mut self) {
		self.scope_level += 1;
	}

	fn pop_scope(&mut self) {
		self.scope_level -= 1;
		let mut i = 0;

		while i < 256 {
			if let Some(variable) = &self.variables[i] {
				let this_size = variable.t.size as usize;
				if variable.scope_level > self.scope_level {
					self.variables[i as usize] = None;
				}
				i += this_size;
			} else {
				i += 1;
			}
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
				match def {
					types::Definition::Def(ref mut sub_def) => {
						writeln!(output, "def {this_name}@{name} equ {bytecode_index}")
							.map_err(|err| err.to_string())?;
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
	type_table: &TypeTable,
	vtable: &mut VariableTable,
	str_table: &mut Vec<String>,
	output: &mut W
) -> Result<Option<u8>, String> {
	fn binary_operation<W: Write>(
		l: Box<Rpn>,
		op: &str,
		r: Box<Rpn>,
		env: &Environment,
		type_table: &TypeTable,
		vtable: & mut VariableTable,
		str_table: &mut Vec<String>,
		output: &mut W
	) -> Result<Option<u8>, String> {
		let l = compile_expression(*l, env, type_table, vtable, str_table, output)?
			.ok_or(String::from("Expression has no return value"))?;
		let r = compile_expression(*r, env, type_table, vtable, str_table, output)?
			.ok_or(String::from("Expression has no return value"))?;

		let result_type = Type::from(vtable.type_of(l), vtable.type_of(r));
		let result = vtable.alloc(result_type)?;
		// TODO: make opcodes consider operation size.

		writeln!(output, "\tdb {}, {result}, {l}, {r}", env.expand(&format!("{op}_{result_type}"))?)
			.map_err(|err| err.to_string())?;

		vtable.autofree(l);
		vtable.autofree(r);

		Ok(Some(result))
	}

	match rpn {
		Rpn::Variable(name) => {
			match vtable.lookup(&name) {
				Ok(i) => Ok(Some(i)),
				Err(..) => {
					// TODO: make the default integer type configurable
					let result_type = Type { signed: false, size: 1 };
					let result = vtable.alloc(result_type)?;
					// put (result), value
					writeln!(output, "\tdb {}, {result}, {name}", env.expand(&format!("put_{result_type}"))?)
						.map_err(|err| err.to_string())?;
					Ok(Some(result))
				}
			}
		}
		Rpn::Signed(value) => {
			// The "default" type of an integer is u8 (think C's int)
			// This is because most projects will probably only have the 8-bit bytecode installed.
			// TODO: make the default integer type configurable
			let result_type = Type { signed: false, size: 1 };
			let result = vtable.alloc(result_type)?;
			// put (result), value
			writeln!(output, "\tdb {}, {result}, ${value:X}", env.expand(&format!("put_{result_type}"))?)
				.map_err(|err| err.to_string())?;
			Ok(Some(result))
		}
		Rpn::String(string) => {
			let result_type = Type { signed: false, size: 2 };
			let result = vtable.alloc(result_type)?;
			let value = format!(".__string{}", str_table.len());
			// TODO: make this a 16-bit put
			writeln!(output, "\tdb {}, {result}, LOW({value})", env.expand(&format!("put_u8"))?)
				.map_err(|err| err.to_string())?;
			writeln!(output, "\tdb {}, {result} + 1, HIGH({value})", env.expand(&format!("put_u8"))?)
				.map_err(|err| err.to_string())?;
			str_table.push(string);
			Ok(Some(result))
		}
		Rpn::Call(name, args) => {
			match env.lookup(&name)? {
				types::Definition::Def(def) => {
					let mut def_arg_count = 0;
					let mut return_id: Option<u8> = None;

					for i in &def.args {
						match i {
							types::DefinitionParam::Type(..) => def_arg_count += 1,
							types::DefinitionParam::Return(t) => {
								if return_id != None {
									return Err(String::from("A function may only have one return value"));
								}
								return_id = Some(vtable.alloc(type_table.lookup(&t)?)?);
							}
						}
					}

					if args.len() > def_arg_count {
						return Err(String::from("Too many arguments"));
					} else if args.len() < def_arg_count {
						return Err(String::from("Not enough arguments"));
					}

					let mut arg_ids = Vec::<u8>::new();
					let mut index = 0;

					for i in &def.args {
						match i {
							types::DefinitionParam::Type(t) => {
								let this_arg = compile_expression(args[index].clone(), env, type_table, vtable, str_table, output)?
									.ok_or(String::from("Expression has no return value"))?;

								if type_table.lookup(&t)? != vtable.type_of(this_arg) {
									eprintln!("WARN: argument type does not match definition");
								}

								arg_ids.push(this_arg);
								index += 1;

								vtable.autofree(this_arg);
							}
							types::DefinitionParam::Return(..) => arg_ids.push(return_id.unwrap()),
						}
					}
					write!(output, "\tdb {}", env.expand(&name)?)
						.map_err(|err| err.to_string())?;
					for i in arg_ids {
						write!(output, ", {i}")
							.map_err(|err| err.to_string())?;
					}
					writeln!(output, "")
						.map_err(|err| err.to_string())?;

					Ok(return_id)
				}
				types::Definition::Alias(alias) => {
					enum AliasVariant {
						ArgId(usize),
						ExpressionId(u8),
					}

					let mut def_arg_count = 0;
					let mut return_id: Option<u8> = None;

					for i in &alias.args {
						match i {
							types::DefinitionParam::Type(..) => def_arg_count += 1,
							types::DefinitionParam::Return(t) => {
								if return_id != None {
									return Err(String::from("A function may only have one return value"));
								}
								return_id = Some(vtable.alloc(type_table.lookup(&t)?)?);
							}
						}
					}

					if args.len() > def_arg_count {
						return Err(String::from("Too many arguments"));
					} else if args.len() < def_arg_count {
						return Err(String::from("Not enough arguments"));
					}

					let mut arg_ids = Vec::<u8>::new();
					let mut alias_ids = Vec::<AliasVariant>::new();
					let mut index = 0;

					for i in &alias.args {
						match i {
							types::DefinitionParam::Type(t) => {
								let this_arg = compile_expression(args[index].clone(), env, type_table, vtable, str_table, output)?
									.ok_or(String::from("Expression has no return value"))?;

								if type_table.lookup(&t)? != vtable.type_of(this_arg) {
									eprintln!("WARN: argument type does not match definition");
								}

								arg_ids.push(this_arg);
								index += 1;

								vtable.autofree(this_arg);
							}
							types::DefinitionParam::Return(..) => arg_ids.push(return_id.unwrap()),
						}
					}

					for i in &alias.target_args {
						match i {
							types::AliasParam::ArgId(index) => alias_ids.push(AliasVariant::ArgId(*index)),
							types::AliasParam::Expression(rpn) => {
								let this_arg = compile_expression(rpn.clone(), env, type_table, vtable, str_table, output)?
									.ok_or(String::from("Expression has no return value"))?;
								alias_ids.push(AliasVariant::ExpressionId(this_arg));
								vtable.autofree(this_arg);
							}
						}
					}

					write!(output, "\tdb {}", env.expand(&name)?)
						.map_err(|err| err.to_string())?;
					for i in alias_ids {
						match i {
							AliasVariant::ExpressionId(index) => write!(output, ", {index}")
								.map_err(|err| err.to_string())?,
							AliasVariant::ArgId(index) => {
								if index > arg_ids.len() {
									return Err(format!("Argument ID is too large ({index})"));
								}
								write!(output, ", {}", arg_ids[index - 1])
									.map_err(|err| err.to_string())?;
							}
						}
					}
					writeln!(output, "")
						.map_err(|err| err.to_string())?;

					Ok(return_id)
				}
				types::Definition::Macro(mac) => {
					let mut def_arg_count = 0;
					let mut return_id: Option<u8> = None;

					for i in &mac.args {
						match i {
							types::DefinitionParam::Type(..) => def_arg_count += 1,
							types::DefinitionParam::Return(t) => {
								if return_id != None {
									return Err(String::from("A function may only have one return value"));
								}
								return_id = Some(vtable.alloc(type_table.lookup(&t)?)?);
							}
						}
					}

					if args.len() > def_arg_count {
						return Err(String::from("Too many arguments"));
					} else if args.len() < def_arg_count {
						return Err(String::from("Not enough arguments"));
					}

					let mut arg_ids = Vec::<u8>::new();
					let mut index = 0;

					for i in &mac.args {
						match i {
							types::DefinitionParam::Type(t) => {
								let this_arg = compile_expression(args[index].clone(), env, type_table, vtable, str_table, output)?
									.ok_or(String::from("Expression has no return value"))?;

								if type_table.lookup(&t)? != vtable.type_of(this_arg) {
									eprintln!("WARN: argument type does not match definition");
								}

								arg_ids.push(this_arg);
								index += 1;

								vtable.autofree(this_arg);
							}
							types::DefinitionParam::Return(..) => arg_ids.push(return_id.unwrap()),
						}
					}

					write!(output, "\t{}", mac.target)
						.map_err(|err| err.to_string())?;
					for i in arg_ids {
						write!(output, " {i},")
							.map_err(|err| err.to_string())?;
					}
					writeln!(output, "")
						.map_err(|err| err.to_string())?;

					Ok(return_id)
				}
			}
		}
		Rpn::Negate(i) => {
			let operand = compile_expression(*i, env, type_table, vtable, str_table, output)?
				.ok_or(String::from("Expression has no return value"))?;
			let operand_type = vtable.type_of(operand);
			let zero = vtable.alloc(operand_type)?;
			let result = vtable.alloc(operand_type)?;
			// TODO: make opcodes consider operand size.
			writeln!(output, "\tdb {}, {zero}, $0", env.expand(&format!("put_{operand_type}"))?)
				.map_err(|err| err.to_string())?;
			writeln!(output, "\tdb {}, {result}, {zero}, {operand}", env.expand(&format!("sub_{operand_type}"))?)
				.map_err(|err| err.to_string())?;

			vtable.free(zero);
			vtable.autofree(operand);

			Ok(Some(result))
		}
		Rpn::Not(i) => {
			let operand = compile_expression(*i, env, type_table, vtable, str_table, output)?
				.ok_or(String::from("Expression has no return value"))?;
			let operand_type = vtable.type_of(operand);
			// TODO: make the default integer type configurable per-environment
			let ff = vtable.alloc(operand_type)?;
			let result = vtable.alloc(operand_type)?;
			writeln!(output, "\tdb {}, {ff}, $FF", env.expand(&format!("put_{operand_type}"))?)
				.map_err(|err| err.to_string())?;
			writeln!(output, "\tdb {}, {result}, {operand}, {ff}", env.expand(&format!("xor_{operand_type}"))?)
				.map_err(|err| err.to_string())?;

			vtable.free(ff);
			vtable.autofree(operand);

			Ok(Some(result))
		}
		Rpn::Deref(..) => todo!(),
		Rpn::Address(..) => todo!(),
		Rpn::Mul(l, r) => binary_operation(l, "mul", r, env, type_table, vtable, str_table, output),
		Rpn::Div(l, r) => binary_operation(l, "div", r, env, type_table, vtable, str_table, output),
		Rpn::Mod(l, r) => binary_operation(l, "mod", r, env, type_table, vtable, str_table, output),
		Rpn::Add(l, r) => binary_operation(l, "add", r, env, type_table, vtable, str_table, output),
		Rpn::Sub(l, r) => binary_operation(l, "sub", r, env, type_table, vtable, str_table, output),
		Rpn::ShiftLeft(l, r) => binary_operation(l, "shl", r, env, type_table, vtable, str_table, output),
		Rpn::ShiftRight(l, r) => binary_operation(l, "shr", r, env, type_table, vtable, str_table, output),
		Rpn::BinaryAnd(l, r) => binary_operation(l, "band", r, env, type_table, vtable, str_table, output),
		Rpn::BinaryXor(l, r) => binary_operation(l, "bxor", r, env, type_table, vtable, str_table, output),
		Rpn::BinaryOr(l, r) => binary_operation(l, "bor", r, env, type_table, vtable, str_table, output),
		Rpn::Equ(l, r) => binary_operation(l, "equ", r, env, type_table, vtable, str_table, output),
		Rpn::NotEqu(l, r) => binary_operation(l, "nequ", r, env, type_table, vtable, str_table, output),
		Rpn::LessThan(l, r) => binary_operation(l, "lt", r, env, type_table, vtable, str_table, output),
		Rpn::GreaterThan(l, r) => binary_operation(l, "gt", r, env, type_table, vtable, str_table, output),
		Rpn::LessThanEqu(l, r) => binary_operation(l, "lte", r, env, type_table, vtable, str_table, output),
		Rpn::GreaterThanEqu(l, r) => binary_operation(l, "gte", r, env, type_table, vtable, str_table, output),
		Rpn::LogicalAnd(l, r) => binary_operation(l, "land", r, env, type_table, vtable, str_table, output),
		Rpn::LogicalOr(l, r) => binary_operation(l, "lor", r, env, type_table, vtable, str_table, output),
		Rpn::Set(name, i) => {
			// A plain Set may only assign to existing variables.
			let dest = vtable.lookup(&name)?;
			let dest_type = vtable.type_of(dest);
			// TODO: make this directly take ownership of i if it is not an Rpn::Variable.
			let source = compile_expression(*i, env, type_table, vtable, str_table, output)?
				.ok_or(String::from("Expression has no return value"))?;

			writeln!(output, "\tdb {}, {dest}, {source}", env.expand(&format!("mov_{dest_type}"))?)
				.map_err(|err| err.to_string())?;

			vtable.autofree(source);

			Ok(Some(dest))
		}
	}
}

fn compile_statement<W: Write>(
	statement: types::Statement,
	env: &Environment,
	type_table: &TypeTable,
	label_index: &mut u32,
	vtable: &mut VariableTable,
	str_table: &mut Vec<String>,
	output: &mut W
) -> Result<(), String> {
	match statement {
		types::Statement::Expression(rpn) => {
			compile_expression(rpn, env, type_table, vtable, str_table, output)?;
		}
		types::Statement::Declaration(t, name) => {
			let new_var = vtable.alloc(type_table.lookup(&t)?)?;
			*vtable.name_of(new_var) = Some(name);
		}
		types::Statement::DeclareAssign(t, name, rpn) => {
			// Create a new variable
			let new_var = vtable.alloc(type_table.lookup(&t)?)?;
			*vtable.name_of(new_var) = Some(name.clone());
			// Compile the Set.
			compile_expression(rpn, env, type_table, vtable, str_table, output)?;
		},
		types::Statement::If(condition, contents, else_contents) => {
			let condition_result = compile_expression(condition, env, type_table, vtable, str_table, output)?
				.ok_or(String::from("Expression has no return value"))?;
			let l = *label_index;
			*label_index += 1;

			writeln!(
				output,
				"\tdb {}, {condition_result}, LOW(.__else{l}), HIGH(.__else{l})",
				env.expand("goto_if_false")?
			).map_err(|err| err.to_string())?;

			vtable.autofree(condition_result);

			vtable.push_scope();
			for i in contents {
				compile_statement(i, env, type_table, label_index, vtable, str_table, output)?;
			}
			vtable.pop_scope();

			if let Some(..) = else_contents {
				writeln!(
					output,
					"\tdb {}, LOW(.__end{l}), HIGH(.__end{l})",
					env.expand("goto")?
				).map_err(|err| err.to_string())?;
			}

			writeln!(output, ".__else{l}").map_err(|err| err.to_string())?;

			if let Some(else_statements) = else_contents {
				vtable.push_scope();
				for i in else_statements {
					compile_statement(i, env, type_table, label_index, vtable, str_table, output)?;
				}
				vtable.pop_scope();
			}

			writeln!(output, ".__end{l}").map_err(|err| err.to_string())?;
		}
		types::Statement::While(condition, contents) => {
			let l = *label_index;
			*label_index += 1;

			// Jump to the condition first.
			writeln!(
				output,
				"\tdb {}, LOW(.__end{l}), HIGH(.__end{l})",
				env.expand("goto")?
			).map_err(|err| err.to_string())?;

			writeln!(output, ".__while{l}").map_err(|err| err.to_string())?;

			vtable.push_scope();
			for i in contents {
				compile_statement(i, env, type_table, label_index, vtable, str_table, output)?;
			}
			vtable.pop_scope();
			
			writeln!(output, ".__end{l}").map_err(|err| err.to_string())?;

			let condition_result = compile_expression(condition, env, type_table, vtable, str_table, output)?
				.ok_or(String::from("Expression has no return value"))?;

			writeln!(
				output,
				"\tdb {}, {condition_result}, LOW(.__while{l}), HIGH(.__while{l})",
				env.expand("goto_if_true")?
			).map_err(|err| err.to_string())?;

			vtable.autofree(condition_result);
		}
		types::Statement::Do(condition, contents) => {
			let l = *label_index;
			*label_index += 1;

			writeln!(output, ".__while{l}").map_err(|err| err.to_string())?;

			vtable.push_scope();
			for i in contents {
				compile_statement(i, env, type_table, label_index, vtable, str_table, output)?;
			}
			vtable.pop_scope();
			
			writeln!(output, ".__end{l}").map_err(|err| err.to_string())?;

			let condition_result = compile_expression(condition, env, type_table, vtable, str_table, output)?
				.ok_or(String::from("Expression has no return value"))?;

			writeln!(
				output,
				"\tdb {}, {condition_result}, LOW(.__while{l}), HIGH(.__while{l})",
				env.expand("goto_if_true")?
			).map_err(|err| err.to_string())?;

			vtable.autofree(condition_result);
		}
		types::Statement::For(prologue, condition, epilogue, contents) => {
			let l = *label_index;
			*label_index += 1;

			// Execute prologue
			compile_statement(*prologue, env, type_table, label_index, vtable, str_table, output)?;

			// Jump to the condition first.
			writeln!(
				output,
				"\tdb {}, LOW(.__end{l}), HIGH(.__end{l})",
				env.expand("goto")?
			).map_err(|err| err.to_string())?;

			writeln!(output, ".__for{l}").map_err(|err| err.to_string())?;

			vtable.push_scope();
			for i in contents {
				compile_statement(i, env, type_table, label_index, vtable, str_table, output)?;
			}
			vtable.pop_scope();

			// Execute epliogue before checking condition
			compile_statement(*epilogue, env, type_table, label_index, vtable, str_table, output)?;
			
			writeln!(output, ".__end{l}").map_err(|err| err.to_string())?;

			let condition_result = compile_expression(condition, env, type_table, vtable, str_table, output)?
				.ok_or(String::from("Expression has no return value"))?;

			writeln!(
				output,
				"\tdb {}, {condition_result}, LOW(.__for{l}), HIGH(.__for{l})",
				env.expand("goto_if_true")?
			).map_err(|err| err.to_string())?;

			vtable.autofree(condition_result);
		}
		types::Statement::Repeat(repeat_count, contents) => {
			let l = *label_index;
			*label_index += 1;

			// Execute prologue
			let repeat_index = compile_expression(repeat_count, env, type_table, vtable, str_table, output)?
				.ok_or(String::from("Expression has no return value"))?;

			writeln!(output, ".__repeat{l}").map_err(|err| err.to_string())?;

			vtable.push_scope();
			for i in contents {
				compile_statement(i, env, type_table, label_index, vtable, str_table, output)?;
			}
			vtable.pop_scope();

			// Execute epilogue before checking condition
			let scratch = vtable.alloc(Type { signed: false, size: 1 })?;

			writeln!(
				output,
				"\tdb {}, {scratch}, $1",
				env.expand("put_u8")?
			).map_err(|err| err.to_string())?;

			writeln!(
				output,
				"\tdb {}, {repeat_index}, {repeat_index}, {scratch}",
				env.expand("sub_u8")?
			).map_err(|err| err.to_string())?;
			
			writeln!(output, ".__end{l}").map_err(|err| err.to_string())?;

			writeln!(
				output,
				"\tdb {}, {scratch}, $0",
				env.expand("put_u8")?
			).map_err(|err| err.to_string())?;

			writeln!(
				output,
				"\tdb {}, {scratch}, {repeat_index}, {scratch}",
				env.expand("equ_u8")?
			).map_err(|err| err.to_string())?;

			writeln!(
				output,
				"\tdb {}, {scratch}, LOW(.__repeat{l}), HIGH(.__repeat{l})",
				env.expand("goto_if_false")?
			).map_err(|err| err.to_string())?;

			vtable.autofree(scratch);
			vtable.autofree(repeat_index);
		}
		types::Statement::Loop(contents) => {
			let l = *label_index;
			*label_index += 1;

			writeln!(output, ".__loop{l}").map_err(|err| err.to_string())?;

			vtable.push_scope();
			for i in contents {
				compile_statement(i, env, type_table, label_index, vtable, str_table, output)?;
			}
			vtable.pop_scope();

			writeln!(
				output,
				"\tdb {}, LOW(.__loop{l}), HIGH(.__loop{l})",
				env.expand("goto")?
			).map_err(|err| err.to_string())?;
			
			writeln!(output, ".__end{l}").map_err(|err| err.to_string())?;
		}
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
	let mut str_table = Vec::<String>::new();
	let mut label_index = 0;

	writeln!(output, "\nsection \"{name} evscript fn\", romx\n{name}::")
		.map_err(|err| err.to_string())?;

	for i in func.contents {
		compile_statement(i, env, type_table, &mut label_index, &mut vtable, &mut str_table, output)?;
	}

	writeln!(output, "\tdb 0")
		.map_err(|err| err.to_string())?;

	let mut i = 0;
	while i < str_table.len() {
		writeln!(output, ".__string{i} db \"{}\", 0", str_table[i])
			.map_err(|err| err.to_string())?;
		i += 1;
	}

	// TODO: make this message optional via command-line argument.
	println!("({name}) Peak usage: {}", vtable.peak_usage);

	Ok(())
}

fn compile_ast<W: Write>(
	ast: Vec<types::Root>,
	environment_table: &mut EnvironmentTable,
	type_table: &mut TypeTable,
	output: &mut W
) -> Result<(), String> {
	for i in ast {
		match i {
			types::Root::Environment(name, env) => {
				let new_env = compile_environment(&name, env, &environment_table, output)?;
				environment_table.insert(name, new_env);
			}
			types::Root::Function(name, func) => {
				compile_function(&name, func, &environment_table, &type_table, output)?;
			}
			types::Root::Assembly(contents) => {
				writeln!(output, "{}", contents).map_err(|err| err.to_string())?;
			}
			types::Root::Include(path) => {
				let input = &match read_to_string(&path) {
					Ok(input) => input,
					Err(err) => {
						eprintln!("{path}: {err}");
						exit(1);
					}
				};

				let ast = match crate::parse(input) {
					Ok(ast) => ast,
					Err(err) => {
						eprintln!("{path}: {err}");
						exit(1);
					}
				};

				if let Err(err) = compile_ast(ast, environment_table, type_table, output) {
					eprintln!("{path}: {err}");
					exit(1);
				}
			}
		}
	}

	Ok(())
}

pub fn compile<W: Write>(ast: Vec<types::Root>, output: &mut W) -> Result<(), String> {
	let mut environment_table = EnvironmentTable::from([
		(String::from("std"), Environment::std()),
	]);

	let mut type_table = TypeTable { table: HashMap::<String, Type>::from([
		(String::from("u8"), Type { signed: false, size: 1 } ),
		(String::from("u16"), Type { signed: false, size: 2 } ),
	]) };

	compile_ast(ast, &mut environment_table, &mut type_table, output)
}
