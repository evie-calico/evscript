use clap::Parser;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use evscript::compiler::CompilerOptions;
use lalrpop_util::ParseError;

use std::fs::{self, read_to_string};
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

	let ast = match evscript::parse(input) {
		Ok(ast) => ast,
		Err(err) => {
			let (message, range) = match err {
				ParseError::InvalidToken { location } => {
					(String::from("Invalid token"), Some(location..location))
				}
				ParseError::UnrecognizedEof { location, expected } => {
					let mut message = "Unexpected EOF, expected one of:".to_string();
					for i in expected {
						message += " ";
						message += &i;
					}
					(message, Some(location..location))
				}
				ParseError::UnrecognizedToken { token, expected } => {
					let (l, t, r) = token;
					let mut message = format!("Unexepected token. Got \"{t}\", expected one of:");
					for i in expected {
						message += " ";
						message += &i;
					}
					(message, Some(l..r))
				}
				ParseError::ExtraToken { token } => {
					let (l, t, r) = token;
					(format!("Extra token: \"{t}\""), Some(l..r))
				}
				ParseError::User { error } => (error.to_string(), None::<std::ops::Range<usize>>),
			};
			let mut files = SimpleFiles::new();
			let file_id = files.add(&cli.input, input);

			let diagnostic = if let Some(range) = range {
				Diagnostic::error()
					.with_labels(vec![Label::primary(file_id, range)])
					.with_message(message)
			} else {
				Diagnostic::error().with_message(message)
			};

			let writer = StandardStream::stderr(ColorChoice::Auto);
			let config = term::Config::default();
			if let Err(err) = term::emit(&mut writer.lock(), &config, &files, &diagnostic) {
				eprintln!("Failed to print error: {err}");
			}
			exit(1);
		}
	};

	let mut compiler_options = CompilerOptions::new();
	compiler_options.report_usage = cli.report_usage;

	let mut output = String::new();
	if let Err(err) = evscript::compile(ast, &cli.input, &mut output, compiler_options) {
		let mut files = SimpleFiles::new();
		let file_id = files.add(&cli.input, input);

		let diagnostic = if let Some(range) = err.get_range() {
			Diagnostic::error()
				.with_labels(vec![Label::primary(file_id, range)])
				.with_message(err.msg)
		} else {
			Diagnostic::error().with_message(err.msg)
		};

		let writer = StandardStream::stderr(ColorChoice::Auto);
		let config = term::Config::default();
		if let Err(err) = term::emit(&mut writer.lock(), &config, &files, &diagnostic) {
			eprintln!("Failed to print error: {err}");
		}
		exit(1);
	}

	if let Err(msg) = fs::write(&cli.output, output) {
		eprintln!("Failed to write output file ({}): {msg}", cli.output);
	}
}
