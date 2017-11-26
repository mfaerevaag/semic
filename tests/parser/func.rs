extern crate cmm;

use cmm::ast::*;

#[test]
fn func_empty() {
    let actual = cmm::parse_func(r#"
        void main (void) {}
    "#);

    let actual2 = cmm::parse_func(r#"
        void main () {}
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: None,
            name: "main",
            params: vec![],
        },
        body: CStmt::Block((0,0), vec![]),
    };

    assert!(actual.is_ok());
    assert!(actual2.is_ok());
    assert_eq!(format!("{:?}", expected.clone()), format!("{:?}", actual.unwrap()));
    assert_eq!(format!("{:?}", expected.clone()), format!("{:?}", actual2.unwrap()));
}

#[test]
fn func_return_type() {
    let actual = cmm::parse_func(r#"
        int main (void) {}
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        body: CStmt::Block((0,0), vec![]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn func_param_type_single() {
    let actual = cmm::parse_func(r#"
        int main (int a) {}
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![(CType::Int, "a")],
        },
        body: CStmt::Block((0,0), vec![]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn func_param_type_mult() {
    let actual = cmm::parse_func(r#"
        int main (int a, char b) {}
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![(CType::Int, "a"), (CType::Char, "b")],
        },
        body: CStmt::Block((0,0), vec![]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn func_decl_single_type_single_ident() {
    let actual = cmm::parse_func(r#"
        int main (void) {
            int x;
        }
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        body: CStmt::Block((0,0), vec![Box::new(CStmt::Decl((0,0), CType::Int, "x", None))]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn func_decl_single_type_single_array() {
    let actual = cmm::parse_func(r#"
        int main (void) {
            int x[7];
        }
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        body: CStmt::Block((0,0), vec![
            Box::new(CStmt::Decl((0,0), CType::Ref(Box::new(CType::Int)), "x", Some(CExpr::Int((0,0), 7))))
        ]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn func_decl_single_type_mult_ident() {
    let actual = cmm::parse_func(r#"
        int main (void) {
            int x, y;
        }
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        body: CStmt::Block((0,0), vec![
            Box::new(CStmt::Decl((0,0), CType::Int, "x", None)),
            Box::new(CStmt::Decl((0,0), CType::Int, "y", None))
        ]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn func_decl_single_type_mult_array() {
    let actual = cmm::parse_func(r#"
        int main (void) {
            int x[7], y[8];
        }
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        body: CStmt::Block((0,0), vec![
            Box::new(CStmt::Decl((0,0), CType::Ref(Box::new(CType::Int)), "x", Some(CExpr::Int((0,0), 7)))),
            Box::new(CStmt::Decl((0,0), CType::Ref(Box::new(CType::Int)), "y", Some(CExpr::Int((0,0), 8))))
        ]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn func_decl_mult_type_single_ident() {
    let actual = cmm::parse_func(r#"
        int main (void) {
            int x;
            char y;
        }
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        body: CStmt::Block((0,0), vec![
            Box::new(CStmt::Decl((0,0), CType::Int, "x", None)),
            Box::new(CStmt::Decl((0,0), CType::Char, "y", None))
        ]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn func_decl_mult_type_mult_ident() {
    let actual = cmm::parse_func(r#"
        int main (void) {
            int x, y;
            char a, b;
        }
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        body: CStmt::Block((0,0), vec![
            Box::new(CStmt::Decl((0,0), CType::Int, "x", None)),
            Box::new(CStmt::Decl((0,0), CType::Int, "y", None)),
            Box::new(CStmt::Decl((0,0), CType::Char, "a", None)),
            Box::new(CStmt::Decl((0,0), CType::Char, "b", None))
        ]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn func_decl_after_stmt() {
    let actual = cmm::parse_func(r#"
        void main (void) {
            return;
            char a;
            return;
        }
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: None,
            name: "main",
            params: vec![],
        },
        body: CStmt::Block((0,0), vec![
            Box::new(CStmt::Return((0,0), None)),
            Box::new(CStmt::Decl((0,0), CType::Char, "a", None)),
            Box::new(CStmt::Return((0,0), None)),
        ]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn func_decl_imm_init() {
    let actual = cmm::parse_func(r#"
        void main (void) {
            char a = 'a';
        }
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: None,
            name: "main",
            params: vec![],
        },
        body: CStmt::Block((0,0), vec![
            Box::new(CStmt::Decl((0,0), CType::Char, "a", None)),
            Box::new(CStmt::Assign((0,0), "a", None, CExpr::Char((0,0), 'a'))),
        ]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn func_decl_imm_init_string() {
    let actual = cmm::parse_func(r#"
        void main (void) {
            char a[] = "foobar";
        }
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: None,
            name: "main",
            params: vec![],
        },
        body: CStmt::Block((0,0), vec![
            Box::new(CStmt::Decl((0,0), CType::Ref(Box::new(CType::Char)), "a", None)),
            Box::new(CStmt::Assign((0,0), "a", None, CExpr::Str((0,0), "foobar".chars()))),
        ]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn func_no_decl_single_stmt() {
    let actual = cmm::parse_func(r#"
        int main (void) {
            return 0;
        }
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        body: CStmt::Block((0,0), vec![
            Box::new(CStmt::Return((0,0), Some(CExpr::Int((0,0), 0))))
        ]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn func_single_decl_single_stmt() {
    let actual = cmm::parse_func(r#"
        int main (void) {
            int x;
            x = 1;
        }
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        body: CStmt::Block((0,0), vec![
            Box::new(CStmt::Decl((0,0), CType::Int, "x", None)),
            Box::new(CStmt::Assign((0,0), "x", None, CExpr::Int((0,0), 1)))
        ]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn func_single_array_single_stmt() {
    let actual = cmm::parse_func(r#"
        int main (void) {
            char x[7];
            x[6] = '\0';
        }
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        body: CStmt::Block((0,0), vec![
            Box::new(CStmt::Decl((0,0), CType::Ref(Box::new(CType::Char)), "x", Some(CExpr::Int((0,0), 7)))),
            Box::new(CStmt::Assign((0,0), "x", Some(CExpr::Int((0,0), 6)), CExpr::Char((0,0), '\0')))]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn func_stmt_mult() {
    let actual = cmm::parse_func(r#"
        int main (void) {
            int x, y;
            x = 1;
            y = 2;
            return x + y;
        }
    "#);

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        body: CStmt::Block((0,0), vec![
            Box::new(CStmt::Decl((0,0), CType::Int, "x", None)),
            Box::new(CStmt::Decl((0,0), CType::Int, "y", None)),
            Box::new(CStmt::Assign((0,0), "x", None, CExpr::Int((0,0), 1))),
            Box::new(CStmt::Assign((0,0), "y", None, CExpr::Int((0,0), 2))),
            Box::new(CStmt::Return((0,0), Some(CExpr::BinOp((0,0),
                COp::Add,
                Box::new(CExpr::Ident((0,0), "x")),
                Box::new(CExpr::Ident((0,0), "y"))))))
        ]),
    };

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}
