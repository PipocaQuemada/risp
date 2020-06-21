use risp::ast::LispVal::*;
use risp::ast::*;
use std::rc::Rc;

#[test]
fn test_render_true() {
    let s = format!("{}", Bool(true));
    assert_eq!(s, "#t")
}

#[test]
fn test_render_false() {
    let s = format!("{}", Bool(true));
    assert_eq!(s, "#t")
}

#[test]
fn test_render_string() {
    let s = format!("{}", Str("foo".to_string()));
    assert_eq!(s, "\"foo\"")
}

#[test]
fn test_render_int() {
    let s = format!("{}", Number(1));
    assert_eq!(s, "1")
}

#[test]
fn test_render_empty_list() {
    let s = format!("{}", Nil);
    assert_eq!(s, "()")
}

#[test]
fn test_render_one_item_list() {
    let s = format!(
        "{}",
        ConsList(Rc::new(Cons {
            car: Rc::new(Number(1)),
            cdr: Nil
        }))
    );
    assert_eq!(s, "(1)")
}

#[test]
fn test_render_two_item_list() {
    let s = format!(
        "{}",
        LispVal::cons(Number(1), LispVal::cons(Number(2), Nil))
    );
    assert_eq!(s, "(1 2)")
}

#[test]
fn test_render_one_item_dotted_list() {
    let s = format!("{}", LispVal::cons(Nil, Number(1)));
    assert_eq!(s, "(() . 1)")
}
