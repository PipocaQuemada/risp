mod ast;
mod eval;
mod parser;

use std::collections::HashMap;

fn main() {
    repl();
}

fn repl() {
    let mut rl = rustyline::Editor::<()>::new();
    let mut env = HashMap::new();
    loop {
        match rl.readline("risp λ") {
            Ok(s) => {
                rl.add_history_entry(s.as_str());
                match s.as_str() {
                    ":q" | "quit" | "exit" => break,
                    ",print_env" => println!(" {:?}", env),
                    input => {
                        match parser::parser_combinator::scheme(input)
                            .and_then(|ast| eval::eval(&mut env, &ast))
                        {
                            Ok(res) => println!("  {}", res),
                            Err(e) => println!("Error: {:?}", e),
                        }
                    }
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
