use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    Integer,
    Boolean,
    Null,
    Return,
    Error,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjectType::Integer => write!(f, "Integer"),
            ObjectType::Boolean => write!(f, "Boolean"),
            ObjectType::Null => write!(f, "Null"),
            ObjectType::Return => write!(f, "Return"),
            ObjectType::Error => write!(f, "Error"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    Return(Box<Object>),
    Error(String),
}

impl Object {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Integer(_) => ObjectType::Integer,
            Object::Boolean(_) => ObjectType::Boolean,
            Object::Null => ObjectType::Null,
            Object::Return(_) => ObjectType::Return,
            Object::Error(_) => ObjectType::Error,
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(val) => val.to_string(),
            Object::Boolean(val) => val.to_string(),
            Object::Null => "NULL".to_string(),
            Object::Return(val) => val.inspect(),
            Object::Error(val) => format!("ERROR: {}", val),
        }
    }
}
