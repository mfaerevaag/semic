extern crate lalrpop_util;

pub mod cmm;
pub mod ast;

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}

#[test]
fn parse_prog() {
    let mut errors = Vec::new();

    assert_eq!(&format!("{:?}", cmm::parse_Prog(&mut errors, "").unwrap()),
               "[]");
    assert_eq!(&format!("{:?}", cmm::parse_Prog(&mut errors, "22 * 44 + 66").unwrap()),
               "[((22 * 44) + 66)]");
    assert_eq!(&format!("{:?}", cmm::parse_Prog(&mut errors, "22 * 44 + 66,").unwrap()),
               "[((22 * 44) + 66)]");
    assert_eq!(&format!("{:?}", cmm::parse_Prog(&mut errors, "22 * 44 + 66, 13*3").unwrap()),
               "[((22 * 44) + 66), (13 * 3)]");
    assert_eq!(&format!("{:?}", cmm::parse_Prog(&mut errors, "22 + 44 * 66, 13*3,").unwrap()),
               "[(22 + (44 * 66)), (13 * 3)]");
}

#[test]
fn parse_error() {
    let mut errors = Vec::new();

    assert_eq!(&format!("{:?}", cmm::parse_Prog(&mut errors, "22 * + 3").unwrap()),
               "[((22 * error) + 3)]");
    assert_eq!(&format!("{:?}", cmm::parse_Prog(&mut errors, "22 * 44 + 66, *3").unwrap()),
               "[((22 * 44) + 66), (error * 3)]");
    assert_eq!(&format!("{:?}", cmm::parse_Prog(&mut errors, "*").unwrap()),
               "[(error * error)]");

    assert_eq!(errors.len(), 4);
}
