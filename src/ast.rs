use crate::token::Token;

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    If,
}
impl Statement {
    pub fn token_literal(&self) -> String {
        match self {
            Statement::Let(t) => t.token.t_literal.clone(),
            _ => "".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
}
impl Expression {
    pub fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(t) => t.token.t_literal.clone(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<Expression>,
}

pub struct Program {
    pub statements: Vec<Statement>,
}
