use std::fmt::{Debug, Formatter, Error};


pub type CProg<'input> = Vec<Box<CProgElem<'input>>>;

pub enum CProgElem<'input> {
    VarDecl(Box<CVarDecl<'input>>),
    Proto(Box<CProto<'input>>),
    Func(Box<CFunc<'input>>),
    Error,
}

pub type CProto<'input> = (Option<CType>, Vec<Box<(CIdent<'input>, Vec<Box<CParam<'input>>>)>>);

#[derive(Debug)]
pub struct CFunc<'input> {
    pub ret_type: Option<CType>,
    pub name: CIdent<'input>,
    pub params: Vec<Box<CParam<'input>>>,
    pub decls: Vec<Box<CVarDecl<'input>>>,
    pub stmts: Vec<Box<CStmt<'input>>>,
}

pub type CParam<'input> = (CType, CIdent<'input>);

pub type CVarDecl<'input> = (CType, Vec<CIdent<'input>>);

pub enum CStmt<'input> {
    Assign(CLoc, CIdent<'input>, Box<CExpr<'input>>),
    Return(CLoc, Option<Box<CExpr<'input>>>),
    Error,
}

pub enum CExpr<'input> {
    Number(CNum),
    Ident(CIdent<'input>),
    BinOp(Box<CExpr<'input>>, COp, Box<CExpr<'input>>),
    Call(CIdent<'input>, Vec<Box<CExpr<'input>>>),
    Error,
}

#[derive(Copy, Clone)]
pub enum COp {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Copy, Clone)]
pub enum CType {
    Char,
    Int,
}

pub type CLoc = (usize, usize);

pub type CNum = i32;

pub type CIdent<'input> = &'input str;


// debug trait

impl<'input> Debug for CProgElem<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CProgElem::*;
        match *self {
            VarDecl(ref x) => write!(fmt, "{:?}", x),
            Proto(ref x) => write!(fmt, "{:?}", x),
            Func(ref x) => write!(fmt, "{:?}", x),
            Error => write!(fmt, "error"),
        }
    }
}

impl<'input> Debug for CStmt<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CStmt::*;
        match *self {
            Assign(_, ref l, ref r) => write!(fmt, "{:?} = {:?}", l, r),
            Return(_, ref o) => {
                match *o {
                    Some(ref e) => write!(fmt, "return {:?}", e),
                    None => write!(fmt, "return"),
                }
            }
            Error => write!(fmt, "error"),
        }
    }
}

impl<'input> Debug for CExpr<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CExpr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Ident(ref s) => write!(fmt, "{}", &s),
            BinOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Call(ref i, ref p) => {
                let mut s: String = format!("{:?}", p[0]);
                for e in p[1..].iter() {
                    s.push_str(&format!(", {:?}", e));
                }
                write!(fmt, "{}({})", i, s)
            },
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for COp {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::COp::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

impl Debug for CType {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CType::*;
        match *self {
            Char => write!(fmt, "char"),
            Int => write!(fmt, "int"),
        }
    }
}
