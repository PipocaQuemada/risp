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

fn evals_to(e: &str, to: &str) {
    let expr = eval_str(e);
    let res = parse(to);
    assert_eq!(expr, res)
}

#[test]
fn test_cons() {
    evals_to("(cons 1 '())", "(1)");
    evals_to("(cons 2 (cons 1 '()))", "(2 1)");
    evals_to("(cons 3 (cons 2 (cons 1 '())))", "(3 2 1)");
}

#[test]
fn test_car() {
    evals_to("(car (cons 1 2))", "1");
}

#[test]
fn test_cdr() {
    evals_to("(cdr (cons 1 2))", "2");
    evals_to("(cdr '(1))", "()");
    evals_to("(cdr '((1 2 3) 4))", "(4)");
}

