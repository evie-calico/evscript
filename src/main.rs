use evscript::parser::parse;

use std::process::exit;

fn main() {
	match parse(&mut "
		#asm
			include \"hardware.inc\"
		#end

		include asm \"hardware.inc\"

		env script {}

		script fn Main {
			u8 var;
		}
		".chars().peekable()
	) {
		Ok(ast) => println!("{ast:#?}"),
		Err(err) => {
			eprintln!("Failed to parse: {err}");
			exit(1);
		}
	}
}
