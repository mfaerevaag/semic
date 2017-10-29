use std::collections::HashMap;
use ast::*;

pub type FuncTab<'input> = Tab<'input, (&'input CProto<'input>, Option<&'input CFunc<'input>>)>;
pub type SymTab<'input> = Tab<'input, (&'input CVarDecl<'input>, Option<i32>)>;

pub struct Tab<'input, T: 'input> {
    tab: Vec<HashMap<&'input str, T>>,
}

impl<'input, T> Tab<'input, T> {
    pub fn new() -> Tab<'input, T> {
        Tab { tab: vec![] }
    }

    pub fn push_frame(&mut self) {
        self.tab.push(HashMap::new());
    }

    pub fn pop_frame(&mut self) {
        self.tab.pop();
    }

    pub fn insert(&mut self, key: &'input str, value: T) -> Result<Option<T>, ()> {
        match self.tab.last_mut() {
            None => Err(()),
            Some(frame) => Ok(frame.insert(key, value)),
        }
    }

    pub fn get(&self, key: &'input str) -> Result<Option<&T>, ()> {
        match self.tab.last() {
            None => Err(()),
            Some(frame) => Ok(frame.get(key)),
        }
    }
}
