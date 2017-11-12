extern crate cmm;

use cmm::ast::*;

#[test]
fn stmt_return_empty() {
    let actual = cmm::parse_stmt(r#"return;"#);

    let expected = CStmt::Return((0,0), None);

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn stmt_return_expr() {
    let actual = cmm::parse_stmt(r#"return 1 + - 2;"#);

    let expected = CStmt::Return
        ((0,0),
         Some(CExpr::BinOp((0,0), COp::Add,
                           Box::new(CExpr::Int((0,0), 1)),
                           Box::new(CExpr::UnOp((0,0), COp::Sub,
                                                Box::new(CExpr::Int((0,0), 2)))))));

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn stmt_if_single() {
    let actual = cmm::parse_stmt(r#"if (1) return;"#);

    let expected = CStmt::If((0,0),
                             CExpr::Int((0,0), 1),
                             Box::new(CStmt::Return((0,0), None)),
                             None);

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn stmt_if_block() {
    let actual = cmm::parse_stmt(r#"if (1) { return; }"#);

    let expected = CStmt::If((0,0),
                             CExpr::Int((0,0), 1),
                             Box::new(CStmt::Block(
                                 (0,0),
                                 vec![Box::new(CStmt::Return((0,0), None))])),
                             None);

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn stmt_if_else_single() {
    let actual = cmm::parse_stmt(r#"if (1) return; else return;"#);

    let expected = CStmt::If((0,0),
                             CExpr::Int((0,0), 1),
                             Box::new(CStmt::Return((0,0), None)),
                             Some(Box::new(CStmt::Return((0,0), None))));

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn stmt_if_else_block() {
    let actual = cmm::parse_stmt(r#"if (1) { return; } else { return; }"#);

    let expected = CStmt::If((0,0),
                             CExpr::Int((0,0), 1),
                             Box::new(CStmt::Block(
                                 (0,0),
                                 vec![Box::new(CStmt::Return((0,0), None))])),
                             Some(Box::new(CStmt::Block(
                                 (0,0),
                                 vec![Box::new(CStmt::Return((0,0), None))]))));

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn stmt_if_else_mixed() {
    let actual = cmm::parse_stmt(r#"if (1) return; else { return; }"#);
    let actual2 = cmm::parse_stmt(r#"if (1) { return; } else return;"#);

    let expected = CStmt::If((0,0),
                             CExpr::Int((0,0), 1),
                             Box::new(CStmt::Return((0,0), None)),
                             Some(Box::new(CStmt::Block(
                                 (0,0),
                                 vec![Box::new(CStmt::Return((0,0), None))]))));

    let expected2 = CStmt::If((0,0),
                             CExpr::Int((0,0), 1),
                             Box::new(CStmt::Block(
                                 (0,0),
                                 vec![Box::new(CStmt::Return((0,0), None))])),
                             Some(Box::new(CStmt::Return((0,0), None))));

    assert!(actual.is_ok());
    assert!(actual2.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
    assert_eq!(format!("{:?}", expected2), format!("{:?}", actual2.unwrap()));
}

#[test]
fn stmt_if_else_nested() {
    let actual = cmm::parse_stmt(r#"if (1) { if (1) return; else return; }"#);

    let expected = CStmt::If((0,0),
                             CExpr::Int((0,0), 1),
                             Box::new(CStmt::Block(
                                 (0,0),
                                 vec![Box::new(CStmt::If((0,0),
                                                         CExpr::Int((0,0), 1),
                                                         Box::new(CStmt::Return((0,0), None)),
                                                         Some(Box::new(CStmt::Return((0,0), None)))))])),
                             None);

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn stmt_else_single() {
    let actual = cmm::parse_stmt(r#"while (1) return;"#);

    let expected = CStmt::While((0,0),
                                CExpr::Int((0,0), 1),
                                Box::new(CStmt::Return((0,0), None)));

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn stmt_else_block() {
    let actual = cmm::parse_stmt(r#"while (1) { return; }"#);

    let expected = CStmt::While((0,0),
                                CExpr::Int((0,0), 1),
                                Box::new(CStmt::Block(
                                    (0,0),
                                    vec![Box::new(CStmt::Return((0,0), None))])));

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn stmt_for_single() {
    let actual = cmm::parse_stmt(r#"for (;;) return;"#);

    let mut top = vec![];
    // init
    // cond
    let cond = CExpr::Int((0,0), 1);
    // inc
    let body = Box::new(CStmt::Return((0,0), None));
    // expected
    top.push(Box::new(CStmt::While((0,0), cond, body)));
    let expected = Box::new(CStmt::Block((0,0), top));

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn stmt_for_block() {
    let actual = cmm::parse_stmt(r#"for (;;) { return; }"#);

    let mut top = vec![];
    // init
    // cond
    let cond = CExpr::Int((0,0), 1);
    // inc
    let body = Box::new(CStmt::Block((0,0), vec![Box::new(CStmt::Return((0,0), None))]));
    // expected
    top.push(Box::new(CStmt::While((0,0), cond, body)));
    let expected = Box::new(CStmt::Block((0,0), top));

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn stmt_for_init() {
    let actual = cmm::parse_stmt(r#"for (i = 0;;) return;"#);

    let mut top = vec![];
    // init
    top.push(Box::new(CStmt::Assign((0,0), "i", None, CExpr::Int((0,0), 0))));
    // cond
    let cond = CExpr::Int((0,0), 1);
    // inc
    let body = Box::new(CStmt::Return((0,0), None));
    // expected
    top.push(Box::new(CStmt::While((0,0), cond, body)));
    let expected = Box::new(CStmt::Block((0,0), top));

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn stmt_for_cond() {
    let actual = cmm::parse_stmt(r#"for (;1;) return;"#);

    let mut top = vec![];
    // init
    // cond
    let cond = CExpr::Int((0,0), 1);
    // inc
    let body = Box::new(CStmt::Return((0,0), None));
    // expected
    top.push(Box::new(CStmt::While((0,0), cond, body)));
    let expected = Box::new(CStmt::Block((0,0), top));

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn stmt_for_inc() {
    let actual = cmm::parse_stmt(r#"for (;;i = i + 1) return;"#);

    let mut top = vec![];
    // init
    // cond
    let cond = CExpr::Int((0,0), 1);
    // inc
    let mut body = vec![];
    body.push(Box::new(CStmt::Return((0,0), None)));
    body.push(Box::new(CStmt::Assign((0,0), "i", None,
                                     CExpr::BinOp((0,0), COp::Add,
                                                  Box::new(CExpr::Ident((0,0), "i")),
                                                  Box::new(CExpr::Int((0,0), 1))))));
    let body = Box::new(CStmt::Block((0,0), body));
    // expected
    top.push(Box::new(CStmt::While((0,0), cond, body)));
    let expected = Box::new(CStmt::Block((0,0), top));

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}

#[test]
fn stmt_for_all() {
    let actual = cmm::parse_stmt(r#"for (i = 0;1;i = i + 1) return;"#);

    let mut top = vec![];
    // init
    top.push(Box::new(CStmt::Assign((0,0), "i", None, CExpr::Int((0,0), 0))));
    // cond
    let cond = CExpr::Int((0,0), 1);
    // inc
    let mut body = vec![];
    body.push(Box::new(CStmt::Return((0,0), None)));
    body.push(Box::new(CStmt::Assign((0,0), "i", None,
                                     CExpr::BinOp((0,0), COp::Add,
                                                  Box::new(CExpr::Ident((0,0), "i")),
                                                  Box::new(CExpr::Int((0,0), 1))))));
    let body = Box::new(CStmt::Block((0,0), body));
    // expected
    top.push(Box::new(CStmt::While((0,0), cond, body)));
    let expected = Box::new(CStmt::Block((0,0), top));

    assert!(actual.is_ok());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual.unwrap()));
}
