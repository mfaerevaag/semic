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

#[derive(Clone)]
pub struct SymTab<'a> {
    tab: HashMap<&'a str, SymEntry>
}

pub type SymEntry = (CType, Option<usize>, Option<SymVal>);

#[derive(PartialEq, Debug, Clone)]
pub enum SymVal {
    Num(i32),
    Char(char),
    Bool(bool),
    Array(Vec<Box<SymVal>>)
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

    pub fn set_val(&mut self, key: &'a str, i: Option<usize>, val: SymVal) {
        let clone = self.tab.clone();
        let &(ref t, s, ref prev) = match clone.get(key) {
            Some(v) => v,
            _ => panic!("variable '{}' not declared", key),
        };

        // set var or array index
        match i {
            None => self.tab.insert(key, (t.clone(), s, Some(val))),
            Some(i) => match prev {
                &Some(SymVal::Array(ref a)) => {
                    let mut a = a.clone();
                    a.remove(i); // remove old val
                    a.insert(i, Box::new(val)); // set new val
                    let new = SymVal::Array(a);
                    self.tab.insert(key, (t.clone(), s, Some(new)))
                },
                &None => {
                    let mut a = Vec::new();
                    let size = s.unwrap();
                    for j in 0..size {
                        if j == i {
                            a.push(Box::new(val.clone()));
                        } else {
                            a.push(Box::new(SymVal::Num(0)));
                        }
                    }
                    let new = SymVal::Array(a);
                    self.tab.insert(key, (t.clone(), s, Some(new)))
                }
                x => panic!("expected array, got {:?}", x),
            }
        };
    }

    pub fn insert(&mut self, key: &'a str, val: SymEntry) -> Option<SymEntry> {
        self.tab.insert(key, val)
    }
}
