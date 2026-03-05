use monke::{ast::Statement, lexer::Lexer, parser::Parser};

#[test]
fn test_let_statements() {
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

    let input = "
    let x = 5;
    let y = 10;
    let foobar = 838383;
    ";

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().expect("Failed to parse program");
    assert_eq!(
        program.statements.len(),
        3,
        "program.Statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    let expecteds = vec!["x", "y", "foobar"];

    for (expected, statement) in expecteds.iter().zip(program.statements.iter()) {
        test_let_statement(statement, expected);
    }
}
