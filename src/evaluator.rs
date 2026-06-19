use crate::{
    ast::{BlockStatement, Expression, Identifier, Program, Statement},
    environment::Environment,
    object::{Function, Object, ObjectType},
};

pub fn eval_program(program: &Program, environment: &mut Environment) -> Object {
    let mut result = Object::Null;

    for statement in &program.statements {
        result = eval_statement(statement, environment);

        match result {
            Object::Return(return_obj) => return *return_obj,
            Object::Error(_) => return result,
            _ => continue,
        }
    }

    result
}

fn eval_statement(statement: &Statement, environment: &mut Environment) -> Object {
    match statement {
        Statement::Expression(expr_stmt) => {
            if let Some(expr) = &expr_stmt.value {
                eval(expr, environment)
            } else {
                Object::Null
            }
        }
        Statement::Block(block_stmt) => eval_block_statement(block_stmt, environment),
        Statement::Return(return_stmt) => {
            if let Some(expr) = &return_stmt.value {
                let return_obj = eval(expr, environment);
                if is_error(&return_obj) {
                    return return_obj;
                };
                Object::Return(Box::new(return_obj))
            } else {
                Object::Null
            }
        }
        Statement::Let(let_stmt) => {
            if let Some(expr) = &let_stmt.value {
                let let_obj = eval(expr, environment);
                if is_error(&let_obj) {
                    return let_obj;
                };
                environment.set(let_stmt.name.value.clone(), let_obj)
            } else {
                Object::Null
            }
        }
    }
}

fn eval_block_statement(block_stmt: &BlockStatement, environment: &mut Environment) -> Object {
    let mut result = Object::Null;
    for statement in &block_stmt.statements {
        result = eval_statement(statement, environment);

        if result.object_type() == ObjectType::Return || result.object_type() == ObjectType::Error {
            return result;
        }
    }
    result
}

pub fn eval(expression: &Expression, environment: &mut Environment) -> Object {
    match expression {
        Expression::IntegerLiteral(node) => Object::Integer(node.value),
        Expression::BooleanLiteral(node) => Object::Boolean(node.value),
        Expression::Prefix(node) => {
            let right = match node.right.as_deref() {
                Some(expr) => eval(expr, environment),
                None => return Object::Null,
            };
            if is_error(&right) {
                return right;
            }
            eval_prefix_expression(&node.operator, right)
        }
        Expression::Infix(node) => {
            let left = match node.left.as_deref() {
                Some(expr) => eval(expr, environment),
                None => return Object::Null,
            };
            if is_error(&left) {
                return left;
            }
            let right = match node.right.as_deref() {
                Some(expr) => eval(expr, environment),
                None => return Object::Null,
            };
            if is_error(&right) {
                return right;
            }
            eval_infix_expression(&node.operator, left, right)
        }
        Expression::If(node) => match (&node.condition, &node.consequence, &node.alternative) {
            (Some(cond_expr), conseq, _) => {
                eval_if_expression(cond_expr, conseq, node.alternative.as_ref(), environment)
            }
            (_, _, _) => Object::Null,
        },
        Expression::Identifier(node) => eval_identifier(node, environment),
        Expression::FunctionLiteral(node) => {
            Object::Function(Function {
                parameters: node.parameters.clone(),
                body: node.body.clone(),
                environment: environment.to_owned(), // IMPORTANT: This creates snapshot
                                                     // behaviour!!! May have to check
                                                     // Rc<RefCell<Environment>> to avoid this behaviour
            })
        }
        _ => Object::Null,
    }
}

fn eval_identifier(identifier: &Identifier, environment: &mut Environment) -> Object {
    match environment.get(&identifier.value) {
        Some(val) => val.clone(),
        None => new_error(format!("identifier not found: {}", identifier.value)),
    }
}

fn eval_if_expression(
    cond_expr: &Expression,
    conseq: &BlockStatement,
    alternative: Option<&BlockStatement>,
    environment: &mut Environment,
) -> Object {
    let condition = eval(cond_expr, environment);
    if is_error(&condition) {
        return condition;
    }
    if is_truthy(condition) {
        eval_block_statement(conseq, environment)
    } else if alternative.is_some() {
        eval_block_statement(alternative.unwrap(), environment)
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
    match (&left, &right) {
        (Object::Integer(left_int), Object::Integer(right_int)) => {
            eval_integer_infix_expression(operator, *left_int, *right_int)
        }
        (Object::Boolean(left_bool), Object::Boolean(right_bool)) => match operator {
            "==" => Object::Boolean(left_bool == right_bool),
            "!=" => Object::Boolean(left_bool != right_bool),
            _ => new_error(format!(
                "unknown operator: {} {} {}",
                ObjectType::Boolean,
                operator,
                ObjectType::Boolean
            )),
        },
        _ => {
            if left.object_type() != right.object_type() {
                return new_error(format!(
                    "type mismatch: {} {} {}",
                    &left.object_type(),
                    operator,
                    &right.object_type()
                ));
            }
            new_error(format!(
                "unknown operator: {} {} {}",
                &left.object_type(),
                operator,
                &right.object_type()
            ))
        }
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
        _ => new_error(format!(
            "unknown operator: {} {} {}",
            ObjectType::Integer,
            operator,
            ObjectType::Integer
        )),
    }
}

fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => new_error(format!(
            "unknown operator: {}{}",
            operator,
            right.object_type()
        )),
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer(val) => Object::Integer(-val),
        _ => new_error(format!("unknown operator: -{}", right.object_type())),
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

fn new_error(message: String) -> Object {
    Object::Error(message)
}

fn is_error(object: &Object) -> bool {
    object.object_type() == ObjectType::Error
}
