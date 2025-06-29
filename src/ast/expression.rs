use std::fmt::Display;

use crate::{
    ast::{BooleanLiteral, Identifier, IntegerLiteral, Node},
    token::Token,
};

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    Integer(IntegerLiteral),
    Boolean(BooleanLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(expr) => expr.token_literal(),
            Expression::Integer(expr) => expr.token_literal(),
            Expression::Boolean(expr) => expr.token_literal(),
            Expression::Prefix(expr) => expr.token_literal(),
            Expression::Infix(expr) => expr.token_literal(),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(expr) => expr.fmt(f),
            Expression::Integer(expr) => expr.fmt(f),
            Expression::Boolean(expr) => expr.fmt(f),
            Expression::Prefix(expr) => expr.fmt(f),
            Expression::Infix(expr) => expr.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub operator: Token,
    pub right: Box<Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.operator.to_string()
    }
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.operator.to_string()
    }
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

// Helper methods for constructing expressions
impl Expression {
    pub fn integer(value: i64) -> Self {
        Expression::Integer(IntegerLiteral {
            token: Token::Int(value),
            value,
        })
    }

    pub fn operator(left: Expression, operator: Token, right: Expression) -> Self {
        Expression::Infix(InfixExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}

impl Expression {
    pub fn as_identifier(&self) -> Option<&Identifier> {
        match self {
            Expression::Identifier(exp) => Some(exp),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<&IntegerLiteral> {
        match self {
            Expression::Integer(stmt) => Some(stmt),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<&BooleanLiteral> {
        match self {
            Expression::Boolean(stmt) => Some(stmt),
            _ => None,
        }
    }

    pub fn as_prefix(&self) -> Option<&PrefixExpression> {
        match self {
            Expression::Prefix(stmt) => Some(stmt),
            _ => None,
        }
    }

    pub fn as_infix(&self) -> Option<&InfixExpression> {
        match self {
            Expression::Infix(stmt) => Some(stmt),
            _ => None,
        }
    }
}
