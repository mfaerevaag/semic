extern crate cmm;

// use cmm::ast::*;

#[test]
fn prog_empty() {
    let ast = cmm::parse(&mut vec![], r#""#).unwrap();

    let actual = cmm::check(&ast);

    assert!(actual.is_err());
}

#[test]
fn prog_main() {
    let ast = cmm::parse(&mut vec![], r#"int main () {}"#).unwrap();

    let actual = cmm::check(&ast);

    assert!(actual.is_ok());
}
