use std::str::Chars;
use std::fmt::{Debug, Formatter, Error};


pub type CProg<'input> = Vec<CProgElem<'input>>;

#[derive(Clone)]
pub enum CProgElem<'input> {
    Decl(CType, CIdent<'input>, Option<usize>),
    Proto(CProto<'input>),
    Func(CFunc<'input>),
    Error,
}

#[derive(Clone, Debug)]
pub struct CProto<'input> {
    pub ret: Option<CType>,
    pub name: CIdent<'input>,
    pub params: Vec<(CType, CIdent<'input>)>,
}

#[derive(Clone, Debug)]
pub struct CFunc<'input> {
    pub proto: CProto<'input>,
    pub stmts: Vec<CStmt<'input>>,
}

#[derive(Clone)]
pub enum CStmt<'input> {
    Decl(CLoc, CType, CIdent<'input>, Option<usize>),
    Assign(CLoc, CIdent<'input>, Option<usize>, CExpr<'input>),
    Return(CLoc, Option<CExpr<'input>>),
    Block(CLoc, Vec<Box<CStmt<'input>>>),
    If(CLoc, CExpr<'input>, Box<CStmt<'input>>, Option<Box<CStmt<'input>>>),
    While(CLoc, CExpr<'input>, Box<CStmt<'input>>),
    Print(CLoc, CExpr<'input>),
    Error,
}

#[derive(Clone)]
pub enum CExpr<'input> {
    Int(CInt),
    Float(CFloat),
    Str(CString<'input>),
    Char(CChar),
    Ident(CIdent<'input>),
    UnOp(COp, Box<CExpr<'input>>),
    BinOp(COp, Box<CExpr<'input>>, Box<CExpr<'input>>),
    Call(CIdent<'input>, Vec<Box<CExpr<'input>>>),
    Index(CIdent<'input>, Box<CExpr<'input>>),
    Error,
}

#[derive(Copy, Clone)]
pub enum COp {
    // arith
    Mul,
    Div,
    Add,
    Sub,
    // rel
    Neq,
    Eq,
    Lt,
    Lte,
    Gt,
    Gte,
    // logical
    And,
    Or,
    // unary
    Neg,
    Not,
}

#[derive(PartialEq, Clone)]
pub enum CType {
    Char,
    Int,
    Float,
    Ref(Box<CType>),
}

pub type CLoc = (usize, usize);

pub type CInt = i32;
pub type CFloat = f32;
pub type CString<'input> = Chars<'input>;
pub type CChar = char;

pub type CIdent<'input> = &'input str;


// debug trait

impl<'input> Debug for CProgElem<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CProgElem::*;
        match *self {
            Decl(ref t, id, s) => match s {
                Some(i) => write!(fmt, "{:?} {}[{}]", t, id, i),
                None => write!(fmt, "{:?} {}", t, id),
            },
            Proto(ref x) => write!(fmt, "{:?}", x),
            Func(ref x) => write!(fmt, "{:#?}", x),
            Error => write!(fmt, "error"),
        }
    }
}

impl<'input> Debug for CStmt<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CStmt::*;
        match *self {
            Assign(_, ref l, i, ref r) => match i {
                Some(i) => write!(fmt, "{}[{}] = {:?}", l, i, r),
                None => write!(fmt, "{} = {:?}", l, r),
            },
            Decl(_, ref t, id, s) => match s {
                Some(i) => write!(fmt, "{:?} {}[{}]", t, id, i),
                None => write!(fmt, "{:?} {}", t, id),
            },
            Return(_, ref o) => {
                match *o {
                    Some(ref e) => write!(fmt, "return {:?}", e),
                    None => write!(fmt, "return"),
                }
            }
            Block(_, ref stmts) => write!(fmt, "{:#?}", stmts),
            If(_, ref cond, ref stmt, ref opt) => match opt.clone() {
                Some(ref stmt2) => write!(fmt, "if {:?} {:?} else {:?}", cond, stmt, stmt2),
                None => write!(fmt, "if {:?} {:?}", cond, stmt),
            },
            While(_, ref cond, ref stmt) =>
                write!(fmt, "while {:?} {:?}", cond, stmt),
            Print(_, ref e) => write!(fmt, "printf({:?})", e),
            Error => write!(fmt, "error"),
        }
    }
}

impl<'input> Debug for CExpr<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CExpr::*;
        match *self {
            Int(i) => write!(fmt, "{:?}", i),
            Float(f) => write!(fmt, "{:?}", f),
            Str(ref s) => write!(fmt, "\"{}\"", s.as_str()),
            Char(c) => write!(fmt, "{:?}", c),
            Ident(ref s) => write!(fmt, "{}", &s),
            UnOp(op, ref l) => write!(fmt, "({:?}{:?})", op, l),
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
            Eq  => write!(fmt, "=="),
            Neq => write!(fmt, "!="),
            Lt  => write!(fmt, "<"),
            Lte => write!(fmt, "<="),
            Gt  => write!(fmt, ">"),
            Gte => write!(fmt, ">="),
            And => write!(fmt, "&&"),
            Or  => write!(fmt, "||"),
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
            Float => write!(fmt, "float"),
            Ref(ref t) => write!(fmt, "{:?}*", t),
        }
    }
}
