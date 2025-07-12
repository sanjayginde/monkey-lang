use crate::{lexer::Lexer, parser::Parser};
use std::io::{self, Write};

const MONKEY_FACE: &str = r#"
            __,__
   .--.  .-"     "-.  .--.
  / .. \/  .-. .-.  \/ .. \
 | |  '|  /   Y   \  |'  | |
 | \   \  \ 0 | 0 /  /   / |
  \ '- ,\.-"""""""-./, -' /
   ''-' /_   ^ ^   _\ '-''
       |  \._   _./  |
       \   \ '~' /   /
        '._ '-=-' _.'
           '-----'
"#;

pub fn start_repl() {
    let mut input = String::new();
    while !["exit".to_string(), "quit".to_string()].contains(&input.trim().to_string()) {
        input = "".to_string();

        print!(">> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let mut lexer = Lexer::new(input.clone());
                let mut parser = Parser::new(&mut lexer);

                let program = parser.parse_program();
                if parser.errors.is_empty() {
                    println!("{program}");
                } else {
                    println!("{MONKEY_FACE}");
                    println!("Woops! We ran into some monkey business here!\n");
                    println!("Parser errors:");
                    for error in parser.errors {
                        println!("\t{error}");
                    }
                }
            }
            Err(error) => {
                println!("error: {error}");
                break;
            }
        }
    }
}
