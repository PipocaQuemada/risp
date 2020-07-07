mod ast;
mod eval;
mod parser;

use std::collections::HashMap;

fn main() {
    repl();
}

fn repl() {
    let env = HashMap::new();

    let input = "(+ 1 1)";

    let ast = parser::parser_combinator::scheme(input).unwrap().1;
    let evaled = eval::eval(&env, &ast).unwrap();
    println!("> {}", evaled)
}
