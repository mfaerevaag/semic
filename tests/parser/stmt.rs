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
         Some(Box::new(CExpr::BinOp(COp::Add,
                                    Box::new(CExpr::Number(1)),
                                    Box::new(CExpr::UnOp(
                                        COp::Sub, Box::new(CExpr::Number(2))))))));

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

#[test]
fn stmt_if_single() {
    let mut errors = Vec::new();

    let actual = cmm::parse_stmt(&mut errors, r#"if (1) return;"#).unwrap();

    let expected = CStmt::If((0,0),
                             Box::new(CExpr::Number(1)),
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
                             Box::new(CExpr::Number(1)),
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
                             Box::new(CExpr::Number(1)),
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
                             Box::new(CExpr::Number(1)),
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
                             Box::new(CExpr::Number(1)),
                             Box::new(CStmt::Return((0,0), None)),
                             Some(Box::new(CStmt::Block(
                                 (0,0),
                                 vec![Box::new(CStmt::Return((0,0), None))]))));

    let expected2 = CStmt::If((0,0),
                             Box::new(CExpr::Number(1)),
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
                             Box::new(CExpr::Number(1)),
                             Box::new(CStmt::Block(
                                 (0,0),
                                 vec![Box::new(CStmt::If((0,0),
                                                    Box::new(CExpr::Number(1)),
                                                    Box::new(CStmt::Return((0,0), None)),
                                                         Some(Box::new(CStmt::Return((0,0), None)))))])),
                             None);

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}
