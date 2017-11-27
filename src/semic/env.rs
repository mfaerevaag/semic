use std::fmt::{Debug, Formatter, Error};
use std::collections::HashMap;

use ast::*;
use error::CError;

// function table

#[derive(Clone)]
pub struct FuncTab<'a> {
    tab: HashMap<&'a str, FuncEntry<'a>>
}

pub type FuncEntry<'a> = (&'a CProto<'a>, Option<&'a CFunc<'a>>);

impl<'a> FuncTab<'a> {
    pub fn new() -> FuncTab<'a> {
        FuncTab { tab: HashMap::new() }
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

    pub fn insert(
        &mut self,
        key: &'a str, proto: &'a CProto<'a>,
        func: Option<&'a CFunc<'a>>
    ) -> Option<FuncEntry<'a>>
    {
        self.tab.insert(key, (proto, func))
    }
}

// symbol table

#[derive(Debug, Clone)]
pub struct SymTab<'a> {
    stack: Vec<HashMap<&'a str, SymEntry>>
}

pub type SymEntry = (CType, Option<usize>, Vec<(Option<SymVal>, Option<usize>)>);

#[derive(PartialEq, Clone)]
pub enum SymVal {
    Int(i32),
    Float(f32),
    Char(char),
    Bool(bool),
    Array(Vec<Box<SymVal>>)
}

impl<'a> SymTab<'a> {
    pub fn new() -> SymTab<'a> {
        SymTab { stack: vec![HashMap::new()] }
    }

    pub fn get_type(&self, key: &'a str) -> Option<(CType, Option<usize>)> {
        let tab = self.stack.last().unwrap();
        match tab.get(key) {
            Some(&(ref t, s, _)) => Some((t.clone(), s)),
            _ => None,
        }
    }

    pub fn get_val(&self, key: &'a str) -> Option<SymVal> {
        let tab = self.stack.last().unwrap();
        match tab.get(key) {
            Some(&(_, _, ref v)) => match v.last() {
                Some(&(ref v, _)) => v.clone(),
                _ => None
            },
            _ => None
        }
    }

    pub fn get_val_parent(&self, key: &'a str) -> Option<SymVal> {
        for i in (1..self.stack.len()).rev() {
            let tab = self.stack.get(i - 1).unwrap();
            match tab.get(key) {
                Some(&(_, _, ref v)) => match v.last() {
                    Some(&(ref v, _)) => return v.clone(),
                    _ => ()
                },
                _ => ()
            }
        }
        None
    }

    pub fn get_trace(&self, key: &'a str) -> Option<Vec<(Option<SymVal>, Option<usize>)>> {
        let tab = self.stack.last().unwrap();
        match tab.get(key) {
            Some(&(_, _, ref v)) => Some(v.clone()),
            _ => None
        }
    }

    pub fn set_val(
        &mut self,
        key: &'a str,
        i: Option<usize>,
        val: SymVal,
        loc: Option<usize>
    ) -> Result <(), String>
    {
        let mut tab = self.stack.last_mut().unwrap();
        let clone = tab.clone();
        let &(ref t, s, ref prev) = match clone.get(key) {
            Some(v) => v,
            _ => return Err(format!("Variable '{}' not declared", key)),
        };

        match i {
            // set array
            Some(i) => {
                let mut vec = prev.clone();
                let (last_val, _) = prev.last().unwrap().clone();
                let new = match last_val {
                    // set in existing array
                    Some(SymVal::Array(ref a)) => {
                        let mut a = a.clone();
                        a.remove(i); // remove old val
                        a.insert(i, Box::new(val)); // set new val
                        SymVal::Array(a)
                    },
                    Some(x) => return Err(format!("Expected array, got {:?}", x)),
                    // create init array
                    None => {
                        let mut a = Vec::new();
                        let size = s.unwrap();
                        for j in 0..size {
                            if j == i {
                                a.push(Box::new(val.clone()));
                            } else {
                                a.push(Box::new(SymVal::Int(0)));
                            }
                        }
                        SymVal::Array(a)
                    },
                };

                vec.push((Some(new), loc));
                tab.insert(key, (t.clone(), s, vec));
            },
            // set var
            None => {
                let mut vec = prev.clone();
                vec.push((Some(val), loc));
                tab.insert(key, (t.clone(), s, vec));
            }
        }

        Ok(())
    }

    pub fn insert(
        &mut self,
        key: &'a str,
        t: CType,
        s: Option<usize>,
        val: Option<SymVal>,
        loc: Option<usize>
    ) -> Option<SymEntry>
    {
        let mut tab = self.stack.last_mut().unwrap();
        let vec = vec![(val, loc)];
        tab.insert(key, (t, s, vec))
    }

    pub fn push_frame(&mut self) {
        self.stack.push(HashMap::new())
    }

    pub fn pop_frame(&mut self) -> Result<(), CError> {
        match self.stack.pop() {
            Some(_) => Ok(()),
            None => Err(CError::UnknownError("Cannot pop frame of empty symbol table".to_owned()))
        }
    }
}

impl Debug for SymVal {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::SymVal::*;
        match *self {
            Int(i) => write!(fmt, "{}", i),
            Float(f) => write!(fmt, "{:.5}", f),
            Char(c) => write!(fmt, "'{}'", c),
            Bool(b) => write!(fmt, "{}", b),
            Array(ref a) => {
                if let Some(ref x) = a.last() {
                    match ***x {
                        Char(_) => {
                            let cs: Vec<char> = a.iter().map(|v: &Box<SymVal>| match **v {
                                SymVal::Char(c) => c,
                                _ => panic!("Expected array of chars")}
                            ).collect();
                            let s: String = cs.into_iter().collect();
                            return write!(fmt, "{}", s);
                        },
                        _ => ()
                    }
                }

                write!(fmt, "{:?}", a)
            }
        }
    }
}
