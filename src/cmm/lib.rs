extern crate lalrpop_util;

pub mod ast;
pub mod parser;
pub mod checker;
pub mod env;

use lalrpop_util::ErrorRecovery;
use ast::{CProg, CFunc, CStmt, CExpr};

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

pub fn parse_expr<'input, 'err,>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
) -> Result<CExpr<'input>, lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
{
    parser::parse_Expr(errors, input)
}

// checker functions

/// Check validity of AST
///
/// # Examples
///
/// ```
/// let mut err = Vec::new();
/// let ast = cmm::parse(&mut vec![], r#"void main () {}"#).unwrap();
/// assert!(cmm::check(&mut err, &ast).is_ok());
/// ```
///
/// ```
/// let mut err = Vec::new();
/// let ast = cmm::parse(&mut vec![], r#"int x, x;"#).unwrap();
/// assert!(cmm::check(&mut err, &ast).is_err());
/// ```
pub fn check<'input, 'err>(
    errors: &'err mut Vec<checker::CheckErr>,
    ast: &'input CProg
) -> Result<(), ()> {
    checker::check_prog(errors, ast)
}
