use evscript::lexer::Location;
use evscript::parser::parse;

use std::process::exit;

fn main() {
	let mut loc = Location::new("<test>");

	match parse(&mut "
		#asm
			include \"hardware.inc\"
		#end

		include asm \"hardware.inc\"

		env script {
			use std;
		}

		script fn Main {
			u8 var;
		}
		".chars().peekable(),
		&mut loc
	) {
		Ok(ast) => println!("{ast:#?}"),
		Err(err) => {
			eprintln!("{loc}: {err}");
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
}