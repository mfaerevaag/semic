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
