use crate::ast::LispVal::*;
use crate::ast::*;
use std::collections::hash_map::{Entry, HashMap};

pub type Env = HashMap<String, LispVal>;

use LispErr::*;

pub fn eval(env: &mut Env, e: &LispVal) -> Result<LispVal, LispErr> {
    // pattern matching really sucks on 'Rc's.  This makes pattern matching really suck for ConsList, so
    // matching on special forms also really sucks.
    // to get around that, transform the cons list into a slice
    // problem: Atom("foo") and cons(Atom("foo", Nil)) will both be transformed to [Atom("foo")], so we need to test to see if we're matching on a list or not.
    match e.iter().collect::<Vec<&LispVal>>().as_slice() {
        [Number(_)] | [Str(_)] | [Bool(_)] if !e.is_cons() => Ok(e.clone()),
        [] if *e == Nil => Ok(e.clone()),
        [Atom(a)] if !e.is_cons() => env
            .get(a)
            .cloned()
            .ok_or_else(|| UnboundVar("Retrieved an unbound variable".to_string(), a.clone())),
        [Atom(quote), quoted] if quote == "quote" => Ok((*quoted).clone()),
        [Atom(set), Atom(var), form] if set == "set!" => set_var(env, var.to_string(), form),
        [Atom(define), Atom(var), form] if define == "define" => {
            define_var(env, var.to_string(), form)
        }
        [Atom(iff), cond, if_branch, else_branch] if iff == "if" => {
            match eval(env, cond) {
                Ok(Bool(true)) => eval(env, if_branch),
                Ok(Bool(false)) => eval(env, else_branch),
                Ok(expr) => Err(TypeMismatch("if's condition must evaluate to a boolean".to_string(), expr)),
                e@Err(_) => e
            }
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
        "debug" => Some(print_debug(args)),
        "+" => Some(monoidal_numeric_op(|x, y| x + y, 0, args)),
        "*" => Some(monoidal_numeric_op(|x, y| x * y, 1, args)),
        "quotient" =>  Some(binary_numeric_op(|x, y| Number(x / y), args)),
        "remainder" => Some(binary_numeric_op(|x, y| Number(x % y), args)),
        // todo: - and / should really take n args and work as negation/reciprocal for 1 arg
        "-" => Some(binary_numeric_op(|x, y| Number(x - y), args)),
        "/" => Some(binary_numeric_op(|x, y| Number(x / y), args)),

        // todo: these should be n-ary
        "=" => Some(binary_numeric_op(|x, y| Bool(x == y), args)),
        ">" => Some(binary_numeric_op(|x, y| Bool(x > y), args)),
        "<" => Some(binary_numeric_op(|x, y| Bool(x < y), args)),
        ">=" => Some(binary_numeric_op(|x, y| Bool(x >= y), args)),
        "<=" => Some(binary_numeric_op(|x, y| Bool(x <= y), args)),

        "||" => Some(monoidal_op(|x, y| x || y, |b: &LispVal| b.boolean(),|b| Bool(b), true, args)),
        "&&" => Some(monoidal_op(|x, y| x && y, |b: &LispVal| b.boolean(),|b| Bool(b), false, args)),

        "eq?" => Some(eqv(args)),
        "eqv?" => Some(eqv(args)),
        // todo: equal

        "cons" => Some(binary_op( LispVal::cons, args)),
        "car" => Some(unary_op( LispVal::car, args)),
        "cdr" => Some(unary_op( LispVal::cdr, args)),
        _ => None,
    }
}

pub fn print_debug(args: &[LispVal]) -> Result<LispVal, LispErr> {
    for arg in args.iter() {
        println!("{:?}", arg)
    }
    Ok(args[0].clone())
}

pub fn eqv(args: &[LispVal]) -> Result<LispVal, LispErr> {
    match args {
        [Number(x), Number(y)] => Ok(Bool(*x == *y)),
        [Atom(x), Atom(y)] => Ok(Bool(*x == *y)),
        [Nil, Nil] => Ok(Bool(true)),
        [Str(x), Str(y)] => Ok(Bool(*x == *y)),
        [Bool(x), Bool(y)] => Ok(Bool(*x == *y)),
        [ConsList(x), ConsList(y)] => if x.car != y.car { Ok(Bool(false)) } else { eqv(&[x.cdr.as_ref().clone(), y.cdr.as_ref().clone()]) },
        [_, _] => Ok(Bool(false)), // no implicit conversions
        _ => Err(NumArgs(2, Nil)), // todo: fix err
    }
}

pub fn unary_op<F>(f: F, args: &[LispVal]) -> Result<LispVal, LispErr>
where
    F: Fn(&LispVal) -> Result<LispVal, LispErr>,
{
    match args {
        [x] => f(&x),
        _ => Err(NumArgs(1, Nil)), // todo: fix err
    }
}

pub fn binary_op<F>(f: F, args: &[LispVal]) -> Result<LispVal, LispErr>
where
    F: Fn(LispVal, LispVal) -> LispVal,
{
    match args {
        [x, y] => Ok(f(x.clone(),y.clone())),
        _ => Err(NumArgs(2, Nil)), // todo: fix err
    }
}

pub fn binary_numeric_op<F>(f: F, args: &[LispVal]) -> Result<LispVal, LispErr>
where
    F: Fn(i32, i32) -> LispVal,
{
    match args {
        [Number(x), Number(y)] => Ok(f(*x,*y)),
        [_, _] => Err(TypeMismatch("Wrong type arguments for primive function".to_string(), Nil)), // todo: fix err
        _ => Err(NumArgs(2, Nil)), // todo: fix err
    }
}

pub fn monoidal_op<F, G, H, A>(f: F, from_lispval: G, to_lispval: H, init: A, args: &[LispVal]) -> Result<LispVal, LispErr>
where
    F: Fn(A, A) -> A,
    G: Fn(&LispVal) -> Result<A, LispErr>,
    H: Fn(A) -> LispVal,
{
    let mut res = init;
    for arg in args {
        res = f(res, from_lispval(arg)?);
    }
    Ok(to_lispval(res))
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

