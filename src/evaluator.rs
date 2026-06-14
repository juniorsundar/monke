use crate::{
    ast::{BlockStatement, Expression, Program, Statement},
    object::Object,
};

pub fn eval_program(program: &Program) -> Object {
    let mut result = Object::Null;

    for statement in &program.statements {
        result = eval_statement(statement);
        if let Object::Return(return_obj) = result {
            return *return_obj;
        }
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
        Statement::Block(block_stmt) => eval_block_statement(block_stmt),
        Statement::Return(return_stmt) => {
            if let Some(expr) = &return_stmt.value {
                let return_obj = eval(expr);
                Object::Return(Box::new(return_obj))
            } else {
                Object::Null
            }
        }
        _ => todo!(),
    }
}

fn eval_block_statement(block_stmt: &BlockStatement) -> Object {
    let mut result = Object::Null;
    for statement in &block_stmt.statements {
        result = eval_statement(statement);
    }
    result
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
        Expression::Infix(node) => {
            let left = match node.left.as_deref() {
                Some(expr) => eval(expr),
                None => return Object::Null,
            };
            let right = match node.right.as_deref() {
                Some(expr) => eval(expr),
                None => return Object::Null,
            };
            eval_infix_expression(&node.operator, left, right)
        }
        Expression::If(node) => match (&node.condition, &node.consequence, &node.alternative) {
            (Some(cond_expr), conseq, _) => {
                eval_if_expression(cond_expr, conseq, node.alternative.as_ref())
            }
            (_, _, _) => Object::Null,
        },
        _ => Object::Null,
    }
}

fn eval_if_expression(
    cond_expr: &Expression,
    conseq: &BlockStatement,
    alternative: Option<&BlockStatement>,
) -> Object {
    let condition = eval(cond_expr);
    if is_truthy(condition) {
        eval_block_statement(conseq)
    } else if alternative.is_some() {
        eval_block_statement(alternative.unwrap())
    } else {
        Object::Null
    }
}

fn is_truthy(condition: Object) -> bool {
    match condition {
        Object::Null => false,
        Object::Boolean(true) => true,
        Object::Boolean(false) => false,
        _ => true,
    }
}

fn eval_infix_expression(operator: &str, left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Integer(left_int), Object::Integer(right_int)) => {
            eval_integer_infix_expression(operator, left_int, right_int)
        }
        (Object::Boolean(left_bool), Object::Boolean(right_bool)) => match operator {
            "==" => Object::Boolean(left_bool == right_bool),
            "!=" => Object::Boolean(left_bool != right_bool),
            _ => Object::Null,
        },
        _ => Object::Null,
    }
}

fn eval_integer_infix_expression(operator: &str, left_int: i64, right_int: i64) -> Object {
    match operator {
        "+" => Object::Integer(left_int + right_int),
        "-" => Object::Integer(left_int - right_int),
        "*" => Object::Integer(left_int * right_int),
        "/" => Object::Integer(left_int / right_int),
        "<" => Object::Boolean(left_int < right_int),
        ">" => Object::Boolean(left_int > right_int),
        "==" => Object::Boolean(left_int == right_int),
        "!=" => Object::Boolean(left_int != right_int),
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
