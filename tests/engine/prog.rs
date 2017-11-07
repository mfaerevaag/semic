extern crate cmm;

use cmm::engine;
use cmm::env::SymVal;

#[test]
fn empty() {
    let ast = cmm::parse(&mut vec![], r#"
void main () {

}
"#).unwrap();

    let actual = engine::run_prog(&ast).unwrap();

    let expected = None;

    assert_eq!(expected, actual);
}

#[test]
fn int() {
    let ast = cmm::parse(&mut vec![], r#"
int main () {
return 0;
}
"#).unwrap();

    let actual = engine::run_prog(&ast).unwrap();

    let expected = Some(SymVal::Num(0));

    assert_eq!(expected, actual);
}

#[test]
fn argc() {
    let ast = cmm::parse(&mut vec![], r#"
int main (int argc, char argv[][]) {
return argc;
}
"#).unwrap();

    let actual = engine::run_prog(&ast).unwrap();

    let expected = Some(SymVal::Num(0));

    assert_eq!(expected, actual);
}
