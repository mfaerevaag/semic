extern crate cmm;

use cmm::engine;
use cmm::ast::CType;
use cmm::env::{FuncTab, SymTab, SymVal};
use cmm::repl::Repl;

#[test]
fn assign_int() {
    let vtab = FuncTab::new();
    let global = SymTab::new();
    let mut local = SymTab::new();
    let repl = Repl::new(false, "", false);

    local.insert("i", CType::Int, None, None, None);

    let ast = cmm::parse_stmt(r#" i = 7; "#).unwrap();

    let (_, tab, actual, _) = engine::run_stmt(&ast, &vtab, global, local, repl).unwrap();
    let expected = None;
    assert_eq!(expected, actual);

    let meta = tab.get_type("i");
    assert!(meta.is_some());

    let (t, s) = meta.unwrap();
    let val = tab.get_val("i");
    assert_eq!(CType::Int, t);
    assert_eq!(None, s);
    assert_eq!(Some(SymVal::Int(7)), val);
}

#[test]
fn assign_array() {
    let vtab = FuncTab::new();
    let global = SymTab::new();
    let mut local = SymTab::new();
    let repl = Repl::new(false, "", false);

    local.insert("s", CType::Char, Some(2), None, None);

    let ast = cmm::parse_stmt(r#" s[1] = '\0'; "#).unwrap();

    let (_, tab, actual, _) = engine::run_stmt(&ast, &vtab, global, local, repl).unwrap();
    let expected = None;
    assert_eq!(expected, actual);

    let meta = tab.get_type("s");
    assert!(meta.is_some());

    let (t, s) = meta.unwrap();
    let val = tab.get_val("s");
    assert_eq!(CType::Char, t);
    assert_eq!(Some(2), s);
    assert_eq!(Some(SymVal::Array(vec![Box::new(SymVal::Int(0)),
                                       Box::new(SymVal::Char('\0'))])), val);
}

#[test]
fn assign_string() {
    let vtab = FuncTab::new();
    let global = SymTab::new();
    let mut local = SymTab::new();
    let repl = Repl::new(false, "", false);

    local.insert("s", CType::Char, Some(2), None, None);

    let ast = cmm::parse_stmt(r#" s = "a"; "#).unwrap();

    let (_, tab, actual, _) = engine::run_stmt(&ast, &vtab, global, local, repl).unwrap();
    let expected = None;
    assert_eq!(expected, actual);

    let meta = tab.get_type("s");
    assert!(meta.is_some());

    let (t, s) = meta.unwrap();
    let val = tab.get_val("s");
    assert_eq!(CType::Char, t);
    assert_eq!(Some(2), s);
    assert_eq!(Some(SymVal::Array(vec![Box::new(SymVal::Char('a')),
                                       Box::new(SymVal::Char('\0'))])), val);
}
