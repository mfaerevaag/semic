use std::collections::HashMap;
use std::collections::hash_map::Iter;
use ast::*;

// function table

pub struct FuncTab<'a> {
    tab: HashMap<&'a str, FuncEntry<'a>>
}

pub type FuncEntry<'a> = (&'a CProto<'a>, Option<&'a CFunc<'a>>);

impl<'a> FuncTab<'a> {
    pub fn new() -> FuncTab<'a> {
        FuncTab { tab: HashMap::new() }
    }

    pub fn iter(&self) -> Iter<&'a str, FuncEntry<'a>> {
        self.tab.iter()
    }

    pub fn get_proto(&self, key: &'a str) -> Option<&'a CProto<'a>> {
        match self.tab.get(key) {
            Some(&(proto, _)) => Some(proto),
            _ => None,
        }
    }

    pub fn get_func(&self, key: &'a str) -> Option<&'a CFunc<'a>> {
        match self.tab.get(key) {
            Some(&(_, f)) => f,
            _ => None
        }
    }

    pub fn insert(&mut self, key: &'a str, val: FuncEntry<'a>) -> Option<FuncEntry<'a>> {
        self.tab.insert(key, val)
    }
}

// symbol table

pub struct SymTab<'a> {
    tab: HashMap<&'a str, SymEntry>
}

pub type SymEntry = (CType, Option<usize>, Option<SymVal>);

#[derive(Clone)]
pub enum SymVal {
    Str(String),
    Num(i32),
    Char(char),
    Bool(bool)
}

impl<'a> SymTab<'a> {
    pub fn new() -> SymTab<'a> {
        SymTab { tab: HashMap::new() }
    }

    pub fn iter(&self) -> Iter<&'a str, SymEntry> {
        self.tab.iter()
    }

    pub fn get_type(&self, key: &'a str) -> Option<(CType, Option<usize>)> {
        match self.tab.get(key) {
            Some(&(ref t, s, _)) => Some((t.clone(), s)),
            _ => None,
        }
    }

    pub fn get_val(&self, key: &'a str) -> Option<SymVal> {
        match self.tab.get(key) {
            Some(&(_, _, Some(ref v))) => Some(v.clone()),
            _ => None
        }
    }

    pub fn set_val(&mut self, key: &'a str, val: SymVal) {
        let clone = self.tab.clone();
        let &(ref t, s, ref prev) = match clone.get(key) {
            Some(v) => v,
            _ => panic!("variable '{}' not declared", key),
        };
        self.tab.insert(key, (t.clone(), s, Some(val)));
    }

    pub fn insert(&mut self, key: &'a str, val: SymEntry) -> Option<SymEntry> {
        self.tab.insert(key, val)
    }
}
