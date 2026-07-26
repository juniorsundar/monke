use crate::{
    ast::{BlockStatement, Expression, Identifier, Program, Statement},
    environment::{EnvId, Environment},
    object::{Function, Object, ObjectType},
};

pub struct Runtime {
    environments: Vec<Environment>,
}
impl Runtime {
    pub fn new() -> Self {
        Runtime {
            environments: Vec::<Environment>::new(),
        }
    }

    pub fn new_environment(&mut self) -> EnvId {
        self.environments.push(Environment::new());
        self.environments.len() - 1
    }

    fn new_enclosed_environment(&mut self, outer: EnvId) -> EnvId {
        let new_enc = Environment::new_enclosed(outer);
        self.environments.push(new_enc);
        self.environments.len() - 1
    }

    fn get(&self, env_id: EnvId, name: &str) -> Option<Object> {
        if env_id >= self.environments.len() {
            return None;
        }
        let mut current = Some(env_id);
        while let Some(id) = current {
            if let Some(obj) = self.environments[id].get(name) {
                return Some(obj);
            } else {
                current = self.environments[id].get_outer()
            }
        }
        None
    }

    fn set(&mut self, env_id: EnvId, name: String, val: Object) -> Object {
        self.environments[env_id].set(name, val)
    }
}
impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

pub fn eval_program(program: &Program, runtime: &mut Runtime, env_id: EnvId) -> Object {
    let mut result = Object::Null;

    for statement in &program.statements {
        result = eval_statement(statement, runtime, env_id);

        match result {
            Object::Return(return_obj) => return *return_obj,
            Object::Error(_) => return result,
            _ => continue,
        }
    }

    result
}

fn eval_statement(statement: &Statement, runtime: &mut Runtime, env_id: EnvId) -> Object {
    match statement {
        Statement::Expression(expr_stmt) => {
            if let Some(expr) = &expr_stmt.value {
                eval(expr, runtime, env_id)
            } else {
                Object::Null
            }
        }
        Statement::Block(block_stmt) => eval_block_statement(block_stmt, runtime, env_id),
        Statement::Return(return_stmt) => {
            if let Some(expr) = &return_stmt.value {
                let return_obj = eval(expr, runtime, env_id);
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
                let let_obj = eval(expr, runtime, env_id);
                if is_error(&let_obj) {
                    return let_obj;
                };
                runtime.set(env_id, let_stmt.name.value.clone(), let_obj)
            } else {
                Object::Null
            }
        }
    }
}

fn eval_block_statement(
    block_stmt: &BlockStatement,
    runtime: &mut Runtime,
    env_id: EnvId,
) -> Object {
    let mut result = Object::Null;
    for statement in &block_stmt.statements {
        result = eval_statement(statement, runtime, env_id);

        if result.object_type() == ObjectType::Return || result.object_type() == ObjectType::Error {
            return result;
        }
    }
    result
}

pub fn eval(expression: &Expression, runtime: &mut Runtime, env_id: EnvId) -> Object {
    match expression {
        Expression::IntegerLiteral(node) => Object::Integer(node.value),
        Expression::BooleanLiteral(node) => Object::Boolean(node.value),
        Expression::Prefix(node) => {
            let right = match node.right.as_deref() {
                Some(expr) => eval(expr, runtime, env_id),
                None => return Object::Null,
            };
            if is_error(&right) {
                return right;
            }
            eval_prefix_expression(&node.operator, right)
        }
        Expression::Infix(node) => {
            let left = match node.left.as_deref() {
                Some(expr) => eval(expr, runtime, env_id),
                None => return Object::Null,
            };
            if is_error(&left) {
                return left;
            }
            let right = match node.right.as_deref() {
                Some(expr) => eval(expr, runtime, env_id),
                None => return Object::Null,
            };
            if is_error(&right) {
                return right;
            }
            eval_infix_expression(&node.operator, left, right)
        }
        Expression::If(node) => match (&node.condition, &node.consequence, &node.alternative) {
            (Some(cond_expr), conseq, _) => eval_if_expression(
                cond_expr,
                conseq,
                node.alternative.as_ref(),
                runtime,
                env_id,
            ),
            (_, _, _) => Object::Null,
        },
        Expression::Identifier(node) => eval_identifier(node, runtime, env_id),
        Expression::FunctionLiteral(node) => Object::Function(Function {
            parameters: node.parameters.clone(),
            body: node.body.clone(),
            environment: env_id,
        }),
        Expression::Call(node) => {
            let function = eval(&node.function, runtime, env_id);
            if is_error(&function) {
                return function;
            }

            let args = eval_expression(&node.arguments, runtime, env_id);
            if args.len() == 1 && is_error(&args[0]) {
                return args[0].clone();
            }

            apply_function(&function, &args, runtime)
        }
        Expression::StringLiteral(node) => Object::String(node.value.to_string()),
    }
}

fn apply_function(function: &Object, args: &[Object], runtime: &mut Runtime) -> Object {
    if let Object::Function(func) = function {
        if func.parameters.len() != args.len() {
            return new_error(format!(
                "wrong number of arguments: got={}, want={}",
                args.len(),
                func.parameters.len()
            ));
        }
        let extended_environment = extend_function_env(func, args, runtime);
        let evaluated = eval_block_statement(&func.body, runtime, extended_environment);
        unwrap_return_value(evaluated)
    } else {
        new_error(format!("not a function: {}", function.object_type()))
    }
}

fn unwrap_return_value(evaluated: Object) -> Object {
    if let Object::Return(value) = evaluated {
        return *value;
    }
    evaluated
}

fn extend_function_env(function: &Function, args: &[Object], runtime: &mut Runtime) -> EnvId {
    let new_env = runtime.new_enclosed_environment(function.environment);
    for (idx, param) in function.parameters.iter().enumerate() {
        runtime.set(new_env, param.value.clone(), args[idx].clone());
    }
    new_env
}

fn eval_expression(arguments: &[Expression], runtime: &mut Runtime, env_id: EnvId) -> Vec<Object> {
    let mut result: Vec<Object> = Vec::new();
    for exp in arguments {
        let evaluated = eval(exp, runtime, env_id);
        if is_error(&evaluated) {
            return [evaluated].to_vec();
        }
        result.push(evaluated);
    }
    result
}

fn eval_identifier(identifier: &Identifier, runtime: &mut Runtime, env_id: EnvId) -> Object {
    match runtime.get(env_id, &identifier.value) {
        Some(val) => val,
        None => new_error(format!("identifier not found: {}", identifier.value)),
    }
}

fn eval_if_expression(
    cond_expr: &Expression,
    conseq: &BlockStatement,
    alternative: Option<&BlockStatement>,
    runtime: &mut Runtime,
    env_id: EnvId,
) -> Object {
    let condition = eval(cond_expr, runtime, env_id);
    if is_error(&condition) {
        return condition;
    }
    if is_truthy(condition) {
        eval_block_statement(conseq, runtime, env_id)
    } else if alternative.is_some() {
        eval_block_statement(alternative.unwrap(), runtime, env_id)
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
        (Object::String(left_str), Object::String(right_str)) => match operator {
            "+" => eval_string_infix_expression(left_str, right_str),
            _ => new_error(format!(
                "unknown operator: {} {} {}",
                ObjectType::String,
                operator,
                ObjectType::String,
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

fn eval_string_infix_expression(left_str: &str, right_str: &str) -> Object {
    Object::String(left_str.to_owned() + right_str)
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
