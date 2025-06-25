mod expression;
mod program;
mod statement;

pub use expression::{Expression, OperatorExpression};

pub use statement::{LetStatement, ReturnStatement, Statement};

pub use program::Program;

use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub name: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}
