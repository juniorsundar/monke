use monke::{
    ast::{Expression, Identifier, LetStatement, Program, Statement},
    token::{Token, TokenType},
};

#[test]
fn test_string() {
    let mut program = Program {
        statements: vec![Statement::Let(LetStatement {
            token: Token {
                t_type: TokenType::Let,
                t_literal: "let".to_string(),
            },
            name: Identifier {
                token: Token {
                    t_type: TokenType::Ident,
                    t_literal: "my_var".to_string(),
                },
                value: "my_var".to_string(),
            },
            value: Some(Box::new(Expression::Identifier(Identifier {
                token: Token {
                    t_type: TokenType::Ident,
                    t_literal: "another_var".to_string(),
                },
                value: "another_var".to_string(),
            }))),
        })],
    };

    assert_eq!(program.string(), "let my_var = another_var;".to_string());
}
