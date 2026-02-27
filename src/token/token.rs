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
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
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
    True,
    False,
    If,
    Else,
    Return
}

#[derive(Debug, Clone)]
pub struct Token {
    pub t_type: TokenType,
    pub t_literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: u8) -> Self {
        Self {
            t_type: token_type,
            t_literal: (ch as char).to_string(),
        }
    }

    pub fn lookup_identifier(word: &str) -> TokenType {
        match word {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            _ => TokenType::Ident,
        }
    }
}
