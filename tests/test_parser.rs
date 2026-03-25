use std::ops::Deref;

use monke::{
    ast::{Expression, Statement},
    lexer::Lexer,
    parser::Parser,
};

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
            "Expression was not IntegerLiteral, ot: {:?}",
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
    let inputs = vec!["!5;", "-15;"];
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
