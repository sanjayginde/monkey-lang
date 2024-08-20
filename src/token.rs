#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Token {
    Illegal(String),
    Eof,

    // Identifiers + literals
    Ident(String),
    Int(i64),

    // Operators
    Assign,
    Plus,

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
}
