#![allow(clippy::single_char_add_str)]
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<Expression>>,
}
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Option<Box<Expression>>,
}
#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub value: Option<Box<Expression>>,
}
#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
}
impl Statement {
    pub fn token_literal(&self) -> String {
        match self {
            Statement::Let(t) => t.token.t_literal.clone(),
            Statement::Return(t) => t.token.t_literal.clone(),
            Statement::Expression(t) => t.token.t_literal.clone(),
            Statement::Block(t) => t.token.t_literal.clone(),
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
            Statement::Block(t) => {
                let mut out = String::new();

                for s in t.statements.iter() {
                    out.push_str(&s.string());
                }
                out
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
impl Identifier {
    pub fn string(&self) -> String {
        self.value.clone()
    }
}
#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}
impl IntegerLiteral {
    pub fn string(&self) -> String {
        self.token.t_literal.clone()
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct Infix {
    pub token: Token,
    pub left: Option<Box<Expression>>,
    pub operator: String,
    pub right: Option<Box<Expression>>,
}
impl Infix {
    pub fn string(&self) -> String {
        let (Some(right), Some(left)) = (self.right.as_deref(), self.left.as_deref()) else {
            return "".to_string();
        };

        let out = format!("({} {} {})", left.string(), self.operator, right.string());
        out
    }
}
#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}
impl BooleanLiteral {
    pub fn string(&self) -> String {
        self.token.t_literal.clone()
    }
}
#[derive(Debug, Clone)]
pub struct If {
    pub token: Token,
    pub condition: Option<Box<Expression>>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}
impl If {
    pub fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("if");
        let Some(cond) = self.condition.as_deref() else {
            return "".to_string();
        };
        out.push_str(&cond.string());
        out.push_str(" ");
        out.push_str(&Statement::Block(self.consequence.clone()).string());
        if let Some(alt) = &self.alternative {
            out.push_str(" else ");
            out.push_str(&Statement::Block(alt.clone()).string());
        }
        out
    }
}
#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>, // Has to be Identifier
    pub body: BlockStatement,        // Has to be Block
}
impl FunctionLiteral {
    pub fn string(&self) -> String {
        let mut out = String::new();

        out.push_str(&self.token.t_literal);
        out.push_str("(");
        let mut params: Vec<String> = Vec::new();
        for param in &self.parameters {
            params.push(param.string());
        }
        let joined_param: String = params.join(",");
        out.push_str(&joined_param);
        out.push_str(")");
        let block_wrapper = Statement::Block(self.body.clone());
        out.push_str(&block_wrapper.string());
        out
    }
}
#[derive(Debug, Clone)]
pub struct Call {
    pub token: Token,
    pub function: Box<Expression>, // Identifer or FunctionLiteral
    pub arguments: Vec<Expression>,
}
impl Call {
    pub fn string(&self) -> String {
        let mut out = String::new();
        let mut args = Vec::<String>::new();
        for arg in self.arguments.iter() {
            args.push(arg.string());
        }

        out.push_str(&self.function.string());
        out.push_str("(");
        out.push_str(&args.join(", "));
        out.push_str(")");

        out
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Prefix(Prefix),
    Infix(Infix),
    BooleanLiteral(BooleanLiteral),
    If(If),
    FunctionLiteral(FunctionLiteral),
    Call(Call),
}
impl Expression {
    pub fn string(&self) -> String {
        match self {
            Expression::Identifier(t) => t.string(),
            Expression::IntegerLiteral(t) => t.string(),
            Expression::Prefix(t) => t.string(),
            Expression::Infix(t) => t.string(),
            Expression::BooleanLiteral(t) => t.string(),
            Expression::If(t) => t.string(),
            Expression::FunctionLiteral(t) => t.string(),
            Expression::Call(t) => t.string(),
        }
    }
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
