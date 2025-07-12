# Alternatives to `Box<dyn Trait>` in Rust

This document explains various alternatives to using `Box<dyn Trait>` for representing polymorphic data structures in Rust, with a focus on AST (Abstract Syntax Tree) implementations.

## The Problem with `Box<dyn Trait>`

When you need to store different types that implement the same trait, `Box<dyn Trait>` is often the first solution that comes to mind:

```rust
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

pub struct LetStatement {
    pub value: Box<dyn Expression>,
}
```

However, this approach has several drawbacks:
- **Heap allocation** for every node
- **Dynamic dispatch overhead** at runtime
- **Runtime type checking** instead of compile-time
- **Difficult to clone** (requires manual implementation)
- **Cannot derive Debug** easily

## Alternative 1: Enums (Recommended)

The most common and efficient alternative is using enums:

```rust
#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    BinaryExpression(BinaryExpression),
}

pub struct Program {
    pub statements: Vec<Statement>, // No Box needed!
}

pub struct LetStatement {
    pub value: Expression, // No Box needed!
}
```

### Benefits of Enums:
- **Stack allocation** for most nodes
- **Static dispatch** (no runtime overhead)
- **Compile-time type checking**
- **Easy to derive** Clone, Debug, PartialEq, etc.
- **Exhaustive pattern matching**
- **Better performance**

### When You Still Need Box:
For recursive types (like binary expressions), you still need `Box`:

```rust
pub struct BinaryExpression {
    pub left: Box<Expression>,   // Box needed for recursion
    pub operator: String,
    pub right: Box<Expression>,  // Box needed for recursion
}
```

## Alternative 2: Generic Types

If you have a fixed set of types, you can use generics:

```rust
pub struct Program<T> {
    pub statements: Vec<T>,
}

pub struct LetStatement<T> {
    pub value: T,
}

// Usage with specific types
type TypedProgram = Program<LetStatement<IntegerLiteral>>;
```

## Alternative 3: Type Erasure with `Any`

For cases where you need runtime type checking:

```rust
use std::any::Any;

pub struct Program {
    pub statements: Vec<Box<dyn Any>>,
}

// Usage
let program = Program {
    statements: vec![
        Box::new(LetStatement { /* ... */ }),
        Box::new(ReturnStatement { /* ... */ }),
    ],
};

// Type checking at runtime
for statement in &program.statements {
    if let Some(let_stmt) = statement.downcast_ref::<LetStatement>() {
        // Handle let statement
    } else if let Some(ret_stmt) = statement.downcast_ref::<ReturnStatement>() {
        // Handle return statement
    }
}
```

## Alternative 4: Visitor Pattern

For complex AST traversal:

```rust
pub trait Visitor {
    fn visit_let_statement(&mut self, stmt: &LetStatement);
    fn visit_return_statement(&mut self, stmt: &ReturnStatement);
    fn visit_integer_literal(&mut self, lit: &IntegerLiteral);
    fn visit_binary_expression(&mut self, expr: &BinaryExpression);
}

impl Statement {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Statement::Let(stmt) => visitor.visit_let_statement(stmt),
            Statement::Return(stmt) => visitor.visit_return_statement(stmt),
        }
    }
}
```

## Alternative 5: Small Vec Optimization

For cases where you have a small, fixed set of variants:

```rust
use smallvec::SmallVec;

pub struct Program {
    pub statements: SmallVec<[Statement; 4]>, // Stack-allocated for up to 4 items
}
```

## Migration Guide

### From `Box<dyn Trait>` to Enums:

1. **Replace trait objects with enums:**
   ```rust
   // Before
   Vec<Box<dyn Statement>>
   
   // After
   Vec<Statement>
   ```

2. **Update pattern matching:**
   ```rust
   // Before
   statement.token_literal()
   
   // After
   match statement {
       Statement::Let(stmt) => stmt.token_literal(),
       Statement::Return(stmt) => stmt.token_literal(),
   }
   ```

3. **Add helper methods for construction:**
   ```rust
   impl Statement {
       pub fn let_statement(name: String, value: Expression) -> Self {
           Statement::Let(LetStatement { name, value })
       }
   }
   ```

4. **Update parser to return enum variants:**
   ```rust
   // Before
   Ok(Box::new(stmt) as Box<dyn Statement>)
   
   // After
   Ok(Statement::Let(stmt))
   ```

## Performance Comparison

| Approach | Memory | Dispatch | Type Safety | Clone | Debug |
|----------|--------|----------|-------------|-------|-------|
| `Box<dyn Trait>` | Heap | Dynamic | Runtime | Manual | Manual |
| Enums | Stack | Static | Compile-time | Auto | Auto |
| Generics | Stack | Static | Compile-time | Auto | Auto |
| `Any` | Heap | Dynamic | Runtime | Manual | Manual |

## When to Use Each Approach

- **Use Enums** for most cases, especially ASTs
- **Use `Box<dyn Trait>`** when you need true polymorphism with unknown types
- **Use Generics** when you have a fixed set of types
- **Use `Any`** when you need runtime type checking
- **Use Visitor Pattern** for complex traversal logic

## Example Implementation

See `examples/comparison.rs` for a complete example showing both approaches side by side.

## Conclusion

For AST implementations and most other use cases, enums are the preferred alternative to `Box<dyn Trait>`. They provide better performance, type safety, and developer experience while eliminating the need for heap allocation in most cases. 