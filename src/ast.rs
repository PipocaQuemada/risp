use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use std::sync::Arc;
use LispVal::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum LispVal {
    Atom(String),
    ConsList(Rc<Cons>),
    Nil,
    Number(i32),
    Str(String),
    Bool(bool),
}

impl LispVal {
    pub fn cons(car: LispVal, cdr: LispVal) -> LispVal {
        ConsList(Rc::new(Cons {
            car: Rc::new(car),
            cdr: cdr,
        }))
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Cons {
    pub car: Rc<LispVal>,
    pub cdr: LispVal,
}

impl Cons {
    fn fmtCons(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (&*self.car).fmt(f)?;

        match &self.cdr {
            Nil => write!(f, ""),
            ConsList(cons) => {
                write!(f, " ")?;
                (&*cons).fmtCons(f)
            }
            _ => {
                // Dotted list
                write!(f, " . ")?;
                self.cdr.fmt(f)
            }
        }
    }
}
impl fmt::Display for LispVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "({}, {})", self.x, self.y)
        match self {
            Atom(s) => write!(f, "{}", s),
            Str(s) => write!(f, "\"{}\"", s),
            Number(i) => write!(f, "{}", i),
            Bool(true) => write!(f, "#t"),
            Bool(false) => write!(f, "#f"),
            Nil => write!(f, "()"),
            ConsList(cons) => {
                write!(f, "(")?;
                (&**cons).fmtCons(f)?;
                write!(f, ")")
            }
        }
    }
}
