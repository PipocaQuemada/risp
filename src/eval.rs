use crate::ast::LispVal::*;
use crate::ast::*;
use std::collections::HashMap;

type Env = HashMap<String, LispVal>;

use LispErr::*;

pub fn eval(env: &Env, e: &LispVal) -> Result<LispVal, LispErr> {
    match e {
        Nil | Number(_) | Str(_) | Bool(_) => Ok(e.clone()),
        Atom(a) => env.get(a).cloned().ok_or_else(|| UnboundVar(
            "Retrieved an unbound variable".to_string(),
            a.clone(),
        )),
        ConsList(c) => {
            match &*c.car {
                Atom(a) => {
                    match a.as_str() {
                        "quote" => Ok(c.cdr.clone()),
                        //"if" => // todo
                        _ => {
                            let args = eval_args(env, &c.cdr)?;
                            apply(&a, &args)
                        }
                    }
                }
                _ => Err(BadSpecialForm(
                    "Unrecognized special form".to_string(),
                    e.clone(),
                )), // evalFunction(env, &*c.car, &c.cdr),
            }
        }
    }
}

pub fn eval_args(env: &Env, args: &LispVal) -> Result<Vec<LispVal>, LispErr> {
    let mut v = Vec::new();

    for arg in args.iter() {
        v.push(eval(env, arg)?);
    }

    Ok(v)
}

pub fn apply(func: &str, args: &[LispVal]) -> Result<LispVal, LispErr> {
    apply_prim(func, args).unwrap_or_else(|| Err(NotFunction(
        func.to_string(),
        "is not a function".to_string(),
    )))
}

pub fn apply_prim(func: &str, args: &[LispVal]) -> Option<Result<LispVal, LispErr>> {
    match func {
        "+" => Some(monoidal_numeric_op(|x, y| x + y, 0, args)),
        "*" => Some(monoidal_numeric_op(|x, y| x * y, 1, args)),
        _ => None,
    }
}

// According to the r5rs spec, monoidal numeric functions return the identity element if they're invoked with 0 args
// e.g. (+) evaluates to 0, and (*) evaluates to 1
pub fn monoidal_numeric_op<F>(f: F, init: i32, args: &[LispVal]) -> Result<LispVal, LispErr>
where
    F: Fn(i32, i32) -> i32,
{
    let res = args.iter().fold(Ok(init), |acc, arg| {
        acc.and_then(|x| arg.integer().map(|y| f(x, y)))
    })?;
    Ok(Number(res))
}
