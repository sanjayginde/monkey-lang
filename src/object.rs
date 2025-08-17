use crate::ast::Node;

// #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub enum Object {
    Null,
    Integer(i64),
    Boolean(bool),
}

impl Object {
    pub fn inspect(&self) -> String {
        match &self {
            Object::Null => "null".to_owned(),
            Object::Integer(i) => i.to_string(),
            Object::Boolean(b) => b.to_string(),
        }
    }
}

pub fn eval(_n: Node) -> Object {
    todo!()
}
