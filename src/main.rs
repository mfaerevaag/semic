extern crate cmm;

fn main() {
    let mut errors = Vec::new();

    let ast = cmm::parse(&mut errors, r#"


int main (void)
{
    int x[2];
    return - 1 + - x[1];
}

"#).unwrap();

    println!("AST: {:#?}", &ast);

    match cmm::analyzer::check(&ast) {
        Ok(()) => println!("Check OK"),
        Err(s) => println!("Check Error: {:?}", s),
    };
}
