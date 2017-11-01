use ast::*;
use env::{FuncTab, SymTab, SymVal};
use checker;

pub fn run_prog<'input>(ast: &'input CProg<'input>) -> Result<Option<i32>, ()> {
    // tables
    let mut vtab = FuncTab::new();
    let mut global_symtab = SymTab::new();

    match checker::analyze_prog(&ast) {
        Ok((v, s)) => {
            for (k, v) in v.iter() { vtab.insert(k, *v); }
            for (k, v) in s.iter() { global_symtab.insert(k, v.clone()); }
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
            for (k, v) in s.iter() { global_symtab.insert(k, v.clone()); }
        },
        Err(ref e) => {
            print_errors(e);
            return Err(());
        },
    };

    // load decls
    let mut main_symtab = SymTab::new();
    for decl in main.decls.iter() {
        let (ref t, ref id, s) = *decl;
        main_symtab.insert(id, (t.clone(), s, None));
    }

    // check each element
    for stmt in main.stmts.iter() {
        match *stmt {
            CStmt::Assign(_, id, ref e) => {
                main_symtab.set_val(id, run_expr(e));
            },

            _ => println!("TODO: run {:?}", stmt),
        }
    }

    Ok(None)
}

fn run_expr<'input>(expr: &'input CExpr<'input>) -> SymVal {
    match *expr {
        CExpr::Num(n) => SymVal::Num(n),
        _ => panic!("TODO: run expr")
    }
}

fn print_errors(errors: &Vec<checker::CheckErr>) {
    println!("checker failed:");
    for err in errors.iter() {
        println!("{:?}", err);
    }
}
