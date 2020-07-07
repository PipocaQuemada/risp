use crate::ast::LispVal::*;
use crate::ast::*;
use std::collections::HashMap;

type Env = HashMap<String, LispVal>;

use LispErr::*;

pub fn eval<'a>(env: &'a Env, e: &'a LispVal) -> Result<LispVal, LispErr> {
    match e {
        Nil | Number(_) | Str(_) | Bool(_) => Ok(e.clone()),
        Atom(a) => env.get(a).cloned().ok_or(UnboundVar(
            "Retrieved an unbound variable".to_string(),
            a.clone(),
        )),
        ConsList(c) => {
            match (&*c.car) {
                Atom(a) => {
                    match a.as_str() {
                        "quote" => Ok(c.cdr.clone()),
                        //"if" => // todo
                        _ => evalFunction(env, a, &c.cdr),
                    }
                }
                _ =>  Err(BadSpecialForm("Unrecognized special form".to_string(), e.clone())), // evalFunction(env, &*c.car, &c.cdr),
            }
        }
    }
}

pub fn evalFunction<'a>(
    env: &'a Env,
    func: &str,
    cdr: &'a LispVal,
) -> Result<LispVal, LispErr> {
    let args = evalArgs(env, cdr)?;
    apply(&func, &args)

    /*
    match func {
      Atom(a) =>

      _ => Err(TypeMismatch("expected an atom".to_string(), func.clone()))
    }
    */
}

pub fn evalArgs<'a>(env: &'a Env, mut args: &'a LispVal) -> Result<Vec<LispVal>, LispErr> {
    let mut v = Vec::new();

    for arg in args.iter() {
        v.push(eval(env, arg)?);
    }

    Ok(v)
}

pub fn apply<'a>(func: &str, args: &[LispVal]) -> Result<LispVal, LispErr> {
    apply_prim(func, args)
      .unwrap_or(Err(NotFunction(func.to_string(), "is not a function".to_string())))
}

pub fn apply_prim<'a>(func: &str, args: &[LispVal]) -> Option<Result<LispVal, LispErr>> {
  match func {
    "+" => Some(monoidal_numeric_op(|x, y| x + y, 0, args)),
    "*" => Some(monoidal_numeric_op(|x, y| x * y, 1, args)),
    _ => None
  }
}

// According to the r5rs spec, monoidal numeric functions return the identity element if they're invoked with 0 args
// e.g. (+) evaluates to 0, and (*) evaluates to 1
pub fn monoidal_numeric_op<'a, F>(f: F , init: i32, args: &[LispVal]) -> Result<LispVal, LispErr>
  where
    F: Fn(i32, i32) -> i32
{
  let res = args.iter().fold(Ok(init), |acc, arg| acc.and_then(|x| arg.integer().map(|y| f(x, y))))?;
  Ok(Number(res))
}
