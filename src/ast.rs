use std::fmt::{Debug, Formatter, Error};

pub enum Expr {
    Number(i32),
    Ident(String),
    BinOp(Box<Expr>, Op, Box<Expr>),
    Error,
}

#[derive(Copy, Clone)]
pub enum Op {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Ident(ref s) => write!(fmt, "{}", &s),
            BinOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for Op {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Op::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}
