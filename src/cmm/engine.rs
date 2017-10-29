use parser;
use checker;

pub fn run<'input>(prog: &'input str) -> Result<(), ()> {
    let mut parser_errors = Vec::new();
    let mut checker_errors = Vec::new();

    let ast = match parser::parse_Prog(&mut parser_errors, prog) {
        Ok(ast) => ast,
        Err(err) => {
            println!("{:?}", err);
            println!("parse errors:");
            for err in parser_errors.iter() {
                println!("{:?}", err);
            };
            return Err(());
        }
    };

    println!("ast: {:#?}", &ast);

    match checker::check_prog(&mut checker_errors, &ast) {
        Ok(()) => (),
        Err(()) => {
            println!("checker failed:");
            for err in checker_errors.iter() {
                println!("{:?}", err);
            };
            return Err(());
        },
    };

    // run
    Ok(())
}
