#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    Eof,
    // Idenifiers + Literals
    Ident,
    Int,
    // Operators
    Assign,
    Plus,
    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    // Keywords
    Function,
    Let,
}

pub struct Token {
    token_type: TokenType,
    token_literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, token_literal: String) -> Self {
        Self {
            token_type,
            token_literal,
        }
    }
}
