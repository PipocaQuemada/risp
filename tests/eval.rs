use risp::ast::LispVal::*;
use risp::ast::*;
use risp::eval::*;
use risp::eval::Env;
use risp::parser::parser_combinator;
use std::collections::hash_map::HashMap;


fn eval_str(expr: &str) -> Result<LispVal, LispErr> {
    let mut env: Env = HashMap::new();
    eval_str_with_env(&mut env, expr)
}
fn eval_str_with_env(env: &mut Env, expr: &str) -> Result<LispVal, LispErr> {
    parser_combinator::scheme(expr)
        .and_then(|ast| eval(env, &ast))
}
fn parse(expr: &str) -> Result<LispVal, LispErr> {
    parser_combinator::scheme(expr)
}

#[test]
fn test_car() {
    let expr = eval_str("(car (cons 1 2))");
    let res = parse("1");
    assert_eq!(expr, res)
}

#[test]
fn test_cdr() {
    let expr = eval_str("(cdr (cons 1 2))");
    let res = parse("2");
    assert_eq!(expr, res)
}

