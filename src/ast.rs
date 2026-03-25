#![allow(clippy::single_char_add_str)]
use crate::token::Token;

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    If,
}
impl Statement {
    pub fn token_literal(&self) -> String {
        match self {
            Statement::Let(t) => t.token.t_literal.clone(),
            Statement::Return(t) => t.token.t_literal.clone(),
            Statement::Expression(t) => t.token.t_literal.clone(),
            _ => "".to_string(),
        }
    }

    pub fn string(&self) -> String {
        match self {
            Statement::Let(t) => {
                let mut out = String::new();
                out.push_str(&self.token_literal());
                out.push_str(" ");
                out.push_str(&t.name.string());
                out.push_str(" = ");

                if let Some(value) = &t.value {
                    out.push_str(&value.string());
                }
                out.push_str(";");
                out
            }
            Statement::Return(t) => {
                let mut out = String::new();
                out.push_str(&self.token_literal());
                out.push_str(" ");

                if let Some(value) = &t.value {
                    out.push_str(&value.string());
                }
                out.push_str(";");
                out
            }
            Statement::Expression(t) => {
                if let Some(value) = &t.value {
                    return value.string();
                }
                "".to_string()
            }
            _ => "".to_string(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
impl Identifier {
    pub fn string(&self) -> String {
        self.value.clone()
    }
}
#[derive(Debug, Default)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}
impl IntegerLiteral {
    pub fn string(&self) -> String {
        self.token.t_literal.clone()
    }
}
#[derive(Debug, Default)]
pub struct Prefix {
    pub token: Token,
    pub operator: String,
    pub right: Option<Box<Expression>>,
}
impl Prefix {
    pub fn string(&self) -> String {
        let Some(right) = self.right.as_deref() else {
            return "".to_string();
        };
        let out = format!("({}{})", self.operator, right.string());
        out
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Prefix(Prefix),
}
impl Expression {
    // pub fn token_literal(&self) -> String {
    //     match self {
    //         Expression::Identifier(t) => t.token.t_literal.clone(),
    //         Expression::IntegerLiteral(t) => t.token.t_literal.clone(),
    //         Expression::Prefix(t) => todo!(),
    //     }
    // }

    pub fn string(&self) -> String {
        match self {
            Expression::Identifier(t) => t.string(),
            Expression::IntegerLiteral(t) => t.string(),
            Expression::Prefix(t) => t.string(),
        }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<Expression>>,
}
#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Option<Box<Expression>>,
}
#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub value: Option<Box<Expression>>,
}

pub struct Program {
    pub statements: Vec<Statement>,
}
impl Program {
    pub fn string(&mut self) -> String {
        let mut out = String::new();

        for statement in self.statements.iter() {
            out.push_str(&statement.string());
        }

        out
    }
}
