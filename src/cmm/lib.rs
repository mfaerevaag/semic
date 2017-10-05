extern crate lalrpop_util;

pub mod ast;
pub mod parser;

use lalrpop_util::ErrorRecovery;
use ast::CFunc;


pub fn parse<'input, 'err,>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
) -> Result<Box<CFunc<'input>>, lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
{
    return parser::parse_Prog(errors, input);
}
