extern crate cmm;

use cmm::ast::*;

#[test]
fn func_empty() {
    let mut errors = Vec::new();
    let mut errors2 = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        void main (void) {}
    "#).unwrap();

    let actual2 = cmm::parse_func(&mut errors2, r#"
        void main () {}
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: None,
            name: "main",
            params: vec![],
        },
        stmts: vec![],
    };

    assert!(errors.is_empty());
    assert!(errors2.is_empty());
    assert_eq!(format!("{:?}", expected.clone()), format!("{:?}", actual));
    assert_eq!(format!("{:?}", expected.clone()), format!("{:?}", actual2));
}

#[test]
fn func_return_type() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        int main (void) {}
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        stmts: vec![],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn func_param_type_single() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        int main (int a) {}
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![(CType::Int, "a")],
        },
        stmts: vec![],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn func_param_type_mult() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        int main (int a, char b) {}
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![(CType::Int, "a"), (CType::Char, "b")],
        },
        stmts: vec![],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn func_decl_single_type_single_ident() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        int main (void) {
            int x;
        }
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        stmts: vec![CStmt::Decl((0, 0), CType::Int, "x", None)],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn func_decl_single_type_single_array() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        int main (void) {
            int x[7];
        }
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        stmts: vec![CStmt::Decl((0, 0), CType::Ref(Box::new(CType::Int)), "x", Some(7))],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn func_decl_single_type_mult_ident() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        int main (void) {
            int x, y;
        }
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        stmts: vec![CStmt::Decl((0, 0), CType::Int, "x", None),
                    CStmt::Decl((0, 0), CType::Int, "y", None)],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn func_decl_single_type_mult_array() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        int main (void) {
            int x[7], y[8];
        }
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        stmts: vec![CStmt::Decl((0, 0), CType::Ref(Box::new(CType::Int)), "x", Some(7)),
                    CStmt::Decl((0, 0), CType::Ref(Box::new(CType::Int)), "y", Some(8))],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn func_decl_mult_type_single_ident() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        int main (void) {
            int x;
            char y;
        }
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        stmts: vec![CStmt::Decl((0, 0), CType::Int, "x", None),
                    CStmt::Decl((0, 0), CType::Char, "y", None)],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn func_decl_mult_type_mult_ident() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        int main (void) {
            int x, y;
            char a, b;
        }
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        stmts: vec![
            CStmt::Decl((0, 0), CType::Int, "x", None),
            CStmt::Decl((0, 0), CType::Int, "y", None),
            CStmt::Decl((0, 0), CType::Char, "a", None),
            CStmt::Decl((0, 0), CType::Char, "b", None)
        ],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn func_decl_after_stmt() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        void main (void) {
            return;
            char a;
            return;
        }
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: None,
            name: "main",
            params: vec![],
        },
        stmts: vec![
            CStmt::Return((0, 0), None),
            CStmt::Decl((0, 0), CType::Char, "a", None),
            CStmt::Return((0, 0), None),
        ],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn func_decl_imm_init() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        void main (void) {
            char a = 'a';
        }
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: None,
            name: "main",
            params: vec![],
        },
        stmts: vec![
            CStmt::Decl((0, 0), CType::Char, "a", None),
            CStmt::Assign((0, 0), "a", None, CExpr::Char('a')),
        ],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn func_decl_imm_init_string() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        void main (void) {
            char a[] = "foobar";
        }
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: None,
            name: "main",
            params: vec![],
        },
        stmts: vec![
            CStmt::Decl((0, 0), CType::Ref(Box::new(CType::Char)), "a", None),
            CStmt::Assign((0, 0), "a", None, CExpr::Str("foobar".chars())),
        ],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn func_no_decl_single_stmt() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        int main (void) {
            return 0;
        }
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        stmts: vec![CStmt::Return((0, 0), Some(CExpr::Num(0)))],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn func_single_decl_single_stmt() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        int main (void) {
            int x;
            x = 1;
        }
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        stmts: vec![
            CStmt::Decl((0, 0), CType::Int, "x", None),
            CStmt::Assign((0, 0), "x", None, CExpr::Num(1))
        ],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn func_single_array_single_stmt() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        int main (void) {
            char x[7];
            x[6] = '\0';
        }
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        stmts: vec![
            CStmt::Decl((0, 0), CType::Ref(Box::new(CType::Char)), "x", Some(7)),
            CStmt::Assign((0, 0), "x", Some(6), CExpr::Char('\0'))],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn func_stmt_mult() {
    let mut errors = Vec::new();

    let actual = cmm::parse_func(&mut errors, r#"
        int main (void) {
            int x, y;
            x = 1;
            y = 2;
            return x + y;
        }
    "#).unwrap();

    let expected = CFunc {
        proto: CProto {
            ret: Some(CType::Int),
            name: "main",
            params: vec![],
        },
        stmts: vec![
            CStmt::Decl((0, 0), CType::Int, "x", None),
            CStmt::Decl((0, 0), CType::Int, "y", None),
            CStmt::Assign((0, 0), "x", None, CExpr::Num(1)),
            CStmt::Assign((0, 0), "y", None, CExpr::Num(2)),
            CStmt::Return((0, 0), Some(CExpr::BinOp(
                COp::Add,
                Box::new(CExpr::Ident("x")),
                Box::new(CExpr::Ident("y")))))
        ],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}
