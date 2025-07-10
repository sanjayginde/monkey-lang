use crate::{ast::prelude::*, token::Token};
use std::fmt::Display;

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(stmt) => stmt.token_literal(),
            Statement::Return(stmt) => stmt.token_literal(),
            Statement::Expression(stmt) => stmt.token_literal(),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(stmt) => stmt.fmt(f),
            Statement::Return(stmt) => stmt.fmt(f),
            Statement::Expression(stmt) => stmt.fmt(f),
        }
    }
}

// Helper methods for constructing statements
impl Statement {
    pub fn let_statement(token: Token, name: Identifier, value: Expression) -> Self {
        Statement::Let(LetStatement {
            token,
            identifier: name,
            value,
        })
    }

    pub fn return_statement(value: Expression) -> Self {
        Statement::Return(ReturnStatement {
            token: Token::Return,
            value,
        })
    }

    pub fn expression_statement(token: Token, value: Expression) -> Self {
        Statement::Expression(ExpressionStatement {
            token,
            expression: value,
        })
    }
}

// Pattern matching helpers
impl Statement {
    pub fn as_let(&self) -> Option<&LetStatement> {
        match self {
            Statement::Let(stmt) => Some(stmt),
            _ => None,
        }
    }

    pub fn as_return(&self) -> Option<&ReturnStatement> {
        match self {
            Statement::Return(stmt) => Some(stmt),
            _ => None,
        }
    }

    pub fn as_expression(&self) -> Option<&ExpressionStatement> {
        match self {
            Statement::Expression(stmt) => Some(stmt),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub identifier: Identifier,
    pub value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} = {};", self.token, self.identifier, self.value)
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Expression,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.token, self.value)
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}
