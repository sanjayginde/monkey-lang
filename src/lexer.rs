use crate::token::Token;
use crate::token::TokenType::*;

pub struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: Option<char>,     // current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut result = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        result.read_char();
        result
    }

    pub fn read_char(&mut self) {
        if self.read_position > self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        if self.ch.is_none() {
            return Token::from_str(Eof, "");
        }

        let ch = self.ch.unwrap();
        self.read_char();

        println!("sym: {}", ch);
        match ch {
            '=' => Token::from_char(Assign, ch),
            ';' => Token::from_char(Semicolon, ch),
            '(' => Token::from_char(LeftParen, ch),
            ')' => Token::from_char(RightParen, ch),
            ',' => Token::from_char(Comma, ch),
            '+' => Token::from_char(Plus, ch),
            '{' => Token::from_char(LeftBrace, ch),
            '}' => Token::from_char(RightBrace, ch),
            _ => {
                if ch.is_whitespace() {
                    while self.ch.is_some() && self.ch.unwrap().is_whitespace() {
                        self.read_char();
                    }

                    return self.next_token();
                } else if ch.is_alphabetic() {
                    let mut literal = String::new();
                    literal.push(ch);
                    while self.ch.is_some() && self.ch.unwrap().is_alphabetic() {
                        literal.push(self.ch.unwrap());
                        self.read_char();
                    }

                    match literal.as_str() {
                        "let" => Token::from_str(Let, &literal),
                        "fn" => Token::from_str(Function, &literal),
                        _ => Token::from_str(Ident, &literal),
                    }
                } else if ch.is_digit(10) {
                    let mut literal = String::new();
                    literal.push(ch);
                    while self.ch.is_some() && self.ch.unwrap().is_digit(10) {
                        literal.push(self.ch.unwrap());
                        self.read_char();
                    }

                    Token::from_str(Int, &literal)
                } else {
                    Token::from_char(Illegal, ch)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::token::Token;
    use crate::token::TokenType::*;

    use super::Lexer;

    #[test]
    fn test_next_token_simple() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input.to_string());

        let expected = vec![
            Token::from_str(Assign, "="),
            Token::from_str(Plus, "+"),
            Token::from_str(LeftParen, "("),
            Token::from_str(RightParen, ")"),
            Token::from_str(LeftBrace, "{"),
            Token::from_str(RightBrace, "}"),
            Token::from_str(Comma, ","),
            Token::from_str(Semicolon, ";"),
            Token::from_str(Eof, ""),
        ];

        for token in expected {
            assert_eq!(lexer.next_token(), token);
        }
    }

    #[test]
    fn test_next_token_complex() {
        let input = r#"
let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
"#;
        let mut lexer = Lexer::new(input.to_string());

        let expected = vec![
            Token::from_str(Let, "let"),
            Token::from_str(Ident, "five"),
            Token::from_str(Assign, "="),
            Token::from_str(Int, "5"),
            Token::from_str(Semicolon, ";"),
            Token::from_str(Let, "let"),
            Token::from_str(Ident, "ten"),
            Token::from_str(Assign, "="),
            Token::from_str(Int, "10"),
            Token::from_str(Semicolon, ";"),
            Token::from_str(Let, "let"),
            Token::from_str(Ident, "add"),
            Token::from_str(Assign, "="),
            Token::from_str(Function, "fn"),
            Token::from_str(LeftParen, "("),
            Token::from_str(Ident, "x"),
            Token::from_str(Comma, ","),
            Token::from_str(Ident, "y"),
            Token::from_str(RightParen, ")"),
            Token::from_str(LeftBrace, "{"),
            Token::from_str(Ident, "x"),
            Token::from_str(Plus, "+"),
            Token::from_str(Ident, "y"),
            Token::from_str(Semicolon, ";"),
            Token::from_str(RightBrace, "}"),
            Token::from_str(Semicolon, ";"),
            Token::from_str(Let, "let"),
            Token::from_str(Ident, "result"),
            Token::from_str(Assign, "="),
            Token::from_str(Ident, "add"),
            Token::from_str(LeftParen, "("),
            Token::from_str(Ident, "five"),
            Token::from_str(Comma, ","),
            Token::from_str(Ident, "ten"),
            Token::from_str(RightParen, ")"),
            Token::from_str(Semicolon, ";"),
            Token::from_str(Eof, ""),
        ];

        for token in expected {
            let next_token = lexer.next_token();
            println!("{:?}", token);
            assert_eq!(next_token, token);
        }
    }
}
