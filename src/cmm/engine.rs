use ast::*;
use env::{FuncTab, SymTab, SymVal};
use checker;

pub fn run_prog<'input>(ast: &'input CProg<'input>) -> Result<Option<SymVal>, ()> {
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
            CStmt::Assign(_, id, i, ref e) => {
                let cloned = main_symtab.clone();
                main_symtab.set_val(id, i, run_expr(e, &vtab, &global_symtab, &cloned));
            },
            CStmt::Return(_, ref s) => match s {
                &Some(ref e) => return Ok(Some(run_expr(e, &vtab, &global_symtab, &main_symtab))),
                _ => return Ok(None),
            },
            _ => println!("TODO: run {:?}", stmt),
        }
    }

    Ok(None)
}

fn run_expr<'input>(
    expr: &'input CExpr<'input>,
    vtab: &'input FuncTab<'input>,
    global_symtab: &'input SymTab<'input>,
    local_symtab: &'input SymTab<'input>,
) -> SymVal {
    match expr {
        &CExpr::Num(n) => SymVal::Num(n),
        &CExpr::Str(ref s) => SymVal::Str(s.clone()),
        &CExpr::Char(c) => SymVal::Char(c),
        &CExpr::Ident(id) => match local_symtab.get_val(id) {
            Some(v) => v,
            _ => match global_symtab.get_val(id) {
                Some(v) => v,
                _ => panic!("variable '{}' not initialized", id),
            }
        },

        &CExpr::UnOp(op, ref e) => {
            let v = run_expr(e, vtab, global_symtab, local_symtab);
            match op {
                COp::Not => match v {
                    SymVal::Bool(b) => SymVal::Bool(!b),
                    v => panic!("cannot negate {:?}", v),
                },
                COp::Neg => match v {
                    SymVal::Num(b) => SymVal::Num(-b),
                    v => panic!("cannot negate {:?}", v),
                },
                _ => panic!("unsupported unary operator {:?}", op),
            }
        },
        &CExpr::BinOp(op, ref e1, ref e2) => {
            let n1 = match run_expr(e1, vtab, global_symtab, local_symtab) {
                SymVal::Num(n) => n,
                x => panic!("expected number, got {:?}", x),
            };
            let n2 = match run_expr(e2, vtab, global_symtab, local_symtab) {
                SymVal::Num(n) => n,
                x => panic!("expected number, got {:?}", x),
            };
            match op {
                COp::Mul => SymVal::Num(n1 * n2),
                COp::Div => SymVal::Num(n1 / n2),
                COp::Add => SymVal::Num(n1 + n2),
                COp::Sub => SymVal::Num(n1 - n2),
                _ => panic!("unsupported binary arithmetic operator {:?}", op),
            }
        },
        &CExpr::RelOp(op, ref e1, ref e2) => {
            let n1 = match run_expr(e1, vtab, global_symtab, local_symtab) {
                SymVal::Num(n) => n,
                x => panic!("expected number, got {:?}", x),
            };
            let n2 = match run_expr(e2, vtab, global_symtab, local_symtab) {
                SymVal::Num(n) => n,
                x => panic!("expected number, got {:?}", x),
            };
            match op {
                COp::Neq => SymVal::Bool(n1 != n2),
                COp::Eq => SymVal::Bool(n1 == n2),
                COp::Lt => SymVal::Bool(n1 < n2),
                COp::Lte => SymVal::Bool(n1 <= n2),
                COp::Gt => SymVal::Bool(n1 > n2),
                COp::Gte => SymVal::Bool(n1 >= n2),
                _ => panic!("unsupported binary relational operator {:?}", op),
            }
        },
        &CExpr::LogOp(op, ref e1, ref e2) => {
            let b1 = match run_expr(e1, vtab, global_symtab, local_symtab) {
                SymVal::Bool(b) => b,
                x => panic!("expected bool, got {:?}", x),
            };
            let b2 = match run_expr(e2, vtab, global_symtab, local_symtab) {
                SymVal::Bool(b) => b,
                x => panic!("expected bool, got {:?}", x),
            };
            match op {
                COp::And => SymVal::Bool(b1 && b2),
                COp::Or => SymVal::Bool(b1 || b2),
                _ => panic!("unsupported binary logical operator {:?}", op),
            }
        },

        &CExpr::Call(id, ref params) => {
            let f = match vtab.get_func(id) {
                Some(f) => f,
                None => panic!("function '{}' not initialized", id),
            };
            let args: Vec<SymVal> = params.iter().map(|x| run_expr(x, vtab, global_symtab, local_symtab)).collect();
            panic!("TODO: run func '{}' with args '{:?}'", id, args)
        },

        &CExpr::Index(id, ref e) => {
            let sym = match local_symtab.get_val(id) {
                Some(v) => v,
                _ => match global_symtab.get_val(id) {
                    Some(v) => v,
                    _ => panic!("variable '{}' not initialized", id),
                }
            };
            let a = match sym {
                SymVal::Array(a) => a,
                x => panic!("expected array, got {:?}", x),
            };

            // get index
            let i = match run_expr(e, vtab, global_symtab, local_symtab) {
                SymVal::Num(n) => n,
                x => panic!("expected array index, got {:?}", x),
            };

            // check bounds
            if i >= (a.len() as i32) {
                panic!("index {} out of bounds (range: {})", i, a.len())
            };

            (*a[i as usize]).clone()
        },

        &CExpr::Error => panic!("unexpected Error expr"),
    }
}

fn print_errors(errors: &Vec<checker::CheckErr>) {
    println!("checker failed:");
    for err in errors.iter() {
        println!("{:?}", err);
    }
}
