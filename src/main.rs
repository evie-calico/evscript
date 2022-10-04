extern crate lalrpop_util;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);

use std::process::exit;

fn main() {
	let input = "env name { def hi(); def bye(); }";
	let result = parser::EnvParser::new().parse(input);
	match result {
		Ok(ast) => println!("{ast:#?}"),
		Err(err) => {
			eprintln!("<test>: {err}");
			exit(1);
		}
	}
}

#[cfg(test)]
mod tests {
	use evscript::lexer::Location;
	use evscript::parser::parse;

	#[test]
	fn syntax() {
		// Test all basic lexing & parsing.
		let mut loc = Location::new("<syntax test>");
		let mut input = "
			#asm
				include \"hardware.inc\"
			#end

			include asm \"hardware.inc\"

			env script {
				use std;
				def example(u8);
			}

			script fn Main {}
			".chars().peekable();

		if let Err(err) = parse(&mut input, &mut loc){
			eprintln!("{loc}: {err}");
			panic!("{loc}: {err}");
		}
	}

	#[test]
	fn syntax() {
		// Test all basic lexing & parsing.
		let mut loc = Location::new("<expression test>");
		let mut input = "script fn Main { x + y; }".chars().peekable();

		if let Err(err) = parse(&mut input, &mut loc){
			eprintln!("{loc}: {err}");
			panic!("{loc}: {err}");
		}
	}
}