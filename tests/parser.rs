extern crate cmm;

#[test]
fn parser_empty() {
    let mut errors = Vec::new();

    let prog = r#"
void main ( void ) {}
"#;
    let res = cmm::parse(&mut errors, prog).unwrap();

    assert_eq!(r#"(None, "main", [], [], [])"#, format!("{:?}", res));
}

#[test]
fn parser_return_type() {
    let mut errors = Vec::new();

    let prog = r#"
int main ( void ) {}
"#;
    let res = cmm::parse(&mut errors, prog).unwrap();

    assert_eq!(r#"(Some(int), "main", [], [], [])"#, format!("{:?}", res));
}

#[test]
fn parser_param_type_single() {
    let mut errors = Vec::new();

    let prog = r#"
int main ( int a ) {}
"#;
    let res = cmm::parse(&mut errors, prog).unwrap();

    assert_eq!(r#"(Some(int), "main", [(int, "a")], [], [])"#, format!("{:?}", res));
}

#[test]
fn parser_param_type_mult() {
    let mut errors = Vec::new();

    let prog = r#"
int main ( int a, char b ) {}
"#;
    let res = cmm::parse(&mut errors, prog).unwrap();

    assert_eq!(r#"(Some(int), "main", [(int, "a"), (char, "b")], [], [])"#, format!("{:?}", res));
}

#[test]
fn parser_decl_single_type_single_ident() {
    let mut errors = Vec::new();

    let prog = r#"
int main ( void ) {
    int x;
}
"#;
    let res = cmm::parse(&mut errors, prog).unwrap();

    assert_eq!(r#"(Some(int), "main", [], [(int, ["x"])], [])"#, format!("{:?}", res));
}

#[test]
fn parser_decl_single_type_mult_ident() {
    let mut errors = Vec::new();

    let prog = r#"
int main ( void ) {
    int x, y;
}
"#;
    let res = cmm::parse(&mut errors, prog).unwrap();

    assert_eq!(r#"(Some(int), "main", [], [(int, ["x", "y"])], [])"#, format!("{:?}", res));
}

#[test]
fn parser_decl_mult_type_single_ident() {
    let mut errors = Vec::new();

    let prog = r#"
int main ( void ) {
    int x;
    char y;
}
"#;
    let res = cmm::parse(&mut errors, prog).unwrap();

    assert_eq!(r#"(Some(int), "main", [], [(int, ["x"]), (char, ["y"])], [])"#, format!("{:?}", res));
}

#[test]
fn parser_decl_mult_type_mult_ident() {
    let mut errors = Vec::new();

    let prog = r#"
int main ( void ) {
    int x, y;
    char a, b;
}
"#;
    let res = cmm::parse(&mut errors, prog).unwrap();

    assert_eq!(r#"(Some(int), "main", [], [(int, ["x", "y"]), (char, ["a", "b"])], [])"#, format!("{:?}", res));
}

#[test]
fn parser_no_decl_single_stmt() {
    let mut errors = Vec::new();

    let prog = r#"
int main ( void ) {
    x = 1;
}
"#;
    let res = cmm::parse(&mut errors, prog).unwrap();

    assert_eq!(r#"(Some(int), "main", [], [], ["x" = 1])"#, format!("{:?}", res));
}

#[test]
fn parser_single_decl_single_stmt() {
    let mut errors = Vec::new();

    let prog = r#"
int main ( void ) {
    int x;
    x = 1;
}
"#;
    let res = cmm::parse(&mut errors, prog).unwrap();

    assert_eq!(r#"(Some(int), "main", [], [(int, ["x"])], ["x" = 1])"#, format!("{:?}", res));
}

#[test]
fn parser_stmt_mult() {
    let mut errors = Vec::new();

    let prog = r#"
int main ( void ) {
    int x, y;
    x = 1;
    y = 2;
}
"#;
    let res = cmm::parse(&mut errors, prog).unwrap();

    assert_eq!(r#"(Some(int), "main", [], [(int, ["x", "y"])], ["x" = 1, "y" = 2])"#, format!("{:?}", res));
}
