extern crate cmm;

use cmm::ast::*;

#[test]
fn prog_empty() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#""#).unwrap();

    assert_eq!(0, errors.len());
    assert_eq!(format!("[]"), format!("{:?}", actual));
}

#[test]
fn prog_single_proto() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int foo (void);
    "#).unwrap();

    let expected = CProto {
        ret: Some(CType::Int),
        name: "foo",
        params: vec![],
    };

    assert_eq!(0, errors.len());
    assert_eq!(format!("{:?}", vec![expected]), format!("{:?}", actual));
}
