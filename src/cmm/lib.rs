extern crate lalrpop_util;

pub mod ast;
pub mod parser;
pub mod checker;
pub mod env;

use lalrpop_util::ErrorRecovery;
use ast::{CProg, CFunc, CStmt};

// parser functions

/// Parse program
///
/// # Examples
///
/// ```
/// let mut err = Vec::new();
/// assert!(cmm::parse(&mut err, r#"int main () {}"#).is_ok());
/// ```
///
/// ```
/// let mut err = Vec::new();
/// assert!(cmm::parse(&mut err, r#"main () {}"#).is_err());
/// ```
pub fn parse<'input, 'err,>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
) -> Result<CProg<'input>, lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
{
    parser::parse_Prog(errors, input)
}

pub fn parse_func<'input, 'err,>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
) -> Result<CFunc<'input>, lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
{
    parser::parse_Func(errors, input)
}

pub fn parse_stmt<'input, 'err,>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
) -> Result<CStmt<'input>, lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
{
    parser::parse_Stmt(errors, input)
}
