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
        Expression::Prefix(node) => {
            let right = match node.right.as_deref() {
                Some(expr) => eval(expr),
                None => return Object::Null,
            };
            eval_prefix_expression(&node.operator, right)
        }
        _ => Object::Null,
    }
}

fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Object::Null,
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer(val) => Object::Integer(-val),
        _ => Object::Null,
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        Object::Boolean(true) => Object::Boolean(false),
        Object::Boolean(false) => Object::Boolean(true),
        Object::Null => Object::Boolean(true),
        _ => Object::Boolean(false),
    }
}
