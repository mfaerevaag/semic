use std::error::Error;
use std::fmt;

use ast::*;
use env::{FuncTab, SymTab};

// error type

#[derive(Debug)]
pub struct CheckErr {
    message: String,
}

impl CheckErr {
    pub fn new (message: String) -> CheckErr {
        CheckErr { message }
    }
}

impl Error for CheckErr {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for CheckErr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", &self.message)
    }
}

// checker functions

pub fn analyze_prog<'input, 'err>(
    ast: &'input CProg<'input>,
) -> Result<(FuncTab<'input>, SymTab<'input>), Vec<CheckErr>>
{
    let mut vtab = FuncTab::new();
    let mut symtab = SymTab::new();
    let mut errors = vec![];

    // check each element
    for elem in ast.iter() {
        match *elem {
            CProgElem::Decl(ref t, ref name, s) => {
                let val = (t.clone(), s, None);
                match symtab.insert(*name, val) {
                    Some(_) => {
                        let msg = format!("variable {:?} already declared", name);
                        errors.push(CheckErr::new(msg));
                    },
                    None => (),
                };
            },

            CProgElem::Func(ref func) => {
                let CFunc { ref proto, .. } = *func;
                let CProto { ref name, .. } = *proto;
                let val = (proto, Some(func));

                match vtab.insert(*name, val) {
                    Some((_, Some(_))) => {
                        let msg = format!("function {:?} already declared", name);
                        errors.push(CheckErr::new(msg));
                    },
                    _ => (),
                };
            },

            CProgElem::Proto(ref proto) => {
                let CProto { ref name, .. } = *proto;
                let val = (proto, None);

                match vtab.insert(*name, val) {
                    Some(_) => {
                        let msg = format!("function {:?} already defined", name);
                        errors.push(CheckErr::new(msg));
                    },
                    None => (),
                };
            },

            CProgElem::Error => (),
        };
    };

    // check for main function
    match vtab.get_func("main") {
        None => {
            let msg = format!("function 'main' missing");
            errors.push(CheckErr::new(msg));
        },
        _ => (),
    };

    // check if local errors
    match errors.len() {
        0 => Ok((vtab, symtab)),
        _ => Err(errors),
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
