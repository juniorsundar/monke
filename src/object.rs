use std::fmt;

use crate::{
    ast::{BlockStatement, Identifier},
    environment::Environment,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    Integer,
    Boolean,
    Null,
    Return,
    Error,
    Function,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjectType::Integer => write!(f, "Integer"),
            ObjectType::Boolean => write!(f, "Boolean"),
            ObjectType::Null => write!(f, "Null"),
            ObjectType::Return => write!(f, "Return"),
            ObjectType::Error => write!(f, "Error"),
            ObjectType::Function => write!(f, "Function"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub environment: Environment,
}
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.parameters == other.parameters && self.body == other.body
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    Return(Box<Object>),
    Error(String),
    Function(Function),
}

impl Object {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Integer(_) => ObjectType::Integer,
            Object::Boolean(_) => ObjectType::Boolean,
            Object::Null => ObjectType::Null,
            Object::Return(_) => ObjectType::Return,
            Object::Error(_) => ObjectType::Error,
            Object::Function(_) => ObjectType::Function,
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(val) => val.to_string(),
            Object::Boolean(val) => val.to_string(),
            Object::Null => "NULL".to_string(),
            Object::Return(val) => val.inspect(),
            Object::Error(val) => format!("ERROR: {}", val),
            Object::Function(val) => {
                let mut out = String::new();
                let mut params: Vec<String> = Vec::new();
                for param in &val.parameters {
                    params.push(param.string());
                }

                out.push_str("fn");
                out.push_str("(");
                out.push_str(&params.join(", "));
                out.push_str(") {\n");
                out.push_str(&val.body.string());
                out.push_str("\n}");

                out
            }
        }
    }
}
