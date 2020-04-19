use std::fmt::Display;
use std::fmt;
use LispVal::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum LispVal {
    Atom(String),
    List(Vec<LispVal>),
    DottedList(Vec<LispVal>, Box<LispVal>),
    Number(i32),
    Str(String),
    Bool(bool),
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
      List(xs) => {
        match xs.split_last(){
          None => write!(f, "()"),
          Some((last, prev)) => {
            write!(f, "(");
            for e in prev {
              write!(f, "{} ", e);
            }
            write!(f, "{})", last)
          }
        }
      },
      DottedList(xs, cdr) => {
        match xs.split_last(){
          None => write!(f, "( . {})", cdr),
          Some((last, prev)) => {
            write!(f, "(");
            for e in prev {
              write!(f, "{} ", e);
            }
            write!(f, "{} . {})", last, cdr)
          }
        }
      },
    }
  }

}
