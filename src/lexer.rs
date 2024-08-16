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
        if (self.read_position > self.input.len()) {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }

        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    pub fn next_token(&mut self) -> Token {
        if self.ch.is_none() {
            return Token::from_str(EOF, "");
        }

        let ch = self.ch.unwrap();
        self.read_char();

        println!("sym: {}", ch);
        return match ch {
            '=' => Token::from_char(ASSIGN, ch),
            ';' => Token::from_char(SEMICOLON, ch),
            '(' => Token::from_char(LPAREN, ch),
            ')' => Token::from_char(RPAREN, ch),
            ',' => Token::from_char(COMMA, ch),
            '+' => Token::from_char(PLUS, ch),
            '{' => Token::from_char(LBRACE, ch),
            '}' => Token::from_char(RBRACE, ch),
            _ => Token::from_char(ILLEGAL, ch),
        };
    }
}

#[cfg(test)]
mod test {
    use crate::token::Token;
    use crate::token::TokenType::*;

    use super::Lexer;

    #[test]
    fn test_solve() {
        let input = "=+(){},;";

        let mut lexer = Lexer::new(input.to_string());

        let expected = vec![
            Token::from_str(ASSIGN, "="),
            Token::from_str(PLUS, "+"),
            Token::from_str(LPAREN, "("),
            Token::from_str(RPAREN, ")"),
            Token::from_str(LBRACE, "{"),
            Token::from_str(RBRACE, "}"),
            Token::from_str(COMMA, ","),
            Token::from_str(SEMICOLON, ";"),
            Token::from_str(EOF, ""),
        ];

        for token in expected {
            assert_eq!(lexer.next_token(), token);
        }
    }
}
