use crate::token::Token::{self, *};

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
        self.ch = self.input.chars().nth(self.read_position);
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        if self.ch.is_none() {
            return Eof;
        }

        let ch = self.ch.unwrap();
        self.read_char();

        match ch {
            '=' => match self.ch {
                Some('=') => {
                    self.read_char();
                    Equal
                }
                _ => Assign,
            },
            '+' => Plus,
            '-' => Minus,
            '!' => match self.ch {
                Some('=') => {
                    self.read_char();
                    NotEqual
                }
                _ => Bang,
            },
            '*' => Asterisk,
            '/' => Slash,
            '<' => LessThan,
            '>' => GreaterThan,
            ';' => Semicolon,
            '(' => LeftParen,
            ')' => RightParen,
            ',' => Comma,
            '{' => LeftBrace,
            '}' => RightBrace,
            _ => {
                if ch.is_whitespace() {
                    while self.ch.is_some() && self.ch.unwrap().is_whitespace() {
                        self.read_char();
                    }

                    self.next_token()
                } else if ch.is_alphabetic() {
                    let mut literal = String::new();
                    literal.push(ch);
                    while self.ch.is_some() && self.ch.unwrap().is_alphabetic() {
                        literal.push(self.ch.unwrap());
                        self.read_char();
                    }

                    match literal.as_str() {
                        "fn" => Function,
                        "let" => Let,
                        "true" => True,
                        "false" => False,
                        "if" => If,
                        "else" => Else,
                        "return" => Return,
                        _ => Ident(literal),
                    }
                } else if ch.is_ascii_digit() {
                    let mut literal = String::new();
                    literal.push(ch);
                    while self.ch.is_some() && self.ch.unwrap().is_ascii_digit() {
                        literal.push(self.ch.unwrap());
                        self.read_char();
                    }

                    let possible_int = literal.parse();
                    match possible_int {
                        Ok(int) => Int(int),
                        Err(_) => Illegal(literal),
                    }
                } else {
                    Illegal(ch.to_string())
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::token::Token::*;

    use super::Lexer;

    #[test]
    fn test_next_token_simple() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input.to_string());

        let expected = vec![
            Assign, Plus, LeftParen, RightParen, LeftBrace, RightBrace, Comma, Semicolon, Eof,
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
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;

"#;
        let mut lexer = Lexer::new(input.to_string());

        let expected = vec![
            Let,
            Ident("five".to_string()),
            Assign,
            Int(5),
            Semicolon,
            Let,
            Ident("ten".to_string()),
            Assign,
            Int(10),
            Semicolon,
            Let,
            Ident("add".to_string()),
            Assign,
            Function,
            LeftParen,
            Ident("x".to_string()),
            Comma,
            Ident("y".to_string()),
            RightParen,
            LeftBrace,
            Ident("x".to_string()),
            Plus,
            Ident("y".to_string()),
            Semicolon,
            RightBrace,
            Semicolon,
            Let,
            Ident("result".to_string()),
            Assign,
            Ident("add".to_string()),
            LeftParen,
            Ident("five".to_string()),
            Comma,
            Ident("ten".to_string()),
            RightParen,
            Semicolon,
            Bang,
            Minus,
            Slash,
            Asterisk,
            Int(5),
            Semicolon,
            Int(5),
            LessThan,
            Int(10),
            GreaterThan,
            Int(5),
            Semicolon,
            If,
            LeftParen,
            Int(5),
            LessThan,
            Int(10),
            RightParen,
            LeftBrace,
            Return,
            True,
            Semicolon,
            RightBrace,
            Else,
            LeftBrace,
            Return,
            False,
            Semicolon,
            RightBrace,
            Int(10),
            Equal,
            Int(10),
            Semicolon,
            Int(10),
            NotEqual,
            Int(9),
            Semicolon,
            Eof,
        ];

        for token in expected {
            let next_token = lexer.next_token();
            println!("{:?}", next_token);
            assert_eq!(next_token, token);
        }
    }
}
