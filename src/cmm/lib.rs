extern crate lalrpop_util;

pub mod ast;
pub mod parser;

use lalrpop_util::ErrorRecovery;
use ast::{CProg, CStmt};


pub fn parse<'input, 'err,>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
) -> Result<CProg<'input>, lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
{
    return parser::parse_Prog(errors, input);
}

pub fn parse_stmt<'input, 'err,>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
) -> Result<Box<CStmt<'input>>, lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
{
    return parser::parse_Stmt(errors, input);
}
