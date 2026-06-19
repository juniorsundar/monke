use monke::{
    environment::Environment,
    evaluator::eval_program,
    lexer::Lexer,
    object::{Object, ObjectType},
    parser::Parser,
};

#[test]
fn test_eval_integer_expressions() {
    let tests = [
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("-10", -10),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("20 + 2 * -10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for test_item in tests {
        let evaluated = test_eval(test_item.0.to_string());
        test_integer_object(evaluated, test_item.1);
    }
}

#[test]
fn test_eval_boolean_expressions() {
    let tests = [
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
    ];

    for test_item in tests {
        let evaluated = test_eval(test_item.0.to_string());
        test_boolean_object(evaluated, test_item.1);
    }
}

#[test]
fn test_bang_operator() {
    let tests = [
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
    ];

    for test_item in tests {
        let evaluated = test_eval(test_item.0.to_string());
        test_boolean_object(evaluated, test_item.1);
    }
}

#[test]
fn test_if_else_expressions() {
    let tests = [
        ("if (true) { 10 }", Some(10)),
        ("if (false) { 10 }", None),
        ("if (1) { 10 }", Some(10)),
        ("if (1 < 2) { 10 }", Some(10)),
        ("if (1 > 2) { 10 }", None),
        ("if (1 > 2) { 10 } else { 20 }", Some(20)),
        ("if (1 < 2) { 10 } else { 20 }", Some(10)),
    ];

    for test_item in tests {
        let evaluated = test_eval(test_item.0.to_string());
        match test_item.1 {
            Some(val) => test_integer_object(evaluated, val),
            None => test_null_object(evaluated),
        }
    }
}

#[test]
fn test_return_statements() {
    let tests = [
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2 * 5; 9;", 10),
        (
            "if (10 > 1) {
                if (10 > 1) {
                    return 10;
                }
                return 1;
            }",
            10,
        ),
    ];

    for test_item in tests {
        let evaluated = test_eval(test_item.0.to_string());
        test_integer_object(evaluated, test_item.1);
    }
}

#[test]
fn test_error_handling() {
    let tests = [
        ("5 + true;", "type mismatch: Integer + Boolean"),
        ("5 + true; 5;", "type mismatch: Integer + Boolean"),
        ("-true", "unknown operator: -Boolean"),
        ("true + false;", "unknown operator: Boolean + Boolean"),
        ("5; true + false; 5", "unknown operator: Boolean + Boolean"),
        (
            "if (10 > 1) { true + false; }",
            "unknown operator: Boolean + Boolean",
        ),
        (
            "if (10 > 1) {
                if (10 > 1) {
                    return true + false;
                }
                return 1;
            }
            ",
            "unknown operator: Boolean + Boolean",
        ),
        ("foobar", "identifier not found: foobar"),
    ];

    for test_item in tests {
        let evaluated = test_eval(test_item.0.to_string());

        if let Object::Error(message) = evaluated {
            assert_eq!(test_item.1, message)
        } else {
            panic!("No error object returned, got={}", evaluated.object_type())
        }
    }
}

#[test]
fn test_let_statements() {
    let tests = [
        ("let a = 5; a;", 5),
        ("let a = 5 * 5; a;", 25),
        ("let a = 5; let b = a; b;", 5),
        ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
    ];
    for test_item in tests {
        let evaluated = test_eval(test_item.0.to_string());
        test_integer_object(evaluated, test_item.1);
    }
}

#[test]
fn test_function_object() {
    let input = String::from("fn(x) { x + 2; };");
    let evaluated = test_eval(input);

    match evaluated {
        Object::Function(func) => {
            assert_eq!(
                func.parameters.len(),
                1,
                "function has wrong number of parameters. Parameters={:?}",
                func.parameters
            );

            assert_eq!(
                func.parameters[0].string(),
                "x",
                "parameter is not 'x'. got={:?}",
                func.parameters[0]
            );

            assert_eq!(func.body.string(), "(x + 2)")
        }
        _ => panic!("Object is not Function. got={}", evaluated.object_type()),
    }
}

fn test_null_object(evaluated: Object) {
    assert_eq!(
        evaluated.object_type(),
        ObjectType::Null,
        "Evaluated object is not ObjectType::Null, got={:?}",
        evaluated.object_type()
    );
}

fn test_boolean_object(evaluated: Object, test_item: bool) {
    assert_eq!(
        evaluated.object_type(),
        ObjectType::Boolean,
        "Evaluated object is not ObjectType::Boolean, got={:?}",
        evaluated.object_type()
    );

    let Object::Boolean(eval_bool) = evaluated else {
        panic!("Evaluated object was not bool");
    };

    assert_eq!(
        eval_bool, test_item,
        "Evaluated is {}, Provided was {}",
        eval_bool, test_item
    );
}

fn test_integer_object(evaluated: Object, test_item: i64) {
    assert_eq!(
        evaluated.object_type(),
        ObjectType::Integer,
        "Evaluated object is not ObjectType::Integer, got={:?}",
        evaluated.object_type()
    );

    let Object::Integer(eval_int) = evaluated else {
        panic!("Evaluated object was not i64");
    };

    assert_eq!(
        eval_int, test_item,
        "Evaluated is {}, Provided was {}",
        eval_int, test_item
    );
}

fn test_eval(input: String) -> Object {
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    let mut environment = Environment::new();

    eval_program(&program, &mut environment)
}
