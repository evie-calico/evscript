use clap::Parser;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term;
use evscript::compiler::CompilerOptions;

use std::fs::File;
use std::fs::read_to_string;
use std::process::exit;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	/// Output file
	#[clap(short, long, value_parser, value_name = "PATH")]
	output: String,

	/// Report the peak memory usage of each function
	#[clap(long = "report-usage")]
	report_usage: bool,

	/// Input file
	#[clap(value_parser, value_name = "PATH")]
	input: String,
}

fn main() {
	let cli = Cli::parse();

	let input = &match read_to_string(&cli.input) {
		Ok(input) => input,
		Err(err) => {
			eprintln!("{}: {err}", cli.input);
			exit(1);
		}
	};

	let mut output = match File::create(&cli.output) {
		Ok(f) => f,
		Err(err) => {
			eprintln!("{}: {err}", cli.output);
			exit(1);
		}
	};

	let ast = match evscript::parse(input) {
		Ok(ast) => ast,
		Err(err) => {
			eprintln!("{}:{err}", cli.input);
			exit(1);
		}
	};

	let mut compiler_options = CompilerOptions::new();
	compiler_options.report_usage = cli.report_usage;

	if let Err(err) = evscript::compile(ast, &mut output, compiler_options) {
		let mut files = SimpleFiles::new();
		let file_id = files.add(&cli.input, input);

		let diagnostic = if let Some(range) = err.get_range() {
			Diagnostic::error()
				.with_labels(vec![Label::primary(file_id, range)])
				.with_message(err.msg)
		} else {
			Diagnostic::error()
				.with_message(err.msg)
		};

		let writer = StandardStream::stderr(ColorChoice::Auto);
		let config = term::Config::default();
		match term::emit(&mut writer.lock(), &config, &files, &diagnostic) {
			Err(err) => eprintln!("Failed to print error: {err}"),
			_ => {}
		}
		exit(1);
	}
}
