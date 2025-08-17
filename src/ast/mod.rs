mod expression;
pub mod prelude;
mod program;
mod statement;

use std::fmt::Display;

pub use expression::Expression;

pub use statement::{
    BlockStatement, ExpressionStatement, LetStatement, ReturnStatement, Statement,
};

pub use program::Program;

use crate::token::Token;

pub enum Node {
    Ident(Identifier),
    Func(FunctionLiteral),
    Int(IntegerLiteral),
    Bool(BooleanLiteral),
    Stmt(Statement),
    Expr(Expression),
}

impl Node {
    fn token_literal(&self) -> String {
        match &self {
            Node::Ident(i) => i.token_literal(),
            Node::Func(f) => f.token_literal(),
            Node::Int(i) => i.token_literal(),
            Node::Bool(b) => b.token_literal(),
            Node::Stmt(s) => s.token_literal(),
            Node::Expr(e) => e.token_literal(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub name: String,
}

impl Identifier {
    pub fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}

impl BooleanLiteral {
    pub fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for BooleanLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub name: Identifier,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl FunctionLiteral {
    pub fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self
            .parameters
            .iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "fn {}({params}) {}", self.name, self.body)
    }
}
