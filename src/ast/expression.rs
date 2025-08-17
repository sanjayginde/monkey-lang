use std::fmt::Display;

use crate::{ast::prelude::*, token::Token};

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Integer(IntegerLiteral),
    Boolean(BooleanLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    If(IfExpression),
    Call(CallExpression),
    Function(FunctionLiteral),
}

impl Expression {
    pub fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(expr) => expr.token_literal(),
            Expression::Integer(expr) => expr.token_literal(),
            Expression::Boolean(expr) => expr.token_literal(),
            Expression::Prefix(expr) => expr.token_literal(),
            Expression::Infix(expr) => expr.token_literal(),
            Expression::If(expr) => expr.token_literal(),
            Expression::Call(expr) => expr.token_literal(),
            Expression::Function(expr) => expr.token_literal(),
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
            Expression::If(expr) => expr.fmt(f),
            Expression::Call(expr) => expr.fmt(f),
            Expression::Function(expr) => expr.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub operator: Token,
    pub expression: Box<Expression>,
}

impl PrefixExpression {
    pub fn token_literal(&self) -> String {
        self.operator.to_string()
    }
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.expression)
    }
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

impl InfixExpression {
    pub fn token_literal(&self) -> String {
        self.operator.to_string()
    }
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl IfExpression {
    pub fn token_literal(&self) -> String {
        Token::If.to_string()
    }
}

impl Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alternative = self
            .alternative
            .as_ref()
            .map(|alternative| format!("\nelse {alternative}"))
            .unwrap_or("".to_string());

        write!(
            f,
            "if {} {}{}",
            self.condition, self.consequence, alternative
        )
    }
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl CallExpression {
    pub fn token_literal(&self) -> String {
        Token::LeftParen.to_string()
    }
}

impl Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = self
            .arguments
            .iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{}({})", self.function, args)
    }
}

// Helper methods for constructing expressions
impl Expression {
    pub fn identifier(identifier: Identifier) -> Self {
        Expression::Identifier(identifier)
    }

    pub fn integer(integer: IntegerLiteral) -> Self {
        Expression::Integer(integer)
    }

    pub fn boolean(boolean: BooleanLiteral) -> Self {
        Expression::Boolean(boolean)
    }

    pub fn prefix(operator: Token, expression: Expression) -> Self {
        Expression::Prefix(PrefixExpression {
            operator,
            expression: Box::new(expression),
        })
    }

    pub fn infix(left: Expression, operator: Token, right: Expression) -> Self {
        Expression::Infix(InfixExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn if_expression(
        condition: Expression,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> Self {
        Expression::If(IfExpression {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }

    pub fn function(name: Identifier, parameters: Vec<Identifier>, body: BlockStatement) -> Self {
        Expression::Function(FunctionLiteral {
            token: Token::Function,
            name,
            parameters,
            body,
        })
    }

    pub fn call(function: Expression, arguments: Vec<Expression>) -> Self {
        Expression::Call(CallExpression {
            function: Box::new(function),
            arguments,
        })
    }
}

impl Expression {
    pub fn as_identifier(&self) -> Option<&Identifier> {
        match self {
            Expression::Identifier(expr) => Some(expr),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<&IntegerLiteral> {
        match self {
            Expression::Integer(expr) => Some(expr),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<&BooleanLiteral> {
        match self {
            Expression::Boolean(expr) => Some(expr),
            _ => None,
        }
    }

    pub fn as_prefix(&self) -> Option<&PrefixExpression> {
        match self {
            Expression::Prefix(expr) => Some(expr),
            _ => None,
        }
    }

    pub fn as_infix(&self) -> Option<&InfixExpression> {
        match self {
            Expression::Infix(expr) => Some(expr),
            _ => None,
        }
    }

    pub fn as_if(&self) -> Option<&IfExpression> {
        match self {
            Expression::If(expr) => Some(expr),
            _ => None,
        }
    }

    pub fn as_call(&self) -> Option<&CallExpression> {
        match self {
            Expression::Call(expr) => Some(expr),
            _ => None,
        }
    }
}
