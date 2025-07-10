use crate::token::Token;
use core::fmt::Debug;
use std::marker::PhantomData;

pub trait Node {
    fn token_literal(&self) -> String;
}

// Generic AST with phantom types to avoid Box<dyn>
#[derive(Debug, Clone)]
pub struct Program<S> {
    pub statements: Vec<S>,
    _phantom: PhantomData<S>,
}

impl<S> Program<S> {
    pub fn new() -> Self {
        Program {
            statements: vec![],
            _phantom: PhantomData,
        }
    }

    pub fn add_statement(&mut self, statement: S) {
        self.statements.push(statement);
    }
}

impl<S: Node> Node for Program<S> {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}

// Concrete statement types
#[derive(Debug, Clone)]
pub struct LetStatement<E> {
    pub token: Token,
    pub name: Identifier,
    pub value: E,
}

impl<E: Node> Node for LetStatement<E> {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ReturnStatement<E> {
    pub token: Token,
    pub value: E,
}

impl<E: Node> Node for ReturnStatement<E> {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement<E> {
    pub token: Token,
    pub expression: E,
}

impl<E: Node> Node for ExpressionStatement<E> {
    fn token_literal(&self) -> String {
        self.token.to_string()
    }
}

// Expression types
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
pub struct InfixExpression<L, R> {
    pub left: L,
    pub operator: Token,
    pub right: R,
}

impl<L: Node, R: Node> Node for InfixExpression<L, R> {
    fn token_literal(&self) -> String {
        self.operator.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct PrefixExpression<E> {
    pub operator: Token,
    pub right: E,
}

impl<E: Node> Node for PrefixExpression<E> {
    fn token_literal(&self) -> String {
        self.operator.to_string()
    }
}

// Type aliases for common patterns
pub type SimpleExpression = IntegerLiteral;
pub type SimpleBinaryExpression = InfixExpression<IntegerLiteral, IntegerLiteral>;
pub type SimpleLetStatement = LetStatement<SimpleExpression>;
pub type SimpleReturnStatement = ReturnStatement<SimpleExpression>;

// For more complex expressions that can be nested
#[derive(Debug, Clone)]
pub enum ComplexExpression {
    Identifier(Identifier),
    Integer(IntegerLiteral),
    Boolean(BooleanLiteral),
    Infix(Box<InfixExpression<ComplexExpression, ComplexExpression>>),
    Prefix(Box<PrefixExpression<ComplexExpression>>),
}

impl Node for ComplexExpression {
    fn token_literal(&self) -> String {
        match self {
            ComplexExpression::Identifier(expr) => expr.token_literal(),
            ComplexExpression::Integer(expr) => expr.token_literal(),
            ComplexExpression::Boolean(expr) => expr.token_literal(),
            ComplexExpression::Infix(expr) => expr.token_literal(),
            ComplexExpression::Prefix(expr) => expr.token_literal(),
        }
    }
}

// Type aliases for complex statements
pub type ComplexLetStatement = LetStatement<ComplexExpression>;
pub type ComplexReturnStatement = ReturnStatement<ComplexExpression>;

#[derive(Debug, Clone)]
pub enum ComplexStatement {
    Let(ComplexLetStatement),
    Return(ComplexReturnStatement),
    Expression(ExpressionStatement<ComplexExpression>),
}

impl Node for ComplexStatement {
    fn token_literal(&self) -> String {
        match self {
            ComplexStatement::Let(stmt) => stmt.token_literal(),
            ComplexStatement::Return(stmt) => stmt.token_literal(),
            ComplexStatement::Expression(stmt) => stmt.token_literal(),
        }
    }
}

pub type ComplexProgram = Program<ComplexStatement>;

// Builder pattern for easier construction
pub struct ExpressionBuilder;

impl ExpressionBuilder {
    pub fn identifier(token: Token, name: String) -> ComplexExpression {
        ComplexExpression::Identifier(Identifier { token, name })
    }

    pub fn integer(token: Token, value: i64) -> ComplexExpression {
        ComplexExpression::Integer(IntegerLiteral { token, value })
    }

    pub fn boolean(token: Token, value: bool) -> ComplexExpression {
        ComplexExpression::Boolean(BooleanLiteral { token, value })
    }

    pub fn infix(
        left: ComplexExpression,
        operator: Token,
        right: ComplexExpression,
    ) -> ComplexExpression {
        ComplexExpression::Infix(Box::new(InfixExpression {
            left,
            operator,
            right,
        }))
    }

    pub fn prefix(operator: Token, right: ComplexExpression) -> ComplexExpression {
        ComplexExpression::Prefix(Box::new(PrefixExpression { operator, right }))
    }
}

pub struct StatementBuilder;

impl StatementBuilder {
    pub fn let_statement(
        token: Token,
        name: Identifier,
        value: ComplexExpression,
    ) -> ComplexStatement {
        ComplexStatement::Let(LetStatement { token, name, value })
    }

    pub fn return_statement(token: Token, value: ComplexExpression) -> ComplexStatement {
        ComplexStatement::Return(ReturnStatement { token, value })
    }

    pub fn expression_statement(token: Token, expression: ComplexExpression) -> ComplexStatement {
        ComplexStatement::Expression(ExpressionStatement { token, expression })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_simple_generic_ast() {
        let value = IntegerLiteral {
            token: Token::Int(5),
            value: 5,
        };

        let let_stmt = LetStatement {
            token: Token::Let,
            name: Identifier {
                token: Token::Identifier("x".to_string()),
                name: "x".to_string(),
            },
            value,
        };

        let mut program = Program::new();
        program.add_statement(let_stmt);

        assert_eq!(program.statements.len(), 1);
        assert_eq!(program.statements[0].name.name, "x");
    }

    #[test]
    fn test_complex_expression_building() {
        let left = ExpressionBuilder::integer(Token::Int(5), 5);
        let right = ExpressionBuilder::integer(Token::Int(10), 10);
        let expr = ExpressionBuilder::infix(left, Token::Plus, right);

        match expr {
            ComplexExpression::Infix(infix) => {
                assert_eq!(infix.operator, Token::Plus);
                assert!(matches!(infix.left, ComplexExpression::Integer(_)));
                assert!(matches!(infix.right, ComplexExpression::Integer(_)));
            }
            _ => panic!("Expected infix expression"),
        }
    }

    #[test]
    fn test_complex_program() {
        let value = ExpressionBuilder::integer(Token::Int(42), 42);
        let let_stmt = StatementBuilder::let_statement(
            Token::Let,
            Identifier {
                token: Token::Identifier("answer".to_string()),
                name: "answer".to_string(),
            },
            value,
        );

        let mut program = ComplexProgram::new();
        program.add_statement(let_stmt);

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            ComplexStatement::Let(stmt) => {
                assert_eq!(stmt.name.name, "answer");
                match &stmt.value {
                    ComplexExpression::Integer(lit) => assert_eq!(lit.value, 42),
                    _ => panic!("Expected integer literal"),
                }
            }
            _ => panic!("Expected let statement"),
        }
    }
}
