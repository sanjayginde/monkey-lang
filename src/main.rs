use lexer::Lexer;
use token::Token;

mod lexer;
mod token;

fn main() {
    let input = r#"
let five = 5;
let ten = 10;

let add = fn(x, y) {
x + y;
};

let result = add(five, ten);
"#;
    let mut lexer = Lexer::new(input.to_string());

    while let token = lexer.next_token() {
        println!("{:?}", token);
        if token == Token::Eof {
            break;
        }
    }
}
