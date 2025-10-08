use std::fmt;
use std::rc::Rc;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub enum Token {
    Illegal(Rc<str>),
    Eof,

    // Identifiers + literals
    Ident(Rc<str>),
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
        write!(
            f,
            "{}",
            match self {
                Token::Illegal(s) => s,
                Token::Eof => "EOF",
                Token::Ident(s) => s,
                Token::Int(i) => return write!(f, "{i}"),
                Token::Assign => "=",
                Token::Plus => "+",
                Token::Minus => "-",
                Token::Bang => "!",
                Token::Asterisk => "*",
                Token::Slash => "/",
                Token::LessThan => "<",
                Token::GreaterThan => ">",
                Token::Equal => "==",
                Token::NotEqual => "!=",
                Token::Comma => ",",
                Token::Semicolon => ";",
                Token::LeftParen => "(",
                Token::RightParen => ")",
                Token::LeftBrace => "{",
                Token::RightBrace => "}",
                Token::Function => "fn",
                Token::Let => "let",
                Token::True => "true",
                Token::False => "false",
                Token::If => "if",
                Token::Else => "else",
                Token::Return => "return",
            }
        )
    }
}

impl Token {
    pub fn precedence(&self) -> Precedence {
        match &self {
            Token::Equal | Token::NotEqual => Precedence::Equals,
            Token::LessThan | Token::GreaterThan => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash => Precedence::Product,
            Token::LeftParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    fn enum_index(&self) -> u8 {
        match self {
            Precedence::Lowest => 0,
            Precedence::Equals => 1,
            Precedence::LessGreater => 2,
            Precedence::Sum => 3,
            Precedence::Product => 4,
            Precedence::Prefix => 5,
            Precedence::Call => 6,
        }
    }
}

impl Ord for Precedence {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.enum_index().cmp(&other.enum_index())
    }
}

impl PartialOrd for Precedence {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
