use crate::token::Token;
use std::any::Any;
use core::fmt::Debug;

pub trait Node {
    fn token_literal(&self) -> String;
}
impl Debug for dyn Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Node{{{}}}", self.token_literal())
    }
}

pub trait Statement: Node + Any {
    fn statement_node(&self);
}

impl Debug for dyn Statement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Statement{{}}")
    }
}

pub trait Expression: Node + Any {
    fn expression_node(&self);
}

impl Debug for dyn Expression {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Expression{{}}")
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Box<dyn Expression>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}
impl Statement for ReturnStatement {
    fn statement_node(&self) {}
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

impl Expression for IntegerLiteral {
    fn expression_node(&self) {
        todo!()
    }
}

#[derive(Debug)]
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