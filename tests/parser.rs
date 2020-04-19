use risp::ast::LispVal::{Bool, Str, Atom};
use risp::parser::parser_combinator;

use nom::combinator::all_consuming;
use nom::{error::ErrorKind, Err::Error, IResult};

#[test]
fn test_parse_true() {
    let b = parser_combinator::boolean("#t");
    assert_eq!(b, Ok(("", Bool(true))))
}

#[test]
fn test_parse_fail_not_true() {
    fn parse(string: &str) {
        let b = parser_combinator::boolean(string);
        assert_eq!(b, Err(Error((string, ErrorKind::Tag))))
    }
    parse("true");
    parse("#T");
    parse("TRUE");
    parse("TruE");
    //parse("#tf");
}

/*
#[test]
fn test_parse_atom() {
    fn parseSuccess(string: &str) {
        let a = all_consuming(parser_combinator::atom(string));
        assert_eq!(a, Ok(("", Atom(string.to_string()))))
    }
    fn parseFailed(string: &str) {
        let a = all_consuming(parser_combinator::atom(string));
        assert!(a.is_err(), "When parsing {}, result was {:?}", string, a)
    }
    parseSuccess("qwerty");
    parseSuccess("Qwerty");
    parseSuccess("QWERTY");
    parseSuccess("q1234");
    parseSuccess("Q1234");
    parseSuccess("Q1234!#$%&|*+-/:<=>?@^_~");
    parseFailed("1");
    parseFailed("\\");
}
*/
