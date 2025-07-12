use std::error::Error;
use std::fmt::{self, Display};

use crate::ast::prelude::*;
use crate::lexer::Lexer;
use crate::token::{Precedence, Token};

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

    pub fn curr_is(&self, token_type: &Token) -> bool {
        self.curr_token.as_ref() == Some(&token_type)
    }

    pub fn assert_curr_and_consume(&mut self, token_type: &Token) -> Result<(), ParserError> {
        if self.curr_is(token_type) {
            self.advance_tokens();
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken(format!(
                "Expected '{}', got '{}'",
                token_type,
                self.curr_token.as_ref().unwrap_or(&Token::Eof)
            )))
        }
    }

    pub fn peek_is(&self, token_type: &Token) -> bool {
        self.peek_token.as_ref() == Some(&token_type)
    }

    pub fn assert_peek_and_consume(&mut self, token_type: &Token) -> Result<(), ParserError> {
        if self.peek_is(token_type) {
            self.advance_tokens();
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken(format!(
                "Expected '{}', got '{}'",
                token_type,
                self.peek_token.as_ref().unwrap_or(&Token::Eof)
            )))
        }
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
    parser.assert_curr_and_consume(&Token::Let)?;

    let identifier: Identifier = parse_identifier(parser)?;

    parser.advance_tokens();
    parser.assert_curr_and_consume(&Token::Assign)?;

    let value: Expression = parse_expression(parser, Precedence::Lowest)?;

    let statement = LetStatement {
        token: Token::Let,
        identifier,
        value,
    };

    parser.assert_peek_and_consume(&Token::Semicolon)?;

    Ok(statement)
}

fn parse_return_statement(parser: &mut Parser) -> Result<ReturnStatement, ParserError> {
    parser.assert_curr_and_consume(&Token::Return)?;

    let value = parse_expression(parser, Precedence::Lowest)?;

    parser.assert_peek_and_consume(&Token::Semicolon)?;

    Ok(ReturnStatement {
        token: Token::Return,
        value,
    })
}

fn parse_expression_statement(parser: &mut Parser) -> Result<ExpressionStatement, ParserError> {
    let expression = parse_expression(parser, Precedence::Lowest)?;

    Ok(ExpressionStatement {
        token: Token::LeftParen,
        expression,
    })
}

fn parse_block_statement(parser: &mut Parser) -> Result<BlockStatement, ParserError> {
    parser.assert_curr_and_consume(&Token::LeftBrace)?;

    let mut statements = Vec::new();
    while !parser.curr_is(&Token::RightBrace) && parser.curr_token.is_some() {
        match parse_statement(parser) {
            Ok(statement) => {
                statements.push(statement);
            }
            Err(err) => return Err(err),
        }

        parser.advance_tokens();
    }

    parser.assert_curr_and_consume(&Token::RightBrace)?;

    Ok(BlockStatement {
        token: Token::LeftBrace,
        statements,
    })
}

fn parse_expression(
    parser: &mut Parser,
    precedence: Precedence,
) -> Result<Expression, ParserError> {
    let mut left_expression = parse_prefix_expression(parser)?.ok_or_else(|| {
        ParserError::NoPrefixExpressionFound(format!(
            "Expected a prefix expression token, got {}",
            parser.curr_token.as_ref().unwrap_or(&Token::Eof)
        ))
    })?;

    while parser
        .peek_token
        .as_ref()
        .is_some_and(|token| token != &Token::Semicolon && precedence < token.precedence())
    {
        match parse_infix_expression(parser, left_expression.to_owned())? {
            Some(expr) => left_expression = expr,
            None => return Ok(left_expression),
        }
    }

    Ok(left_expression)
}

fn parse_identifier(parser: &mut Parser) -> Result<Identifier, ParserError> {
    let token = parser.curr_token.as_ref();

    match token {
        Some(Token::Ident(name)) => Ok(Identifier {
            token: Token::Ident(name.to_owned()),
            name: name.to_owned(),
        }),
        _ => Err(ParserError::UnexpectedToken(format!(
            "Expected identifier, got {}",
            token.unwrap_or(&Token::Eof)
        ))),
    }
}

fn parse_integer_literal(parser: &mut Parser) -> Result<IntegerLiteral, ParserError> {
    let token = parser.curr_token.as_ref();

    match token {
        Some(Token::Int(value)) => Ok(IntegerLiteral {
            token: Token::Int(*value),
            value: *value,
        }),
        _ => Err(ParserError::UnexpectedToken(format!(
            "Expected integer literal, got {}",
            token.unwrap_or(&Token::Eof)
        ))),
    }
}

fn parse_boolean(parser: &mut Parser) -> Result<BooleanLiteral, ParserError> {
    let token = parser.curr_token.as_ref();

    match token {
        Some(Token::True) => Ok(BooleanLiteral {
            token: Token::True,
            value: true,
        }),
        Some(Token::False) => Ok(BooleanLiteral {
            token: Token::False,
            value: false,
        }),
        _ => Err(ParserError::UnexpectedToken(format!(
            "Expected boolean literal, got {}",
            token.unwrap_or(&Token::Eof)
        ))),
    }
}

fn parse_prefix_expression(parser: &mut Parser) -> Result<Option<Expression>, ParserError> {
    let token = parser.curr_token.as_ref();

    match token {
        Some(Token::Ident(_)) => Ok(Some(Expression::identifier(parse_identifier(parser)?))),
        Some(Token::Int(_)) => Ok(Some(Expression::integer(parse_integer_literal(parser)?))),
        Some(Token::Bang) | Some(Token::Minus) => {
            let operator = token.unwrap().to_owned();
            parser.advance_tokens();

            let expression = parse_expression(parser, Precedence::Prefix)?;
            Ok(Some(Expression::prefix(operator, expression)))
        }
        Some(Token::True) | Some(Token::False) => {
            Ok(Some(Expression::boolean(parse_boolean(parser)?)))
        }
        Some(Token::LeftParen) => {
            parser.advance_tokens();

            let expression = parse_expression(parser, Precedence::Lowest)?;
            parser.assert_peek_and_consume(&Token::RightParen)?;

            Ok(Some(expression))
        }
        Some(Token::If) => {
            parser.assert_peek_and_consume(&Token::LeftParen)?;
            let condition = parse_expression(parser, Precedence::Lowest)?;
            parser.assert_curr_and_consume(&Token::RightParen)?;

            let consequence = parse_block_statement(parser)?;

            let alternative = if parser.curr_is(&Token::Else) {
                parser.advance_tokens();
                Some(parse_block_statement(parser)?)
            } else {
                None
            };

            Ok(Some(Expression::if_expression(
                condition,
                consequence,
                alternative,
            )))
        }
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
) -> Result<Option<Expression>, ParserError> {
    let operator = parser.peek_token.as_ref();
    match operator {
        Some(Token::Plus)
        | Some(Token::Minus)
        | Some(Token::Asterisk)
        | Some(Token::Slash)
        | Some(Token::LessThan)
        | Some(Token::GreaterThan)
        | Some(Token::Equal)
        | Some(Token::NotEqual) => {
            parser.advance_tokens();
            let operator = parser.curr_token.as_ref().unwrap().to_owned();
            parser.advance_tokens();

            let right = parse_expression(parser, operator.precedence())?;
            Ok(Some(Expression::infix(left, operator, right)))
        }
        Some(Token::LeftParen) => {
            parser.advance_tokens();
            let arguments = parse_call_arguments(parser)?;
            Ok(Some(Expression::call(left, arguments)))
        }
        _ => Ok(None),
    }
}

fn parse_call_arguments(parser: &mut Parser) -> Result<Vec<Expression>, ParserError> {
    parser.assert_curr_and_consume(&Token::LeftParen)?;

    let mut arguments = Vec::new();

    println!("call curr token: {:?}", parser.curr_token);

    if parser.curr_token.as_ref().unwrap() == &Token::RightParen {
        parser.advance_tokens();
        return Ok(arguments);
    }

    arguments.push(parse_expression(parser, Precedence::Lowest)?);
    parser.advance_tokens();

    while parser.curr_is(&Token::Comma) {
        parser.advance_tokens();
        arguments.push(parse_expression(parser, Precedence::Lowest)?);
        parser.advance_tokens();
    }

    parser.assert_curr_and_consume(&Token::RightParen)?;

    Ok(arguments)
}

#[cfg(test)]
mod test {
    use crate::ast::prelude::*;
    use crate::lexer::Lexer;

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
    fn test_program_with_errors() {
        let input = "let  = 5; let five = ; let ten = true;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();
        assert_eq!(program.statements.len(), 1);
        assert_eq!(parser.errors.len(), 2);

        let let_stmt = program.statements[0].as_let().unwrap();
        assert_eq!(let_stmt.token_literal(), "let");
        assert_eq!(let_stmt.identifier.name, "ten");
        assert_eq!(let_stmt.value.token_literal(), "true");

        assert_eq!(
            parser.errors[0],
            ParserError::UnexpectedToken("Expected identifier, got =".to_string())
        );
        assert_eq!(
            parser.errors[1],
            ParserError::NoPrefixExpressionFound(
                "Expected a prefix expression token, got ;".to_string()
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
            ("5 + 5;", "5", Token::Plus, "5", "(5 + 5)"),
            ("5-5;", "5", Token::Minus, "5", "(5 - 5)"),
            ("5*5;", "5", Token::Asterisk, "5", "(5 * 5)"),
            ("5 / 5;", "5", Token::Slash, "5", "(5 / 5)"),
            ("5 < 5;", "5", Token::LessThan, "5", "(5 < 5)"),
            ("5 > 5;", "5", Token::GreaterThan, "5", "(5 > 5)"),
            ("5 == 5;", "5", Token::Equal, "5", "(5 == 5)"),
            ("5 != 5;", "5", Token::NotEqual, "5", "(5 != 5)"),
            (
                "true == true;",
                "true",
                Token::Equal,
                "true",
                "(true == true)",
            ),
            (
                "false != true;",
                "false",
                Token::NotEqual,
                "true",
                "(false != true)",
            ),
            (
                "false == false;",
                "false",
                Token::Equal,
                "false",
                "(false == false)",
            ),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0.to_string());
            let mut parser = Parser::new(&mut lexer);

            let stmt = parse_expression_statement(&mut parser).unwrap();
            let infix_exp = stmt.expression.as_infix().unwrap();

            assert_eq!(infix_exp.left.to_string(), test.1);
            assert_eq!(infix_exp.operator, test.2);
            assert_eq!(infix_exp.right.to_string(), test.3);
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
    fn test_boolean_expression_statement() {
        let input = "false;".to_string();
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let stmt = parse_expression_statement(&mut parser).unwrap();
        let boolean = stmt.expression.as_boolean().unwrap();

        assert_eq!(boolean.value, false);
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
    fn test_grouped_expressions() {
        let tests = [
            ("(5 + 5) * 2;", "((5 + 5) * 2)"),
            ("2 * (5 + 5);", "(2 * (5 + 5))"),
            ("-(5 + 5);", "(-(5 + 5))"),
            ("!(true == true);", "(!(true == true))"),
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.0.to_string());
            let mut parser = Parser::new(&mut lexer);

            let program = parser.parse_program();
            assert_eq!(program.to_string(), format!("{}\n", test.1));
        }
    }

    #[test]
    fn test_broken_grouped_expressions() {
        let mut lexer = Lexer::new("(5 + 5 * 2;".to_string());
        let mut parser = Parser::new(&mut lexer);

        let _program = parser.parse_program();
        assert_eq!(parser.errors.len(), 1);

        assert_eq!(
            parser.errors[0],
            ParserError::UnexpectedToken("Expected ')', got ';'".to_string())
        );
    }

    #[test]
    fn test_operator_precedence() {
        let tests = [
            ("-a * b", "((-a) * b)"),
            ("-a * b; a + b", "((-a) * b)\n(a + b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("-5 * 5;", "((-5) * 5)"),
            ("3 + 4; -5 * 5;", "(3 + 4)\n((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            ("3 + 4 + 5 == 3 * 5", "(((3 + 4) + 5) == (3 * 5))"),
            ("3 + 4 * 5 == 3 + 1", "((3 + (4 * 5)) == (3 + 1))"),
            ("3 + 4 * 5 == 1", "((3 + (4 * 5)) == 1)"),
            ("3 + 4 * 5 == 3 + 1", "((3 + (4 * 5)) == (3 + 1))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            // Grouped expressions
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
        ];
        for test in tests {
            Lexer::new(format!("{};", test.0));
            let mut lexer = Lexer::new(format!("{};", test.0));
            let mut parser = Parser::new(&mut lexer);

            let program = parser.parse_program();
            assert_eq!(program.to_string(), format!("{}\n", test.1));
        }
    }

    #[test]
    fn test_if_statement() {
        let input = "if (x < y) { return x; }";
        Lexer::new(input.to_string());
        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();
        assert_eq!(parser.errors.len(), 0);

        assert_eq!(
            program.to_string(),
            format!("if (x < y) {{\nreturn x;\n}}\n",)
        );
    }

    #[test]
    fn test_if_else_statement() {
        let input = "if (x < y) { return x; } else { return y; }";
        Lexer::new(input.to_string());
        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();
        assert_eq!(parser.errors.len(), 0);

        assert_eq!(
            program.to_string(),
            format!("if (x < y) {{\nreturn x;\n}}\nelse {{\nreturn y;\n}}\n",)
        );
    }

    #[test]
    fn test_call_expression() {
        let input = "add(1, 2 * 3, 4 + 5)";
        Lexer::new(input.to_string());
        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program();
        println!("{:?}", parser.errors);
        assert_eq!(parser.errors.len(), 0);

        assert_eq!(program.to_string(), format!("add(1, (2 * 3), (4 + 5))\n",));
    }
}
