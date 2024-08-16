use strum_macros::EnumString;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, EnumString, Clone, Copy, Hash)]
pub enum TokenType {
    // #[strum(serialize = "ILLEGAL")]
    Illegal,
    // #[strum(serialize = "EOF")]
    Eof,

    // Identifiers + literals
    // #[strum(serialize = "IDENT")]
    Ident,
    // #[strum(serialize = "INT")]
    Int,

    // Operators
    #[strum(serialize = "=")]
    Assign,

    #[strum(serialize = "+")]
    Plus,

    // Delimiters
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = ";")]
    Semicolon,

    #[strum(serialize = "(")]
    LeftParen,
    #[strum(serialize = ")")]
    RightParen,
    #[strum(serialize = "{")]
    LeftBrace,
    #[strum(serialize = "}")]
    RightBrace,

    // Keywords
    // #[strum(serialize = "FUNCTION")]
    Function,
    // #[strum(serialize = "LET")]
    Let,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }

    pub fn from_char(token_type: TokenType, ch: char) -> Token {
        Token::new(token_type, ch.to_string())
    }

    pub fn from_str(token_type: TokenType, str: &str) -> Token {
        Token::new(token_type, str.to_string())
    }
}
