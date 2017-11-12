use std::env;
use ast::*;
use env::{FuncTab, SymTab, SymVal};
use checker;
use error::CError;

pub fn run_prog<'input>(ast: &'input CProg<'input>) -> Result<Option<SymVal>, CError> {
    // tables
    let mut vtab = FuncTab::new();
    let mut global_symtab = SymTab::new();

    // load global function and symbol table
    match checker::analyze_prog(&ast) {
        Ok((v, s)) => {
            for (k, v) in v.iter() { vtab.insert(k, *v); }
            for (k, v) in s.iter() { global_symtab.insert(k, v.clone()); }
        },
        Err(e) => {
            return Err(e);
        },
    };

    // get main function
    let main = vtab.get_func("main").unwrap();

    // load command line args
    let mut local_symtab = SymTab::new();
    // argc
    let argc = env::args().len() as i32 - 1;
    local_symtab.insert("argc", (CType::Int, None, Some(SymVal::Int(argc))));
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
    run_func(main, &vtab, &global_symtab, local_symtab)
}

pub fn run_func<'input>(
    func: &'input CFunc<'input>,
    vtab: &'input FuncTab<'input>,
    global_symtab: &'input SymTab<'input>,
    local_symtab: SymTab<'input>,
) -> Result<Option<SymVal>, CError>
{
    // wrap statements in block
    let block = CStmt::Block((0,0), func.stmts.iter().map(|x| Box::new(x.clone())).collect());

    // run block
    let (_, ret) = try!(run_stmt(&block, vtab, global_symtab, local_symtab));

    // unwrap return val
    match ret {
        Some(x) => Ok(x),
        None => Ok(None),
    }
}

pub fn run_stmt<'input>(
    stmt: &'input CStmt<'input>,
    vtab: &'input FuncTab<'input>,
    global_symtab: &'input SymTab<'input>,
    local_symtab: SymTab<'input>,
) -> Result<(SymTab<'input>, Option<Option<SymVal>>), CError>
{
    let mut tmp_symtab = local_symtab.clone();

    let res = match *stmt {
        CStmt::Assign(_, id, i, ref e) => {
            let val = try!(run_expr(e, vtab, global_symtab, &tmp_symtab));
            tmp_symtab.set_val(id, i, val);
            None
        },
        CStmt::Decl(_, ref t, id, s) => {
            tmp_symtab.insert(id, (t.clone(), s, None));
            None
        },
        CStmt::Call((l, _), id, ref args) => {
            // get func
            let f = match vtab.get_func(id) {
                Some(f) => f,
                None => return Err(CError::RuntimeError(format!("Function '{}' not initialized", id), l)),
            };

            // calc and add args to symtab
            let mut tab = SymTab::new();
            for (i, p) in f.proto.params.iter().enumerate() {
                let (ref t, ref pid) = *p;
                let e = match args.iter().nth(i) {
                    Some(x) => x,
                    None => return Err(CError::RuntimeError(format!("Function '{}' missing param '{:?}'", id, p), l)),
                };
                let val = try!(run_expr(e, vtab, global_symtab, &tmp_symtab));
                tab.insert(pid, (t.clone(), None, Some(val)));
            }

            let _ = try!(run_func(&f, vtab, global_symtab, tab));
            None
        },
        CStmt::Return(_, ref s) => match s {
            &Some(ref e) => Some(Some(try!(run_expr(e, vtab, global_symtab, &tmp_symtab)))),
            _ => Some(None),
        },
        CStmt::Block(_, ref stmts) => {
            let mut res = None;
            for s in stmts.iter() {
                let (tab, res2) = try!(run_stmt(s, vtab, global_symtab, tmp_symtab));
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
        CStmt::If((l, _), ref cond, ref s, ref o) => {
            let b = match try!(run_expr(cond, vtab, global_symtab, &tmp_symtab)) {
                SymVal::Bool(b) => b,
                x => return Err(CError::RuntimeError(format!("Expected bool, got {:?}", x), l)),
            };
            if b {
                let (tab, res) = try!(run_stmt(s, vtab, global_symtab, tmp_symtab));
                tmp_symtab = tab;
                res
            } else {
                match *o {
                    Some(ref es) => {
                        let (tab, res) = try!(run_stmt(es, vtab, global_symtab, tmp_symtab));
                        tmp_symtab = tab;
                        res
                    },
                    _ => None
                }
            }
        },
        CStmt::While((l, _), ref cond, ref s) => {
            let b = match try!(run_expr(cond, vtab, global_symtab, &tmp_symtab)) {
                SymVal::Bool(b) => b,
                x => return Err(CError::RuntimeError(format!("Expected bool, got {:?}", x), l)),
            };
            if b {
                let (tab, res) = try!(run_stmt(s, vtab, global_symtab, local_symtab));
                match res {
                    Some(_) => res,
                    _ => {
                        let (tab2, res2) = try!(run_stmt(stmt, vtab, global_symtab, tab));
                        tmp_symtab = tab2;
                        res2
                    }
                }
            } else {
                None
            }
        },
        CStmt::Print(_, ref e) => {
            let val = try!(run_expr(e, vtab, global_symtab, &tmp_symtab));
            println!("{:?}", val);
            None
        },
        _ => panic!("unexpected stmt '{:?}' in ast", stmt)
    };

    Ok((tmp_symtab, res))
}

pub fn run_expr<'input>(
    expr: &'input CExpr<'input>,
    vtab: &'input FuncTab<'input>,
    global_symtab: &'input SymTab<'input>,
    local_symtab: &'input SymTab<'input>,
) -> Result<SymVal, CError>
{
    let res = match *expr {
        CExpr::Int((_, _), i) => SymVal::Int(i),
        CExpr::Float((_, _), f) => SymVal::Float(f),
        CExpr::Str((_, _), ref s) => {
            let mut arr = Vec::with_capacity(s.as_str().len() + 1);
            for c in s.clone() {
                arr.push(Box::new(SymVal::Char(c)));
            }
            // add null char
            arr.push(Box::new(SymVal::Char('\0')));
            SymVal::Array(arr)
        },
        CExpr::Char((_, _), c) => SymVal::Char(c),
        CExpr::Ident((l, _), id) => match local_symtab.get_val(id) {
            Some(v) => v,
            _ => match global_symtab.get_val(id) {
                Some(v) => v,
                _ => return Err(CError::RuntimeError(format!("Variable '{}' not initialized", id), l)),
            }
        },

        CExpr::UnOp((l, _), op, ref e) => {
            let v = try!(run_expr(e, vtab, global_symtab, local_symtab));
            match op {
                COp::Not => match v {
                    SymVal::Int(b) => SymVal::Bool(b != 0),
                    SymVal::Bool(b) => SymVal::Bool(!b),
                    v => return Err(CError::RuntimeError(format!("Cannot negate {:?}", v), l)),
                },
                COp::Neg => match v {
                    SymVal::Int(n) => SymVal::Int(-n),
                    SymVal::Float(n) => SymVal::Float(-n),
                    v => return Err(CError::RuntimeError(format!("Cannot negate {:?}", v), l)),
                },
                _ => return Err(CError::RuntimeError(format!("Unsupported unary operator {:?}", op), l)),
            }
        },
        CExpr::BinOp((l, _), op, ref e1, ref e2) => {
            let v1 = try!(run_expr(e1, vtab, global_symtab, local_symtab));
            let (is_num1, is_int1, i1, is_float1, f1, is_bool1, b1) =
                match v1 {
                    SymVal::Int(x)   => (true,  true,  x, false, 0f32, false, false),
                    SymVal::Float(x) => (true,  false, 0, true,  x,    false, false),
                    SymVal::Bool(x)  => (false, false, 0, false, 0f32, true,  x),
                    _ => return Err(CError::RuntimeError(format!("Unexpected '{:?}' in binary op", v1), l)),
                };
            let v2 = try!(run_expr(e2, vtab, global_symtab, local_symtab));
            let (is_num2, is_int2, i2, is_float2, f2, is_bool2, b2) =
                match v2 {
                    SymVal::Int(x)   => (true,  true,  x, false, 0f32, false, false),
                    SymVal::Float(x) => (true,  false, 0, true,  x,    false, false),
                    SymVal::Bool(x)  => (false, false, 0, false, 0f32, true,  x),
                    _ => return Err(CError::RuntimeError(format!("Unexpected '{:?}' in binary op", v2), l)),
                };

            match op {
                COp::Add => match (is_num1, is_num2) {
                    (true, true) => match (is_int1, is_int2) {
                        (true, true) => SymVal::Int(i1 + i2),
                        (false, true) => SymVal::Float(f1 + i2 as f32),
                        (true, false) => SymVal::Float(i1 as f32 + f2),
                        (false, false) => SymVal::Float(f1 + f2),
                    },
                    _ => return Err(CError::RuntimeError(format!("`+` op expected numbers, got '{:?}' and '{:?}'", v1, v2), l)),
                },
                COp::Sub => match (is_num1, is_num2) {
                    (true, true) => match (is_int1, is_int2) {
                        (true, true) => SymVal::Int(i1 - i2),
                        (false, true) => SymVal::Float(f1 - i2 as f32),
                        (true, false) => SymVal::Float(i1 as f32 - f2),
                        (false, false) => SymVal::Float(f1 - f2),
                    },
                    _ => return Err(CError::RuntimeError(format!("`-` op expected numbers, got '{:?}' and '{:?}'", v1, v2), l)),
                },
                COp::Mul => match (is_num1, is_num2) {
                    (true, true) => match (is_int1, is_int2) {
                        (true, true) => SymVal::Int(i1 * i2),
                        (false, true) => SymVal::Float(f1 * i2 as f32),
                        (true, false) => SymVal::Float(i1 as f32 * f2),
                        (false, false) => SymVal::Float(f1 * f2),
                    },
                    _ => return Err(CError::RuntimeError(format!("`*` op expected numbers, got '{:?}' and '{:?}'", v1, v2), l)),
                },
                COp::Div => match (is_num1, is_num2) {
                    (true, true) => match (is_int1, is_int2) {
                        (true, true) => SymVal::Int(i1 / i2),
                        (false, true) => SymVal::Float(f1 / i2 as f32),
                        (true, false) => SymVal::Float(i1 as f32 / f2),
                        (false, false) => SymVal::Float(f1 / f2),
                    },
                    _ => return Err(CError::RuntimeError(format!("`/` op expected numbers, got '{:?}' and '{:?}'", v1, v2), l)),
                },
                // relational
                COp::Eq => match (is_int1, is_int2, is_float1, is_float2) {
                    (true, true, false, false) => SymVal::Bool(i1 == i2),
                    (false, true, true, false) => SymVal::Bool(f1 == i2 as f32),
                    (true, false, false, true) => SymVal::Bool(i1 as f32 == f2),
                    (false, false, true, true) => SymVal::Bool(f1 == f2),
                    _ => return Err(CError::RuntimeError(format!("`==` op expected pair of numbers, got '{:?}' and '{:?}'", v1, v2), l)),
                },
                COp::Neq => match (is_int1, is_int2, is_float1, is_float2) {
                    (true, true, false, false) => SymVal::Bool(i1 != i2),
                    (false, true, true, false) => SymVal::Bool(f1 != i2 as f32),
                    (true, false, false, true) => SymVal::Bool(i1 as f32 != f2),
                    (false, false, true, true) => SymVal::Bool(f1 != f2),
                    _ => return Err(CError::RuntimeError(format!("`!=` op expected pair of numbers, got '{:?}' and '{:?}'", v1, v2), l)),
                },
                COp::Lt => match (is_int1, is_int2, is_float1, is_float2) {
                    (true, true, false, false) => SymVal::Bool(i1 < i2),
                    (false, true, true, false) => SymVal::Bool(f1 < i2 as f32),
                    (true, false, false, true) => SymVal::Bool((i1 as f32) < f2),
                    (false, false, true, true) => SymVal::Bool(f1 < f2),
                    _ => return Err(CError::RuntimeError(format!("`<` op expected pair of numbers, got '{:?}' and '{:?}'", v1, v2), l)),
                },
                COp::Lte => match (is_int1, is_int2, is_float1, is_float2) {
                    (true, true, false, false) => SymVal::Bool(i1 <= i2),
                    (false, true, true, false) => SymVal::Bool(f1 <= i2 as f32),
                    (true, false, false, true) => SymVal::Bool(i1 as f32 <= f2),
                    (false, false, true, true) => SymVal::Bool(f1 <= f2),
                    _ => return Err(CError::RuntimeError(format!("`<=` op expected pair of numbers, got '{:?}' and '{:?}'", v1, v2), l)),
                },
                COp::Gt => match (is_int1, is_int2, is_float1, is_float2) {
                    (true, true, false, false) => SymVal::Bool(i1 > i2),
                    (false, true, true, false) => SymVal::Bool(f1 > i2 as f32),
                    (true, false, false, true) => SymVal::Bool(i1 as f32 > f2),
                    (false, false, true, true) => SymVal::Bool(f1 > f2),
                    _ => return Err(CError::RuntimeError(format!("`>` op expected pair of numbers, got '{:?}' and '{:?}'", v1, v2), l)),
                },
                COp::Gte => match (is_int1, is_int2, is_float1, is_float2) {
                    (true, true, false, false) => SymVal::Bool(i1 >= i2),
                    (false, true, true, false) => SymVal::Bool(f1 >= i2 as f32),
                    (true, false, false, true) => SymVal::Bool(i1 as f32 >= f2),
                    (false, false, true, true) => SymVal::Bool(f1 >= f2),
                    _ => return Err(CError::RuntimeError(format!("`>=` op expected pair of numbers, got '{:?}' and '{:?}'", v1, v2), l)),
                },
                // logical
                COp::And => match (is_int1, is_int2, is_float1, is_float2, is_bool1, is_bool2) {
                    (true, true, false, false, ..) => SymVal::Bool((i1 != 0) && (i2 != 0)),
                    (false, true, true, false, ..) => SymVal::Bool((f1 != 0.0) && (i2 != 0)),
                    (true, false, false, true, ..) => SymVal::Bool((i1 != 0) && (f2 != 0.0)),
                    (false, false, true, true, ..) => SymVal::Bool((f1 != 0.0) && (f2 != 0.0)),
                    // auto cast
                    (.., false, true) => SymVal::Bool(((i1 as f32 + f1) != 0.0) && b2),
                    (.., true, false) => SymVal::Bool(b1 && ((i1 as f32 + f1) != 0.0)),
                    _ => return Err(CError::RuntimeError(format!("`&&` op expected pair of bools, got '{:?}' and '{:?}'", v1, v2), l)),
                },
                COp::Or => match (is_int1, is_int2, is_float1, is_float2, is_bool1, is_bool2) {
                    (true, true, false, false, ..) => SymVal::Bool((i1 != 0) || (i2 != 0)),
                    (false, true, true, false, ..) => SymVal::Bool((f1 != 0.0) || (i2 != 0)),
                    (true, false, false, true, ..) => SymVal::Bool((i1 != 0) || (f2 != 0.0)),
                    (false, false, true, true, ..) => SymVal::Bool((f1 != 0.0) || (f2 != 0.0)),
                    // auto cast
                    (.., false, true) => SymVal::Bool(((i1 as f32 + f1) != 0.0) || b2),
                    (.., true, false) => SymVal::Bool(b1 || ((i1 as f32 + f1) != 0.0)),
                    _ => return Err(CError::RuntimeError(format!("`&&` op expected pair of bools, got '{:?}' and '{:?}'", v1, v2), l)),
                },
                _ => return Err(CError::RuntimeError(format!("Unsupported operator `{:?}`", op), l)),
            }
        },

        CExpr::Call((l, _), id, ref args) => {
            // get func
            let f = match vtab.get_func(id) {
                Some(f) => f,
                None => return Err(CError::RuntimeError(format!("Function '{}' not initialized", id), l)),
            };

            // calc and add args to symtab
            let mut tab = SymTab::new();
            for (i, p) in f.proto.params.iter().enumerate() {
                let (ref t, ref pid) = *p;
                let e = match args.iter().nth(i) {
                    Some(x) => x,
                    None => return Err(CError::RuntimeError(format!("Function '{}' missing param '{:?}'", id, p), l)),
                };
                let val = try!(run_expr(e, vtab, global_symtab, local_symtab));
                tab.insert(pid, (t.clone(), None, Some(val)));
            }

            match try!(run_func(&f, vtab, global_symtab, tab)) {
                Some(v) => v,
                None => return Err(CError::RuntimeError(format!("Expression returned void"), l)),
            }
        },

        CExpr::Index((l, _), id, ref e) => {
            let sym = match local_symtab.get_val(id) {
                Some(v) => v,
                _ => match global_symtab.get_val(id) {
                    Some(v) => v,
                    _ => return Err(CError::RuntimeError(format!("Variable '{}' not initialized", id), l)),
                }
            };
            let a = match sym {
                SymVal::Array(a) => a,
                x => return Err(CError::RuntimeError(format!("Expected array, got {:?}", x), l)),
            };

            // get index
            let i = match try!(run_expr(e, vtab, global_symtab, local_symtab)) {
                SymVal::Int(n) => n,
                x => return Err(CError::RuntimeError(format!("Expected array index, got {:?}", x), l)),
            };

            // check bounds
            if i >= (a.len() as i32) {
                return Err(CError::RuntimeError(format!("Index {} out of bounds (range: {})", i, a.len()), l))
            };

            (*a[i as usize]).clone()
        },

        _ => panic!("unexpected expr '{:?}' in ast", expr),
    };

    Ok(res)
}
