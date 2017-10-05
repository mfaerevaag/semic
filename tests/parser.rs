extern crate cmm;

#[test]
fn test_parser() {
    let mut errors = Vec::new();

    let prog = "int main ( void ) { int x; x = 7; }";
    let res = cmm::parse(&mut errors, prog).unwrap();

    assert_eq!("(Some(int), \"main\", [], [(int, [\"x\"])], [\"x\" = 7])", format!("{:?}", res));
}
