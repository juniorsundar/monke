use crate::token::{Token, TokenType};

#[derive(Default)]
pub struct Lexer {
    pub input: Vec<u8>,
    // current position in input (points to current char)
    position: usize,
    // current reading position in input (after current char)
    read_position: usize,
    // current char under examination
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut new_lexer = Self {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };

        new_lexer.read_char();
        return new_lexer;
    }

    fn read_char(&mut self) {
        if self.read_position as usize >= self.input.len() {
            self.ch = 0; // NUL byte represents EOF in ASCII
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token = match self.ch {
            b'=' => Token::new(TokenType::Assign, self.ch),
            b';' => Token::new(TokenType::Semicolon, self.ch),
            b'(' => Token::new(TokenType::Lparen, self.ch),
            b')' => Token::new(TokenType::Rparen, self.ch),
            b',' => Token::new(TokenType::Comma, self.ch),
            b'+' => Token::new(TokenType::Plus, self.ch),
            b'{' => Token::new(TokenType::Lbrace, self.ch),
            b'}' => Token::new(TokenType::Rbrace, self.ch),
            0 => Token {
                t_type: TokenType::Eof,
                t_literal: "".to_string(),
            },
            _ => panic!("Illegal!"),
        };

        self.read_char();
        tok
    }
}
