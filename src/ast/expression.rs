use crate::{
    ast::{IntegerLiteral, Node},
    token::Token,
};

#[derive(Debug)]
pub enum Expression {
    // Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    OperatorExpression(OperatorExpression),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::IntegerLiteral(expr) => expr.token_literal(),
            Expression::OperatorExpression(expr) => expr.token_literal(),
        }
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

// Helper methods for constructing expressions
impl Expression {
    pub fn integer(value: i64) -> Self {
        Expression::IntegerLiteral(IntegerLiteral {
            token: Token::Int(value),
            value,
        })
    }

    pub fn operator(left: IntegerLiteral, operator: Token, right: IntegerLiteral) -> Self {
        Expression::OperatorExpression(OperatorExpression {
            left: left,
            operator,
            right: right,
        })
    }
}
