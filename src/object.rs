use crate::ast::Node;

// #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub enum Object {
    Null,
    Integer(i64),
    Boolean(bool),
}

impl Object {
    pub fn _inspect(&self) -> String {
        match &self {
            Object::Null => "null".to_owned(),
            Object::Integer(i) => i.to_string(),
            Object::Boolean(b) => b.to_string(),
        }
    }
}

pub fn _eval(_n: Node) -> Object {
    match _n {
        Node::Ident(_identifier) => todo!(),
        Node::Func(_function_literal) => todo!(),
        Node::Int(_integer_literal) => todo!(),
        Node::Bool(_boolean_literal) => todo!(),
        Node::Stmt(_statement) => todo!(),
        Node::Expr(_expression) => todo!(),
    }
}
