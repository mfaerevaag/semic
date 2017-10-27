use ast::*;
use env::{FuncTab, SymTab};

pub fn check(ast: &CProg) -> Result<(), String> {
    // global function table
    let mut funcs = FuncTab::new();
    funcs.push_frame();
    // symbol table
    let mut syms = SymTab::new();
    syms.push_frame();

    // check each element
    for elem in ast.iter() {
        match *elem {
            CProgElem::VarDecl(ref decl) => {
                let (_, ref name, _) = *decl;
                let val = (decl, None);

                match syms.insert(*name, val) {
                    Ok(Some(_)) => return Err(format!("variable {:?} already declared", name)),
                    Ok(None) => (),
                    Err(_) => panic!("symbol table empty"),
                };
            },

            CProgElem::Func(ref func) => {
                let CFunc { ref proto, .. } = *func;
                let CProto { ref name, .. } = *proto;
                let val = (proto, Some(func));

                match funcs.insert(*name, val) {
                    Ok(Some(x)) => match x {
                        (_, None) => (),
                        (_, Some(_)) => return Err(format!("function {:?} already declared", name)),
                    },
                    Ok(None) => (),
                    Err(_) => panic!("function table empty"),
                };
            },

            CProgElem::Proto(ref proto) => {
                let CProto { ref name, .. } = *proto;
                let val = (proto, None);

                match funcs.insert(*name, val) {
                    Ok(Some(_)) => return Err(format!("function {:?} already defined", name)),
                    Ok(None) => (),
                    Err(_) => panic!("function table empty"),
                };
            },

            CProgElem::Error => {
                return Err(format!("TODO: check Error"));
            },
        };
    };

    Ok(())
}
