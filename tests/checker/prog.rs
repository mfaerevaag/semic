extern crate cmm;

#[test]
fn prog_empty() {
    let ast = cmm::parse_prog(r#""#).unwrap();

    let actual = cmm::check_prog(&ast);

    assert!(actual.is_err());
}

#[test]
fn prog_main() {
    let ast = cmm::parse_prog(r#"int main () {}"#).unwrap();

    let actual = cmm::check_prog(&ast);

    assert!(actual.is_ok());
}
