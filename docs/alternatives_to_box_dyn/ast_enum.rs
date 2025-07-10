use crate::token::Token;
use core::fmt::Debug;

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    OperatorExpression(OperatorExpression),
    Boolean(BooleanLiteral),
    // Add more expression types as needed
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(expr) => expr.token_literal(),
            Expression::IntegerLiteral(expr) => expr.token_literal(),
            Expression::OperatorExpression(expr) => expr.token_literal(),
            Expression::Boolean(expr) => expr.token_literal(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
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

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Expression,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub name: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}

impl Node for BooleanLiteral {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct OperatorExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

impl Node for OperatorExpression {
    fn token_literal(&self) -> String {
        self.operator.to_string()
    }
}

// Helper methods for constructing expressions
impl Expression {
    pub fn identifier(token: Token, name: String) -> Self {
        Expression::Identifier(Identifier { token, name })
    }

    pub fn integer(token: Token, value: i64) -> Self {
        Expression::IntegerLiteral(IntegerLiteral { token, value })
    }

    pub fn boolean(token: Token, value: bool) -> Self {
        Expression::Boolean(BooleanLiteral { token, value })
    }

    pub fn operator(left: Expression, operator: Token, right: Expression) -> Self {
        Expression::OperatorExpression(OperatorExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}

// Helper methods for constructing statements
impl Statement {
    pub fn let_statement(token: Token, name: Identifier, value: Expression) -> Self {
        Statement::Let(LetStatement { token, name, value })
    }

    pub fn return_statement(token: Token, value: Expression) -> Self {
        Statement::Return(ReturnStatement { token, value })
    }

    pub fn expression_statement(token: Token, expression: Expression) -> Self {
        Statement::Expression(ExpressionStatement { token, expression })
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

impl Expression {
    pub fn as_identifier(&self) -> Option<&Identifier> {
        match self {
            Expression::Identifier(expr) => Some(expr),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<&IntegerLiteral> {
        match self {
            Expression::IntegerLiteral(expr) => Some(expr),
            _ => None,
        }
    }

    pub fn as_operator(&self) -> Option<&OperatorExpression> {
        match self {
            Expression::OperatorExpression(expr) => Some(expr),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<&BooleanLiteral> {
        match self {
            Expression::Boolean(expr) => Some(expr),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_enum_ast_construction() {
        let identifier =
            Expression::identifier(Token::Identifier("x".to_string()), "x".to_string());
        let value = Expression::integer(Token::Int(5), 5);

        let let_stmt = Statement::let_statement(
            Token::Let,
            Identifier {
                token: Token::Identifier("x".to_string()),
                name: "x".to_string(),
            },
            value,
        );

        let mut program = Program::new();
        program.statements.push(let_stmt);

        assert_eq!(program.statements.len(), 1);
        assert!(program.statements[0].as_let().is_some());
    }

    #[test]
    fn test_operator_expression() {
        let left = Expression::integer(Token::Int(5), 5);
        let right = Expression::integer(Token::Int(10), 10);
        let op_expr = Expression::operator(left, Token::Plus, right);

        match op_expr {
            Expression::OperatorExpression(ref expr) => {
                assert_eq!(expr.operator, Token::Plus);
                assert!(expr.left.as_integer().is_some());
                assert!(expr.right.as_integer().is_some());
            }
            _ => panic!("Expected operator expression"),
        }
    }
}
