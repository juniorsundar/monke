use crate::{
    ast::{Expression, Program, Statement},
    object::Object,
};

pub fn eval_program(program: &Program) -> Object {
    let mut result = Object::Null;

    for statement in &program.statements {
        result = eval_statement(statement);
    }

    result
}

fn eval_statement(statement: &Statement) -> Object {
    match statement {
        Statement::Expression(expr_stmt) => {
            if let Some(expr) = &expr_stmt.value {
                eval(expr)
            } else {
                Object::Null
            }
        }
        _ => todo!(),
    }
}

pub fn eval(expression: &Expression) -> Object {
    match expression {
        Expression::IntegerLiteral(node) => Object::Integer(node.value),
        Expression::BooleanLiteral(node) => Object::Boolean(node.value),
        _ => Object::Null,
    }
}
