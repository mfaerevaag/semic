extern crate cmm;

#[test]
fn expr_arith() {
    let actual = cmm::parse_expr(&mut vec![], r"- 1 * 2").unwrap();
    let expected = "((-1) * 2)";
    assert_eq!(expected, format!("{:?}", actual));

    let actual2 = cmm::parse_expr(&mut vec![], r"1 + 2 * 2").unwrap();
    let expected2 = "(1 + (2 * 2))";
    assert_eq!(expected2, format!("{:?}", actual2));

    let actual3 = cmm::parse_expr(&mut vec![], r"1 + 2 * 2 * 4").unwrap();
    let expected3 = "(1 + ((2 * 2) * 4))";
    assert_eq!(expected3, format!("{:?}", actual3));

    let actual4 = cmm::parse_expr(&mut vec![], r"(1 + 2) * 2 * 4").unwrap();
    let expected4 = "(((1 + 2) * 2) * 4)";
    assert_eq!(expected4, format!("{:?}", actual4));
}

#[test]
fn expr_rel() {
    let actual = cmm::parse_expr(&mut vec![], r#"s == "foo" || i < 9"#).unwrap();
    let expected = r#"((s == "foo") || (i < 9))"#;
    assert_eq!(expected, format!("{:?}", actual));
}

#[test]
fn expr_float() {
    let actual = cmm::parse_expr(&mut vec![], r#"0.0"#).unwrap();
    let expected = r#"0.0"#;
    assert_eq!(expected, format!("{:?}", actual));
}
