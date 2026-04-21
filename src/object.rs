use std::fmt;

pub enum ObjectType {
    Integer,
    Boolean,
    Null,
}
impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjectType::Integer => write!(f, "Integer"),
            ObjectType::Boolean => write!(f, "Boolean"),
            ObjectType::Null => write!(f, "Null"),
        }
    }
}

pub trait Object {
    fn object_type(&self) -> ObjectType {
        ObjectType::Null
    }
    fn object_inspect(&self) -> String {
        String::new()
    }
}

pub struct Integer {
    value: i64,
}
impl Object for Integer {
    fn object_type(&self) -> ObjectType {
        ObjectType::Integer
    }
    fn object_inspect(&self) -> String {
        format!("{}", self.value)
    }
}

pub struct Boolean {
    value: bool,
}
impl Object for Boolean {
    fn object_type(&self) -> ObjectType {
        ObjectType::Boolean
    }
    fn object_inspect(&self) -> String {
        format!("{}", self.value)
    }
}

pub struct Null {}
impl Object for Null {
    fn object_type(&self) -> ObjectType {
        ObjectType::Null
    }
    fn object_inspect(&self) -> String {
        "NULL".to_string()
    }
}
