use ast::*;
use env::{FuncTab, SymTab};
use checker;

pub fn run_prog<'input>(ast: &'input CProg<'input>) -> Result<(), ()> {
    // tables
    let mut vtab = FuncTab::new();
    let mut global_symtab = SymTab::new();

    match checker::analyze_prog(&ast) {
        Ok((v, s)) => {
            for (k, v) in v.iter() { vtab.insert(k, *v); }
            for (k, v) in s.iter() { global_symtab.insert(k, *v); }
        },
        Err(ref e) => {
            print_errors(e);
            return Err(());
        },
    };

    // get main function
    let main = vtab.get_func("main").unwrap();

    // read symbols
    match checker::analyze_func(&main) {
        Ok(s) => {
            for (k, v) in s.iter() { global_symtab.insert(k, *v); }
        },
        Err(ref e) => {
            print_errors(e);
            return Err(());
        },
    };

    // check each element
    for stmt in main.stmts.iter() {
        println!("TODO: run {:?}", stmt);
    }

    Ok(())
}

fn print_errors(errors: &Vec<checker::CheckErr>) {
    println!("checker failed:");
    for err in errors.iter() {
        println!("{:?}", err);
    }
}
