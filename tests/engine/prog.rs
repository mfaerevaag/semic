extern crate cmm;

use cmm::engine;

#[test]
fn empty() {
    let ast = cmm::parse(&mut vec![], r#"
int main () {

}
"#).unwrap();

    let actual = engine::run_prog(&ast).unwrap();

    let expected = None;

    assert_eq!(expected, actual);
}
