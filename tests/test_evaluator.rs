use monke::{
    evaluator::eval_program,
    lexer::Lexer,
    object::{Object, ObjectType},
    parser::Parser,
};

#[test]
fn test_eval_integer_expression() {
    let tests = [("5", 5), ("10", 10)];

    for test_item in tests {
        let evaluated = test_eval(test_item.0.to_string());
        test_integer_object(evaluated, test_item.1);
    }
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
