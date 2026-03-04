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
        new_lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0; // NUL byte represents EOF in ASCII
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter(self.ch) {
            self.read_char();
        }

        let mut return_string = String::new();
        for i in position..self.position {
            return_string.push(self.input[i] as char);
        }
        return_string
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.is_digit(self.ch) {
            self.read_char();
        }

        let mut return_string = String::new();
        for i in position..self.position {
            return_string.push(self.input[i] as char);
        }
        return_string
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok: Token = match self.ch {
            b'=' => {
                let next_char = self.peek_char();
                if next_char == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = (ch as char).to_string() + &(self.ch as char).to_string();
                    Token {
                        t_type: TokenType::Eq,
                        t_literal: literal,
                    }
                } else {
                    Token::new(TokenType::Assign, self.ch)
                }
            }
            b';' => Token::new(TokenType::Semicolon, self.ch),
            b'(' => Token::new(TokenType::Lparen, self.ch),
            b')' => Token::new(TokenType::Rparen, self.ch),
            b',' => Token::new(TokenType::Comma, self.ch),
            b'+' => Token::new(TokenType::Plus, self.ch),
            b'-' => Token::new(TokenType::Minus, self.ch),
            b'!' => {
                let next_char = self.peek_char();
                if next_char == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = (ch as char).to_string() + &(self.ch as char).to_string();
                    Token {
                        t_type: TokenType::Neq,
                        t_literal: literal,
                    }
                } else {
                    Token::new(TokenType::Bang, self.ch)
                }
            }
            b'*' => Token::new(TokenType::Asterisk, self.ch),
            b'/' => Token::new(TokenType::Slash, self.ch),
            b'<' => Token::new(TokenType::Lt, self.ch),
            b'>' => Token::new(TokenType::Gt, self.ch),
            b'{' => Token::new(TokenType::Lbrace, self.ch),
            b'}' => Token::new(TokenType::Rbrace, self.ch),
            0 => Token {
                t_type: TokenType::Eof,
                t_literal: "".to_string(),
            },
            _ => {
                if self.is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return Token {
                        t_type: Token::lookup_identifier(&literal),
                        t_literal: literal,
                    };
                } else if self.is_digit(self.ch) {
                    let literal = self.read_number();
                    return Token {
                        t_type: TokenType::Int,
                        t_literal: literal,
                    };
                } else {
                    return Token::new(TokenType::Illegal, self.ch);
                }
            }
        };

        self.read_char();
        tok
    }

    fn is_letter(&self, ch: u8) -> bool {
        // (b'a'..=b'z').contains(&ch) || (b'A'..=b'Z').contains(&ch) || ch == b'_'
        ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == b'_'
    }

    fn is_digit(&self, ch: u8) -> bool {
        // (b'0'..=b'9').contains(&ch)
        ch.is_ascii_digit()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }
}
