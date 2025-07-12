#![allow(dead_code)]

use repl::start_repl;

mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands. Type 'exit' or 'quit' to exit.");
    start_repl();
}
