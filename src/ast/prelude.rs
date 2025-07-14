//! AST prelude module for commonly used types.
//!
//! This module re-exports the most commonly used types from the AST module,
//! allowing consumers to import them with a single `use crate::ast::prelude::*;`
//! statement instead of importing each type individually.

// Re-export the main AST node types
pub use super::{Expression, Node, Program, Statement};

// Re-export concrete expression types
pub use super::{BooleanLiteral, FunctionLiteral, Identifier, IntegerLiteral};

// Re-export concrete statement types
pub use super::{BlockStatement, ExpressionStatement, LetStatement, ReturnStatement};
