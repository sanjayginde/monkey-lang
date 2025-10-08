#![allow(dead_code)]

mod ast;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands. Type 'exit' or 'quit' to exit.");
    repl::start_repl();
}
