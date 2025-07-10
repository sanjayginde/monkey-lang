use crate::token::Token;
use std::collections::HashMap;

pub type NodeId = usize;

#[derive(Debug)]
pub struct Arena {
    statements: Vec<Statement>,
    expressions: Vec<Expression>,
    next_statement_id: NodeId,
    next_expression_id: NodeId,
}

impl Arena {
    pub fn new() -> Self {
        Arena {
            statements: Vec::new(),
            expressions: Vec::new(),
            next_statement_id: 0,
            next_expression_id: 0,
        }
    }

    pub fn add_statement(&mut self, statement: Statement) -> NodeId {
        let id = self.next_statement_id;
        self.statements.push(statement);
        self.next_statement_id += 1;
        id
    }

    pub fn add_expression(&mut self, expression: Expression) -> NodeId {
        let id = self.next_expression_id;
        self.expressions.push(expression);
        self.next_expression_id += 1;
        id
    }

    pub fn get_statement(&self, id: NodeId) -> Option<&Statement> {
        self.statements.get(id)
    }

    pub fn get_expression(&self, id: NodeId) -> Option<&Expression> {
        self.expressions.get(id)
    }

    pub fn get_statement_mut(&mut self, id: NodeId) -> Option<&mut Statement> {
        self.statements.get_mut(id)
    }

    pub fn get_expression_mut(&mut self, id: NodeId) -> Option<&mut Expression> {
        self.expressions.get_mut(id)
    }
}

pub trait Node {
    fn token_literal(&self, arena: &Arena) -> String;
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
}

impl Node for Statement {
    fn token_literal(&self, arena: &Arena) -> String {
        match self {
            Statement::Let(stmt) => stmt.token_literal(arena),
            Statement::Return(stmt) => stmt.token_literal(arena),
            Statement::Expression(stmt) => stmt.token_literal(arena),
            Statement::Block(stmt) => stmt.token_literal(arena),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    BooleanLiteral(BooleanLiteral),
    StringLiteral(StringLiteral),
    InfixExpression(InfixExpression),
    PrefixExpression(PrefixExpression),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression),
    ArrayLiteral(ArrayLiteral),
    IndexExpression(IndexExpression),
    HashLiteral(HashLiteral),
}

impl Node for Expression {
    fn token_literal(&self, arena: &Arena) -> String {
        match self {
            Expression::Identifier(expr) => expr.token_literal(arena),
            Expression::IntegerLiteral(expr) => expr.token_literal(arena),
            Expression::BooleanLiteral(expr) => expr.token_literal(arena),
            Expression::StringLiteral(expr) => expr.token_literal(arena),
            Expression::InfixExpression(expr) => expr.token_literal(arena),
            Expression::PrefixExpression(expr) => expr.token_literal(arena),
            Expression::IfExpression(expr) => expr.token_literal(arena),
            Expression::FunctionLiteral(expr) => expr.token_literal(arena),
            Expression::CallExpression(expr) => expr.token_literal(arena),
            Expression::ArrayLiteral(expr) => expr.token_literal(arena),
            Expression::IndexExpression(expr) => expr.token_literal(arena),
            Expression::HashLiteral(expr) => expr.token_literal(arena),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<NodeId>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement_id: NodeId) {
        self.statements.push(statement_id);
    }
}

impl Node for Program {
    fn token_literal(&self, arena: &Arena) -> String {
        if !self.statements.is_empty() {
            if let Some(stmt) = arena.get_statement(self.statements[0]) {
                return stmt.token_literal(arena);
            }
        }
        "".to_string()
    }
}

// Statement implementations
#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: NodeId, // Expression ID
}

impl Node for LetStatement {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: NodeId, // Expression ID
}

impl Node for ReturnStatement {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: NodeId, // Expression ID
}

impl Node for ExpressionStatement {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<NodeId>, // Statement IDs
}

impl Node for BlockStatement {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

// Expression implementations
#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub name: String,
}

impl Node for Identifier {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}

impl Node for BooleanLiteral {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl Node for StringLiteral {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub left: NodeId, // Expression ID
    pub operator: String,
    pub right: NodeId, // Expression ID
}

impl Node for InfixExpression {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: NodeId, // Expression ID
}

impl Node for PrefixExpression {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: NodeId,           // Expression ID
    pub consequence: NodeId,         // Statement ID (BlockStatement)
    pub alternative: Option<NodeId>, // Statement ID (BlockStatement)
}

impl Node for IfExpression {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: NodeId, // Statement ID (BlockStatement)
}

impl Node for FunctionLiteral {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub token: Token,
    pub function: NodeId,       // Expression ID
    pub arguments: Vec<NodeId>, // Expression IDs
}

impl Node for CallExpression {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub token: Token,
    pub elements: Vec<NodeId>, // Expression IDs
}

impl Node for ArrayLiteral {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct IndexExpression {
    pub token: Token,
    pub left: NodeId,  // Expression ID
    pub index: NodeId, // Expression ID
}

impl Node for IndexExpression {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct HashLiteral {
    pub token: Token,
    pub pairs: HashMap<NodeId, NodeId>, // Expression ID -> Expression ID
}

impl Node for HashLiteral {
    fn token_literal(&self, _arena: &Arena) -> String {
        self.token.to_string()
    }
}

// Builder for easier construction
pub struct AstBuilder<'a> {
    arena: &'a mut Arena,
}

impl<'a> AstBuilder<'a> {
    pub fn new(arena: &'a mut Arena) -> Self {
        AstBuilder { arena }
    }

    pub fn identifier(&mut self, token: Token, name: String) -> NodeId {
        let expr = Expression::Identifier(Identifier { token, name });
        self.arena.add_expression(expr)
    }

    pub fn integer_literal(&mut self, token: Token, value: i64) -> NodeId {
        let expr = Expression::IntegerLiteral(IntegerLiteral { token, value });
        self.arena.add_expression(expr)
    }

    pub fn boolean_literal(&mut self, token: Token, value: bool) -> NodeId {
        let expr = Expression::BooleanLiteral(BooleanLiteral { token, value });
        self.arena.add_expression(expr)
    }

    pub fn string_literal(&mut self, token: Token, value: String) -> NodeId {
        let expr = Expression::StringLiteral(StringLiteral { token, value });
        self.arena.add_expression(expr)
    }

    pub fn infix_expression(
        &mut self,
        token: Token,
        left: NodeId,
        operator: String,
        right: NodeId,
    ) -> NodeId {
        let expr = Expression::InfixExpression(InfixExpression {
            token,
            left,
            operator,
            right,
        });
        self.arena.add_expression(expr)
    }

    pub fn prefix_expression(&mut self, token: Token, operator: String, right: NodeId) -> NodeId {
        let expr = Expression::PrefixExpression(PrefixExpression {
            token,
            operator,
            right,
        });
        self.arena.add_expression(expr)
    }

    pub fn let_statement(&mut self, token: Token, name: Identifier, value: NodeId) -> NodeId {
        let stmt = Statement::Let(LetStatement { token, name, value });
        self.arena.add_statement(stmt)
    }

    pub fn return_statement(&mut self, token: Token, value: NodeId) -> NodeId {
        let stmt = Statement::Return(ReturnStatement { token, value });
        self.arena.add_statement(stmt)
    }

    pub fn expression_statement(&mut self, token: Token, expression: NodeId) -> NodeId {
        let stmt = Statement::Expression(ExpressionStatement { token, expression });
        self.arena.add_statement(stmt)
    }

    pub fn block_statement(&mut self, token: Token, statements: Vec<NodeId>) -> NodeId {
        let stmt = Statement::Block(BlockStatement { token, statements });
        self.arena.add_statement(stmt)
    }
}

// Visitor pattern for traversing the AST
pub trait Visitor<T> {
    fn visit_program(&mut self, program: &Program, arena: &Arena) -> T;
    fn visit_statement(&mut self, statement: &Statement, arena: &Arena) -> T;
    fn visit_expression(&mut self, expression: &Expression, arena: &Arena) -> T;
}

// AST walker utility
pub struct AstWalker;

impl AstWalker {
    pub fn walk_program<T, V: Visitor<T>>(program: &Program, arena: &Arena, visitor: &mut V) -> T {
        visitor.visit_program(program, arena)
    }

    pub fn walk_statement<T, V: Visitor<T>>(
        statement_id: NodeId,
        arena: &Arena,
        visitor: &mut V,
    ) -> Option<T> {
        arena
            .get_statement(statement_id)
            .map(|stmt| visitor.visit_statement(stmt, arena))
    }

    pub fn walk_expression<T, V: Visitor<T>>(
        expression_id: NodeId,
        arena: &Arena,
        visitor: &mut V,
    ) -> Option<T> {
        arena
            .get_expression(expression_id)
            .map(|expr| visitor.visit_expression(expr, arena))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_arena_ast_construction() {
        let mut arena = Arena::new();
        let mut builder = AstBuilder::new(&mut arena);

        // Build: let x = 5;
        let value_id = builder.integer_literal(Token::Int(5), 5);
        let let_stmt_id = builder.let_statement(
            Token::Let,
            Identifier {
                token: Token::Identifier("x".to_string()),
                name: "x".to_string(),
            },
            value_id,
        );

        let mut program = Program::new();
        program.add_statement(let_stmt_id);

        assert_eq!(program.statements.len(), 1);

        let stmt = arena.get_statement(let_stmt_id).unwrap();
        match stmt {
            Statement::Let(let_stmt) => {
                assert_eq!(let_stmt.name.name, "x");
                let value_expr = arena.get_expression(let_stmt.value).unwrap();
                match value_expr {
                    Expression::IntegerLiteral(int_lit) => {
                        assert_eq!(int_lit.value, 5);
                    }
                    _ => panic!("Expected integer literal"),
                }
            }
            _ => panic!("Expected let statement"),
        }
    }

    #[test]
    fn test_infix_expression() {
        let mut arena = Arena::new();
        let mut builder = AstBuilder::new(&mut arena);

        // Build: 5 + 10
        let left_id = builder.integer_literal(Token::Int(5), 5);
        let right_id = builder.integer_literal(Token::Int(10), 10);
        let infix_id = builder.infix_expression(Token::Plus, left_id, "+".to_string(), right_id);

        let expr = arena.get_expression(infix_id).unwrap();
        match expr {
            Expression::InfixExpression(infix) => {
                assert_eq!(infix.operator, "+");

                let left_expr = arena.get_expression(infix.left).unwrap();
                let right_expr = arena.get_expression(infix.right).unwrap();

                match (left_expr, right_expr) {
                    (
                        Expression::IntegerLiteral(left_lit),
                        Expression::IntegerLiteral(right_lit),
                    ) => {
                        assert_eq!(left_lit.value, 5);
                        assert_eq!(right_lit.value, 10);
                    }
                    _ => panic!("Expected integer literals"),
                }
            }
            _ => panic!("Expected infix expression"),
        }
    }

    struct PrintVisitor {
        output: String,
    }

    impl PrintVisitor {
        fn new() -> Self {
            PrintVisitor {
                output: String::new(),
            }
        }
    }

    impl Visitor<()> for PrintVisitor {
        fn visit_program(&mut self, program: &Program, arena: &Arena) -> () {
            self.output.push_str("Program:\n");
            for stmt_id in &program.statements {
                if let Some(stmt) = arena.get_statement(*stmt_id) {
                    self.visit_statement(stmt, arena);
                }
            }
        }

        fn visit_statement(&mut self, statement: &Statement, arena: &Arena) -> () {
            match statement {
                Statement::Let(let_stmt) => {
                    self.output
                        .push_str(&format!("Let {} = ", let_stmt.name.name));
                    if let Some(expr) = arena.get_expression(let_stmt.value) {
                        self.visit_expression(expr, arena);
                    }
                    self.output.push_str(";\n");
                }
                Statement::Return(ret_stmt) => {
                    self.output.push_str("Return ");
                    if let Some(expr) = arena.get_expression(ret_stmt.value) {
                        self.visit_expression(expr, arena);
                    }
                    self.output.push_str(";\n");
                }
                _ => {}
            }
        }

        fn visit_expression(&mut self, expression: &Expression, arena: &Arena) -> () {
            match expression {
                Expression::IntegerLiteral(int_lit) => {
                    self.output.push_str(&int_lit.value.to_string());
                }
                Expression::Identifier(ident) => {
                    self.output.push_str(&ident.name);
                }
                Expression::InfixExpression(infix) => {
                    self.output.push('(');
                    if let Some(left) = arena.get_expression(infix.left) {
                        self.visit_expression(left, arena);
                    }
                    self.output.push_str(&format!(" {} ", infix.operator));
                    if let Some(right) = arena.get_expression(infix.right) {
                        self.visit_expression(right, arena);
                    }
                    self.output.push(')');
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_visitor_pattern() {
        let mut arena = Arena::new();
        let mut builder = AstBuilder::new(&mut arena);

        let value_id = builder.integer_literal(Token::Int(42), 42);
        let let_stmt_id = builder.let_statement(
            Token::Let,
            Identifier {
                token: Token::Identifier("answer".to_string()),
                name: "answer".to_string(),
            },
            value_id,
        );

        let mut program = Program::new();
        program.add_statement(let_stmt_id);

        let mut visitor = PrintVisitor::new();
        AstWalker::walk_program(&program, &arena, &mut visitor);

        assert!(visitor.output.contains("Let answer = 42;"));
    }
}
