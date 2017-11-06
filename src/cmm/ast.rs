use std::fmt::{Debug, Formatter, Error};


pub type CProg<'input> = Vec<CProgElem<'input>>;

#[derive(Clone)]
pub enum CProgElem<'input> {
    VarDecl(CVarDecl<'input>),
    Proto(CProto<'input>),
    Func(CFunc<'input>),
    Error,
}

#[derive(Clone, Debug)]
pub struct CProto<'input> {
    pub ret: Option<CType>,
    pub name: CIdent<'input>,
    pub params: Vec<CParam<'input>>,
}

#[derive(Clone, Debug)]
pub struct CFunc<'input> {
    pub proto: CProto<'input>,
    pub decls: Vec<CVarDecl<'input>>,
    pub stmts: Vec<CStmt<'input>>,
}

pub type CParam<'input> = (CType, CIdent<'input>);

pub type CVarDecl<'input> = (CType, CIdent<'input>, Option<usize>);

#[derive(Clone)]
pub enum CStmt<'input> {
    Assign(CLoc, CIdent<'input>, Option<usize>, CExpr<'input>),
    Return(CLoc, Option<CExpr<'input>>),
    Block(CLoc, Vec<Box<CStmt<'input>>>),
    If(CLoc, CExpr<'input>, Box<CStmt<'input>>, Option<Box<CStmt<'input>>>),
    While(CLoc, CExpr<'input>, Box<CStmt<'input>>),
    For(CLoc, Option<Box<CStmt<'input>>>, Option<CExpr<'input>>,
        Option<Box<CStmt<'input>>>, Box<CStmt<'input>>),
    Error,
}

#[derive(Clone)]
pub enum CExpr<'input> {
    Num(CInt),
    Str(CString),
    Char(CChar),
    Ident(CIdent<'input>),
    UnOp(COp, Box<CExpr<'input>>),
    BinOp(COp, Box<CExpr<'input>>, Box<CExpr<'input>>),
    RelOp(COp, Box<CExpr<'input>>, Box<CExpr<'input>>),
    LogOp(COp, Box<CExpr<'input>>, Box<CExpr<'input>>),
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

#[derive(Clone)]
pub enum CType {
    Char,
    Int,
    Ref(Box<CType>),
}

pub type CLoc = (usize, usize);

pub type CInt = i32;
pub type CString = String;
pub type CChar = char;

pub type CIdent<'input> = &'input str;


// debug trait

impl<'input> Debug for CProgElem<'input> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::CProgElem::*;
        match *self {
            VarDecl(ref x) => write!(fmt, "{:?}", x),
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
            Num(n) => write!(fmt, "{:?}", n),
            Str(ref s) => write!(fmt, "{:?}", s),
            Char(n) => write!(fmt, "{:?}", n),
            Ident(ref s) => write!(fmt, "{}", &s),
            UnOp(op, ref l) => write!(fmt, "({:?}{:?})", op, l),
            BinOp(op, ref l, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            RelOp(op, ref l, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            LogOp(op, ref l, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
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
            Ref(ref t) => write!(fmt, "{:?}[]", t),
        }
    }
}
