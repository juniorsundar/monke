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
    Bool(bool),
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

fn test_boolean_literal(expression: &Expression, value: bool) {
    let Expression::BooleanLiteral(ident) = expression else {
        panic!("Expression was not BooleanLiteral, got: {:?}", expression);
    };

    assert_eq!(
        ident.value, value,
        "BooleanLiteral.value not {}, got: {}",
        value, ident.value
    );

    assert_eq!(
        ident.token.t_literal,
        value.to_string(),
        "BooleanLiteral.token.value not {}, got: {}",
        value,
        ident.token.t_literal
    );
}

fn test_literal_expression(expression: &Expression, expected: Expected) {
    match expected {
        Expected::Integer(integer) => test_integer_literal(expression, integer),
        Expected::Identifier(identifier) => test_identifier(expression, identifier),
        Expected::Bool(bool) => test_boolean_literal(expression, bool),
    }
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
fn test_boolean_literal_expression() {
    let input = "true;";

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

    let Some(Expression::BooleanLiteral(boolean_literal)) = e.value.as_deref() else {
        panic!(
            "Expression is missing or not a BooleanLiteral. got={:?}",
            e.value
        );
    };

    assert!(boolean_literal.value);
    assert_eq!(boolean_literal.token.t_literal, "true".to_string());
}

#[test]
fn test_numeric_prefix_expressions() {
    let inputs = ["!5;", "-15;"];
    let parsed_outputs = [("!", 5), ("-", 15)];

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
        test_literal_expression(right, Expected::Integer(expected_value));
    }
}

#[test]
fn test_boolean_prefix_expressions() {
    let inputs = ["!true;", "!false;"];
    let parsed_outputs = [("!", true), ("!", false)];

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
        test_literal_expression(right, Expected::Bool(expected_value));
    }
}

#[test]
fn test_infix_numeric_expressions() {
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

        test_infix_expression(
            &Expression::Infix(infix.clone()),
            Expected::Integer(expected_lv),
            expected_op,
            Expected::Integer(expected_rv),
        );
    }
}

#[test]
fn test_infix_boolean_expressions() {
    let inputs = ["true == true", "true != false", "false == false"];
    let parsed_outputs: Vec<(bool, &str, bool)> = vec![
        (true, "==", true),
        (true, "!=", false),
        (false, "==", false),
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

        test_infix_expression(
            &Expression::Infix(infix.clone()),
            Expected::Bool(expected_lv),
            expected_op,
            Expected::Bool(expected_rv),
        );
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
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
    ];

    for item in &inputs {
        let lexer = Lexer::new(item.0.to_string());
        let mut parser = Parser::new(lexer);
        let mut program = parser.parse_program();
        check_parser_errors(&parser);

        let actual = program.string();
        assert_eq!(actual, item.1, "Expected: {}, Got: {}", item.1, actual)
    }
}

#[test]
fn test_if_expressions() {
    let input = "if (x < y) { x }";
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        1,
        "program.Statements does not contain 1 statements. got={}",
        program.statements.len()
    );

    let Statement::Expression(e) = program.statements[0].clone() else {
        panic!(
            "Expected Statement::Expression(..) got={:?}",
            program.statements[0]
        )
    };

    let Some(Expression::If(if_exp)) = e.value.as_deref() else {
        panic!(
            "Expression is missing or not a IfExpression. got={:?}",
            e.value
        )
    };

    let Some(Expression::Infix(condition)) = if_exp.condition.as_deref() else {
        panic!(
            "Condition is not an InfixExpression. got={:?}",
            if_exp.condition
        )
    };

    test_infix_expression(
        &Expression::Infix(condition.clone()),
        Expected::Identifier("x"),
        "<",
        Expected::Identifier("y"),
    );

    let Statement::Block(consequence_block) = &if_exp.consequence else {
        panic!(
            "Consequence is not a BlockStatement. got={:?}",
            if_exp.consequence
        )
    };

    assert_eq!(
        consequence_block.statements.len(),
        1,
        "consequence.Statements does not container 1 statement. got={}",
        consequence_block.statements.len()
    );

    let Statement::Expression(consequence_expression) = &consequence_block.statements[0] else {
        panic!(
            "Consequence is not an ExpressionStatement. got={:?}",
            consequence_block.statements[0]
        )
    };

    let Some(Expression::Identifier(consequence_identifier)) =
        consequence_expression.value.as_deref()
    else {
        panic!(
            "Consequence expression is not an Identifier. got={:?}",
            consequence_expression.value
        )
    };

    test_identifier(&Expression::Identifier(consequence_identifier.clone()), "x");

    if if_exp.alternative.is_some() {
        panic!(
            "Alternative statement was not None. got={:?}",
            if_exp.alternative
        )
    }
}

#[test]
fn test_if_else_expressions() {
    let input = "if (x < y) { x } else { y }";
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        1,
        "program.Statements does not contain 1 statements. got={}",
        program.statements.len()
    );

    let Statement::Expression(e) = program.statements[0].clone() else {
        panic!(
            "Expected Statement::Expression(..) got={:?}",
            program.statements[0]
        )
    };

    let Some(Expression::If(if_exp)) = e.value.as_deref() else {
        panic!(
            "Expression is missing or not a IfExpression. got={:?}",
            e.value
        )
    };

    let Some(Expression::Infix(condition)) = if_exp.condition.as_deref() else {
        panic!(
            "Condition is not an InfixExpression. got={:?}",
            if_exp.condition
        )
    };

    test_infix_expression(
        &Expression::Infix(condition.clone()),
        Expected::Identifier("x"),
        "<",
        Expected::Identifier("y"),
    );

    let Statement::Block(consequence_block) = &if_exp.consequence else {
        panic!(
            "Consequence is not a BlockStatement. got={:?}",
            if_exp.consequence
        )
    };

    assert_eq!(
        consequence_block.statements.len(),
        1,
        "consequence.Statements does not container 1 statement. got={}",
        consequence_block.statements.len()
    );

    let Statement::Expression(consequence_expression) = &consequence_block.statements[0] else {
        panic!(
            "Consequence is not an ExpressionStatement. got={:?}",
            consequence_block.statements[0]
        )
    };

    let Some(Expression::Identifier(consequence_identifier)) =
        consequence_expression.value.as_deref()
    else {
        panic!(
            "Consequence expression is not an Identifier. got={:?}",
            consequence_expression.value
        )
    };

    test_identifier(&Expression::Identifier(consequence_identifier.clone()), "x");

    let Some(Statement::Block(alternative_block)) = &if_exp.alternative else {
        panic!("Alternative statement was None")
    };

    assert_eq!(
        alternative_block.statements.len(),
        1,
        "alternative.Statements does not container 1 statement. got={}",
        alternative_block.statements.len()
    );

    let Statement::Expression(alternative_expression) = &alternative_block.statements[0] else {
        panic!(
            "alternative is not an ExpressionStatement. got={:?}",
            alternative_block.statements[0]
        )
    };

    let Some(Expression::Identifier(alternative_identifier)) =
        alternative_expression.value.as_deref()
    else {
        panic!(
            "alternative expression is not an Identifier. got={:?}",
            alternative_expression.value
        )
    };

    test_identifier(&Expression::Identifier(alternative_identifier.clone()), "y");
}
