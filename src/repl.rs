use crate::lexer::Lexer;
use std::io::{self, Write};

pub fn start_repl() {
    let mut input = String::new();
    while !["exit".to_string(), "quit".to_string()].contains(&input.trim().to_string()) {
        input = "".to_string();

        print!(">> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let lexer = Lexer::new(&input);
                for token in lexer {
                    println!("{:?}", token);
                }
            }
            Err(error) => {
                println!("error: {error}");
                break;
            }
        }
    }
}
