use crate::ast::LispVal::*;
use crate::ast::*;
use std::collections::hash_map::{Entry, HashMap};

type Env = HashMap<String, LispVal>;

use LispErr::*;

pub fn eval(env: &mut Env, e: &LispVal) -> Result<LispVal, LispErr> {
    // pattern matching really sucks on 'Rc's.  This makes pattern matching really suck for ConsList, so
    // matching on special forms also really sucks.
    // to get around that, transform the cons list into a slice
    // problem: Atom("foo") and cons(Atom("foo", Nil)) will both be transformed to [Atom("foo")], so we need to test to see if we're matching on a list or not.
    match e.iter().collect::<Vec<&LispVal>>().as_slice() {
        [Nil] | [Number(_)] | [Str(_)] | [Bool(_)] if !e.is_cons() => Ok(e.clone()),
        [Atom(a)] if !e.is_cons() => env
            .get(a)
            .cloned()
            .ok_or_else(|| UnboundVar("Retrieved an unbound variable".to_string(), a.clone())),
        [Atom(quote), ..] if quote == "quote" => e.cdr(),
        [Atom(set), Atom(var), form] if set == "set!" => set_var(env, var.to_string(), form),
        [Atom(define), Atom(var), form] if define == "define" => {
            define_var(env, var.to_string(), form)
        }
        [Atom(a), ..] => {
            let args = eval_args(env, &(e.cdr()?))?;
            apply(&a, &args)
        }
        _ => Err(BadSpecialForm(
            "Unrecognized special form".to_string(),
            e.clone(),
        )),
    }
}

pub fn define_var(env: &mut Env, var: String, form: &LispVal) -> Result<LispVal, LispErr> {
    Ok(env
        .entry(var)
        .and_modify(|e| *e = form.clone())
        .or_insert_with(|| form.clone())
        .clone())
}

pub fn set_var(env: &mut Env, var: String, form: &LispVal) -> Result<LispVal, LispErr> {
    match env.entry(var) {
        Entry::Occupied(mut entry) => {
            entry.insert(form.clone());
            Ok(form.clone())
        }
        Entry::Vacant(_) => Err(UnboundVar("".to_string(), "".to_string())), //todo: err message
    }
}

pub fn eval_args(env: &mut Env, args: &LispVal) -> Result<Vec<LispVal>, LispErr> {
    let mut v = Vec::new();

    for arg in args.iter() {
        v.push(eval(env, arg)?);
    }

    Ok(v)
}

pub fn apply(func: &str, args: &[LispVal]) -> Result<LispVal, LispErr> {
    apply_prim(func, args).unwrap_or_else(|| {
        Err(NotFunction(
            func.to_string(),
            "is not a function".to_string(),
        ))
    })
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
