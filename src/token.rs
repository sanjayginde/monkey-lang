use strum_macros::EnumString;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, EnumString, Clone, Copy, Hash)]
pub enum TokenType {
    #[strum(serialize = "ILLEGAL")]
    ILLEGAL,
    #[strum(serialize = "EOF")]
    EOF,

    // Identifiers + literals
    #[strum(serialize = "IDENT")]
    IDENT,
    #[strum(serialize = "INT")]
    INT,

    // Operators
    #[strum(serialize = "=")]
    ASSIGN,

    #[strum(serialize = "+")]
    PLUS,

    // Delimiters
    #[strum(serialize = ",")]
    COMMA,
    #[strum(serialize = ";")]
    SEMICOLON,

    #[strum(serialize = "(")]
    LPAREN,
    #[strum(serialize = ")")]
    RPAREN,
    #[strum(serialize = "{")]
    LBRACE,
    #[strum(serialize = "}")]
    RBRACE,

    // Keywords
    #[strum(serialize = "FUNCTION")]
    FUNCTION,
    #[strum(serialize = "LET")]
    LET,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
