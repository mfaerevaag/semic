use std::env;
use ast::*;
use env::{FuncTab, SymTab, SymVal};
use checker;

pub fn run_prog<'input>(ast: &'input CProg<'input>) -> Result<Option<SymVal>, ()> {
    // tables
    let mut vtab = FuncTab::new();
    let mut global_symtab = SymTab::new();

    // load global function and symbol table
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

    // load command line args
    let mut local_symtab = SymTab::new();
    // argc
    let argc = env::args().len() as i32 - 1;
    local_symtab.insert("argc", (CType::Int, None, Some(SymVal::Num(argc))));
    // argv
    let mut argv = vec![];
    for (i, arg) in env::args().enumerate() {
        if i < 1 { continue };

        // create internal string
        let mut s = Vec::with_capacity(arg.len() + 1);
        for c in arg.chars() {
            s.push(Box::new(SymVal::Char(c)));
        }
        // add null char
        s.push(Box::new(SymVal::Char('\0')));

        argv.push(Box::new(SymVal::Array(s)));
    }
    local_symtab.insert("argv", (CType::Ref(Box::new(CType::Ref(Box::new(CType::Char)))),
                                 None,
                                 Some(SymVal::Array(argv))));
    // run
    let res = run_func(main, &vtab, &global_symtab, local_symtab);

    Ok(res)
}

pub fn run_func<'input>(
    func: &'input CFunc<'input>,
    vtab: &'input FuncTab<'input>,
    global_symtab: &'input SymTab<'input>,
    local_symtab: SymTab<'input>,
) -> Option<SymVal>
{
    let mut tmp_symtab = local_symtab.clone();
    // load decls
    for decl in func.decls.iter() {
        let (ref t, ref id, s) = *decl;
        tmp_symtab.insert(id, (t.clone(), s, None));
    }

    // wrap statements in block
    let block = CStmt::Block((0,0), func.stmts.iter().map(|x| Box::new(x.clone())).collect());

    // run block
    let (_, ret) = run_stmt(&block, vtab, global_symtab, tmp_symtab);

    // unwrap return val
    match ret {
        Some(x) => x,
        None => None,
    }
}

pub fn run_stmt<'input>(
    stmt: &'input CStmt<'input>,
    vtab: &'input FuncTab<'input>,
    global_symtab: &'input SymTab<'input>,
    local_symtab: SymTab<'input>,
) -> (SymTab<'input>, Option<Option<SymVal>>)
{
    let mut tmp_symtab = local_symtab.clone();

    let res = match *stmt {
        CStmt::Assign(_, id, i, ref e) => {
            let val = run_expr(e, vtab, global_symtab, &tmp_symtab);
            tmp_symtab.set_val(id, i, val);
            None
        },
        CStmt::Return(_, ref s) => match s {
            &Some(ref e) => Some(Some(run_expr(e, vtab, global_symtab, &tmp_symtab))),
            _ => Some(None),
        },
        CStmt::Block(_, ref stmts) => {
            let mut res = None;
            for s in stmts.iter() {
                let (tab, res2) = run_stmt(s, vtab, global_symtab, tmp_symtab);
                tmp_symtab = tab;
                match res2 {
                    Some(_) => {
                        res = res2.clone();
                        break
                    },
                    _ => res2
                };
            }
            res
        },
        CStmt::If(_, ref cond, ref s, ref o) => {
            let b = match run_expr(cond, vtab, global_symtab, &tmp_symtab) {
                SymVal::Bool(b) => b,
                x => panic!("expected bool, got {:?}", x),
            };
            if b {
                let (tab, res) = run_stmt(s, vtab, global_symtab, tmp_symtab);
                tmp_symtab = tab;
                res
            } else {
                match *o {
                    Some(ref es) => {
                        let (tab, res) = run_stmt(es, vtab, global_symtab, tmp_symtab);
                        tmp_symtab = tab;
                        res
                    },
                    _ => None
                }
            }
        },
        CStmt::While(_, ref cond, ref s) => {
            let b = match run_expr(cond, vtab, global_symtab, &tmp_symtab) {
                SymVal::Bool(b) => b,
                x => panic!("expected bool, got {:?}", x),
            };
            if b {
                let (tab, res) = run_stmt(s, vtab, global_symtab, local_symtab);
                match res {
                    Some(_) => res,
                    _ => {
                        let (tab2, res2) = run_stmt(stmt, vtab, global_symtab, tab);
                        tmp_symtab = tab2;
                        res2
                    }
                }
            } else {
                None
            }
        },
        CStmt::Print(_, ref e) => {
            let val = run_expr(e, vtab, global_symtab, &tmp_symtab);
            println!("{:?}", val);
            None
        },
        _ => panic!("unexpected token '{:?}' in ast", stmt)
    };

    (tmp_symtab, res)
}

pub fn run_expr<'input>(
    expr: &'input CExpr<'input>,
    vtab: &'input FuncTab<'input>,
    global_symtab: &'input SymTab<'input>,
    local_symtab: &'input SymTab<'input>,
) -> SymVal
{
    match *expr {
        CExpr::Num(n) => SymVal::Num(n),
        CExpr::Str(ref s) => {
            let mut arr = Vec::with_capacity(s.as_str().len() + 1);
            for c in s.clone() {
                arr.push(Box::new(SymVal::Char(c)));
            }
            // add null char
            arr.push(Box::new(SymVal::Char('\0')));
            SymVal::Array(arr)
        },
        CExpr::Char(c) => SymVal::Char(c),
        CExpr::Ident(id) => match local_symtab.get_val(id) {
            Some(v) => v,
            _ => match global_symtab.get_val(id) {
                Some(v) => v,
                _ => panic!("variable '{}' not initialized", id),
            }
        },

        CExpr::UnOp(op, ref e) => {
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
        CExpr::BinOp(op, ref e1, ref e2) => {
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
        CExpr::RelOp(op, ref e1, ref e2) => {
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
        CExpr::LogOp(op, ref e1, ref e2) => {
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

        CExpr::Call(id, ref args) => {
            // get func
            let f = match vtab.get_func(id) {
                Some(f) => f,
                None => panic!("function '{}' not initialized", id),
            };

            // calc and add args to symtab
            let mut tab = SymTab::new();
            for (i, p) in f.proto.params.iter().enumerate() {
                let (ref t, ref id) = *p;
                let e = args.iter().nth(i).unwrap();
                let val = run_expr(e, vtab, global_symtab, local_symtab);
                tab.insert(id, (t.clone(), None, Some(val)));
            }

            match run_func(&f, vtab, global_symtab, tab) {
                Some(v) => v,
                None => panic!("expression returned void"),
            }
        },

        CExpr::Index(id, ref e) => {
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

        CExpr::Error => panic!("unexpected Error expr"),
    }
}

fn print_errors(errors: &Vec<checker::CheckErr>) {
    println!("checker failed:");
    for err in errors.iter() {
        println!("{:?}", err);
    }
}
