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
    let mut errors: Vec<(String, Option<usize>)> = vec![];

    // check each element
    for elem in ast.iter() {
        match *elem {
            CProgElem::Decl((l, _), ref t, ref name, s) => {
                match symtab.insert(*name, t.clone(), s, None, Some(l)) {
                    Some(_) => errors.push((format!("Variable '{}' already declared", name), Some(l))),
                    None => (),
                };
            },

            CProgElem::Func((l, _), ref func) => {
                let CFunc { ref proto, .. } = *func;
                let CProto { ref name, .. } = *proto;

                match vtab.insert(*name, proto, Some(func)) {
                    Some((_, Some(_))) => errors.push((format!("Function '{}' already declared", name), Some(l))),
                    _ => (),
                };
            },

            CProgElem::Proto((l, _), ref proto) => {
                let CProto { ref name, .. } = *proto;

                match vtab.insert(*name, proto, None) {
                    Some(_) => errors.push((format!("Function '{}' already defined", name), Some(l))),
                    None => (),
                };
            },

            CProgElem::Error => (),
        };
    };

    // check for main function
    match vtab.get_func("main") {
        None => errors.push((format!("Function 'main' missing"), None)),
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
