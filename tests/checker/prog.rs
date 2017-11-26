extern crate semic;

#[test]
fn prog_empty() {
    let ast = semic::parse_prog(r#""#).unwrap();

    let actual = semic::check_prog(&ast);

    assert!(actual.is_err());
}

#[test]
fn prog_main() {
    let ast = semic::parse_prog(r#"int main () {}"#).unwrap();

    let actual = semic::check_prog(&ast);

    assert!(actual.is_ok());
}
