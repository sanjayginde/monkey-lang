use std::fmt;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub enum Token {
    Illegal(String),
    Eof,

    // Identifiers + literals
    Identifier(String),
    Int(i64),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,

    // Delimiters
    Comma,
    Semicolon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{:?}", self)
        // or, alternatively:
        fmt::Debug::fmt(self, f)
    }
}

trait TokenTrait {}
pub struct Identifier {
    pub name: String,
}

impl TokenTrait for Identifier {}
