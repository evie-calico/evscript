use crate::types;
use crate::types::Rpn;
use crate::types::Statement;
use crate::types::StatementType;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::From;
use std::fmt;
use std::fs::read_to_string;
use std::io::Write;
use std::process::exit;

pub struct CompilerError {
	pub msg: String,
	pub start: Option<usize>,
	pub end: Option<usize>,
}

impl From<&str> for CompilerError {
	fn from(msg: &str) -> Self {
		CompilerError {
			msg: String::from(msg),
			start: None,
			end: None,
		}
	}
}

impl From<String> for CompilerError {
	fn from(msg: String) -> Self {
		CompilerError {
			msg,
			start: None,
			end: None,
		}
	}
}

impl From<std::io::Error> for CompilerError {
	fn from(msg: std::io::Error) -> Self {
		CompilerError {
			msg: msg.to_string(),
			start: None,
			end: None,
		}
	}
}

impl fmt::Display for CompilerError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(start) = self.start {
			if let Some(end) = self.end {
				write!(f, "{start}-{end} {}", self.msg)
			} else {
				write!(f, "{start} {}", self.msg)
			}
		} else {
			write!(f, " {}", self.msg)
		}
	}
}

impl CompilerError {
	pub fn get_range(&self) -> Option<std::ops::Range<usize>> {
		if let Some(start) = self.start {
			if let Some(end) = self.end {
				Some(start..end)
			} else {
				Some(start..start)
			}
		} else {
			None
		}
	}
}

pub struct CompilerOptions {
	pub report_usage: bool,
}

impl Default for CompilerOptions {
	fn default() -> Self {
		Self::new()
	}
}

impl CompilerOptions {
	pub fn new() -> CompilerOptions {
		CompilerOptions {
			report_usage: false,
		}
	}
}

#[derive(Debug)]
struct Environment {
	name: String,
	definitions: HashMap<String, types::Definition>,
	pool: u16,
}

impl Environment {
	fn expand(&self, name: &str) -> Result<String, String> {
		match self.lookup(name)? {
			types::Definition::Def(..) => Ok(format!("{}@{}", self.name, name)),
			types::Definition::Alias(alias) => self.expand(&alias.target),
			types::Definition::Macro(..) => Err(format!("{name} may not be a macro")),
		}
	}

	fn lookup(&self, name: &str) -> Result<&types::Definition, String> {
		match self.definitions.get(name) {
			Some(def) => Ok(def),
			None => Err(format!("Definition of {name} not found")),
		}
	}
}

type EnvironmentTable = HashMap<String, Environment>;

#[derive(Debug, Clone, PartialEq)]
enum Type {
	Primative(Primative),
	Pointer(Box<Type>),
	Struct(Vec<(String, Type)>),
}

impl Type {
	fn size(&self) -> u8 {
		match self {
			Type::Primative(t) => t.size,
			Type::Pointer(_) => Primative::pointer().size,
			Type::Struct(t) => {
				let mut this_size = 0;

				for (_, i) in t {
					this_size += i.size();
				}

				this_size
			}
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Primative {
	signed: bool,
	size: u8,
}

impl Primative {
	fn from(l: Primative, r: Primative) -> Primative {
		Primative {
			signed: l.signed || r.signed,
			size: if l.size >= r.size { l.size } else { r.size },
		}
	}

	// TODO: make the default_integer type configurable
	fn default_integer() -> Primative {
		Primative {
			signed: false,
			size: 1,
		}
	}

	// TODO: make the default pointer type configurable
	fn pointer() -> Primative {
		Primative {
			signed: false,
			size: 2,
		}
	}
}

impl fmt::Display for Primative {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}{}",
			if self.signed { 'i' } else { 'u' },
			self.size * 8
		)
	}
}

struct TypeTable {
	table: HashMap<String, Type>,
}

impl TypeTable {
	fn lookup_type(&self, name: &str) -> Result<Type, String> {
		match self.table.get(name) {
			Some(t) => Ok(t.clone()),
			None => Err(format!("Type {name} not found")),
		}
	}

	fn lookup_primative(&self, name: &str) -> Result<Primative, String> {
		match self.table.get(name) {
			Some(t) => match t {
				Type::Primative(result) => Ok(*result),
				Type::Pointer(_) => panic!("A type should never be declared as a pointer"),
				Type::Struct(..) => Err(format!("{name} must be a primative type")),
			},
			None => Err(format!("Type {name} not found")),
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
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None, None, None, None, None, None, None, None, None, None, None,
				None, None, None, None,
			],
		}
	}

	fn alloc(&mut self, t: Type) -> Result<u8, String> {
		let mut i = 0;

		while i < 256 {
			match &self.variables[i] {
				Some(var) => i += var.t.size() as usize,
				None => {
					let this_peak = i as u8 + t.size();
					if self.peak_usage < this_peak {
						self.peak_usage = this_peak;
					}

					let new_var = Variable {
						name: None,
						t,
						scope_level: self.scope_level,
					};
					self.variables[i] = Some(new_var);
					return Ok(i as u8);
				}
			}
		}

		Err(String::from(
			"Out of variable space; a single function is limited to 256 bytes",
		))
	}

	fn free(&mut self, i: u8) {
		assert!(
			self.variables[i as usize].is_some(),
			"Variable does not exist"
		);
		self.variables[i as usize] = None;
	}

	fn autofree(&mut self, i: u8) {
		if let Some(var) = &self.variables[i as usize] {
			if var.name.is_none() {
				self.variables[i as usize] = None;
			}
		}
	}

	fn lookup(&self, name: &str) -> Result<u8, String> {
		let mut i = 0;

		let components: Vec<&str> = name.split('.').collect();

		if components.len() == 1 {
			while i < 256 {
				if let Some(variable) = &self.variables[i] {
					if let Some(variable_name) = &variable.name {
						if variable_name == name {
							return Ok(i as u8);
						}
					}
					i += variable.t.size() as usize;
				} else {
					i += 1;
				}
			}
		} else {
			while i < 256 {
				if let Some(variable) = &self.variables[i] {
					if let Some(variable_name) = &variable.name {
						if let Type::Struct(struct_type) = &variable.t {
							if variable_name == components[0] {
								// Now that we've found a struct, we'll traverse it to find the member.
								let mut comp_i = 1;
								'next_component: while comp_i < components.len() {
									let mut offset = 0;

									for (member_name, member) in struct_type {
										if member_name == components[comp_i] {
											match member {
												Type::Primative(primative) => {
													if comp_i + 1 != components.len() {
														return Err(format!("{member_name} is a {primative} and has no members"));
													}
													return Ok((i + offset) as u8);
												}
												Type::Pointer(..) => {
													if comp_i + 1 != components.len() {
														return Err(format!("{member_name} is a pointer and has no members"));
													}
													return Ok((i + offset) as u8);
												}
												Type::Struct(..) => {
													if comp_i + 1 == components.len() {
														return Ok((i + offset) as u8);
													} else {
														comp_i += 1;
														continue 'next_component;
													}
												}
											}
										}

										offset += member.size() as usize;
									}
									break;
								}
								return Err(format!("{name} is not a member of {variable_name}"));
							}
						} else {
							return Err(format!("{} is not a struct", components[0]));
						}
					}
					i += variable.t.size() as usize;
				} else {
					i += 1;
				}
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

	fn is_pointer(&self, id: u8) -> bool {
		let id = id as usize;

		match &self.variables[id] {
			Some(var) => matches!(var.t, Type::Pointer(..)),
			None => panic!("Variable index {id} does not exist"),
		}
	}

	fn type_of(&mut self, id: u8) -> Primative {
		let id = id as usize;

		match &self.variables[id] {
			Some(var) => match var.t {
				Type::Primative(result) => return result,
				Type::Pointer(..) => return Primative::pointer(),
				Type::Struct(..) => {}
			},
			None => {}
		}

		// If the variable does not exist, there's a chance it's a struct member.
		// Walk backwards until a structure is found, then check if it has a member at the provided index.
		let mut index = id;

		loop {
			match &self.variables[index] {
				Some(var) => {
					fn seek_struct(
						members: &Vec<(String, Type)>,
						id: usize,
						mut member_offset: usize,
					) -> Primative {
						for (_, i) in members {
							if id < member_offset {
								panic!("Variable index {id} does not exist");
							}

							match i {
								Type::Primative(primative) => {
									if id == member_offset {
										return *primative;
									}
								}
								Type::Pointer(..) => {
									if id == member_offset {
										return Primative::pointer();
									}
								}
								Type::Struct(members) => {
									if id < member_offset + i.size() as usize {
										return seek_struct(members, id, member_offset);
									}
								}
							}

							member_offset += i.size() as usize;
						}
						panic!("Variable index {id} does not exist");
					}

					match &var.t {
						Type::Primative(..) | Type::Pointer(..) => break,
						Type::Struct(members) => {
							return seek_struct(members, id, index);
						}
					}
				}
				None => {}
			}

			if let Some(result) = index.checked_sub(1) {
				index = result;
			} else {
				break;
			}
		}
		panic!("Variable index {id} does not exist");
	}

	fn push_scope(&mut self) {
		self.scope_level += 1;
	}

	fn pop_scope(&mut self) {
		self.scope_level -= 1;
		let mut i = 0;

		while i < 256 {
			if let Some(variable) = &self.variables[i] {
				let this_size = variable.t.size() as usize;
				if variable.scope_level > self.scope_level {
					self.variables[i] = None;
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
) -> Result<Environment, CompilerError> {
	let mut compiled_env = Environment {
		name: String::from(this_name),
		definitions: HashMap::<String, types::Definition>::new(),
		pool: 0,
	};

	let mut bytecode_index: u8 = 0;

	for i in env.contents {
		match i.t {
			StatementType::Use(name) => {
				let other_env = match environment_table.get(&name) {
					Some(other_env) => other_env,
					None => {
						return Err(CompilerError {
							start: Some(i.start),
							end: Some(i.end),
							msg: format!("Environment {name} does not exist"),
						})
					}
				};

				let mut greatest_bytecode = bytecode_index;

				for (def_name, def) in &other_env.definitions {
					if compiled_env.definitions.get(def_name).is_some() {
						eprintln!(
							"WARN: duplicate definition of {def_name} inside `use` statement."
						);
					}

					let mut new_def = def.clone();

					if let types::Definition::Def(ref mut sub_def) = new_def {
						sub_def.bytecode = bytecode_index
							.checked_add(sub_def.bytecode)
							.ok_or(format!("Hit bytecode limit in environment {this_name}"))?;
						writeln!(
							output,
							"def {this_name}@{def_name} equ {}",
							sub_def.bytecode
						)?;
						if sub_def.bytecode > greatest_bytecode {
							greatest_bytecode = sub_def.bytecode;
						}
					}

					compiled_env.definitions.insert(def_name.clone(), new_def);
				}

				bytecode_index = greatest_bytecode;
			}
			StatementType::Definition(name, mut def) => {
				if compiled_env.definitions.get(&name).is_some() {
					eprintln!("WARN: duplicate definition of {name}");
				}
				if let types::Definition::Def(ref mut sub_def) = def {
					writeln!(output, "def {this_name}@{name} equ {bytecode_index}")?;
					sub_def.bytecode = bytecode_index;
					bytecode_index = bytecode_index
						.checked_add(1)
						.ok_or(format!("Hit bytecode limit in environment {this_name}"))?;
				}

				compiled_env.definitions.insert(name.clone(), def);
			}
			StatementType::Pool(expression) => {
				let pool_size = expression.eval_const()?;

				compiled_env.pool = if pool_size < 0 {
					return Err(CompilerError::from("Pool size may not be negative"));
				} else if pool_size > 256 {
					return Err(CompilerError::from("Pool size is limited to 256 bytes"));
				} else {
					pool_size as u16
				};
			}
			_ => {
				return Err(CompilerError::from(format!(
					"StatementType {i:?} is not allowed within environments."
				)))
			}
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
	output: &mut W,
) -> Result<Option<u8>, CompilerError> {
	fn binary_operation<W: Write>(
		l: Rpn,
		op: &str,
		r: Rpn,
		env: &Environment,
		type_table: &TypeTable,
		vtable: &mut VariableTable,
		str_table: &mut Vec<String>,
		output: &mut W,
	) -> Result<Option<u8>, CompilerError> {
		let l = compile_expression(l, env, type_table, vtable, str_table, output)?
			.ok_or(String::from("Expression has no return value"))?;
		let r = compile_expression(r, env, type_table, vtable, str_table, output)?
			.ok_or(String::from("Expression has no return value"))?;

		let result_type = Primative::from(vtable.type_of(l), vtable.type_of(r));
		let result = vtable.alloc(Type::Primative(result_type))?;
		// TODO: make opcodes consider operation size.

		writeln!(
			output,
			"\tdb {}, {l}, {r}, {result}",
			env.expand(&format!("{op}_{result_type}"))?
		)?;

		vtable.autofree(l);
		vtable.autofree(r);

		Ok(Some(result))
	}

	fn compile_arguments<W: Write>(
		def_args: &Vec<types::DefinitionParam>,
		args: &[Rpn],
		return_id: Option<u8>,
		env: &Environment,
		type_table: &TypeTable,
		vtable: &mut VariableTable,
		str_table: &mut Vec<String>,
		output: &mut W,
	) -> Result<Vec<String>, CompilerError> {
		let mut index = 0;
		let mut arg_ids = Vec::<String>::new();
		let mut to_free = Vec::<u8>::new();

		for i in def_args {
			match i {
				types::DefinitionParam::Type(t) => {
					let this_arg = compile_expression(
						args[index].clone(),
						env,
						type_table,
						vtable,
						str_table,
						output,
					)?
					.ok_or(String::from("Expression has no return value"))?;

					if let Type::Primative(t) = type_table.lookup_type(t)? {
						if t != vtable.type_of(this_arg) {
							eprintln!("WARN: argument type does not match definition");
						}
					}

					arg_ids.push(this_arg.to_string());
					// Free this temporary once all arguments are processed.
					to_free.push(this_arg);
					index += 1;
				}
				types::DefinitionParam::Const(t) => {
					if let Type::Primative(t) = type_table.lookup_type(t)? {
						match &args[index] {
							Rpn::Signed(value) => match t.size {
								1 => arg_ids.push(value.to_string()),
								2 => {
									arg_ids.push(format!("{value} & $FF, {value} >> 8"));
								}
								3 => {
									arg_ids.push(format!("{value} & $FF, ({value} >> 8) & $FF, ({value} >> 16) & $FF"));
								}
								4 => {
									arg_ids.push(format!("{value} & $FF, ({value} >> 8) & $FF, ({value} >> 16) & $FF, ({value} >> 24) & $FF"));
								}
								_ => panic!(
									"Invalid size {}, only up to 32 bits are supported",
									t.size
								),
							},
							Rpn::String(text) => {
								if t.size != 2 {
									return Err(CompilerError::from("A string must be 16-bit"));
								}

								let value = format!(".__string{}", str_table.len());
								str_table.push(text.clone());
								arg_ids.push(format!("LOW({value}), HIGH({value})"));
							}
							Rpn::Variable(value) => match t.size {
								1 => arg_ids.push(value.to_string()),
								2 => {
									arg_ids.push(format!("{value} & $FF, {value} >> 8"));
								}
								3 => {
									arg_ids.push(format!("{value} & $FF, ({value} >> 8) & $FF, ({value} >> 16) & $FF"));
								}
								4 => {
									arg_ids.push(format!("{value} & $FF, ({value} >> 8) & $FF, ({value} >> 16) & $FF, ({value} >> 24) & $FF"));
								}
								_ => panic!(
									"Invalid size {}, only up to 32 bits are supported",
									t.size
								),
							},
							_ => return Err(CompilerError::from("Expression must be constant")),
						}
					} else {
						return Err(CompilerError::from("Constant arguments may not be structs"));
					}
					index += 1;
				}
				types::DefinitionParam::Return(..) => {
					arg_ids.push(return_id.unwrap().to_string());
				}
			}
		}

		for i in to_free {
			vtable.autofree(i);
		}

		Ok(arg_ids)
	}

	fn validate_args(
		args: &Vec<types::DefinitionParam>,
		type_table: &TypeTable,
		vtable: &mut VariableTable,
	) -> Result<(usize, Option<u8>), CompilerError> {
		let mut def_arg_count = 0;
		let mut return_id: Option<u8> = None;

		for i in args {
			match i {
				types::DefinitionParam::Type(..) | types::DefinitionParam::Const(..) => {
					def_arg_count += 1;
				}
				types::DefinitionParam::Return(t) => {
					if return_id.is_some() {
						return Err(CompilerError::from(
							"A function may only have one return value",
						));
					}
					return_id = Some(vtable.alloc(type_table.lookup_type(t)?)?);
				}
			}
		}

		Ok((def_arg_count, return_id))
	}

	match rpn {
		Rpn::Variable(name) => {
			match vtable.lookup(&name) {
				Ok(i) => Ok(Some(i)),
				Err(..) => {
					let result_type = Primative::default_integer();
					let result = vtable.alloc(Type::Primative(result_type))?;
					// put (result), value
					writeln!(
						output,
						"\tdb {}, {result}, {name}",
						env.expand(&format!("put_{result_type}"))?
					)?;
					Ok(Some(result))
				}
			}
		}
		Rpn::Address(name) => {
			match vtable.lookup(&name) {
				Ok(..) => Err(CompilerError::from(
					"Cannot take the address of a local variable!",
				)),
				Err(..) => {
					let result_type = Primative::pointer();
					let result = vtable.alloc(Type::Primative(result_type))?;
					// put (result), value
					writeln!(
						output,
						"\tdb {}, {result}, {name} & $FF",
						env.expand("put_u8")?
					)?;
					writeln!(
						output,
						"\tdb {}, {result} + 1, {name} >> 8",
						env.expand("put_u8")?
					)?;
					Ok(Some(result))
				}
			}
		}
		Rpn::Signed(value) => {
			// The "default" type of an integer is u8 (think C's int)
			// This is because most projects will probably only have the 8-bit bytecode installed.
			// TODO: make the default integer type configurable
			let result_type = Primative {
				signed: false,
				size: 1,
			};
			let result = vtable.alloc(Type::Primative(result_type))?;
			// put (result), value
			writeln!(
				output,
				"\tdb {}, {result}, {value}",
				env.expand(&format!("put_{result_type}"))?
			)?;
			Ok(Some(result))
		}
		Rpn::String(string) => {
			let result_type = Primative {
				signed: false,
				size: 2,
			};
			let result = vtable.alloc(Type::Primative(result_type))?;
			let value = format!(".__string{}", str_table.len());
			// TODO: make this a 16-bit put
			writeln!(
				output,
				"\tdb {}, {result}, LOW({value})",
				env.expand("put_u8")?
			)?;
			writeln!(
				output,
				"\tdb {}, {result} + 1, HIGH({value})",
				env.expand("put_u8")?
			)?;
			str_table.push(string);
			Ok(Some(result))
		}
		Rpn::Call(name, args) => match env.lookup(&name)? {
			types::Definition::Def(def) => {
				let (def_arg_count, return_id) = validate_args(&def.args, type_table, vtable)?;

				match args.len().cmp(&def_arg_count) {
					Ordering::Equal => {}
					Ordering::Greater => return Err(CompilerError::from("Too many arguments")),
					Ordering::Less => return Err(CompilerError::from("Not enough arguments")),
				}

				let arg_ids = compile_arguments(
					&def.args, &args, return_id, env, type_table, vtable, str_table, output,
				)?;

				write!(output, "\tdb {}", env.expand(&name)?)?;
				for i in arg_ids {
					write!(output, ", {i}")?;
				}
				writeln!(output)?;

				Ok(return_id)
			}
			types::Definition::Alias(def) => {
				enum AliasVariant {
					ArgId(usize),
					ExpressionId(String),
				}

				let (def_arg_count, return_id) = validate_args(&def.args, type_table, vtable)?;

				match args.len().cmp(&def_arg_count) {
					Ordering::Equal => {}
					Ordering::Greater => return Err(CompilerError::from("Too many arguments")),
					Ordering::Less => return Err(CompilerError::from("Not enough arguments")),
				}

				let arg_ids = compile_arguments(
					&def.args, &args, return_id, env, type_table, vtable, str_table, output,
				)?;

				let mut alias_ids = Vec::<AliasVariant>::new();

				for i in &def.target_args {
					match i {
						types::AliasParam::ArgId(index) => {
							alias_ids.push(AliasVariant::ArgId(*index))
						}
						types::AliasParam::Expression(rpn) => {
							let this_arg = compile_expression(
								rpn.clone(),
								env,
								type_table,
								vtable,
								str_table,
								output,
							)?
							.ok_or(String::from("Expression has no return value"))?;
							alias_ids.push(AliasVariant::ExpressionId(this_arg.to_string()));
							vtable.autofree(this_arg);
						}
						types::AliasParam::Const(rpn) => match rpn {
							Rpn::Signed(value) => {
								if *value >= 256 || *value < -128 {
									return Err(CompilerError::from(
										"WARN: integer constants can only be 8 bits",
									));
								}
								alias_ids.push(AliasVariant::ExpressionId(value.to_string()));
							}
							Rpn::String(text) => {
								let value = format!(".__string{}", str_table.len());
								str_table.push(text.clone());
								alias_ids.push(AliasVariant::ExpressionId(format!(
									"LOW({value}), HIGH({value})"
								)));
							}
							Rpn::Variable(value) => {
								alias_ids.push(AliasVariant::ExpressionId(value.to_string()));
							}
							_ => return Err(CompilerError::from("Expression must be constant")),
						},
					}
				}

				write!(output, "\tdb {}", env.expand(&name)?)?;
				for i in alias_ids {
					match i {
						AliasVariant::ExpressionId(index) => write!(output, ", {index}")?,
						AliasVariant::ArgId(index) => {
							if index > arg_ids.len() {
								return Err(CompilerError::from(format!(
									"Argument ID is too large ({index})"
								)));
							}
							write!(output, ", {}", arg_ids[index - 1])?;
						}
					}
				}
				writeln!(output)?;

				Ok(return_id)
			}
			types::Definition::Macro(def) => {
				let (def_arg_count, return_id) = validate_args(&def.args, type_table, vtable)?;

				match args.len().cmp(&def_arg_count) {
					Ordering::Equal => {}
					Ordering::Greater => return Err(CompilerError::from("Too many arguments")),
					Ordering::Less => return Err(CompilerError::from("Not enough arguments")),
				}

				let arg_ids = compile_arguments(
					&def.args, &args, return_id, env, type_table, vtable, str_table, output,
				)?;

				write!(output, "\t{}", def.target)?;
				for i in arg_ids {
					write!(output, " {i},")?;
				}
				writeln!(output)?;

				Ok(return_id)
			}
		},
		Rpn::Negate(i) => {
			let operand = compile_expression(*i, env, type_table, vtable, str_table, output)?
				.ok_or(String::from("Expression has no return value"))?;
			let operand_type = vtable.type_of(operand);
			let zero = vtable.alloc(Type::Primative(operand_type))?;
			let result = vtable.alloc(Type::Primative(operand_type))?;
			// TODO: make opcodes consider operand size.
			writeln!(
				output,
				"\tdb {}, {zero}, $0",
				env.expand(&format!("put_{operand_type}"))?
			)?;
			writeln!(
				output,
				"\tdb {}, {zero}, {operand}, {result}",
				env.expand(&format!("sub_{operand_type}"))?
			)?;

			vtable.free(zero);
			vtable.autofree(operand);

			Ok(Some(result))
		}
		Rpn::Not(i) => {
			let operand = compile_expression(*i, env, type_table, vtable, str_table, output)?
				.ok_or(String::from("Expression has no return value"))?;
			let operand_type = vtable.type_of(operand);
			// TODO: make the default integer type configurable per-environment
			let ff = vtable.alloc(Type::Primative(operand_type))?;
			let result = vtable.alloc(Type::Primative(operand_type))?;
			writeln!(
				output,
				"\tdb {}, {ff}, $FF",
				env.expand(&format!("put_{operand_type}"))?
			)?;
			writeln!(
				output,
				"\tdb {}, {operand}, {ff}, {result}",
				env.expand(&format!("xor_{operand_type}"))?
			)?;

			vtable.free(ff);
			vtable.autofree(operand);

			Ok(Some(result))
		}
		Rpn::Deref(i) => {
			let source = compile_expression(*i, env, type_table, vtable, str_table, output)?
				.ok_or(String::from("Expression has no return value"))?;

			if !vtable.is_pointer(source) {
				return Err(CompilerError::from("Attempting to deref a non-pointer! Note that address-of returns a `u16`, not a `u16 ptr`. Try declaring the pointer before dereferencing."));
			}

			let source_type = match &vtable.variables[source as usize] {
				Some(var) => match &var.t {
					Type::Pointer(t) => match **t {
						Type::Primative(t) => t,
						Type::Pointer(..) => Primative::pointer(),
						Type::Struct(..) => {
							return Err(CompilerError::from("A pointer to a structure cannot be dereferenced. Try working with individual members."));
						}
					},
					_ => panic!(),
				},
				None => panic!(),
			};

			let dest = vtable.alloc(Type::Primative(source_type))?;
			let dest_type = vtable.type_of(dest);

			writeln!(
				output,
				"\tdb {}, {dest}, {source}",
				env.expand(&format!("deref_{dest_type}"))?
			)?;

			vtable.autofree(source);

			Ok(Some(dest))
		}
		Rpn::Mul(l, r) => {
			binary_operation(*l, "mul", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::Div(l, r) => {
			binary_operation(*l, "div", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::Mod(l, r) => {
			binary_operation(*l, "mod", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::Add(l, r) => {
			binary_operation(*l, "add", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::Sub(l, r) => {
			binary_operation(*l, "sub", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::ShiftLeft(l, r) => {
			binary_operation(*l, "shl", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::ShiftRight(l, r) => {
			binary_operation(*l, "shr", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::BinaryAnd(l, r) => {
			binary_operation(*l, "band", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::BinaryXor(l, r) => {
			binary_operation(*l, "bxor", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::BinaryOr(l, r) => {
			binary_operation(*l, "bor", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::Equ(l, r) => {
			binary_operation(*l, "equ", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::NotEqu(l, r) => {
			binary_operation(*l, "nequ", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::LessThan(l, r) => {
			binary_operation(*l, "lt", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::GreaterThan(l, r) => {
			binary_operation(*l, "gt", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::LessThanEqu(l, r) => {
			binary_operation(*l, "lte", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::GreaterThanEqu(l, r) => {
			binary_operation(*l, "gte", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::LogicalAnd(l, r) => {
			binary_operation(*l, "land", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::LogicalOr(l, r) => {
			binary_operation(*l, "lor", *r, env, type_table, vtable, str_table, output)
		}
		Rpn::Set(name, i) => {
			// A plain Set may only assign to existing variables.
			let dest = vtable.lookup(&name)?;
			let dest_type = vtable.type_of(dest);
			// TODO: make this directly take ownership of i if it is not an Rpn::Variable.
			let source = compile_expression(*i, env, type_table, vtable, str_table, output)?
				.ok_or(String::from("Expression has no return value"))?;

			writeln!(
				output,
				"\tdb {}, {dest}, {source}",
				env.expand(&format!("mov_{dest_type}"))?
			)?;

			vtable.autofree(source);

			Ok(Some(dest))
		}
	}
}

fn compile_statement<W: Write>(
	statement: Statement,
	env: &Environment,
	type_table: &TypeTable,
	label_index: &mut u32,
	vtable: &mut VariableTable,
	str_table: &mut Vec<String>,
	output: &mut W,
) -> Result<(), CompilerError> {
	// Automatically adds statement.start and statement.end to a compiler error.
	let statement_error = |msg: String| -> CompilerError {
		CompilerError {
			start: Some(statement.start),
			end: Some(statement.end),
			msg,
		}
	};

	match statement.t {
		StatementType::Expression(rpn) => {
			if let Err(msg) = compile_expression(rpn, env, type_table, vtable, str_table, output) {
				// TODO: Give Rpn nodes their own location info.
				return Err(CompilerError {
					start: Some(statement.start),
					end: Some(statement.end),
					msg: msg.to_string(),
				});
			}
		}
		StatementType::Declaration(t, name) => {
			let new_var = vtable.alloc(type_table.lookup_type(&t)?)?;
			*vtable.name_of(new_var) = Some(name);
		}
		StatementType::PointerDeclaration(t, name) => {
			let object_type = type_table.lookup_type(&t)?;
			let new_var = vtable.alloc(Type::Pointer(Box::new(object_type)))?;
			*vtable.name_of(new_var) = Some(name);
		}
		StatementType::DeclareAssign(t, name, rpn) => {
			match rpn {
				Rpn::Variable(source_name) => {
					// Create a new variable
					let dest_type =
						match type_table.lookup_primative(&t) {
							Ok(t) => t,
							Err(..) => return Err(statement_error(String::from(
								"Cannot assign to structures, assign to individual members instead",
							))),
						};
					let dest = vtable.alloc(Type::Primative(dest_type))?;
					let source = vtable.lookup(&source_name)?;

					writeln!(
						output,
						"\tdb {}, {dest}, {source}",
						env.expand(&format!("mov_{dest_type}"))?
					)?;

					*vtable.name_of(dest) = Some(name);

					vtable.autofree(source);
				}
				_ => {
					let new_var =
						compile_expression(rpn, env, type_table, vtable, str_table, output)?
							.ok_or(statement_error(String::from(
								"Expression has no return value",
							)))?;
					*vtable.name_of(new_var) = Some(name);
				}
			}
		}
		StatementType::PointerDeclareAssign(t, name, rpn) => {
			let dest_type = match type_table.lookup_primative(&t) {
				Ok(t) => t,
				Err(..) => {
					return Err(statement_error(String::from(
						"Cannot assign to structures, assign to individual members instead",
					)))
				}
			};
			let dest = vtable.alloc(Type::Pointer(Box::new(Type::Primative(dest_type))))?;
			*vtable.name_of(dest) = Some(name);

			let source = compile_expression(rpn, env, type_table, vtable, str_table, output)?
				.ok_or(statement_error(String::from(
					"Expression has no return value",
				)))?;

			writeln!(output, "\tdb {}, {dest}, {source}", env.expand("mov_u16")?)?;

			vtable.autofree(source);
		}
		StatementType::If(condition, contents, else_contents) => {
			let condition_result =
				compile_expression(condition, env, type_table, vtable, str_table, output)?.ok_or(
					statement_error(String::from("Expression has no return value")),
				)?;
			let l = *label_index;
			*label_index += 1;

			writeln!(
				output,
				"\tdb {}, {condition_result}, LOW(.__else{l}), HIGH(.__else{l})",
				env.expand("jmp_if_false")?
			)?;

			vtable.autofree(condition_result);

			vtable.push_scope();
			for i in contents {
				compile_statement(i, env, type_table, label_index, vtable, str_table, output)?;
			}
			vtable.pop_scope();

			if else_contents.is_some() {
				writeln!(
					output,
					"\tdb {}, LOW(.__end{l}), HIGH(.__end{l})",
					env.expand("jmp")?
				)?;
			}

			writeln!(output, ".__else{l}")?;

			if let Some(else_statements) = else_contents {
				vtable.push_scope();
				for i in else_statements {
					compile_statement(i, env, type_table, label_index, vtable, str_table, output)?;
				}
				vtable.pop_scope();
			}

			writeln!(output, ".__end{l}")?;
		}
		StatementType::While(condition, contents) => {
			let l = *label_index;
			*label_index += 1;

			// Jump to the condition first.
			writeln!(
				output,
				"\tdb {}, LOW(.__end{l}), HIGH(.__end{l})",
				env.expand("jmp")?
			)?;

			writeln!(output, ".__while{l}")?;

			vtable.push_scope();
			for i in contents {
				compile_statement(i, env, type_table, label_index, vtable, str_table, output)?;
			}
			vtable.pop_scope();

			writeln!(output, ".__end{l}")?;

			let condition_result =
				compile_expression(condition, env, type_table, vtable, str_table, output)?.ok_or(
					statement_error(String::from("Expression has no return value")),
				)?;

			writeln!(
				output,
				"\tdb {}, {condition_result}, LOW(.__while{l}), HIGH(.__while{l})",
				env.expand("jmp_if_true")?
			)?;

			vtable.autofree(condition_result);
		}
		StatementType::Do(condition, contents) => {
			let l = *label_index;
			*label_index += 1;

			writeln!(output, ".__while{l}")?;

			vtable.push_scope();
			for i in contents {
				compile_statement(i, env, type_table, label_index, vtable, str_table, output)?;
			}
			vtable.pop_scope();

			writeln!(output, ".__end{l}")?;

			let condition_result =
				compile_expression(condition, env, type_table, vtable, str_table, output)?.ok_or(
					statement_error(String::from("Expression has no return value")),
				)?;

			writeln!(
				output,
				"\tdb {}, {condition_result}, LOW(.__while{l}), HIGH(.__while{l})",
				env.expand("jmp_if_true")?
			)?;

			vtable.autofree(condition_result);
		}
		StatementType::For(prologue, condition, epilogue, contents) => {
			let l = *label_index;
			*label_index += 1;

			// Execute prologue
			compile_statement(
				*prologue,
				env,
				type_table,
				label_index,
				vtable,
				str_table,
				output,
			)?;

			// Jump to the condition first.
			writeln!(
				output,
				"\tdb {}, LOW(.__end{l}), HIGH(.__end{l})",
				env.expand("jmp")?
			)?;

			writeln!(output, ".__for{l}")?;

			vtable.push_scope();
			for i in contents {
				compile_statement(i, env, type_table, label_index, vtable, str_table, output)?;
			}
			vtable.pop_scope();

			// Execute epliogue before checking condition
			compile_statement(
				*epilogue,
				env,
				type_table,
				label_index,
				vtable,
				str_table,
				output,
			)?;

			writeln!(output, ".__end{l}")?;

			let condition_result =
				compile_expression(condition, env, type_table, vtable, str_table, output)?.ok_or(
					statement_error(String::from("Expression has no return value")),
				)?;

			writeln!(
				output,
				"\tdb {}, {condition_result}, LOW(.__for{l}), HIGH(.__for{l})",
				env.expand("jmp_if_true")?
			)?;

			vtable.autofree(condition_result);
		}
		StatementType::Repeat(repeat_count, contents) => {
			let l = *label_index;
			*label_index += 1;

			// Execute prologue
			let mut repeat_index =
				compile_expression(repeat_count, env, type_table, vtable, str_table, output)?
					.ok_or(statement_error(String::from(
						"Expression has no return value",
					)))?;

			if vtable.name_of(repeat_index).is_some() {
				let dest_type = vtable.type_of(repeat_index);
				let unique_index = vtable.alloc(Type::Primative(dest_type))?;
				writeln!(
					output,
					"\tdb {}, {unique_index}, {repeat_index}",
					env.expand(&format!("mov_{dest_type}"))?
				)?;
				repeat_index = unique_index;
			}

			writeln!(output, ".__repeat{l}")?;

			vtable.push_scope();
			for i in contents {
				compile_statement(i, env, type_table, label_index, vtable, str_table, output)?;
			}
			vtable.pop_scope();

			// Execute epilogue before checking condition
			let scratch = vtable.alloc(Type::Primative(Primative {
				signed: false,
				size: 1,
			}))?;

			writeln!(output, "\tdb {}, {scratch}, $1", env.expand("put_u8")?)?;

			writeln!(
				output,
				"\tdb {}, {repeat_index}, {scratch}, {repeat_index}",
				env.expand("sub_u8")?
			)?;

			writeln!(output, ".__end{l}")?;

			writeln!(output, "\tdb {}, {scratch}, $0", env.expand("put_u8")?)?;

			writeln!(
				output,
				"\tdb {}, {repeat_index}, {scratch}, {scratch}",
				env.expand("equ_u8")?
			)?;

			writeln!(
				output,
				"\tdb {}, {scratch}, LOW(.__repeat{l}), HIGH(.__repeat{l})",
				env.expand("jmp_if_false")?
			)?;

			vtable.autofree(scratch);
			vtable.autofree(repeat_index);
		}
		StatementType::Loop(contents) => {
			let l = *label_index;
			*label_index += 1;

			writeln!(output, ".__loop{l}")?;

			vtable.push_scope();
			for i in contents {
				compile_statement(i, env, type_table, label_index, vtable, str_table, output)?;
			}
			vtable.pop_scope();

			writeln!(
				output,
				"\tdb {}, LOW(.__loop{l}), HIGH(.__loop{l})",
				env.expand("jmp")?
			)?;

			writeln!(output, ".__end{l}")?;
		}
		_ => {
			return Err(CompilerError {
				start: Some(statement.start),
				end: Some(statement.end),
				msg: String::from("Statement not allowed in function"),
			})
		}
	};

	Ok(())
}

fn compile_function<W: Write>(
	name: &str,
	func: types::Function,
	environment_table: &EnvironmentTable,
	type_table: &TypeTable,
	output: &mut W,
	options: &CompilerOptions,
) -> Result<(), CompilerError> {
	let env = match environment_table.get(&func.environment) {
		Some(env) => env,
		None => {
			return Err(CompilerError {
				start: Some(func.start),
				end: Some(func.end),
				msg: format!("Environment {} does not exist", func.environment),
			})
		}
	};
	let mut vtable = VariableTable::new();
	let mut str_table = Vec::<String>::new();
	let mut label_index = 0;

	writeln!(output, "\nsection \"{name} evscript fn\", romx\n{name}::")?;

	for i in func.contents {
		compile_statement(
			i,
			env,
			type_table,
			&mut label_index,
			&mut vtable,
			&mut str_table,
			output,
		)?;
	}

	writeln!(output, "\tdb 0")?;

	let mut i = 0;
	while i < str_table.len() {
		writeln!(output, ".__string{i} db \"{}\", 0", str_table[i])?;
		i += 1;
	}

	if options.report_usage {
		println!("({name}) Peak usage: {}", vtable.peak_usage);
	}

	if (vtable.peak_usage as u16) > env.pool {
		eprintln!(
			"WARN: {name} is using {} bytes, more than the maximum pool size for {}: {}",
			vtable.peak_usage, env.name, env.pool,
		);
	}

	Ok(())
}

fn compile_ast<W: Write>(
	ast: Vec<types::Root>,
	environment_table: &mut EnvironmentTable,
	type_table: &mut TypeTable,
	output: &mut W,
	options: &CompilerOptions,
	// Only true if this file was not `include`d.
	// Used for outputting pool sizes.
	is_root: bool,
) -> Result<(), CompilerError> {
	for i in ast {
		match i {
			types::Root::Environment(name, env) => {
				let new_env = compile_environment(&name, env, environment_table, output)?;
				if is_root {
					writeln!(
						output,
						"def {name}__pool_size equ {}\nexport {name}__pool_size",
						new_env.pool
					)?;
				}
				environment_table.insert(name, new_env);
			}
			types::Root::Function(name, func) => {
				compile_function(&name, func, environment_table, type_table, output, options)?;
			}
			types::Root::Assembly(contents) => {
				writeln!(output, "{}", contents)?;
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

				if let Err(err) =
					compile_ast(ast, environment_table, type_table, output, options, false)
				{
					eprintln!("{path}: {err}");
					exit(1);
				}
			}
			types::Root::Typedef { name, t } => {
				type_table.table.insert(name, type_table.lookup_type(&t)?);
			}
			types::Root::Struct { name, contents } => {
				let mut struct_members = Vec::<(String, Type)>::new();

				for i in contents {
					struct_members.push((i.name, type_table.lookup_type(&i.t)?));
				}

				type_table.table.insert(name, Type::Struct(struct_members));
			}
		}
	}

	Ok(())
}

pub fn compile<W: Write>(
	ast: Vec<types::Root>,
	path: &str,
	output: &mut W,
	options: CompilerOptions,
) -> Result<(), CompilerError> {
	let mut environment_table = EnvironmentTable::new();

	let mut type_table = TypeTable {
		table: HashMap::<String, Type>::from([
			(
				String::from("u8"),
				Type::Primative(Primative {
					signed: false,
					size: 1,
				}),
			),
			(
				String::from("u16"),
				Type::Primative(Primative {
					signed: false,
					size: 2,
				}),
			),
		]),
	};

	writeln!(output, "def __EVSCRIPT_FILE__ equs {path:?}")?;
	compile_ast(
		ast,
		&mut environment_table,
		&mut type_table,
		output,
		&options,
		true,
	)?;
	Ok(())
}
