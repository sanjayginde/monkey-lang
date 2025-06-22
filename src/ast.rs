use crate::token::Token;
use std::any::Any;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node + Any {
    fn statement_node(&self);
}

pub trait Expression: Node + Any {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Program {
        Program { statements: vec![] }
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

pub struct LetStatement {
    pub token: Token,
    // Token token.Token // the token.LET token
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}
impl Statement for LetStatement {
    fn statement_node(&self) {}
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Identifier {
    pub token: Token,
    pub name: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {
        todo!()
    }
}

pub struct OperatorExpression {
    pub left: IntegerLiteral,
    pub operator: Token,
    pub right: IntegerLiteral,
}

impl Node for OperatorExpression {
    fn token_literal(&self) -> String {
        self.operator.to_string()
    }
}

impl Expression for OperatorExpression {
    fn expression_node(&self) {
        todo!()
    }
}