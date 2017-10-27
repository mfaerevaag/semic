extern crate cmm;

use cmm::ast::*;

#[test]
fn stmt_return_empty() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"return;"#).unwrap();

    let expected = CStmt::Return((0,0), None);

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn stmt_return_expr() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"return 1 + - 2;"#).unwrap();

    let expected = CStmt::Return
        ((0,0),
         Some(CExpr::BinOp(COp::Add,
                           Box::new(CExpr::Number(1)),
                           Box::new(CExpr::UnOp(COp::Sub,
                                                Box::new(CExpr::Number(2)))))));

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn stmt_if_single() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"if (1) return;"#).unwrap();

    let expected = CStmt::If((0,0),
                             CExpr::Number(1),
                             Box::new(CStmt::Return((0,0), None)),
                             None);

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn stmt_if_block() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"if (1) { return; }"#).unwrap();

    let expected = CStmt::If((0,0),
                             CExpr::Number(1),
                             Box::new(CStmt::Block(
                                 (0,0),
                                 vec![Box::new(CStmt::Return((0,0), None))])),
                             None);

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn stmt_if_else_single() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"if (1) return; else return;"#).unwrap();

    let expected = CStmt::If((0,0),
                             CExpr::Number(1),
                             Box::new(CStmt::Return((0,0), None)),
                             Some(Box::new(CStmt::Return((0,0), None))));

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn stmt_if_else_block() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"if (1) { return; } else { return; }"#).unwrap();

    let expected = CStmt::If((0,0),
                             CExpr::Number(1),
                             Box::new(CStmt::Block(
                                 (0,0),
                                 vec![Box::new(CStmt::Return((0,0), None))])),
                             Some(Box::new(CStmt::Block(
                                 (0,0),
                                 vec![Box::new(CStmt::Return((0,0), None))]))));

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn stmt_if_else_mixed() {
    let mut errors = Vec::new();
    let mut errors2 = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"if (1) return; else { return; }"#).unwrap();
    let actual2 = cmm::parse_stmt(&mut errors2, r#"if (1) { return; } else return;"#).unwrap();

    let expected = CStmt::If((0,0),
                             CExpr::Number(1),
                             Box::new(CStmt::Return((0,0), None)),
                             Some(Box::new(CStmt::Block(
                                 (0,0),
                                 vec![Box::new(CStmt::Return((0,0), None))]))));

    let expected2 = CStmt::If((0,0),
                             CExpr::Number(1),
                             Box::new(CStmt::Block(
                                 (0,0),
                                 vec![Box::new(CStmt::Return((0,0), None))])),
                             Some(Box::new(CStmt::Return((0,0), None))));

    assert_eq!(0, errors.len());
    assert_eq!(0, errors2.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
    assert_eq!(format!("{:?}", expected2), format!("{:?}", actual2));
}

#[test]
fn stmt_if_else_nested() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"if (1) { if (1) return; else return; }"#).unwrap();

    let expected = CStmt::If((0,0),
                             CExpr::Number(1),
                             Box::new(CStmt::Block(
                                 (0,0),
                                 vec![Box::new(CStmt::If((0,0),
                                                         CExpr::Number(1),
                                                         Box::new(CStmt::Return((0,0), None)),
                                                         Some(Box::new(CStmt::Return((0,0), None)))))])),
                             None);

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn stmt_else_single() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"while (1) return;"#).unwrap();

    let expected = CStmt::While((0,0),
                                CExpr::Number(1),
                                Box::new(CStmt::Return((0,0), None)));

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn stmt_else_block() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"while (1) { return; }"#).unwrap();

    let expected = CStmt::While((0,0),
                                CExpr::Number(1),
                                Box::new(CStmt::Block(
                                    (0,0),
                                    vec![Box::new(CStmt::Return((0,0), None))])));

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn stmt_for_single() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"for (;;) return;"#).unwrap();

    let expected = CStmt::For((0,0), None, None, None,
                              Box::new(CStmt::Return((0,0), None)));

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn stmt_for_block() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"for (;;) { return; }"#).unwrap();

    let expected = CStmt::For((0,0), None, None, None,
                              Box::new(CStmt::Block(
                                  (0,0),
                                  vec![Box::new(CStmt::Return((0,0), None))])));

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn stmt_for_init() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"for (i = 0;;) return;"#).unwrap();

    let expected = CStmt::For((0,0),
                              Some(Box::new(CStmt::Assign((0,0), "i", CExpr::Number(0)))),
                              None, None,
                              Box::new(CStmt::Return((0,0), None)));

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn stmt_for_cond() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"for (;1;) return;"#).unwrap();

    let expected = CStmt::For((0,0), None,
                              Some(CExpr::Number(1)),
                              None,
                              Box::new(CStmt::Return((0,0), None)));

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn stmt_for_inc() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"for (;;i = i + 1) return;"#).unwrap();

    let expected = CStmt::For((0,0), None,
                              None,
                              Some(Box::new(CStmt::Assign(
                                  (0,0), "i",
                                  CExpr::BinOp(
                                      COp::Add,
                                      Box::new(CExpr::Ident("i")),
                                      Box::new(CExpr::Number(1)))))),
                              Box::new(CStmt::Return((0,0), None)));

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn stmt_for_all() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"for (i = 0;1;i = i + 1) return;"#).unwrap();

    let expected = CStmt::For((0,0),
                              Some(Box::new(CStmt::Assign((0,0), "i", CExpr::Number(0)))),
                              Some(CExpr::Number(1)),
                              Some(Box::new(CStmt::Assign(
                                  (0,0), "i",
                                  CExpr::BinOp(
                                      COp::Add,
                                      Box::new(CExpr::Ident("i")),
                                      Box::new(CExpr::Number(1)))))),
                              Box::new(CStmt::Return((0,0), None)));

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}
