use std::fmt::{Debug, Formatter, Error};


pub type CProg<'input> = Vec<Box<CProgElem<'input>>>;

#[derive(Clone)]
pub enum CProgElem<'input> {
    VarDecl(Box<CVarDecl<'input>>),
    Proto(Box<CProto<'input>>),
    Func(Box<CFunc<'input>>),
    Error,
}

#[derive(Clone, Debug)]
pub struct CProto<'input> {
    pub ret: Option<CType>,
    pub name: CIdent<'input>,
    pub params: Vec<Box<CParam<'input>>>,
}

#[derive(Clone, Debug)]
pub struct CFunc<'input> {
    pub proto: Box<CProto<'input>>,
    pub decls: Vec<Box<CVarDecl<'input>>>,
    pub stmts: Vec<Box<CStmt<'input>>>,
}

pub type CParam<'input> = (CType, CIdent<'input>);

pub type CVarDecl<'input> = (CType, CIdent<'input>, Option<usize>);

#[derive(Clone)]
pub enum CStmt<'input> {
    Assign(CLoc, CIdent<'input>, Box<CExpr<'input>>),
    Return(CLoc, Option<Box<CExpr<'input>>>),
    Block(CLoc, Vec<Box<CStmt<'input>>>),
    If(CLoc, Box<CExpr<'input>>, Box<CStmt<'input>>, Option<Box<CStmt<'input>>>),
    While(CLoc, Box<CExpr<'input>>, Box<CStmt<'input>>),
    For(CLoc, Option<Box<CStmt<'input>>>, Option<Box<CExpr<'input>>>,
        Option<Box<CStmt<'input>>>, Box<CStmt<'input>>),
    Error,
}

#[derive(Clone)]
pub enum CExpr<'input> {
    Number(CNum),
    Ident(CIdent<'input>),
    UnOp(COp, Box<CExpr<'input>>),
    BinOp(COp, Box<CExpr<'input>>, Box<CExpr<'input>>),
    Call(CIdent<'input>, Vec<Box<CExpr<'input>>>),
    Index(CIdent<'input>, Box<CExpr<'input>>),
    Error,
}

#[derive(Copy, Clone)]
pub enum COp {
    Mul,
    Div,
    Add,
    Sub,

    Neg,
    Not,
}

#[derive(Clone)]
pub enum CType {
    Char,
    Int,
    Ref(Box<CType>),
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
            Block(_, ref stmts) => write!(fmt, "{:?}", stmts),
            If(_, ref cond, ref stmt, ref opt) => match opt.clone() {
                Some(ref stmt2) => write!(fmt, "if ({:?}) {:?} else {:?}", cond, stmt, stmt2),
                None => write!(fmt, "if ({:?}) {:?}", cond, stmt),
            },
            While(_, ref cond, ref stmt) =>
                write!(fmt, "while ({:?}) {:?}", cond, stmt),
            For(_, ref init, ref cond, ref inc, ref stmt) =>
                write!(fmt, "for ({:?};{:?};{:?}) {:?}", init, cond, inc, stmt),
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
            UnOp(op, ref l) => write!(fmt, "({:?} {:?})", op, l),
            BinOp(op, ref l, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Call(ref i, ref p) => {
                let mut s: String = format!("{:?}", p[0]);
                for e in p[1..].iter() {
                    s.push_str(&format!(", {:?}", e));
                }
                write!(fmt, "{}({})", i, s)
            },
            Index(ref i, ref e) => {
                write!(fmt, "{}[{:?}]", i, e)
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
            Neg => write!(fmt, "-"),
            Not => write!(fmt, "!"),
        }
    }
}

impl Debug for CType {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CType::*;
        match *self {
            Char => write!(fmt, "char"),
            Int => write!(fmt, "int"),
            Ref(ref t) => write!(fmt, "{:?}[]", t),
        }
    }
}
