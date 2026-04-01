use std::ops::Deref;

use monke::{
    ast::{Expression, Statement},
    lexer::Lexer,
    parser::Parser,
};

#[derive(Debug)]
pub enum Expected<'a> {
    Integer(i64),
    Identifier(&'a str),
}

fn test_let_statement(statement: &Statement, name: &str) {
    assert_eq!(
        statement.token_literal(),
        "let",
        "token_literal not 'let', got: {}",
        statement.token_literal()
    );

    match statement {
        Statement::Let(t) => {
            assert_eq!(
                t.name.value, name,
                "LetStatement.name.value not {}, got: {}",
                name, t.name.value
            );
            assert_eq!(
                t.name.token.t_literal, name,
                "LetStatement.name.token.t_literal not {}, got: {}",
                name, t.name.token.t_literal
            );
        }
        _ => {
            panic!("Expected LetStatement, got: {:?}", statement);
        }
    }
}

fn test_integer_literal(integer_literal: &Expression, value: i64) {
    let Expression::IntegerLiteral(il) = integer_literal else {
        panic!(
            "Expression was not IntegerLiteral, got: {:?}",
            integer_literal
        );
    };

    assert_eq!(
        il.value, value,
        "IntegerLiteral.value not {}, got: {}",
        value, il.value
    );
    assert_eq!(
        il.token.t_literal,
        value.to_string(),
        "IntegerLiteral.token.value not {}, got: {}",
        value,
        il.token.t_literal
    );
}

fn test_identifier(expression: &Expression, value: &str) {
    let Expression::Identifier(ident) = expression else {
        panic!("Expression was not Identifier, got: {:?}", expression);
    };

    assert_eq!(
        ident.value, value,
        "Identifier.value not {}, got: {}",
        value, ident.value
    );

    assert_eq!(
        ident.token.t_literal,
        value.to_string(),
        "Identifier.token.value not {}, got: {}",
        value,
        ident.token.t_literal
    );
}

fn test_literal_expression(expression: &Expression, expected: Expected) {
    match expected {
        Expected::Integer(integer) => test_integer_literal(expression, integer),
        Expected::Identifier(identifier) => test_identifier(expression, identifier),
    }
    panic!(
        "Type mismatch or unhandled type. Got: {:?}, Expected: {:?}",
        expression, expected
    );
}

fn test_infix_expression(expression: &Expression, left: Expected, operator: &str, right: Expected) {
    let Expression::Infix(infix) = expression else {
        panic!("Expression was not Infix, got: {:?}", expression)
    };

    let Some(left_exp) = infix.left.as_deref() else {
        panic!("Infix did not have left expression, got {:?}", infix.left);
    };
    test_literal_expression(left_exp, left);

    assert_eq!(
        infix.operator,
        operator.to_string(),
        "Operator is not {:?}, got {:?}",
        operator,
        infix.operator
    );

    let Some(right_exp) = infix.right.as_deref() else {
        panic!("Infix did not have right expression, got {:?}", infix.right);
    };
    test_literal_expression(right_exp, right);
}

fn check_parser_errors(parser: &Parser) {
    if parser.errors.is_empty() {
        return;
    }

    for i in 0..parser.errors.len() {
        eprintln!("{}", parser.errors[i]);
    }
    panic!("Parsing Errors!");
}

#[test]
fn test_let_statements() {
    let input = "
    let x = 5;
    let y = 10;
    let foobar = 838383;
    ";

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        3,
        "program.Statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    let expecteds = ["x", "y", "foobar"];

    for (expected, statement) in expecteds.iter().zip(program.statements.iter()) {
        test_let_statement(statement, expected);
    }
}

// #[test]
// fn test_failed_let_statements() {
//     let input = "
//     let x 5;
//     let = 10;
//     let 838383;
//     ";
//
//     let lexer = Lexer::new(input.to_string());
//     let mut parser = Parser::new(lexer);
//
//     let program = parser.parse_program();
//     check_parser_errors(&parser);
//
//     assert_eq!(
//         program.statements.len(),
//         3,
//         "program.Statements does not contain 3 statements. got={}",
//         program.statements.len()
//     );
//
//     let expecteds = vec!["x", "y", "foobar"];
//
//     for (expected, statement) in expecteds.iter().zip(program.statements.iter()) {
//         test_let_statement(statement, expected);
//     }
// }

#[test]
fn test_return_statements() {
    let input = "
    return 5;
    return 10;
    return 993322;
    ";

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        3,
        "program.Statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    for statement in program.statements.iter() {
        match statement {
            Statement::Return(_) => {}
            _ => {
                panic!("Expected ReturnStatement, got: {:?}", statement);
            }
        }
        assert_eq!(
            statement.token_literal(),
            "return",
            "token_literal not 'return', got: {}",
            statement.token_literal()
        );
    }
}

#[test]
fn test_identifier_expression() {
    let input = "foobar;";

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        1,
        "program.Statements does not contain enough statements. got={}",
        program.statements.len()
    );

    if let Statement::Expression(e) = &program.statements[0] {
        if let Some(exp) = &e.value {
            if let Expression::Identifier(ident) = exp.deref() {
                assert_eq!(ident.value, "foobar".to_string());
                assert_eq!(ident.token.t_literal, "foobar".to_string());
            } else {
                panic!("Expression isn't an Identifier")
            }
        } else {
            panic!("Expression has no value")
        }
    } else {
        panic!(
            "Expected Statement::Expression(..) got={:?}",
            program.statements[0]
        )
    };
}

#[test]
fn test_integer_literal_expression() {
    let input = "5;";

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        1,
        "program.Statements does not contain enough statements. got={}",
        program.statements.len()
    );

    let Statement::Expression(e) = &program.statements[0] else {
        panic!(
            "Expected Statement::Expression(..) got={:?}",
            program.statements[0]
        )
    };

    let Some(Expression::IntegerLiteral(integer_literal)) = e.value.as_deref() else {
        panic!(
            "Expression is missing or not a IntegerLiteral. got={:?}",
            e.value
        );
    };

    assert_eq!(integer_literal.value, 5);
    assert_eq!(integer_literal.token.t_literal, "5".to_string());
}

#[test]
fn test_prefix_expressions() {
    let inputs = ["!5;", "-15;"];
    let parsed_outputs: Vec<(&str, i64)> = vec![("!", 5), ("-", 15)];

    for i in 0..inputs.len() {
        let lexer = Lexer::new(inputs[i].to_string());
        let expected_op = parsed_outputs[i].0;
        let expected_value = parsed_outputs[i].1;

        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            1,
            "program.Statements does not contain enough statements. got={}",
            program.statements.len()
        );

        let Statement::Expression(e) = &program.statements[0] else {
            panic!(
                "Expected Statement::Expression(..) got={:?}",
                program.statements[0]
            )
        };
        let Some(Expression::Prefix(prefix)) = e.value.as_deref() else {
            panic!("Expression is missing or not a Prefix. got={:?}", e.value);
        };

        assert_eq!(prefix.operator, expected_op);
        let right = prefix
            .right
            .as_deref()
            .expect("Could not parse expression on Right");
        test_integer_literal(right, expected_value);
    }
}

#[test]
fn test_infix_expressions() {
    let inputs = [
        "5 + 5;", "5 - 5;", "5 * 5;", "5 / 5;", "5 > 5;", "5 < 5;", "5 == 5;", "5 != 5;",
    ];
    let parsed_outputs: Vec<(i64, &str, i64)> = vec![
        (5, "+", 5),
        (5, "-", 5),
        (5, "*", 5),
        (5, "/", 5),
        (5, ">", 5),
        (5, "<", 5),
        (5, "==", 5),
        (5, "!=", 5),
    ];

    for i in 0..inputs.len() {
        let lexer = Lexer::new(inputs[i].to_string());
        let expected_lv = parsed_outputs[i].0;
        let expected_op = parsed_outputs[i].1;
        let expected_rv = parsed_outputs[i].2;

        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            1,
            "program.Statements does not contain enough statements. got={}",
            program.statements.len()
        );

        let Statement::Expression(e) = &program.statements[0] else {
            panic!(
                "Expected Statement::Expression(..) got={:?}",
                program.statements[0]
            )
        };
        let Some(Expression::Infix(infix)) = e.value.as_deref() else {
            panic!("Expression is missing or not a Infix. got={:?}", e.value);
        };

        let left = infix
            .left
            .as_deref()
            .expect("Could not parse expression on Left");
        test_integer_literal(left, expected_lv);
        assert_eq!(infix.operator, expected_op);
        let right = infix
            .right
            .as_deref()
            .expect("Could not parse expression on Right");
        test_integer_literal(right, expected_rv);
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let inputs = [
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
    ];

    for i in 0..inputs.len() {
        let lexer = Lexer::new(inputs[i].0.to_string());
        let mut parser = Parser::new(lexer);
        let mut program = parser.parse_program();
        check_parser_errors(&parser);

        let actual = program.string();
        assert_eq!(
            actual,
            inputs[i].1.to_string(),
            "Expected: {}, Got: {}",
            actual,
            inputs[i].1.to_string()
        )
    }
}
