extern crate semic;

use semic::engine;
use semic::env::SymVal;

#[test]
fn empty() {
    let prog = r#"
    void main () {
    }
    "#;

    let ast = semic::parse_prog(prog).unwrap();

    let actual = engine::run_prog(&ast, prog, &vec![], false, false);
    assert!(actual.is_ok());

    assert_eq!(None, actual.unwrap());
}

#[test]
fn int() {
    let prog =r#"
    int main () {
        return 0;
    }
    "#;

    let ast = semic::parse_prog(prog).unwrap();

    let actual = engine::run_prog(&ast, prog, &vec![], false, false);
    assert!(actual.is_ok());

    assert_eq!(Some(SymVal::Int(0)), actual.unwrap());
}

#[test]
fn argc() {
    let prog = r#"
    int main (int argc, char argv[][]) {
        return argc;
    }
    "#;

    let ast = semic::parse_prog(prog).unwrap();

    let actual = engine::run_prog(&ast, prog, &vec![], false, false);
    assert!(actual.is_ok());

    assert_eq!(Some(SymVal::Int(0)), actual.unwrap());
}
