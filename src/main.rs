use repl::start_repl;

mod lexer;
mod repl;
mod token;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands. Type 'exit' or 'quit' to exit.");
    start_repl();
}
