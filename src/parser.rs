use std::error::Error;
use std::fmt::{self, Display};

use crate::ast::PrefixExpression;
use crate::token::Token;
use crate::{
    ast::{
        Expression, ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement,
        Program, ReturnStatement, Statement,
    },
    lexer::Lexer,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    fn enum_index(&self) -> u8 {
        match *self {
            Precedence::Lowest => 0,
            Precedence::Equals => 1,
            Precedence::LessGreater => 2,
            Precedence::Sum => 3,
            Precedence::Product => 4,
            Precedence::Prefix => 5,
            Precedence::Call => 6,
        }
    }
}

impl Ord for Precedence {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.enum_index().cmp(&other.enum_index())
    }
}

impl PartialOrd for Precedence {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer,

    pub curr_token: Option<Token>,
    pub peek_token: Option<Token>,

    pub errors: Vec<ParserError>,
}

impl Parser<'_> {
    pub fn new(lexer: &mut Lexer) -> Parser {
        let mut result = Parser {
            lexer,
            curr_token: None,
            peek_token: None,
            errors: vec![],
        };
        result.advance_tokens();
        result.advance_tokens();
        result
    }

    pub fn advance_tokens(&mut self) {
        self.curr_token = self.peek_token.take();
        self.peek_token = self.lexer.next();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while let Some(_token) = &self.curr_token.as_mut() {
            let statement: Result<Statement, ParserError> = parse_statement(self);

            match statement {
                Ok(statement) => {
                    program.statements.push(statement);
                }
                Err(error) => {
                    self.errors.push(error);
                    while let Some(token) = &self.curr_token {
                        if token != &Token::Semicolon {
                            self.advance_tokens();
                        } else {
                            break;
                        }
                    }
                }
            }

            self.advance_tokens();
        }

        program
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParserError {
    UnexpectedToken(String),
    Unimplemented(String),
}

impl Display for ParserError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl Error for ParserError {}

fn parse_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    let token = parser.curr_token.as_ref();
    match token {
        Some(Token::Let) => Ok(Statement::Let(parse_let_statement(parser)?)),
        Some(Token::Return) => Ok(Statement::Return(parse_return_statement(parser)?)),
        Some(_) => Ok(Statement::Expression(parse_expression_statement(parser)?)),
        None => Err(ParserError::UnexpectedToken(
            "Unexpected end of input".to_string(),
        )),
    }
}

fn parse_let_statement(parser: &mut Parser) -> Result<LetStatement, ParserError> {
    let token = parser.curr_token.as_ref();
    if token != Some(&Token::Let) {
        return Err(ParserError::UnexpectedToken(format!(
            "Expected 'let', got {}",
            token.unwrap_or(&Token::Eof)
        )));
    }

    parser.advance_tokens();
    let identifier: Identifier = parse_identifier(parser)?;

    parser.advance_tokens();
    let assign_token = parser.curr_token.as_ref();
    if assign_token != Some(&Token::Assign) {
        return Err(ParserError::UnexpectedToken(format!(
            "Expected '=' after identifier, got {}",
            assign_token.unwrap_or(&Token::Eof)
        )));
    }

    parser.advance_tokens();
    let value: Expression = parse_expression(parser, Precedence::Lowest)?;

    let statement = LetStatement {
        token: Token::Let,
        identifier,
        value,
    };

    parser.advance_tokens();

    Ok(statement)
}

fn parse_identifier(parser: &mut Parser) -> Result<Identifier, ParserError> {
    let token = parser.curr_token.as_ref();

    match token {
        Some(Token::Ident(name)) => Ok(Identifier {
            token: token.unwrap().clone(),
            name: name.clone(),
        }),
        _ => Err(ParserError::UnexpectedToken(format!(
            "Expected identifier, got {}",
            token.unwrap_or(&Token::Eof)
        ))),
    }
}

fn parse_expression(
    parser: &mut Parser,
    _precedence: Precedence,
) -> Result<Expression, ParserError> {
    let token = parser.curr_token.as_ref();

    match token {
        Some(Token::Bang) | Some(Token::Minus) => {
            Ok(Expression::Prefix(parse_prefix_expression(parser)?))
        }
        Some(Token::Ident(name)) => Ok(Expression::Identifier(Identifier {
            token: token.unwrap().clone(),
            name: name.clone(),
        })),
        Some(Token::Int(value)) => match parser.peek_token {
            Some(Token::Plus) | Some(Token::Minus) | Some(Token::Asterisk) | Some(Token::Slash) => {
                Ok(Expression::Infix(parse_infix_expression(parser)?))
            }
            Some(Token::Semicolon) => Ok(Expression::integer(*value)),
            _ => Err(ParserError::UnexpectedToken(format!(
                "Expected an infix operator or ';', got {}",
                parser.peek_token.as_ref().unwrap_or(&Token::Eof)
            ))),
        },

        Some(Token::LeftParen) => Err(ParserError::Unimplemented(
            "Parsing prefix expressions not implemented, got '('".to_string(),
        )),
        _ => Err(ParserError::UnexpectedToken(format!(
            "Only support integer literals, got {}",
            parser.curr_token.as_ref().unwrap_or(&Token::Eof)
        ))),
    }
}

fn parse_integer_literal(parser: &mut Parser) -> Result<IntegerLiteral, ParserError> {
    let token = parser.curr_token.as_ref();

    match token {
        Some(Token::Int(value)) => Ok(IntegerLiteral {
            token: token.unwrap().clone(),
            value: *value,
        }),
        _ => Err(ParserError::UnexpectedToken(format!(
            "Expected integer literal, got {}",
            token.unwrap_or(&Token::Eof)
        ))),
    }
}

fn parse_prefix_expression(parser: &mut Parser) -> Result<PrefixExpression, ParserError> {
    let operator = parser.curr_token.as_ref().unwrap().clone();
    parser.advance_tokens();

    let right = Box::new(parse_expression(parser, Precedence::Prefix)?);

    Ok(PrefixExpression { operator, right })
}

fn parse_infix_expression(parser: &mut Parser) -> Result<InfixExpression, ParserError> {
    let left = Expression::Integer(parse_integer_literal(parser)?);
    parser.advance_tokens();

    let operator = parser.curr_token.as_ref();

    match operator {
        Some(Token::Plus) | Some(Token::Minus) | Some(Token::Asterisk) | Some(Token::Slash) => {
            let operator = parser.curr_token.as_ref().unwrap().clone();
            parser.advance_tokens();

            let right = parse_expression(parser, Precedence::Lowest)?;
            parser.advance_tokens();

            Ok(InfixExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            })
        }
        _ => Err(ParserError::UnexpectedToken(format!(
            "Expected operator, got {}",
            operator.unwrap_or(&Token::Eof)
        ))),
    }
}

fn parse_return_statement(parser: &mut Parser) -> Result<ReturnStatement, ParserError> {
    let token = parser.curr_token.as_ref();
    if token != Some(&Token::Return) {
        return Err(ParserError::UnexpectedToken(format!(
            "Expected 'return', got {}",
            token.unwrap_or(&Token::Eof)
        )));
    }
    parser.advance_tokens();

    let value = parse_expression(parser, Precedence::Lowest)?;
    parser.advance_tokens();

    Ok(ReturnStatement {
        token: Token::Return,
        value,
    })
}

fn parse_expression_statement(parser: &mut Parser) -> Result<ExpressionStatement, ParserError> {
    let expression = parse_expression(parser, Precedence::Lowest)?;
    parser.advance_tokens();

    Ok(ExpressionStatement {
        token: Token::LeftParen,
        expression,
    })
}

#[cfg(test)]
mod test {
    use crate::{ast::Node, lexer::Lexer};

    use super::*;

    #[test]
    fn test_parse_let_statement() {
        let input = "let x = 5;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let result = parse_let_statement(&mut parser);
        let let_stmt = result.unwrap();

        assert_eq!(let_stmt.token_literal(), "let");
        assert_eq!(let_stmt.identifier.token_literal(), "x");
        assert_eq!(let_stmt.value.token_literal(), "5");
    }

    #[test]
    fn test_parse_let_statement_with_infix_expression() {
        let input = "let x = 5 + 5;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let result = parse_let_statement(&mut parser);
        let let_stmt = result.unwrap();

        assert_eq!(let_stmt.token_literal(), "let");
        assert_eq!(let_stmt.identifier.token_literal(), "x");
        assert_eq!(let_stmt.value.token_literal(), "+");
    }

    #[test]
    fn test_invalid_parse_let_statement() {
        let input = "let x = ;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let result = parse_let_statement(&mut parser);
        assert!(result.is_err());
    }

    #[test]
    fn test_program_let_statements() {
        let input = "let x = 5; let five = 5;let ten = 10;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 3);
        assert_eq!(parser.errors.len(), 0);

        let let_stmt = program.statements[0].as_let().unwrap();

        assert_eq!(let_stmt.token_literal(), "let");
        assert_eq!(let_stmt.identifier.name, "x");
        assert_eq!(let_stmt.value.token_literal(), "5");
    }

    #[test]
    fn test_program_with_error() {
        let input = "let  = 5; let five = ; let ten = 10;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 1);
        assert_eq!(parser.errors.len(), 2);

        let let_stmt = program.statements[0].as_let().unwrap();
        assert_eq!(let_stmt.token_literal(), "let");
        assert_eq!(let_stmt.identifier.name, "ten");
        assert_eq!(let_stmt.value.token_literal(), "10");

        assert_eq!(
            parser.errors[0],
            ParserError::UnexpectedToken("Expected identifier, got =".to_string())
        );
        assert_eq!(
            parser.errors[1],
            ParserError::UnexpectedToken("Only support integer literals, got ;".to_string())
        );
    }

    #[test]
    fn test_program_return_statement() {
        let input = "return 12; ".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let result = parse_return_statement(&mut parser);
        let ret_stmt = result.unwrap();

        assert_eq!(ret_stmt.token_literal(), "return");
        assert_eq!(ret_stmt.value.token_literal(), "12");
    }

    #[test]
    fn test_infix_expression_statement() {
        let input = "15 + 10; ".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let result = parse_expression_statement(&mut parser);
        let exp_stmt = result.unwrap();

        assert_eq!(exp_stmt.to_string(), "(15 + 10)");
    }

    #[test]
    fn test_identifier_expression_statement() {
        let input = "12; ".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let result = parse_expression_statement(&mut parser);
        let int_stmt = result.unwrap();

        assert_eq!(int_stmt.to_string(), "12");
    }

    #[test]
    fn test_prefix_expressions() {
        let tests = [("!12;", "(!12)"), ("-4;", "(-4)")];

        for test in tests {
            let mut lexer = Lexer::new(test.0.to_string());
            let mut parser = Parser::new(&mut lexer);

            let result = parse_expression_statement(&mut parser);
            let int_stmt = result.unwrap();

            assert_eq!(int_stmt.to_string(), test.1);
        }
    }
}
