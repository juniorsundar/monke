pub mod lexer;
pub use self::lexer::Lexer;

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        token::{Token, TokenType},
    };

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let expected_tokens = vec![
            Token {
                t_type: TokenType::Assign,
                t_literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Plus,
                t_literal: "+".to_string(),
            },
            Token {
                t_type: TokenType::Lparen,
                t_literal: "(".to_string(),
            },
            Token {
                t_type: TokenType::Rparen,
                t_literal: ")".to_string(),
            },
            Token {
                t_type: TokenType::Lbrace,
                t_literal: "{".to_string(),
            },
            Token {
                t_type: TokenType::Rbrace,
                t_literal: "}".to_string(),
            },
            Token {
                t_type: TokenType::Comma,
                t_literal: ",".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                t_literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Eof,
                t_literal: "".to_string(),
            },
        ];
        let mut l = Lexer::new(input.to_string());

        for expected in expected_tokens {
            let tok = l.next_token();

            assert_eq!(expected.t_type, tok.t_type);
            assert_eq!(expected.t_literal, tok.t_literal);
        }
    }

    #[test]
    fn test_next_token_extended() {
        let input = "let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        ";

        let expected_tokens = vec![
            Token {
                t_type: TokenType::Let,
                t_literal: "let".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "five".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                t_literal: "=".to_string(),
            },
        ];

        let mut l = Lexer::new(input.to_string());

        for expected in expected_tokens {
            let tok = l.next_token();

            assert_eq!(expected.t_type, tok.t_type);
            assert_eq!(expected.t_literal, tok.t_literal);
        }
    }
}
