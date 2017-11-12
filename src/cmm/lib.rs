extern crate lalrpop_util;

pub mod ast;
pub mod parser;
pub mod checker;
pub mod env;
pub mod engine;
pub mod error;

use lalrpop_util::ErrorRecovery;
use ast::{CProg, CFunc, CStmt, CExpr};
use error::CError;

// engine functions

pub fn run(filename: String, prog: String) -> Result<Option<env::SymVal>, ()> {
    let error_printer = error::ErrorPrinter::new(&filename, &prog);

    let ast = match parse_prog(&prog) {
        Ok(ast) => {
            println!("ast: {:#?}", &ast); // TODO: debug
            ast
        },
        Err(err) => {
            error_printer.print_err(err);
            return Err(());
        }
    };

    match engine::run_prog(&ast) {
        Ok(ret) => {
            println!("returned: {:?}", ret); // TODO: debug
            Ok(ret)
        }
        Err(err) => {
            error_printer.print_err(err);
            Err(())
        }
    }
}

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
pub fn parse_prog<'input, 'err,>(
    // errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
) -> Result<CProg<'input>, CError>
{
    match parser::parse_Prog(&mut vec![], input) {
        Ok(x) => Ok(x),
        Err(err) => Err(CError::from_lalrpop(err)),
    }
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
/// let ast = cmm::parse(&mut vec![], r#"void main () {}"#).unwrap();
/// assert!(cmm::check(&ast).is_ok());
/// ```
///
/// ```
/// let ast = cmm::parse(&mut vec![], r#"int x, x;"#).unwrap();
/// assert!(cmm::check(&ast).is_err()); // main missing
/// ```
pub fn check<'input, 'err>(
    ast: &'input CProg
) -> Result<(env::FuncTab<'input>, env::SymTab<'input>), CError>
{
    checker::analyze_prog(ast)
}
