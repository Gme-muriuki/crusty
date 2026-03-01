use std::ops::Deref;

use crate::tokenize::{Literal, Token, TokenType};

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Num {
        value: f64,
    },
    Str {
        value: String,
    },
    Nil,
    Bool {
        value: bool,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    fn str(value: impl Into<String>) -> Expr {
        Expr::Str {
            value: value.into(),
        }
    }
    fn bool(value: bool) -> Expr {
        Expr::Bool { value }
    }
    fn num(value: f64) -> Expr {
        Expr::Num { value }
    }
    fn nil() -> Expr {
        Expr::Nil
    }
    fn binary(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary {
            left: left.into(),
            operator,
            right: right.into(),
        }
    }
    fn unary(operator: Token, right: Expr) -> Expr {
        Expr::Unary {
            operator,
            right: right.into(),
        }
    }
    fn grouping(expression: Expr) -> Expr {
        Expr::Grouping {
            expression: expression.into(),
        }
    }
}

pub fn fmt_expr(e: Expr) -> String {
    match e {
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            format!(
                "({} {} {})",
                operator.lexeme,
                fmt_expr(*left),
                fmt_expr(*right)
            )
        }
        Expr::Grouping { expression } => {
            format!("(group {})", fmt_expr(*expression))
        }
        Expr::Unary { operator, right } => {
            format!("({} {})", operator.lexeme, fmt_expr(*right))
        }
        Expr::Num { value } => {
            format!("{}", value)
        }
        Expr::Str { value } => {
            format!("{:?}", value)
        }
        Expr::Nil => "nil".to_string(),
        Expr::Bool { value } => {
            format!("{:?}", value)
        }
    }
}

pub fn main() {
    let expression = Expr::binary(
        Expr::unary(
            Token::new(TokenType::Minus, "-", 1, Literal::None),
            Expr::num(123.0),
        ),
        Token::new(TokenType::Star, "*", 1, Literal::None),
        Expr::grouping(Expr::num(45.67)),
    );

    println!("{}", fmt_expr(expression));
}
