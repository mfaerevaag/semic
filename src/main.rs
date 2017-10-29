extern crate cmm;

fn main() {
    let mut parser_errors = Vec::new();
    let mut checker_errors = Vec::new();

    let ast = cmm::parse(&mut parser_errors, r#"

int main(void)
{
    char c, s[2];
    c = 'a';
    s = "a";
    return 0;
}

"#).unwrap();

    println!("AST: {:#?}", &ast);

    match cmm::check(&mut checker_errors, &ast) {
        Ok(()) => println!("Checker: OK"),
        Err(()) => println!("Checker: Failed"),
    };
}
