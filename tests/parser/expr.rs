extern crate cmm;

#[test]
fn expr_arith() {
    let actual = cmm::parse_expr(r"- 1 * 2");
    let expected = "((-1) * 2)";
    assert!(actual.is_ok());
    assert_eq!(expected, format!("{:?}", actual.unwrap()));

    let actual2 = cmm::parse_expr(r"1 + 2 * 2");
    let expected2 = "(1 + (2 * 2))";
    assert!(actual2.is_ok());
    assert_eq!(expected2, format!("{:?}", actual2.unwrap()));

    let actual3 = cmm::parse_expr(r"1 + 2 * 2 * 4");
    let expected3 = "(1 + ((2 * 2) * 4))";
    assert!(actual3.is_ok());
    assert_eq!(expected3, format!("{:?}", actual3.unwrap()));

    let actual4 = cmm::parse_expr(r"(1 + 2) * 2 * 4");
    let expected4 = "(((1 + 2) * 2) * 4)";
    assert!(actual4.is_ok());
    assert_eq!(expected4, format!("{:?}", actual4.unwrap()));
}

#[test]
fn expr_rel() {
    let actual = cmm::parse_expr(r#"s == "foo" || i < 9"#);
    let expected = r#"((s == "foo") || (i < 9))"#;
    assert!(actual.is_ok());
    assert_eq!(expected, format!("{:?}", actual.unwrap()));
}

#[test]
fn expr_float() {
    let actual = cmm::parse_expr(r#"0.0"#);
    let expected = r#"0.00"#;
    assert!(actual.is_ok());
    assert_eq!(expected, format!("{:?}", actual.unwrap()));
}
