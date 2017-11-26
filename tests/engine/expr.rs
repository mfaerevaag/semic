extern crate semic;

use semic::engine;
use semic::env::{FuncTab, SymTab, SymVal};
use semic::repl::Repl;

#[test]
fn bin_op_int() {
    let vtab = FuncTab::new();
    let global = SymTab::new();
    let local = SymTab::new();
    let repl = Repl::new(false, "", false);

    let ast = semic::parse_expr(r#" 1 + 1 "#).unwrap();

    let actual = engine::run_expr(&ast, &vtab, &global, &local, &repl);

    let expected = SymVal::Int(2);

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn bin_op_float() {
    let vtab = FuncTab::new();
    let global = SymTab::new();
    let local = SymTab::new();
    let repl = Repl::new(false, "", false);

    let ast = semic::parse_expr(r#" 1.0 + 0.1 "#).unwrap();

    let actual = engine::run_expr(&ast, &vtab, &global, &local, &repl);

    let expected = SymVal::Float(1.1);

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn rel_op() {
    let vtab = FuncTab::new();
    let global = SymTab::new();
    let local = SymTab::new();
    let repl = Repl::new(false, "", false);

    let ast = semic::parse_expr(r#" 1 == 1 "#).unwrap();

    let actual = engine::run_expr(&ast, &vtab, &global, &local, &repl);

    let expected = SymVal::Bool(true);

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn log_op() {
    let vtab = FuncTab::new();
    let global = SymTab::new();
    let local = SymTab::new();
    let repl = Repl::new(false, "", false);

    let ast = semic::parse_expr(r#" (1 == 1) && (1 != 0) "#).unwrap();

    let actual = engine::run_expr(&ast, &vtab, &global, &local, &repl);

    let expected = SymVal::Bool(true);

    println!("{:?}", actual);
    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}
