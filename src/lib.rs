pub mod compiler;
pub mod types;

pub use compiler::compile;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub parser);

pub fn parse(
	input: &str,
) -> Result<
	std::vec::Vec<crate::types::Root>,
	lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'_>, &'static str>,
> {
	parser::FileParser::new().parse(input)
}
