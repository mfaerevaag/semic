use std::collections::HashMap;
use ast::*;

pub type FuncTab<'input> = HashMap<&'input str, &'input CProto<'input>>;
