extern crate cmm;

use cmm::ast::*;

#[test]
fn empty() {
    let mut errors = Vec::new();

    let ast = cmm::parse(&mut vec![], r#""#).unwrap();

    let actual = cmm::check(&mut errors, &ast);

    assert!(errors.is_empty());
    assert!(actual.is_ok());
}
