extern crate lalrpop_util;

pub mod ast;
pub mod parser;

#[cfg(not(test))]
fn main() {
    let mut errors = Vec::new();
    let res = parser::parse_Prog(&mut errors, "int main ( void ) { int x; x = 7; }").unwrap();

    println!("{:?}", res);
}

#[test]
fn parse_prog() {
    let mut errors = Vec::new();

    assert_eq!(&format!("{:?}", parser::parse_Prog(&mut errors, "").unwrap()),
               "[]");
    assert_eq!(&format!("{:?}", parser::parse_Prog(&mut errors, "22 * x + 66").unwrap()),
               "[((22 * x) + 66)]");
    assert_eq!(&format!("{:?}", parser::parse_Prog(&mut errors, "22 * x + 66,").unwrap()),
               "[((22 * x) + 66)]");
    assert_eq!(&format!("{:?}", parser::parse_Prog(&mut errors, "22 * x + 66, 13*3").unwrap()),
               "[((22 * x) + 66), (13 * 3)]");
    assert_eq!(&format!("{:?}", parser::parse_Prog(&mut errors, "22 + x * 66, 13*3,").unwrap()),
               "[(22 + (x * 66)), (13 * 3)]");
}

#[test]
fn parse_error() {
    let mut errors = Vec::new();

    assert_eq!(&format!("{:?}", parser::parse_Prog(&mut errors, "22 * + 3").unwrap()),
               "[((22 * error) + 3)]");
    assert_eq!(&format!("{:?}", parser::parse_Prog(&mut errors, "22 * 44 + 66, *3").unwrap()),
               "[((22 * 44) + 66), (error * 3)]");
    assert_eq!(&format!("{:?}", parser::parse_Prog(&mut errors, "*").unwrap()),
               "[(error * error)]");

    assert_eq!(errors.len(), 4);
}
