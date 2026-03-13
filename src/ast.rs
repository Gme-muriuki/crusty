use std::{fmt::Display, ops::Deref};

use crate::tokenize::{Literal, Token, TokenType};

#[derive(Debug, PartialEq)]
pub struct AST {
    pub top: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    OAdd,
    OSub,
    OMul,
    ODiv,
    OLt,
    OLeq,
    OGt,
    OGeq,
    OEq,
    ONeq,
    ONot,
    OAnd,
    OOr,
}

impl Display for Operator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Operator::OAdd => "+",
            Operator::OSub => "-",
            Operator::OMul => "*",
            Operator::ODiv => "/",
            Operator::OLt => "<",
            Operator::OLeq => "<=",
            Operator::OGt => ">",
            Operator::OGeq => ">=",
            Operator::OEq => "=",
            Operator::ONeq => "!=",
            Operator::ONot => "!",
            Operator::OAnd => "and",
            Operator::OOr => "or",
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    EBinary {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    EGrouping {
        expression: Box<Expr>,
    },
    ENum {
        value: String,
    },
    EStr {
        value: String,
    },
    ENil,
    EBool {
        value: bool,
    },
    EUnary {
        operator: Operator,
        right: Box<Expr>,
    },
    EVarDecl {
        name: String,
    },
}

impl Expr {
    pub fn str(value: impl Into<String>) -> Expr {
        Expr::EStr {
            value: value.into(),
        }
    }
    pub fn bool(value: bool) -> Expr {
        Expr::EBool { value }
    }
    pub fn num(value: impl Into<String>) -> Expr {
        Expr::ENum {
            value: value.into(),
        }
    }
    pub fn nil() -> Expr {
        Expr::ENil
    }
    pub fn binary(left: Expr, operator: Operator, right: Expr) -> Expr {
        Expr::EBinary {
            left: left.into(),
            operator,
            right: right.into(),
        }
    }
    pub fn unary(operator: Operator, right: Expr) -> Expr {
        Expr::EUnary {
            operator,
            right: right.into(),
        }
    }
    pub fn grouping(expression: Expr) -> Expr {
        Expr::EGrouping {
            expression: expression.into(),
        }
    }
    pub fn variable(name: impl Into<String>) -> Expr {
        Expr::EVarDecl { name: name.into() }
    }
}

pub fn fmt_expr(e: Expr) -> String {
    match e {
        Expr::EBinary {
            left,
            operator,
            right,
        } => {
            format!(
                "({} {} {})",
                fmt_ops(&operator),
                fmt_expr(*left),
                fmt_expr(*right)
            )
        }
        Expr::EGrouping { expression } => {
            format!("(group {})", fmt_expr(*expression))
        }
        Expr::EUnary { operator, right } => {
            format!("({} {})", fmt_ops(&operator), fmt_expr(*right))
        }
        Expr::ENum { value } => {
            format!("{}", value)
        }
        Expr::EStr { value } => {
            format!("{:?}", value)
        }
        Expr::ENil => "nil".to_string(),
        Expr::EBool { value } => {
            format!("{:?}", value)
        }
        Expr::EVarDecl { name } => format!("{name}"),
    }
}

// Statements
#[derive(Debug, PartialEq)]
pub enum Stmt {
    SPrint {
        expression: Expr,
    },
    SExpression {
        expression: Expr,
    },
    SVar {
        name: String,
        initializer: Option<Expr>,
    },
}

// constructors for statements
impl Stmt {
    pub fn print(expression: Expr) -> Stmt {
        Stmt::SPrint { expression }
    }

    pub fn expression(expression: Expr) -> Stmt {
        Stmt::SExpression { expression }
    }

    pub fn var(name: impl Into<String>, initializer: Option<Expr>) -> Stmt {
        Stmt::SVar {
            name: name.into(),
            initializer,
        }
    }
}

fn fmt_ops(operation: &Operator) -> &'static str {
    match operation {
        Operator::OAdd => "+",
        Operator::OSub => "-",
        Operator::OMul => "*",
        Operator::ODiv => "/",
        Operator::OLt => "<",
        Operator::OLeq => "<=",
        Operator::OGt => ">",
        Operator::OGeq => ">=",
        Operator::OEq => "==",
        Operator::ONeq => "!=",
        Operator::ONot => "!",
        Operator::OAnd => "and",
        Operator::OOr => "or",
    }
}

pub fn main() {
    println!("Inside the ast file");
    let expression = Expr::binary(
        Expr::unary(Operator::OSub, Expr::num("123.0")),
        Operator::OMul,
        Expr::grouping(Expr::num("45.67")),
    );

    println!("{}", fmt_expr(expression));
    println!("Outside the ast file");
}
