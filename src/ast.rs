use std::ops::Deref;

use crate::tokenize::{Literal, Token, TokenType};

#[derive(Debug)]
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

#[derive(Debug)]
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
        value: f64,
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
}

impl Expr {
    fn str(value: impl Into<String>) -> Expr {
        Expr::EStr {
            value: value.into(),
        }
    }
    fn bool(value: bool) -> Expr {
        Expr::EBool { value }
    }
    fn num(value: f64) -> Expr {
        Expr::ENum { value }
    }
    fn nil() -> Expr {
        Expr::ENil
    }
    fn binary(left: Expr, operator: Operator, right: Expr) -> Expr {
        Expr::EBinary {
            left: left.into(),
            operator,
            right: right.into(),
        }
    }
    fn unary(operator: Operator, right: Expr) -> Expr {
        Expr::EUnary {
            operator,
            right: right.into(),
        }
    }
    fn grouping(expression: Expr) -> Expr {
        Expr::EGrouping {
            expression: expression.into(),
        }
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
        Expr::unary(Operator::OSub, Expr::num(123.0)),
        Operator::OMul,
        Expr::grouping(Expr::num(45.67)),
    );

    println!("{}", fmt_expr(expression));
    println!("Outside the ast file");
}
