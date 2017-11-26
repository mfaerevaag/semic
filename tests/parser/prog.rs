extern crate semic;

use semic::ast::*;

#[test]
fn prog_empty() {
    let actual = semic::parse_prog(r#""#);

    assert!(actual.is_ok());
    assert_eq!(format!("[]"), format!("{:?}", actual.unwrap()));
}

#[test]
fn prog_proto_void() {
    let actual = semic::parse_prog(r#"
        void foo (void);
    "#);

    let actual2 = semic::parse_prog(r#"
        void foo ();
    "#);

    let expected = CProto {
        ret: None,
        name: "foo",
        params: vec![],
    };

    assert!(actual.is_ok());
    assert!(actual2.is_ok());
    assert_eq!(format!("[{:?}]", expected.clone()), format!("{:?}", actual.unwrap()));
    assert_eq!(format!("[{:?}]", expected.clone()), format!("{:?}", actual2.unwrap()));
}

#[test]
fn prog_proto_types() {
    let actual = semic::parse_prog(r#"
        int foo (int a, char b);
    "#);

    let expected = CProto {
        ret: Some(CType::Int),
        name: "foo",
        params: vec![(CType::Int, "a"), (CType::Char, "b")],
    };

    assert!(actual.is_ok());
    assert_eq!(format!("[{:?}]", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn prog_proto_names_good() {
    let actual = semic::parse_prog(r#"
        int foo42 ();
    "#);

    let actual2 = semic::parse_prog(r#"
        int foo_42 ();
    "#);

    let expected = CProto {
        ret: Some(CType::Int),
        name: "foo42",
        params: vec![],
    };

    let expected2 = CProto {
        ret: Some(CType::Int),
        name: "foo_42",
        params: vec![],
    };

    assert!(actual.is_ok());
    assert!(actual2.is_ok());
    assert_eq!(format!("[{:?}]", expected), format!("{:?}", actual.unwrap()));
    assert_eq!(format!("[{:?}]", expected2), format!("{:?}", actual2.unwrap()));
}

#[test]
fn prog_proto_names_bad() {
    let actual = semic::parse_prog(r#"
        int 42foo42 ();
    "#);

    let actual2 = semic::parse_prog(r#"
        int _foo_42 ();
    "#);

    assert!(actual.is_err());
    assert!(actual2.is_err());
}

#[test]
fn prog_proto_mult() {
    let actual = semic::parse_prog(r#"
        int foo(int a), bar(char b);
    "#);

    let expected = vec![CProto {
        ret: Some(CType::Int),
        name: "foo",
        params: vec![(CType::Int, "a")],
    }, CProto {
        ret: Some(CType::Int),
        name: "bar",
        params: vec![(CType::Char, "b")],
    }];

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}
