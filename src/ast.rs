use crate::eval::Env;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use LispErr::*;
use LispVal::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum LispErr {
    TypeMismatch(String, LispVal),
    BadSpecialForm(String, LispVal),
    ParseError(String),
    NotFunction(String, String),
    UnboundVar(String, String),
    NumArgs(i32, LispVal),
    Default(String),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum LispVal {
    Atom(String),
    ConsList(Rc<Cons>),
    Nil,
    Number(i32),
    Str(String),
    Bool(bool),
    Func {
        params: Vec<String>,
        vararg: Option<String>,
        body: Vec<LispVal>,
        env: Env,
    },
    //PrimitiveFunc(Func)
}

impl LispVal {
    pub fn is_cons(&self) -> bool {
        match self {
            ConsList(_) => true,
            _ => false,
        }
    }

    pub fn car(&self) -> Result<LispVal, LispErr> {
        match self {
            ConsList(c) => Ok(c.car.as_ref().clone()),
            _ => Err(TypeMismatch(
                "Expected an cons cell".to_string(),
                self.clone(),
            )),
        }
    }

    pub fn cdr(&self) -> Result<LispVal, LispErr> {
        match self {
            ConsList(c) => Ok(c.cdr.as_ref().clone()),
            _ => Err(TypeMismatch(
                "Expected a cons cell".to_string(),
                self.clone(),
            )),
        }
    }

    pub fn cons(car: LispVal, cdr: LispVal) -> LispVal {
        ConsList(Rc::new(Cons {
            car: Rc::new(car),
            cdr: Rc::new(cdr),
        }))
    }

    pub fn iter(&self) -> LispIter {
        LispIter { val: self }
    }

    pub fn boolean(&self) -> Result<bool, LispErr> {
        match self {
            Bool(b) => Ok(*b),
            _ => Err(TypeMismatch(
                "Expected an boolean".to_string(),
                self.clone(),
            )),
        }
    }

    pub fn integer(&self) -> Result<i32, LispErr> {
        match self {
            Number(i) => Ok(*i),
            _ => Err(TypeMismatch(
                "Expected an integer".to_string(),
                self.clone(),
            )),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Cons {
    pub car: Rc<LispVal>,
    pub cdr: Rc<LispVal>,
}

impl Cons {
    pub fn is_dotted(&self) -> bool {
        match self.cdr.as_ref() {
            ConsList(cons) => cons.is_dotted(),
            Nil => false,
            _ => true,
        }
    }

    fn fmt_cons(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (&*self.car).fmt(f)?;

        match &self.cdr.as_ref() {
            Nil => write!(f, ""),
            ConsList(cons) => {
                write!(f, " ")?;
                (&*cons).fmt_cons(f)
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
        fn spaced<T: fmt::Display>(vec: &Vec<T>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // slightly hacky way to intercalate a space between every argument
            let mut is_first = true;
            for v in vec.iter() {
                if (is_first) {
                    is_first = false;
                    write!(f, "{}", v)?;
                } else {
                    write!(f, " {}", v)?;
                }
            }
            Ok(())
        }
        match self {
            Func {
                params,
                vararg,
                body,
                env,
            } => {
                write!(f, "(lambda (")?;
                spaced(params, f)?;
                for v in vararg.iter() {
                    //
                    write!(f, " . {}", v)?;
                }
                write!(f, ") ")?;
                spaced(body, f)?;
                write!(f, ")")
            }
            Atom(s) => write!(f, "{}", s),
            Str(s) => write!(f, "\"{}\"", s),
            Number(i) => write!(f, "{}", i),
            Bool(true) => write!(f, "#t"),
            Bool(false) => write!(f, "#f"),
            Nil => write!(f, "()"),
            ConsList(cons) => {
                write!(f, "(")?;
                (&**cons).fmt_cons(f)?;
                write!(f, ")")
            }
        }
    }
}

pub struct LispIter<'a> {
    val: &'a LispVal,
}

impl<'a> Iterator for LispIter<'a> {
    type Item = &'a LispVal;

    fn next(&mut self) -> Option<&'a LispVal> {
        let val = self.val;
        self.val = match self.val {
            ConsList(cons) => &(*cons).cdr,
            _ => &Nil,
        };
        match val {
            ConsList(cons) => Some(&*(*cons).car),
            Nil => None,
            _ => Some(val),
        }
    }
}
