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
            Token {
                t_type: TokenType::Int,
                t_literal: "5".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                t_literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Let,
                t_literal: "let".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "ten".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                t_literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                t_literal: "10".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                t_literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Let,
                t_literal: "let".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "add".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                t_literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Function,
                t_literal: "fn".to_string(),
            },
            Token {
                t_type: TokenType::Lparen,
                t_literal: "(".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "x".to_string(),
            },
            Token {
                t_type: TokenType::Comma,
                t_literal: ",".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "y".to_string(),
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
                t_type: TokenType::Ident,
                t_literal: "x".to_string(),
            },
            Token {
                t_type: TokenType::Plus,
                t_literal: "+".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "y".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                t_literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Rbrace,
                t_literal: "}".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                t_literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Let,
                t_literal: "let".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "result".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                t_literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "add".to_string(),
            },
            Token {
                t_type: TokenType::Lparen,
                t_literal: "(".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "five".to_string(),
            },
            Token {
                t_type: TokenType::Comma,
                t_literal: ",".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "ten".to_string(),
            },
            Token {
                t_type: TokenType::Rparen,
                t_literal: ")".to_string(),
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
    fn test_next_token_extended_set() {
        let input = "let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }
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
            Token {
                t_type: TokenType::Int,
                t_literal: "5".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                t_literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Let,
                t_literal: "let".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "ten".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                t_literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                t_literal: "10".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                t_literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Let,
                t_literal: "let".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "add".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                t_literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Function,
                t_literal: "fn".to_string(),
            },
            Token {
                t_type: TokenType::Lparen,
                t_literal: "(".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "x".to_string(),
            },
            Token {
                t_type: TokenType::Comma,
                t_literal: ",".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "y".to_string(),
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
                t_type: TokenType::Ident,
                t_literal: "x".to_string(),
            },
            Token {
                t_type: TokenType::Plus,
                t_literal: "+".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "y".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                t_literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Rbrace,
                t_literal: "}".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                t_literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Let,
                t_literal: "let".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "result".to_string(),
            },
            Token {
                t_type: TokenType::Assign,
                t_literal: "=".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "add".to_string(),
            },
            Token {
                t_type: TokenType::Lparen,
                t_literal: "(".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "five".to_string(),
            },
            Token {
                t_type: TokenType::Comma,
                t_literal: ",".to_string(),
            },
            Token {
                t_type: TokenType::Ident,
                t_literal: "ten".to_string(),
            },
            Token {
                t_type: TokenType::Rparen,
                t_literal: ")".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                t_literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Bang,
                t_literal: "!".to_string(),
            },
            Token {
                t_type: TokenType::Minus,
                t_literal: "-".to_string(),
            },
            Token {
                t_type: TokenType::Slash,
                t_literal: "/".to_string(),
            },
            Token {
                t_type: TokenType::Asterisk,
                t_literal: "*".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                t_literal: "5".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                t_literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                t_literal: "5".to_string(),
            },
            Token {
                t_type: TokenType::Lt,
                t_literal: "<".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                t_literal: "10".to_string(),
            },
            Token {
                t_type: TokenType::Gt,
                t_literal: ">".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                t_literal: "5".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                t_literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::If,
                t_literal: "if".to_string(),
            },
            Token {
                t_type: TokenType::Lparen,
                t_literal: "(".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                t_literal: "5".to_string(),
            },
            Token {
                t_type: TokenType::Lt,
                t_literal: "<".to_string(),
            },
            Token {
                t_type: TokenType::Int,
                t_literal: "10".to_string(),
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
                t_type: TokenType::Return,
                t_literal: "return".to_string(),
            },
            Token {
                t_type: TokenType::True,
                t_literal: "true".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                t_literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Rbrace,
                t_literal: "}".to_string(),
            },
            Token {
                t_type: TokenType::Else,
                t_literal: "else".to_string(),
            },
            Token {
                t_type: TokenType::Lbrace,
                t_literal: "{".to_string(),
            },
            Token {
                t_type: TokenType::Return,
                t_literal: "return".to_string(),
            },
            Token {
                t_type: TokenType::False,
                t_literal: "false".to_string(),
            },
            Token {
                t_type: TokenType::Semicolon,
                t_literal: ";".to_string(),
            },
            Token {
                t_type: TokenType::Rbrace,
                t_literal: "}".to_string(),
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
}
