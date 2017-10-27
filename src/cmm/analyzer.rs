use ast::*;
use environment::FuncTab;

pub fn check(ast: &CProg) -> Result<(), String> {
    let mut funcs = FuncTab::new();

    // check each element
    for elem in ast.iter() {
        match *elem {
            CProgElem::VarDecl((ref ret, ref var, ref size)) => {
                return Err(format!("TODO: check VarDecl"));
            },
            CProgElem::Func(ref func) => {
                let CFunc { ref proto, .. } = *func;
                let CProto { ref name, .. } = *proto;

                // check if name already exists
                match funcs.get(*name) {
                    Some(_) => return Err(format!("function {} already exists", name)),
                    None => funcs.insert(*name, proto)
                };
            },
            CProgElem::Proto(ref proto) => {
                let CProto { ref name, .. } = *proto;

                // check if name already exists
                match funcs.get(*name) {
                    Some(_) => return Err(format!("function {} already exists", name)),
                    None => funcs.insert(*name, proto)
                };
            },
            CProgElem::Error => {
                return Err(format!("TODO: check Error"));
            },
        }
    };

    Ok(())
}
