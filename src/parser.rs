use std::error::Error;
use std::fmt::{self, Display};

use crate::ast::{
    Expression, Identifier, IntegerLiteral, LetStatement, OperatorExpression, Program,
    ReturnStatement, Statement,
};
use crate::lexer::Lexer;
use crate::token::Token;

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
        _ => Err(ParserError::UnexpectedToken(format!(
            "Unexpected token {}",
            token.unwrap()
        ))),
    }
}

fn parse_let_statement(parser: &mut Parser) -> Result<LetStatement, ParserError> {
    let token = parser.curr_token.as_ref();
    if token != Some(&Token::Let) {
        return Err(ParserError::UnexpectedToken(format!(
            "Expected 'let', got {}",
            token.unwrap()
        )));
    }

    parser.advance_tokens();
    let identifier: Identifier = parse_identifier(parser)?;

    parser.advance_tokens();
    let assign_token = parser.curr_token.as_ref();
    if assign_token != Some(&Token::Assign) {
        return Err(ParserError::UnexpectedToken(format!(
            "Expected '=' after identifier, got {}",
            assign_token.unwrap()
        )));
    }

    parser.advance_tokens();
    let value: Expression = parse_expression(parser)?;

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
            token.unwrap()
        ))),
    }
}

fn parse_expression(parser: &mut Parser) -> Result<Expression, ParserError> {
    let token = parser.curr_token.as_ref();

    match token {
        Some(Token::Int(value)) => match parser.peek_token {
            Some(Token::Plus) | Some(Token::Minus) | Some(Token::Asterisk) | Some(Token::Slash) => {
                Ok(Expression::OperatorExpression(parse_operator_expression(
                    parser,
                )?))
            }
            Some(Token::Semicolon) => Ok(Expression::integer(*value)),
            _ => Err(ParserError::UnexpectedToken(format!(
                "Expected '+' or ';', got {}",
                parser.peek_token.as_ref().unwrap()
            ))),
        },
        Some(Token::LeftParen) => Err(ParserError::Unimplemented(
            "Parsing prefix expressions not implemented, got '('".to_string(),
        )),
        _ => Err(ParserError::UnexpectedToken(format!(
            "Only support integer literals, got {}",
            parser.curr_token.as_ref().unwrap()
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
            token.unwrap()
        ))),
    }
}

fn parse_operator_expression(parser: &mut Parser) -> Result<OperatorExpression, ParserError> {
    let left = parse_integer_literal(parser)?;
    parser.advance_tokens();

    let operator = parser.curr_token.as_ref();

    match operator {
        Some(Token::Plus) | Some(Token::Minus) | Some(Token::Asterisk) | Some(Token::Slash) => {
            let operator = parser.curr_token.as_ref().unwrap().clone();
            parser.advance_tokens();
            Ok(OperatorExpression {
                left: left,
                operator: operator,
                right: parse_integer_literal(parser)?,
            })
        }
        _ => Err(ParserError::UnexpectedToken(format!(
            "Expected operator, got {}",
            operator.unwrap()
        ))),
    }
}

fn parse_return_statement(parser: &mut Parser) -> Result<ReturnStatement, ParserError> {
    let token = parser.curr_token.as_ref();
    if token != Some(&Token::Return) {
        return Err(ParserError::UnexpectedToken(format!(
            "Expected 'return', got {}",
            token.unwrap()
        )));
    }
    parser.advance_tokens();

    let value = parse_expression(parser)?;
    parser.advance_tokens();

    Ok(ReturnStatement {
        token: Token::Return,
        value,
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
    fn test_parse_let_with_operator_statement() {
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
}
