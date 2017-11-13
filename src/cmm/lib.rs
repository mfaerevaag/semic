extern crate lalrpop_util;

pub mod ast;
pub mod parser;
pub mod checker;
pub mod env;
pub mod engine;
pub mod error;
pub mod repl;
pub mod util;

use ast::{CProg, CProgElem, CFunc, CStmt, CExpr};
use error::CError;

/// Run program
///
/// # Examples
///
/// ```
/// use cmm::env::SymVal;
/// let filename = "foo.cmm".to_owned();
/// let program = r#"int main () { return 0; }"#.to_owned();
/// let result = cmm::run(filename, program, false);
/// assert!(result.is_ok());
/// assert_eq!(Some(SymVal::Int(0)), result.unwrap());
/// ```
pub fn run(
    filename: String,
    program: String,
    interactive: bool,
    verbose: bool
) -> Result<Option<env::SymVal>, ()>
{
    let error_printer = error::ErrorPrinter::new(&filename, &program);

    let ast = match parse_prog(&program) {
        Ok(ast) => {
            if verbose { println!("ast: {:#?}", &ast); }
            ast
        },
        Err(err) => {
            error_printer.print_err(err);
            return Err(());
        }
    };

    match engine::run_prog(&ast, &program, interactive) {
        Ok(ret) => {
            if verbose { println!("returned: {:?}", ret); }
            Ok(ret)
        }
        Err(err) => {
            error_printer.print_err(err);
            Err(())
        }
    }
}

/// Parse program
///
/// # Examples
///
/// ```
/// assert!(cmm::parse_prog(r#"int main () { return 0; }"#).is_ok());
/// ```
///
/// ```
/// assert!(cmm::parse_prog(r#"main () {}"#).is_err());
/// ```
pub fn parse_prog<'input, 'err,>(input: &'input str) -> Result<CProg<'input>, CError> {
    match parser::parse_Prog(&mut vec![], input) {
        Ok(x) => Ok(x),
        Err(err) => Err(CError::from_lalrpop(err)),
    }
}

pub fn parse_func<'input, 'err,>(input: &'input str,) -> Result<CFunc<'input>, CError> {
    match parser::parse_Func(&mut vec![], input) {
        Ok(ref x) => match x.first().unwrap() {
            &CProgElem::Func(_, ref f) => Ok(f.clone()),
            x => panic!("unexpected prog elem '{:?}'", x),
        },
        Err(err) => Err(CError::from_lalrpop(err)),
    }
}

pub fn parse_stmt<'input, 'err,>(input: &'input str,) -> Result<CStmt<'input>, CError> {
    match parser::parse_Stmt(&mut vec![], input) {
        Ok(x) => Ok(x),
        Err(err) => Err(CError::from_lalrpop(err)),
    }
}

pub fn parse_expr<'input, 'err,>(input: &'input str,) -> Result<CExpr<'input>, CError> {
    match parser::parse_Expr(&mut vec![], input) {
        Ok(x) => Ok(x),
        Err(err) => Err(CError::from_lalrpop(err)),
    }
}

/// Check validity of AST
///
/// # Examples
///
/// ```
/// let ast = cmm::parse_prog(r#"void main () {}"#).unwrap();
/// assert!(cmm::check_prog(&ast).is_ok());
/// ```
///
/// ```
/// let ast = cmm::parse_prog(r#"int x, x;"#).unwrap();
/// assert!(cmm::check_prog(&ast).is_err()); // main missing
/// ```
pub fn check_prog<'input, 'err>(
    ast: &'input CProg
) -> Result<(env::FuncTab<'input>, env::SymTab<'input>), CError>
{
    checker::analyze_prog(ast)
}
