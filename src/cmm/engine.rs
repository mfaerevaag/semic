use std::char;

use ast::*;
use env::{FuncTab, SymTab, SymVal};
use checker;
use error::CError;
use repl::Repl;

pub fn run_prog<'input>(
    ast: &'input CProg<'input>,
    program: &'input str,
    args: &'input Vec<String>,
    interactive: bool,
    verbose: bool,
) -> Result<Option<SymVal>, CError>
{
    // load global function and symbol table
    let (vtab, global_symtab) = match checker::analyze_prog(&ast) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    // get main function
    let main = vtab.get_func("main").unwrap();

    // load command line args
    let mut local_symtab = SymTab::new();
    // argc
    let argc = args.len() as i32;
    local_symtab.insert("argc", CType::Int, None, Some(SymVal::Int(argc)), None);
    // argv
    let mut argv = vec![];
    for arg in args.iter() {
        // create internal string
        let mut s = Vec::with_capacity(arg.len() + 1);
        for c in arg.chars() {
            s.push(Box::new(SymVal::Char(c)));
        }
        // add null char
        s.push(Box::new(SymVal::Char('\0')));

        argv.push(Box::new(SymVal::Array(s)));
    }
    local_symtab.insert("argv",
                        CType::Ref(Box::new(CType::Ref(Box::new(CType::Char)))),
                        None,
                        Some(SymVal::Array(argv)),
                        None);

    // repl
    let mut repl = Repl::new(interactive, program, verbose);

    // run
    let (ret, res_sym_tab, res_glob_tab) = try!(run_func(main, &vtab, global_symtab.clone(), local_symtab.clone(), repl.clone()));

    // show repl
    repl.finished(&res_sym_tab, &res_glob_tab)?;

    Ok(ret)
}

pub fn run_func<'input>(
    func: &'input CFunc<'input>,
    vtab: &'input FuncTab<'input>,
    global_symtab: SymTab<'input>,
    local_symtab: SymTab<'input>,
    repl: Repl,
) -> Result<(Option<SymVal>, SymTab<'input>, SymTab<'input>), CError>
{
    // run block
    let (ret, local_symtab, global_symtab, _) = try!(run_stmt(&func.body, vtab, global_symtab, local_symtab, repl));

    // unwrap return val
    match ret {
        Some(x) => Ok((x, local_symtab, global_symtab)),
        None => Ok((None, local_symtab, global_symtab)),
    }
}

pub fn run_stmt<'input>(
    stmt: &'input CStmt<'input>,
    vtab: &'input FuncTab<'input>,
    global_symtab: SymTab<'input>,
    local_symtab: SymTab<'input>,
    repl: Repl
) -> Result<(Option<Option<SymVal>>, SymTab<'input>, SymTab<'input>, Repl), CError>
{
    let mut tmp_repl = repl.clone();
    tmp_repl.show(stmt, &global_symtab, &local_symtab)?;

    let mut tmp_global_symtab = global_symtab.clone();
    let mut tmp_symtab = local_symtab.clone();

    let res = match *stmt {
        CStmt::Decl((l, _), ref t, id, ref eo) => {
            // get index
            let so = match *eo {
                Some(ref e) => {
                    let l2 = try!(loc_of_expr(e));
                    let sym = try!(run_expr(e, vtab, &tmp_global_symtab, &tmp_symtab, &tmp_repl));
                    match sym {
                        SymVal::Int(i) => Some(i as usize),
                        _ => return Err(CError::RuntimeError("Array index must be int".to_owned(), l2))
                    }
                },
                None => None
            };
            tmp_symtab.insert(id, t.clone(), so, None, Some(l));
            None
        },
        CStmt::Assign((l, _), id, ref eo, ref e) => {
            // get index
            let so = match *eo {
                Some(ref e) => {
                    let l2 = try!(loc_of_expr(e));
                    let sym = try!(run_expr(e, vtab, &tmp_global_symtab, &tmp_symtab, &tmp_repl));
                    match sym {
                        SymVal::Int(i) => Some(i as usize),
                        _ => return Err(CError::RuntimeError("Array index must be int".to_owned(), l2))
                    }
                },
                None => None
            };
            let val = try!(run_expr(e, vtab, &tmp_global_symtab, &tmp_symtab, &tmp_repl));
            let l2 = try!(loc_of_expr(e));
            match tmp_symtab.get_type(id) {
                // set in global
                Some((t, _)) => {
                    let casted = try!(auto_cast(&val, l2, &t));
                    match tmp_symtab.set_val(id, so, casted, Some(l)) {
                        Ok(()) => None,
                        Err(s) => return Err(CError::RuntimeError(s, l)),
                    }
                },
                // if not, assume global
                None => match tmp_global_symtab.get_type(id) {
                    Some((t, _)) => {
                        let casted = try!(auto_cast(&val, l2, &t));
                        match tmp_global_symtab.set_val(id, so, casted, Some(l)) {
                            Ok(()) => None,
                            Err(s) => return Err(CError::RuntimeError(s, l)),
                        }
                    },
                    None => return Err(CError::RuntimeError(format!("Variable '{:?}' not declared", id), l))
                }
            }
        },
        CStmt::Call((l, _), id, ref args) => {
            // get func
            let f = match vtab.get_func(id) {
                Some(f) => f,
                None => return Err(CError::RuntimeError(format!("Function '{}' not initialized", id), l)),
            };

            // calc and add args to symtab
            let mut tab = tmp_symtab.clone();
            tab.push_frame();
            for (i, p) in f.proto.params.iter().enumerate() {
                let (ref t, ref pid) = *p;
                let e = match args.iter().nth(i) {
                    Some(x) => x,
                    None => return Err(CError::RuntimeError(format!("Function '{}' missing param '{:?}'", id, p), l)),
                };
                let val = try!(run_expr(e, vtab, &tmp_global_symtab, &tmp_symtab, &tmp_repl));
                tab.insert(pid, t.clone(), None, Some(val), Some(l));
            }

            let (_, _, mut tab2) = try!(run_func(&f, vtab, global_symtab, tab, repl));
            tab2.pop_frame()?;
            tmp_symtab = tab2;
            None
        },
        CStmt::Return(_, ref s) => match s {
            &Some(ref e) => Some(Some(try!(run_expr(e, vtab, &tmp_global_symtab, &tmp_symtab, &tmp_repl)))),
            _ => Some(None),
        },
        CStmt::Block(_, ref stmts) => {
            let mut res = None;
            for s in stmts.iter() {
                let (res2, gtab, tab, repl) = try!(run_stmt(s, vtab, tmp_global_symtab, tmp_symtab, tmp_repl));
                tmp_global_symtab = gtab;
                tmp_symtab = tab;
                tmp_repl = repl;
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
            let b = match try!(run_expr(cond, vtab, &tmp_global_symtab, &tmp_symtab, &tmp_repl)) {
                SymVal::Int(i) => i != 0,
                SymVal::Bool(b) => b,
                x => return Err(CError::RuntimeError(format!("Expected bool, got {:?}", x), l)),
            };
            if b {
                let (res, gtab, tab, repl) = try!(run_stmt(s, vtab, tmp_global_symtab, tmp_symtab, tmp_repl));
                tmp_global_symtab = gtab;
                tmp_symtab = tab;
                tmp_repl = repl;
                res
            } else {
                match *o {
                    Some(ref es) => {
                        let (res, gtab, tab, repl) = try!(run_stmt(es, vtab, tmp_global_symtab, tmp_symtab, tmp_repl));
                        tmp_global_symtab = gtab;
                        tmp_symtab = tab;
                        tmp_repl = repl;
                        res
                    },
                    _ => None
                }
            }
        },
        CStmt::While((l, _), ref cond, ref s) => {
            let b = match try!(run_expr(cond, vtab, &tmp_global_symtab, &tmp_symtab, &tmp_repl)) {
                SymVal::Int(i) => i != 0,
                SymVal::Bool(b) => b,
                x => return Err(CError::RuntimeError(format!("Expected bool, got {:?}", x), l)),
            };
            if b {
                let (res, gtab, tab, repl) = try!(run_stmt(s, vtab, global_symtab, local_symtab, repl));
                match res {
                    Some(_) => res,
                    _ => {
                        let (res2, gtab2, tab2, repl2) = try!(run_stmt(stmt, vtab, gtab, tab, repl));
                        tmp_global_symtab = gtab2;
                        tmp_symtab = tab2;
                        tmp_repl = repl2;
                        res2
                    }
                }
            } else {
                None
            }
        },
        CStmt::Print(_, ref e) => {
            let val = try!(run_expr(e, vtab, &tmp_global_symtab, &tmp_symtab, &tmp_repl));
            println!(" {:?}", val);
            None
        },
        _ => return Err(CError::UnknownError(format!("unexpected stmt '{:?}' in ast", stmt)))
    };

    Ok((res, tmp_global_symtab, tmp_symtab, tmp_repl))
}

pub fn run_expr<'input>(
    expr: &'input CExpr<'input>,
    vtab: &'input FuncTab<'input>,
    global_symtab: &'input SymTab<'input>,
    local_symtab: &'input SymTab<'input>,
    repl: &'input Repl
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
            let v = try!(run_expr(e, vtab, global_symtab, local_symtab, repl));
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
            let v1 = try!(run_expr(e1, vtab, global_symtab, local_symtab, repl));
            let (is_num1, is_int1, i1, is_float1, f1, is_bool1, b1) =
                match v1 {
                    SymVal::Int(x)   => (true,  true,  x, false, 0f32, false, false),
                    SymVal::Float(x) => (true,  false, 0, true,  x,    false, false),
                    SymVal::Bool(x)  => (false, false, 0, false, 0f32, true,  x),
                    _ => return Err(CError::RuntimeError(format!("Unexpected '{:?}' in binary op", v1), l)),
                };
            let v2 = try!(run_expr(e2, vtab, global_symtab, local_symtab, repl));
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
                    (.., true, false) => SymVal::Bool(b1 && ((i2 as f32 + f2) != 0.0)),
                    // normal
                    (.., true, true) => SymVal::Bool(b1 && b2),
                    _ => return Err(CError::RuntimeError(format!("`&&` op expected pair of bools, got '{:?}' and '{:?}'", v1, v2), l)),
                },
                COp::Or => match (is_int1, is_int2, is_float1, is_float2, is_bool1, is_bool2) {
                    (true, true, false, false, ..) => SymVal::Bool((i1 != 0) || (i2 != 0)),
                    (false, true, true, false, ..) => SymVal::Bool((f1 != 0.0) || (i2 != 0)),
                    (true, false, false, true, ..) => SymVal::Bool((i1 != 0) || (f2 != 0.0)),
                    (false, false, true, true, ..) => SymVal::Bool((f1 != 0.0) || (f2 != 0.0)),
                    // auto cast
                    (.., false, true) => SymVal::Bool(((i1 as f32 + f1) != 0.0) || b2),
                    (.., true, false) => SymVal::Bool(b1 || ((i2 as f32 + f2) != 0.0)),
                    // normal
                    (.., true, true) => SymVal::Bool(b1 && b2),
                    _ => return Err(CError::RuntimeError(format!("`||` op expected pair of bools, got '{:?}' and '{:?}'", v1, v2), l)),
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
                let val = try!(run_expr(e, vtab, global_symtab, local_symtab, repl));
                tab.insert(pid, t.clone(), None, Some(val), Some(l));
            }

            match try!(run_func(&f, vtab, global_symtab.clone(), tab, repl.clone())) {
                (Some(v), ..) => v,
                _ => return Err(CError::RuntimeError(format!("Expression returned void"), l)),
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
            let i = match try!(run_expr(e, vtab, global_symtab, local_symtab, repl)) {
                SymVal::Int(n) => n,
                x => return Err(CError::RuntimeError(format!("Expected array index, got {:?}", x), l)),
            };

            // check bounds
            if i >= (a.len() as i32) {
                return Err(CError::RuntimeError(format!("Index {} out of bounds (range: {})", i, a.len()), l))
            };

            (*a[i as usize]).clone()
        },

        _ => return Err(CError::UnknownError(format!("unexpected expr '{:?}' in ast", expr)))
    };

    Ok(res)
}

fn loc_of_expr<'input>(expr: &'input CExpr) -> Result<usize, CError> {
    match *expr {
        CExpr::Int((l, _), ..) => Ok(l),
        CExpr::Float((l, _), ..)  => Ok(l),
        CExpr::Str((l, _), ..) => Ok(l),
        CExpr::Char((l, _), ..) => Ok(l),
        CExpr::Ident((l, _), ..) => Ok(l),
        CExpr::UnOp((l, _), ..) => Ok(l),
        CExpr::BinOp((l, _), ..) => Ok(l),
        CExpr::Call((l, _), ..) => Ok(l),
        CExpr::Index((l, _), ..) => Ok(l),
        _ => Err(CError::UnknownError(format!("unexpected expr '{:?}'", expr)))
    }
}

fn auto_cast<'input>(val: &'input SymVal, loc: usize, t: &'input CType) -> Result<SymVal, CError> {
    match *t {
        CType::Int => match *val {
            SymVal::Int(_) => Ok(val.clone()),
            SymVal::Float(f) => Ok(SymVal::Int(f as i32)),
            SymVal::Char(c) => match c.to_digit(16) {
                Some(i) => Ok(SymVal::Int(i as i32)),
                None => Err(CError::RuntimeError("Failed to cast char type to int".to_owned(), loc))
            },
            SymVal::Bool(b) => Ok(SymVal::Int(if b { 1 } else { 0 })),
            SymVal::Array(_) => Err(CError::RuntimeError("Cannot auto cast array type to int".to_owned(), loc)),
        },
        CType::Float => match *val {
            SymVal::Int(i) => Ok(SymVal::Float(i as f32)),
            SymVal::Float(_) => Ok(val.clone()),
            SymVal::Char(_) => Err(CError::RuntimeError("Cannot auto cast char type to float".to_owned(), loc)),
            SymVal::Bool(_) => Err(CError::RuntimeError("Cannot auto cast bool type to float".to_owned(), loc)),
            SymVal::Array(_) => Err(CError::RuntimeError("Cannot auto cast array type to float".to_owned(), loc))
        },
        CType::Char => match *val {
            SymVal::Int(i) => match char::from_digit(i as u32, 16) {
                Some(c) => Ok(SymVal::Char(c)),
                None => Err(CError::RuntimeError("Failed to cast int type to char".to_owned(), loc))
            },
            SymVal::Float(_) => Err(CError::RuntimeError("Cannot auto cast float type to char".to_owned(), loc)),
            SymVal::Char(_) => Ok(val.clone()),
            SymVal::Bool(_) => Err(CError::RuntimeError("Cannot auto cast bool type to char".to_owned(), loc)),
            SymVal::Array(_) => Err(CError::RuntimeError("Cannot auto cast array type to char".to_owned(), loc))
        },
        CType::Ref(_) => Ok(val.clone()) // TODO: throw error
        // Array(_) => Err(CError::RuntimeError("Cannot cast array type".to_owned(), loc))
    }
}
