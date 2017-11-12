use ast::*;
use env::{FuncTab, SymTab};
use error::CError;

// checker functions

pub fn analyze_prog<'input, 'err>(
    ast: &'input CProg<'input>,
) -> Result<(FuncTab<'input>, SymTab<'input>), CError>
{
    let mut vtab = FuncTab::new();
    let mut symtab = SymTab::new();
    let mut errors: Vec<String> = vec![];

    // check each element
    for elem in ast.iter() {
        match *elem {
            CProgElem::Decl(ref t, ref name, s) => {
                let val = (t.clone(), s, None);
                match symtab.insert(*name, val) {
                    Some(_) => errors.push(format!("variable {:?} already declared", name)),
                    None => (),
                };
            },

            CProgElem::Func(ref func) => {
                let CFunc { ref proto, .. } = *func;
                let CProto { ref name, .. } = *proto;
                let val = (proto, Some(func));

                match vtab.insert(*name, val) {
                    Some((_, Some(_))) => errors.push(format!("function {:?} already declared", name)),
                    _ => (),
                };
            },

            CProgElem::Proto(ref proto) => {
                let CProto { ref name, .. } = *proto;
                let val = (proto, None);

                match vtab.insert(*name, val) {
                    Some(_) => errors.push(format!("function {:?} already defined", name)),
                    None => (),
                };
            },

            CProgElem::Error => (),
        };
    };

    // check for main function
    match vtab.get_func("main") {
        None => errors.push(format!("function 'main' missing")),
        _ => (),
    };

    // check if local errors
    match errors.len() {
        0 => Ok((vtab, symtab)),
        _ => Err(CError::CheckerError(errors)),
    }
}

// pub fn analyze_func<'input, 'err>(
//     func: &'input CFunc<'input>,
// ) -> Result<SymTab, Vec<CheckErr>>
// {
//     let mut symtab = SymTab::new();
//     let mut errors = vec![];

//     // let CFunc { ref proto, .. } = *func;
//     // let CProto { ref name, .. } = *proto;

//     // check if local errors
//     match errors.len() {
//         0 => Ok(symtab),
//         _ => Err(errors),
//     }
// }
