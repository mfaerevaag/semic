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
    tab: HashMap<&'a str, SymEntry<'a>>
}

pub type SymEntry<'a> = (&'a CType, Option<usize>, Option<u32>);

impl<'a> SymTab<'a> {
    pub fn new() -> SymTab<'a> {
        SymTab { tab: HashMap::new() }
    }

    pub fn iter(&self) -> Iter<&'a str, SymEntry<'a>> {
        self.tab.iter()
    }

    pub fn get_type(&self, key: &'a str) -> Option<(&'a CType, Option<usize>)> {
        match self.tab.get(key) {
            Some(&(t, s, _)) => Some((t, s)),
            _ => None,
        }
    }

    pub fn get_val(&self, key: &'a str) -> Option<u32> {
        match self.tab.get(key) {
            Some(&(_, _, v)) => v,
            _ => None
        }
    }

    pub fn insert(&mut self, key: &'a str, val: SymEntry<'a>) -> Option<SymEntry<'a>> {
        self.tab.insert(key, val)
    }
}
