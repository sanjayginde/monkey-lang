use std::error::Error;
use std::fmt::{self, Display};

use crate::ast::PrefixExpression;
use crate::token::{Precedence, Token};
use crate::{
    ast::{
        Expression, ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement,
        Program, ReturnStatement, Statement,
    },
    lexer::Lexer,
};

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
    NoPrefixExpressionFound(String),
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
    precedence: Precedence,
) -> Result<Expression, ParserError> {
    let left_expr = parse_prefix_expression(parser)?;

    if left_expr.is_none() {
        return Err(ParserError::NoPrefixExpressionFound(format!(
            "Expected an prefix expression token, got {}",
            parser.curr_token.as_ref().unwrap_or(&Token::Eof)
        )));
    }

    let mut left_expression: Expression = left_expr.unwrap();

    println!("left: {} ({:?})", left_expression, precedence);

    let token = parser.peek_token.clone().unwrap();
    println!("\tpeek: {:?})", token);
    while token != Token::Semicolon && precedence < token.precedence() {
        let infix_expression = parse_infix_expression(parser, left_expression.clone())?;
        if infix_expression.is_none() {
            println!("no infix expression");
            return Ok(left_expression);
        }

        left_expression = Expression::Infix(infix_expression.unwrap());
        println!("\tleft: {}", left_expression);
    }

    Ok(left_expression)
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

fn parse_prefix_expression(parser: &mut Parser) -> Result<Option<Expression>, ParserError> {
    let token = parser.curr_token.as_ref();

    match token {
        Some(Token::Ident(_)) => Ok(Some(Expression::Identifier(parse_identifier(parser)?))),
        Some(Token::Int(_)) => Ok(Some(Expression::Integer(parse_integer_literal(parser)?))),
        Some(Token::Bang) | Some(Token::Minus) => {
            let operator = token.unwrap().clone();
            parser.advance_tokens();

            let expression = parse_expression(parser, Precedence::Prefix)?;
            Ok(Some(Expression::Prefix(PrefixExpression {
                operator,
                expression: Box::new(expression),
            })))
        }
        // Some(Token::True) => Ok(parse_boolean(parser)),
        // Some(Token::False) => Ok(parse_boolean(parser)),
        Some(_) => Ok(None),
        None => Err(ParserError::UnexpectedToken(format!(
            "Expected prefix expression token, got {}",
            token.unwrap_or(&Token::Eof)
        ))),
    }
}

fn parse_infix_expression(
    parser: &mut Parser,
    left: Expression,
) -> Result<Option<InfixExpression>, ParserError> {
    parser.advance_tokens();

    let operator = parser.curr_token.as_ref();
    println!("\t\top: ({:?})", operator);
    match operator {
        Some(Token::Plus)
        | Some(Token::Minus)
        | Some(Token::Asterisk)
        | Some(Token::Slash)
        | Some(Token::LessThan)
        | Some(Token::GreaterThan)
        | Some(Token::Equal)
        | Some(Token::NotEqual) => {
            let operator = operator.unwrap().clone();
            parser.advance_tokens();

            let right = parse_expression(parser, operator.precedence())?;
            println!("\t\tright: ({:?})", right);

            Ok(Some(InfixExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }))
        }
        _ => Ok(None),
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
        println!("{:?}", let_stmt.value);
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
            ParserError::NoPrefixExpressionFound(
                "Expected an prefix expression token, got ;".to_string()
            )
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
        let tests = [
            ("5 + 5;", 5, Token::Plus, 5, "(5 + 5)"),
            ("5-5;", 5, Token::Minus, 5, "(5 - 5)"),
            ("5*5;", 5, Token::Asterisk, 5, "(5 * 5)"),
            ("5 / 5;", 5, Token::Slash, 5, "(5 / 5)"),
            ("5 < 5;", 5, Token::LessThan, 5, "(5 < 5)"),
            ("5 > 5;", 5, Token::GreaterThan, 5, "(5 > 5)"),
            ("5 == 5;", 5, Token::Equal, 5, "(5 == 5)"),
            ("5 != 5;", 5, Token::NotEqual, 5, "(5 != 5)"),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0.to_string());
            let mut parser = Parser::new(&mut lexer);

            let stmt = parse_expression_statement(&mut parser).unwrap();
            let infix_exp = stmt.expression.as_infix().unwrap();

            assert_eq!(infix_exp.left.as_integer().unwrap().value, test.1);
            assert_eq!(infix_exp.operator, test.2);
            assert_eq!(infix_exp.right.as_integer().unwrap().value, test.3);
            assert_eq!(infix_exp.to_string(), test.4);
        }
    }

    #[test]
    fn test_integer_expression_statement() {
        let input = "12;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let stmt = parse_expression_statement(&mut parser).unwrap();
        let integer = stmt.expression.as_integer().unwrap();

        assert_eq!(integer.value, 12);
    }

    #[test]
    fn test_identifier_expression_statement() {
        let input = "foobar;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let stmt = parse_expression_statement(&mut parser).unwrap();
        let identifier = stmt.expression.as_identifier().unwrap();

        assert_eq!(identifier.name, "foobar");
    }

    #[test]
    fn test_prefix_expressions() {
        let tests = [
            ("!12;", Token::Bang, 12, "(!12)"),
            ("-4;", Token::Minus, 4, "(-4)"),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0.to_string());
            let mut parser = Parser::new(&mut lexer);

            let stmt = parse_expression_statement(&mut parser).unwrap();
            let prefix_exp = stmt.expression.as_prefix().unwrap();
            let right = prefix_exp.expression.as_integer().unwrap();

            assert_eq!(prefix_exp.operator, test.1);
            assert_eq!(right.value, test.2);
            assert_eq!(prefix_exp.to_string(), test.3);
        }
    }

    #[test]
    fn test_operator_precendence() {
        let tests = [
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5;", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
        ];
        for test in tests {
            Lexer::new(format!("{};", test.0))
                .into_iter()
                .for_each(|token| {
                    println!("{:?}", token);
                });
            let mut lexer = Lexer::new(format!("{};", test.0));
            let mut parser = Parser::new(&mut lexer);

            let program = parser.parse_program();
            println!("{}", program.statements.len());
            assert_eq!(program.to_string(), format!("{}\n", test.1));
        }
    }
}
