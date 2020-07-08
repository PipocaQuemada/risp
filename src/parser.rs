pub mod parser_combinator {
    extern crate nom;
    use crate::ast::LispErr;
    use crate::ast::LispErr::ParseError;
    use crate::ast::LispVal;
    use nom::{
        branch::alt,
        bytes::complete::{escaped, tag},
        character::complete::{alpha1, alphanumeric1, digit1, none_of, one_of, space1},
        combinator::{all_consuming, flat_map, map},
        do_parse,
        multi::{many0, separated_nonempty_list},
        named,
        sequence::{delimited, separated_pair},
        IResult,
    };
    use std::str::FromStr;

    pub fn boolean(i: &str) -> IResult<&str, LispVal> {
        let t = map(tag("#t"), |_t| LispVal::Bool(true));
        let f = map(tag("#f"), |_t| LispVal::Bool(false));
        alt((t, f))(i)
    }

    pub fn string(i: &str) -> IResult<&str, LispVal> {
        // TODO: for some reason, using escaped results in the tests spinning forever.
        // This "works", although it doesn't handle escaping characters.

        //let contents = escaped(many0(none_of("\"")), '\\', one_of("n\\\"t"));
        // map(string, |s: &str| LispVal::Str(s.into()))(i)
        let contents = many0(none_of("\""));
        let string = delimited(tag("\""), contents, tag("\""));
        map(string, |s: Vec<char>| LispVal::Str(s.into_iter().collect()))(i)
    }
    // one_of returns a parser of char, while alpha1 returns a parser of &str.
    // to get the types to line up, use one_of to reimplement alpha1 for now.
    pub fn symbol(i: &str) -> IResult<&str, char> {
        one_of("!#$%&|*+-/:<=>?@^_~")(i)
    }
    pub fn alpha(i: &str) -> IResult<&str, char> {
        one_of("qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM")(i)
    }
    pub fn alphanumeric(i: &str) -> IResult<&str, char> {
        one_of("qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890")(i)
    }

    pub fn atom(i: &str) -> IResult<&str, LispVal> {
        let first = alt((alpha, symbol));
        //let rest = many0(alt((alphanumeric,symbol)) );
        fn rest(i: &str) -> IResult<&str, Vec<char>> {
            many0(alt((alphanumeric, symbol)))(i)
        };
        flat_map(first, |f| {
            map(rest, move |r| {
                let mut string = String::new();
                string.push(f);
                for c in r {
                    string.push(c);
                }
                LispVal::Atom(string)
            })
        })(i)
    }

    pub fn number(i: &str) -> IResult<&str, LispVal> {
        map(digit1, |s| LispVal::Number(i32::from_str(s).unwrap()))(i)
    }

    pub fn list(i: &str) -> IResult<&str, LispVal> {
        fn to_list(exprs: Vec<LispVal>) -> LispVal {
            let mut list = LispVal::Nil;
            for e in exprs.iter().rev() {
                list = LispVal::cons(e.clone(), list)
            }
            list
        }

        let list_contents = map(separated_nonempty_list(space1, expr), |exprs| {
            to_list(exprs)
        });

        let empty = map(tag("()"), |_| LispVal::Nil);
        let non_empty = delimited(tag("("), list_contents, tag(")"));
        alt((empty, non_empty))(i)
    }

    pub fn quoted(i: &str) -> IResult<&str, LispVal> {
        flat_map(tag("\'"), |_| {
            map(expr, |e| {
                LispVal::cons(
                    LispVal::Atom("quote".to_string()),
                    LispVal::cons(e, LispVal::Nil),
                )
            })
        })(i)
    }

    pub fn dotted_list(i: &str) -> IResult<&str, LispVal> {
        fn to_list(exprs: &[LispVal], last: LispVal) -> LispVal {
            let mut list = last;
            for e in exprs.iter().rev() {
                list = LispVal::cons(e.clone(), list)
            }
            list
        }

        /*
        flat_map(separated_nonempty_list(space1, expr), |first| {
          flat_map(space1, |_| {
          flat_map(tag("."), |_| {
          flat_map(space1, |_| {
          map(expr, |end| {
              to_list(&first, end)
          })})})})})(i)
        */

        let contents = flat_map(separated_nonempty_list(space1, expr), |first| {
            map(delimited(tag(" . "), expr, tag("")), move |end| {
                to_list(&first, end)
            })
        });

        delimited(tag("("), contents, tag(")"))(i)
    }

    pub fn expr(i: &str) -> IResult<&str, LispVal> {
        alt((atom, number, string, boolean, dotted_list, list, quoted))(i)
    }

    pub fn scheme(i: &str) -> Result<LispVal, LispErr> {
        all_consuming(expr)(i)
            .map(|(_, expr)| expr)
            .map_err(|err| ParseError(err.to_string()))
    }
}

mod recursive_descent {
    /*
    pub fn tokenize<'a>(source: &'a String) -> SplitWhitespace<'a> {
      source
        .replace("(", " ( ")
        //.replace(")", " ) ")
        .split_whitespace()
    }

    pub fn tokenize<'a>(source: &'a String) -> SplitWhitespace<'a> {
      let source2 = source
        .replace("(", " ( ")
        .replace(")", " ) ");

      Cow::from(source2)
        .split_whitespace()
    }
    */

    /*
    fn padParens (source: &String)-> String {
      source
        .replace("(", " ( ")
        .replace(")", " ) ")
    }

    pub fn lex_and_parse(source: &String) -> Option<ast::LispVal> {
      let padded = padParens(source);
      let mut tokenized = padded.split_whitespace();
      parse(&mut tokenized).1
    }
    pub fn parse<'a>(mut tokens: &'a mut SplitWhitespace<'a>) -> (&'a mut SplitWhitespace<'a>, Option<ast::LispVal>) {
      let lispVal = match tokens.next() {
        Some("true") => Some(ast::LispVal::Bool(true)),
        Some("false") => Some(ast::LispVal::Bool(false)),
        Some("(") => {
            let list = parseList(tokens);
            tokens = list.0;
            list.1
          },
        Some(atom) => Some(ast::LispVal::Atom(atom.to_string())),
        None => None
      };
      (tokens, lispVal)
    }

    pub fn parseList<'a>(mut tokens: &'a mut SplitWhitespace<'a>) -> (&'a mut SplitWhitespace<'a>, Option<ast::LispVal>) {
      let mut list = Vec::new();
      let lispVal = loop {
        match tokens.peekable().peek() {
          Some(&")") => break Some(ast::LispVal::List(Box::new(list))),
          None => break None,
          Some(&nonList) => {
              match parse(tokens) {
                (t, Some(lispVal)) => {
                  tokens = t;
                  list.push(lispVal)
                  },
                (_, None) => break None,
              }
            },
        }
      };
      (tokens, lispVal)
    }
    */
}
