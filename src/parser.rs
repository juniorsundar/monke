use crate::{
    ast::{Expression, Identifier, LetStatement, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};
use std::fmt;

#[derive(Debug)]
pub enum ParserError {
    ParseProgramError,
    ParseStatementError,
    ParseLetStatementError,
    IncorrectNextToken,
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::ParseProgramError => write!(f, "Parser failed to parse program!"),
            ParserError::ParseStatementError => write!(f, "Parser failed to parse statement!"),
            ParserError::ParseLetStatementError => {
                write!(f, "Parser failed to parse 'let' statement!")
            }
            ParserError::IncorrectNextToken => write!(f, "Incorrect next token!"),
        }
    }
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Token::default(),
            peek_token: Token::default(),
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut program = Program {
            statements: Vec::<Statement>::new(),
        };

        while self.current_token.t_type != TokenType::Eof {
            let statement_result = self.parse_statement();
            if let Ok(statement) = statement_result {
                program.statements.push(statement);
            } else {
                return Err(ParserError::ParseProgramError);
            }
            self.next_token();
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token.t_type {
            TokenType::Let => self.parse_let_statement(),
            _ => Err(ParserError::ParseStatementError),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        let statement_token = self.current_token.clone();
        if !self.expect_peek(TokenType::Ident) {
            return Err(ParserError::IncorrectNextToken);
        }

        let statement_name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.t_literal.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return Err(ParserError::IncorrectNextToken);
        }

        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Ok(Statement::Let(LetStatement {
            token: statement_token,
            name: statement_name,
            value: Box::new(Expression::Identifier(Identifier {
                token: Token::default(),
                value: "".to_string(),
            })),
        }))
    }

    fn expect_peek(&mut self, expected_token: TokenType) -> bool {
        if self.peek_token_is(expected_token) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn current_token_is(&self, wanted_token: TokenType) -> bool {
        self.current_token.t_type == wanted_token
    }

    fn peek_token_is(&self, wanted_token: TokenType) -> bool {
        self.peek_token.t_type == wanted_token
    }
}
