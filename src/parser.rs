use crate::{
    ast::{Expression, Identifier, LetStatement, Program, ReturnStatement, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};
use std::fmt;

#[derive(Debug)]
pub enum ParserError {
    IncorrectNextToken(TokenType, TokenType), // Expected vs Received
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::IncorrectNextToken(ex, recv) => write!(
                f,
                "Incorrect next token! Expected: {:?}, Received: {:?}",
                ex, recv
            ),
        }
    }
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    pub errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Token::default(),
            peek_token: Token::default(),
            errors: Vec::<ParserError>::new(),
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::<Statement>::new(),
        };

        while self.current_token.t_type != TokenType::Eof {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.t_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let statement_token = self.current_token.clone();
        if !self.expect_peek(&TokenType::Ident) {
            return None;
        }

        let statement_name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.t_literal.clone(),
        };

        if !self.expect_peek(&TokenType::Assign) {
            return None;
        }

        while !self.current_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let(LetStatement {
            token: statement_token,
            name: statement_name,
            value: Box::new(Expression::Identifier(Identifier {
                token: Token::default(),
                value: "".to_string(),
            })),
        }))
    }

    fn expect_peek(&mut self, expected_token: &TokenType) -> bool {
        if self.peek_token_is(expected_token) {
            self.next_token();
            true
        } else {
            self.peek_error(expected_token.clone());
            false
        }
    }

    fn current_token_is(&self, wanted_token: &TokenType) -> bool {
        self.current_token.t_type == wanted_token.clone()
    }

    fn peek_token_is(&self, wanted_token: &TokenType) -> bool {
        self.peek_token.t_type == wanted_token.clone()
    }

    fn peek_error(&mut self, token: TokenType) {
        self.errors.push(ParserError::IncorrectNextToken(
            token,
            self.peek_token.t_type.clone(),
        ));
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let statement_token = self.current_token.clone();
        let statement_value = Box::new(Expression::Identifier(Identifier {
            token: Token::default(),
            value: "".to_string(),
        }));
        let statement = Some(Statement::Return(ReturnStatement {
            token: statement_token,
            value: statement_value,
        }));

        self.next_token();
        while !self.current_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        statement
    }
}
