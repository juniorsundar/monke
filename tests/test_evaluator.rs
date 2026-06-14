use monke::{
    evaluator::eval_program,
    lexer::Lexer,
    object::{Object, ObjectType},
    parser::Parser,
};

#[test]
fn test_eval_integer_expression() {
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
fn test_eval_boolean_expression() {
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

    eval_program(&program)
}
