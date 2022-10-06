use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
	let path = &args().collect::<Vec<String>>()[1];

	let input = &match read_to_string(path) {
		Ok(input) => input,
		Err(err) => {
			eprintln!("{path}: {err}");
			exit(1);
		}
	};

	match evscript::parse(input) {
		Ok(ast) => println!("{ast:?}"),
		Err(err) => {
			eprintln!("{path}: {err}");
			exit(1);
		}
	}
}

mod tests {
	#[cfg(test)]
	use evscript::types::*;
	use std::fs::read_to_string;

	fn test_parsing(path: &str) {
		let input = &match read_to_string(path) {
			Ok(input) => input,
			Err(err) => panic!("{path}: {err}"),
		};

		match evscript::parse(input) {
			Ok(ast) => println!("{ast:?}"),
			Err(err) => panic!("{path}: {err}"),
		}
	}

	#[test]
	fn header() {
		test_parsing("scripts/header.evs");
	}

	#[test]
	fn npc_script() {
		test_parsing("scripts/npc_script.evs");
	}

	#[test]
	fn dungeon_generator() {
		test_parsing("scripts/dungeon_generator.evs");
	}

	#[test]
	fn eval_const_expression() {
		let input = r#"
	env script {
		pool = 9 * (5 == 6) + 3;
	}
	"#;

		match evscript::parse(input) {
			Ok(ast) => {
				let env = match &ast[0] {
					Root::Environment(env) => env,
					_ => panic!("First root should be env!")
				};
				let expression = match &env.contents[0] {
					Statement::Pool(rpn) => rpn,
					_ => panic!("First statement should be pool!")
				};
				assert!(expression.eval_const() == Ok(3), "Incorrect expression result");
			}
			Err(err) => panic!("<example_expression>: {err}"),
		}
	}
}
