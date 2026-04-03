use crate::{
    ast::{
        BooleanLiteral, Expression, ExpressionStatement, Identifier, Infix, IntegerLiteral,
        LetStatement, Prefix, Program, ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};
use std::fmt;

#[derive(Debug)]
pub enum ParserError {
    IncorrectNextToken(TokenType, TokenType), // Expected vs Received
    IntegerParse(String),                     // Literal
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::IncorrectNextToken(ex, recv) => write!(
                f,
                "Incorrect next token! Expected: {:?}, Received: {:?}",
                ex, recv
            ),
            ParserError::IntegerParse(recv) => write!(f, "Failed to parse {:?} as integer!", recv),
        }
    }
}

// #[derive(Debug, Clone, Copy)]
// enum PrefixParser {
//     Identifier,
//     // Integer,
//     // Boolean,
//     // PrefixOperator, // for !x, -x
//     // GroupedExpression,
//     // IfExpression,
//     // FunctionLiteral,
// }

// #[derive(Clone, Copy)]
// enum InfixParser {
//     InfixOperator, // +, -, *, /
//     Call,
//     Index, // array[index]
// }

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
enum Precedence {
    Lowest,      // Starting point
    Equals,      // == !=
    LessGreater, // < > <= >=
    Sum,         // + -
    Product,     // * /
    Prefix,      // -X !X
                 // Call,        // fn()
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

    fn current_precedence(&self) -> Precedence {
        match self.current_token.t_type {
            TokenType::Eq => Precedence::Equals,
            TokenType::Neq => Precedence::Equals,
            TokenType::Lt => Precedence::LessGreater,
            TokenType::Gt => Precedence::LessGreater,
            TokenType::Plus => Precedence::Sum,
            TokenType::Minus => Precedence::Sum,
            TokenType::Slash => Precedence::Product,
            TokenType::Asterisk => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    fn peek_token_is(&self, wanted_token: &TokenType) -> bool {
        self.peek_token.t_type == wanted_token.clone()
    }

    fn peek_precedence(&self) -> Precedence {
        match self.peek_token.t_type {
            TokenType::Eq => Precedence::Equals,
            TokenType::Neq => Precedence::Equals,
            TokenType::Lt => Precedence::LessGreater,
            TokenType::Gt => Precedence::LessGreater,
            TokenType::Plus => Precedence::Sum,
            TokenType::Minus => Precedence::Sum,
            TokenType::Slash => Precedence::Product,
            TokenType::Asterisk => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    fn peek_error(&mut self, token: TokenType) {
        self.errors.push(ParserError::IncorrectNextToken(
            token,
            self.peek_token.t_type.clone(),
        ));
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
            _ => self.parse_expression_statement(),
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
            value: Some(Box::new(Expression::Identifier(Identifier {
                token: Token::default(),
                value: "".to_string(),
            }))),
        }))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let statement_token = self.current_token.clone();
        let statement_value = Box::new(Expression::Identifier(Identifier {
            token: Token::default(),
            value: "".to_string(),
        }));
        let statement = Some(Statement::Return(ReturnStatement {
            token: statement_token,
            value: Some(statement_value),
        }));

        self.next_token();
        while !self.current_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        statement
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let statement_token = self.current_token.clone();
        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Expression(ExpressionStatement {
            token: statement_token,
            value: Some(Box::new(expression)),
        }))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left_expression = match self.current_token.t_type {
            TokenType::Ident => self.parse_identifier_expression()?,
            TokenType::Int => self.parse_integer_literal_expression()?,
            TokenType::Bang => self.parse_prefix_expression()?,
            TokenType::Minus => self.parse_prefix_expression()?,
            TokenType::True => self.parse_boolean_literal_expression()?,
            TokenType::False => self.parse_boolean_literal_expression()?,
            TokenType::Lparen => self.parse_grouped_expression()?,
            _ => return None,
        };

        while !self.peek_token_is(&TokenType::Semicolon) && precedence < self.peek_precedence() {
            self.next_token();
            left_expression = match self.current_token.t_type {
                TokenType::Plus => self.parse_infix_expression(left_expression)?,
                TokenType::Minus => self.parse_infix_expression(left_expression)?,
                TokenType::Slash => self.parse_infix_expression(left_expression)?,
                TokenType::Asterisk => self.parse_infix_expression(left_expression)?,
                TokenType::Eq => self.parse_infix_expression(left_expression)?,
                TokenType::Neq => self.parse_infix_expression(left_expression)?,
                TokenType::Lt => self.parse_infix_expression(left_expression)?,
                TokenType::Gt => self.parse_infix_expression(left_expression)?,
                _ => return Some(left_expression),
            };
        }

        Some(left_expression)
    }

    fn parse_identifier_expression(&mut self) -> Option<Expression> {
        Some(Expression::Identifier(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.t_literal.clone(),
        }))
    }

    fn parse_integer_literal_expression(&mut self) -> Option<Expression> {
        let integer_literal_token = self.current_token.clone();

        let integer_literal_value = if let Ok(t) = self.current_token.t_literal.parse::<i64>() {
            t
        } else {
            self.errors.push(ParserError::IntegerParse(
                self.current_token.t_literal.to_string(),
            ));
            return None;
        };

        Some(Expression::IntegerLiteral(IntegerLiteral {
            token: integer_literal_token,
            value: integer_literal_value,
        }))
    }

    fn parse_boolean_literal_expression(&mut self) -> Option<Expression> {
        let boolean_literal_token = self.current_token.clone();
        let boolean_literal_value = self.current_token_is(&TokenType::True);

        Some(Expression::BooleanLiteral(BooleanLiteral {
            token: boolean_literal_token,
            value: boolean_literal_value,
        }))
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let exp_token = self.current_token.clone();
        let exp_operator = self.current_token.t_literal.clone();

        self.next_token();

        let exp_right = self.parse_expression(Precedence::Prefix);
        let exp = Expression::Prefix(Prefix {
            token: exp_token,
            operator: exp_operator,
            right: exp_right.map(Box::new),
        });
        Some(exp)
    }

    fn parse_infix_expression(&mut self, left_expression: Expression) -> Option<Expression> {
        let exp_token = self.current_token.clone();
        let exp_operator = self.current_token.t_literal.clone();
        let left = Some(left_expression.clone());

        let precedence = self.current_precedence();
        self.next_token();
        let right = self.parse_expression(precedence);
        let exp = Expression::Infix(Infix {
            token: exp_token,
            operator: exp_operator,
            left: left.map(Box::new),
            right: right.map(Box::new),
        });
        Some(exp)
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let exp = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(&TokenType::Rparen) {
            return None;
        }

        Some(exp)
    }
}
