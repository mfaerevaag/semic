extern crate cmm;

use cmm::ast::*;

#[test]
fn func_empty() {
    let mut errors = Vec::new();
    let mut errors2 = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        void main (void) {}
    "#).unwrap();

    let actual2 = cmm::parse(&mut errors2, r#"
        void main () {}
    "#).unwrap();

    let expected = CFunc {
        proto: Box::new(CProto {
            ret: None,
            name: "main",
            params: vec![],
        }),
        decls: vec![],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(0, errors2.len());
    assert_eq!(format!("{:?}", vec![expected.clone()]), format!("{:?}", actual));
    assert_eq!(format!("{:?}", vec![expected.clone()]), format!("{:?}", actual2));
}

#[test]
fn func_return_type() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {}
    "#).unwrap();

    let expected = CFunc {
        proto: Box::new(CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        }),
        decls: vec![],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", vec![expected]), format!("{:?}", actual));
}

#[test]
fn func_param_type_single() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (int a) {}
    "#).unwrap();

    let expected = CFunc {
        proto: Box::new(CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![Box::new((CType::Int, "a"))],
        }),
        decls: vec![],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", vec![expected]), format!("{:?}", actual));
}

#[test]
fn func_param_type_mult() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (int a, char b) {}
    "#).unwrap();

    let expected = CFunc {
        proto: Box::new(CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![Box::new((CType::Int, "a")),
                         Box::new((CType::Char, "b"))],
        }),
        decls: vec![],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", vec![expected]), format!("{:?}", actual));
}

#[test]
fn func_decl_single_type_single_ident() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {
            int x;
        }
    "#).unwrap();

    let expected = CFunc {
        proto: Box::new(CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        }),
        decls: vec![Box::new((CType::Int, "x", None))],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", vec![expected]), format!("{:?}", actual));
}

#[test]
fn func_decl_single_type_single_array() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {
            int x[7];
        }
    "#).unwrap();

    let expected = CFunc {
        proto: Box::new(CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        }),
        decls: vec![Box::new((CType::Array(Box::new(CType::Int)), "x", Some(7)))],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", vec![expected]), format!("{:?}", actual));
}

#[test]
fn func_decl_single_type_mult_ident() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {
            int x, y;
        }
    "#).unwrap();

    let expected = CFunc {
        proto: Box::new(CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        }),
        decls: vec![Box::new((CType::Int, "x", None)),
                    Box::new((CType::Int, "y", None))],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", vec![expected]), format!("{:?}", actual));
}

#[test]
fn func_decl_single_type_mult_array() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {
            int x[7], y[8];
        }
    "#).unwrap();

    let expected = CFunc {
        proto: Box::new(CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        }),
        decls: vec![Box::new((CType::Array(Box::new(CType::Int)), "x", Some(7))),
                    Box::new((CType::Array(Box::new(CType::Int)), "y", Some(8)))],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", vec![expected]), format!("{:?}", actual));
}

#[test]
fn func_decl_mult_type_single_ident() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {
            int x;
            char y;
        }
    "#).unwrap();

    let expected = CFunc {
        proto: Box::new(CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        }),
        decls: vec![Box::new((CType::Int, "x", None)),
                    Box::new((CType::Char, "y", None))],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", vec![expected]), format!("{:?}", actual));
}

#[test]
fn func_decl_mult_type_mult_ident() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {
            int x, y;
            char a, b;
        }
    "#).unwrap();

    let expected = CFunc {
        proto: Box::new(CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        }),
        decls: vec![Box::new((CType::Int, "x", None)), Box::new((CType::Int, "y", None)),
                    Box::new((CType::Char, "a", None)), Box::new((CType::Char, "b", None))],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", vec![expected]), format!("{:?}", actual));
}

#[test]
fn func_no_decl_single_stmt() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {
            return 0;
        }
    "#).unwrap();

    let expected = CFunc {
        proto: Box::new(CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        }),
        decls: vec![],
        stmts: vec![Box::new(CStmt::Return((0, 0), Some(Box::new(CExpr::Number(0)))))],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", vec![expected]), format!("{:?}", actual));
}

#[test]
fn func_single_decl_single_stmt() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {
            int x;
            x = 1;
        }
    "#).unwrap();

    let expected = CFunc {
        proto: Box::new(CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        }),
        decls: vec![Box::new((CType::Int, "x", None))],
        stmts: vec![Box::new(CStmt::Assign((0, 0), "x", Box::new(CExpr::Number(1))))],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", vec![expected]), format!("{:?}", actual));
}

#[test]
fn func_stmt_mult() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {
            int x, y;
            x = 1;
            y = 2;
            return x + y;
        }
    "#).unwrap();

    let expected = CFunc {
        proto: Box::new(CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        }),
        decls: vec![Box::new((CType::Int, "x", None)),
                    Box::new((CType::Int, "y", None))],
        stmts: vec![Box::new(CStmt::Assign((0, 0), "x", Box::new(CExpr::Number(1)))),
                    Box::new(CStmt::Assign((0, 0), "y", Box::new(CExpr::Number(2)))),
                    Box::new(CStmt::Return((0, 0), Some(Box::new(CExpr::BinOp(
                        COp::Add,
                        Box::new(CExpr::Ident("x")),
                        Box::new(CExpr::Ident("y")))))))],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", vec![expected]), format!("{:?}", actual));
}
