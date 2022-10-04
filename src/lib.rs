pub mod parser;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub internal_parser);

pub fn parse(input: &str) -> Result<std::vec::Vec<crate::parser::Root>, lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token<'_>, &'static str>> {
	internal_parser::FileParser::new().parse(input)
}
