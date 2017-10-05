extern crate cmm;

use cmm::ast::*;

#[test]
fn parser_empty() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        void main (void) {}
    "#).unwrap();

    let expected = CFunc {
        ret_type: None,
        name: "main".to_string(),
        params: vec![],
        decls: vec![],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn parser_return_type() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {}
    "#).unwrap();

    let expected = CFunc {
        ret_type: Some(CType::Int),
        name: "main".to_string(),
        params: vec![],
        decls: vec![],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn parser_param_type_single() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (int a) {}
    "#).unwrap();

    let expected = CFunc {
        ret_type: Some(CType::Int),
        name: "main".to_string(),
        params: vec![Box::new((CType::Int, "a".to_string()))],
        decls: vec![],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn parser_param_type_mult() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (int a, char b) {}
    "#).unwrap();

    let expected = CFunc {
        ret_type: Some(CType::Int),
        name: "main".to_string(),
        params: vec![Box::new((CType::Int, "a".to_string())),
                     Box::new((CType::Char, "b".to_string()))],
        decls: vec![],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn parser_decl_single_type_single_ident() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {
            int x;
        }
    "#).unwrap();

    let expected = CFunc {
        ret_type: Some(CType::Int),
        name: "main".to_string(),
        params: vec![],
        decls: vec![Box::new((CType::Int, vec!["x".to_string()]))],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn parser_decl_single_type_mult_ident() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {
            int x, y;
        }
    "#).unwrap();

    let expected = CFunc {
        ret_type: Some(CType::Int),
        name: "main".to_string(),
        params: vec![],
        decls: vec![Box::new((CType::Int, vec!["x".to_string(), "y".to_string()]))],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn parser_decl_mult_type_single_ident() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {
            int x;
            char y;
        }
    "#).unwrap();

    let expected = CFunc {
        ret_type: Some(CType::Int),
        name: "main".to_string(),
        params: vec![],
        decls: vec![Box::new((CType::Int, vec!["x".to_string()])),
                    Box::new((CType::Char, vec!["y".to_string()]))],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn parser_decl_mult_type_mult_ident() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {
            int x, y;
            char a, b;
        }
    "#).unwrap();

    let expected = CFunc {
        ret_type: Some(CType::Int),
        name: "main".to_string(),
        params: vec![],
        decls: vec![Box::new((CType::Int, vec!["x".to_string(), "y".to_string()])),
                    Box::new((CType::Char, vec!["a".to_string(), "b".to_string()]))],
        stmts: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn parser_no_decl_single_stmt() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {
            return 0;
        }
    "#).unwrap();

    let expected = CFunc {
        ret_type: Some(CType::Int),
        name: "main".to_string(),
        params: vec![],
        decls: vec![],
        stmts: vec![Box::new(CStmt::Return(Some(Box::new(CExpr::Number(0)))))],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn parser_single_decl_single_stmt() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int main (void) {
            int x;
            x = 1;
        }
    "#).unwrap();

    let expected = CFunc {
        ret_type: Some(CType::Int),
        name: "main".to_string(),
        params: vec![],
        decls: vec![Box::new((CType::Int, vec!["x".to_string()]))],
        stmts: vec![Box::new(CStmt::Assign("x".to_string(), Box::new(CExpr::Number(1))))],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn parser_stmt_mult() {
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
        ret_type: Some(CType::Int),
        name: "main".to_string(),
        params: vec![],
        decls: vec![Box::new((CType::Int, vec!["x".to_string(), "y".to_string()]))],
        stmts: vec![Box::new(CStmt::Assign("x".to_string(), Box::new(CExpr::Number(1)))),
                    Box::new(CStmt::Assign("y".to_string(), Box::new(CExpr::Number(2)))),
                    Box::new(CStmt::Return(Some(Box::new(CExpr::BinOp(
                        Box::new(CExpr::Ident("x".to_string())),
                        COp::Add,
                        Box::new(CExpr::Ident("y".to_string())))))))],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}
