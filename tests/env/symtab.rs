extern crate cmm;

use cmm::ast::CType;
use cmm::env::{SymTab, SymVal};

#[test]
fn get_type() {
    let mut tab = SymTab::new();

    assert!(tab.get_type("i").is_none());

    tab.insert("i", CType::Ref(Box::new(CType::Int)), Some(4), None, None);

    let meta = tab.get_type("i");
    assert!(meta.is_some());

    let (t, s) = meta.unwrap();
    assert_eq!(CType::Ref(Box::new(CType::Int)), t);
    assert_eq!(Some(4), s);
}

#[test]
fn get_val() {
    let mut tab = SymTab::new();

    assert!(tab.get_val("i").is_none());

    tab.insert("i", CType::Int, None, None, None);

    assert!(tab.get_val("i").is_none());

    tab.insert("j", CType::Int, None, Some(SymVal::Int(2)), None);

    let val = tab.get_val("j");
    assert!(val.is_some());
    assert_eq!(Some(SymVal::Int(2)), val);
}

#[test]
fn set_val() {
    let mut tab = SymTab::new();

    assert!(tab.get_val("i").is_none());

    tab.insert("i", CType::Int, None, None, None);

    assert!(tab.get_val("i").is_none());

    let res = tab.set_val("i", None, SymVal::Int(2), None);
    assert!(res.is_ok());

    let val = tab.get_val("i");
    assert!(val.is_some());
    assert_eq!(Some(SymVal::Int(2)), val);
}

#[test]
fn set_val_array() {
    let mut tab = SymTab::new();

    assert!(tab.get_val("i").is_none());

    tab.insert("i", CType::Ref(Box::new(CType::Int)), Some(2), None, None);

    assert!(tab.get_val("i").is_none());

    let res = tab.set_val("i", Some(0), SymVal::Int(1), None);
    assert!(res.is_ok());

    let valo = tab.get_val("i");
    assert!(valo.is_some());
    let val = valo.unwrap();
    assert_eq!(SymVal::Array(vec![Box::new(SymVal::Int(1)),
                                  Box::new(SymVal::Int(0))]), val);

    let res2 = tab.set_val("i", Some(1), SymVal::Int(2), None);
    assert!(res2.is_ok());

    let valo2 = tab.get_val("i");
    assert!(valo2.is_some());
    let val2 = valo2.unwrap();
    assert_eq!(SymVal::Array(vec![Box::new(SymVal::Int(1)),
                                  Box::new(SymVal::Int(2))]), val2);
}

#[test]
fn get_trace() {
    let mut tab = SymTab::new();

    assert!(tab.get_trace("i").is_none());

    tab.insert("i", CType::Int, None, None, Some(0));

    let traceo = tab.get_trace("i");
    assert!(traceo.is_some());
    let trace = traceo.unwrap();
    assert_eq!(1, trace.len());
    let val = trace.get(0);
    assert!(val.is_some());
    assert_eq!((None, Some(0)), *val.unwrap());

    let res = tab.set_val("i", None, SymVal::Int(2), Some(2));
    assert!(res.is_ok());

    let traceo2 = tab.get_trace("i");
    assert!(traceo2.is_some());
    let trace2 = traceo2.unwrap();
    assert_eq!(2, trace2.len());
    let val2 = trace2.get(1);
    assert!(val2.is_some());
    assert_eq!((Some(SymVal::Int(2)), Some(2)), *val2.unwrap());
}
