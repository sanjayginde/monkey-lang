use std::error::Error;
use std::fmt::{self, Display};

use crate::ast::{Expression, Identifier, IntegerLiteral, LetStatement, Program};
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    pub curr_token: Option<Token>,
    pub peek_token: Option<Token>,
}

impl Parser<'_> {
    pub fn new(lexer: &mut Lexer) -> Parser {
        let mut result = Parser {
            lexer,
            curr_token: None,
            peek_token: None,
        };
        result.advance_tokens();
        result.advance_tokens();
        result
    }

    pub fn advance_tokens(&mut self) {
        self.curr_token = self.peek_token.take();
        self.peek_token = self.lexer.next();
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut program = Program { statements: vec![] };

        while let Some(token) = &self.curr_token.as_mut() {
            let statement: Result<LetStatement, ParserError> = match token {
                Token::Let => {
                    parse_let_statement(self)
                }
                _ => Err(ParserError::UnexpectedToken(format!("Unexpected token ${}", token))),
            };

            program.statements.push(Box::new(statement?));

            self.advance_tokens();
        }

        // for (self.curr_token != EOF_TOKEN) {
        //     statement = null
        //     if (currentToken() == LET_TOKEN) {
        //       statement = parseLetStatement()
        //     } else if (currentToken() == RETURN_TOKEN) {
        //       statement = parseReturnStatement()
        //     } else if (currentToken() == IF_TOKEN) {
        //       statement = parseIfStatement()
        //     }

        //     if (statement != null) {
        //       program.Statements.push(statement)
        //     }

        //     advanceTokens()
        //   }

        Ok(program)
    }
}

fn parse_let_statement(parser: &mut Parser) -> Result<LetStatement, ParserError> {
    let token = parser.curr_token.as_ref();
    if token != Some(&Token::Let) {
        return Err(ParserError::UnexpectedToken(format!("Expected 'let', got ${}", token.unwrap())));
    }

    parser.advance_tokens();
    let identifier: Identifier = parse_identifier(parser)?;

    parser.advance_tokens();
    let assign_token = parser.curr_token.as_ref();
    if assign_token != Some(&Token::Assign) {
        return Err(ParserError::UnexpectedToken(format!("Expected '=' after identifier, got ${}", assign_token.unwrap())));
    }

    parser.advance_tokens();
    let value: Box<dyn Expression> = parse_expression(parser)?;

    let statement = LetStatement {
        token: Token::Let,
        name: identifier,
        value: value,
    };

    Ok(statement)   
}

fn parse_identifier(parser: &mut Parser) -> Result<Identifier, ParserError> {
    let token = parser.curr_token.as_ref();

    match token {
        Some(Token::Ident(name)) => Ok(Identifier {
            token: token.unwrap().clone(),
            name: name.clone(),
        }),
        _ => Err(ParserError::UnexpectedToken(format!("Expected identifier, got ${}", token.unwrap()))),
    }
}

fn parse_expression(parser: &mut Parser) -> Result<Box<dyn Expression>, ParserError> {
    let token = parser.curr_token.as_ref();
    
    match token {
        Some(Token::Int(value)) => {
            match parser.peek_token {
                Some(Token::Plus) => {
                    return Err(ParserError::Unimplemented("Parsing prefix expressions not implemented, got +".to_string()));
                },
                Some(Token::Semicolon) => {
                    return Ok(Box::new(IntegerLiteral {
                        token: parser.curr_token.as_ref().unwrap().clone(),
                        value: *value,
                    }));
                },
                _ => {
                    Err(ParserError::UnexpectedToken(format!("Expected '+' or ';', got ${}", parser.peek_token.as_ref().unwrap())))
                }
            }
        },
        Some(Token::LeftParen) => {
            return Err(ParserError::Unimplemented("Parsing prefix expressions not implemented, got '('".to_string()));
        },
        _ => {
            Err(ParserError::UnexpectedToken(format!("Only support integer literals, got ${}", parser.curr_token.as_ref().unwrap())))
        }
    }
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
    use crate::{ast::Statement, lexer::Lexer};

    use super::*;

    #[test]
    fn test_parse_let_statement() {
        let input = "let x = 5;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let result = parse_let_statement(&mut parser);
        let let_stmt = result.unwrap();

        assert_eq!(let_stmt.token, Token::Let);
        assert_eq!(let_stmt.name.name, "x");
        // TODO  assert_eq!(let_stmt.value, "5");
    }

    #[test]
    fn test_invalid_parse_let_statement() {
        let input = "let x = ;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let result = parse_let_statement(&mut parser);
        assert!(result.is_err());
    }

    fn test_program_let_statements() {
        let input = "let x = 5; let five = 5;let ten = 10;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 3);
        let statement: &(dyn Statement + 'static) = program.statements[0].as_ref();
        assert_eq!(statement.token_literal(), "let");
    }
}
