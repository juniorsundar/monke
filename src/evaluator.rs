use crate::{ast::Expression, object::Object};

pub fn eval(expression: &Expression) -> Object {
    match expression {
        Expression::IntegerLiteral(node) => Object::Integer(node.value),
        _ => Object::Null,
    }
}
