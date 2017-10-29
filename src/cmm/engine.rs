use ast::*;
use env::{FuncTab, SymTab};

pub fn run_prog<'input>(
    ast: &'input CProg<'input>,
) -> Result<(), ()>
{
    // tables
    let mut vtab = FuncTab::new();
    let mut symtab = SymTab::new();

    // global function table
    vtab.push_frame();
    // symbol table
    symtab.push_frame();

    for elem in ast.iter() {
        // something
    }

    Ok(())
}
