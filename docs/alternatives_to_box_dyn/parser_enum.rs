use std::error::Error;
use std::fmt::{self, Display};

use crate::ast_enum::{
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

        while let Some(token) = &self.curr_token.as_mut() {
            let statement: Result<Statement, ParserError> = match token {
                Token::Let => {
                    parse_let_statement(self).map(|stmt| Statement::Let(stmt))
                }
                Token::Return => {
                    parse_return_statement(self).map(|stmt| Statement::Return(stmt))
                }
                // Token::If => {
                //     parse_if_statement(self)
                // }
                _ => Err(ParserError::UnexpectedToken(format!(
                    "Unexpected token {}",
                    token
                ))),
            };

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
        name: identifier,
        value: value,
    };

    parser.advance_tokens();

    Ok(statement)
}

fn parse_identifier(parser: &mut Parser) -> Result<Identifier, ParserError> {
    let token = parser.curr_token.as_ref();

    match token {
        Some(Token::Identifier(name)) => Ok(Identifier {
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
                Ok(parse_operator_expression(parser)?)
            }
            Some(Token::Semicolon) => Ok(Expression::IntegerLiteral(IntegerLiteral {
                token: parser.curr_token.as_ref().unwrap().clone(),
                value: *value,
            })),
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

fn parse_operator_expression(parser: &mut Parser) -> Result<Expression, ParserError> {
    let left = parse_integer_literal(parser)?;
    parser.advance_tokens();

    let operator = parser.curr_token.as_ref();

    match operator {
        Some(Token::Plus) | Some(Token::Minus) | Some(Token::Asterisk) | Some(Token::Slash) => {
            let operator = parser.curr_token.as_ref().unwrap().clone();
            parser.advance_tokens();
            let right = parse_integer_literal(parser)?;
            
            Ok(Expression::OperatorExpression(OperatorExpression {
                left: Box::new(Expression::IntegerLiteral(left)),
                operator: operator,
                right: Box::new(Expression::IntegerLiteral(right)),
            }))
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

    let return_value = parse_expression(parser)?;
    parser.advance_tokens();

    Ok(ReturnStatement {
        token: Token::Return,
        value: return_value,
    })
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(String),
    Unimplemented(String),
}

impl Display for ParserError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Credit card error: Could not retrieve credit card.")
    }
}

impl Error for ParserError {}

#[cfg(test)]
mod test {
    use crate::{
        ast_enum::{Node, Statement},
        lexer::Lexer,
    };

    use super::*;

    #[test]
    fn test_parse_let_statement() {
        let input = "let x = 5;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let result = parse_let_statement(&mut parser);
        let let_stmt = result.unwrap();

        assert_eq!(let_stmt.token, Token::Let);
        assert_eq!(let_stmt.name.token_literal(), "Identifier(\"x\")");
        assert_eq!(let_stmt.value.token_literal(), "Int(5)");
    }

    #[test]
    fn test_parse_let_with_operator_statement() {
        let input = "let x = 5 + 5;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let result = parse_let_statement(&mut parser);
        let let_stmt = result.unwrap();

        assert_eq!(let_stmt.token, Token::Let);
        assert_eq!(let_stmt.name.token_literal(), "Identifier(\"x\")");
        assert_eq!(let_stmt.value.token_literal(), "Plus");
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
        let statement = &program.statements[0];
        assert_eq!(statement.token_literal(), "Let");
    }

    #[test]
    fn test_program_with_error() {
        let input = "let  = 5; let five = ; let ten = 10;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 1);
        assert_eq!(parser.errors.len(), 2);

        let statement = &program.statements[0];
        assert_eq!(statement.token_literal(), "Let");
    }

    #[test]
    fn test_program_return_statement() {
        let input = "return 12; ".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let result = parse_return_statement(&mut parser);
        let ret_stmt = result.unwrap();

        assert_eq!(ret_stmt.token, Token::Return);
        assert_eq!(ret_stmt.value.token_literal(), "Int(12)");
    }
}
