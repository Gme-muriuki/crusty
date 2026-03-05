use std::ops;

use crate::ast::{
    AST, Expr,
    Operator::{self, *},
};

pub type Output = LoxValue;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum LoxValue {
    LNill,
    LBoolean(bool),
    LNumber(f64),
    LString(String),
}

#[derive(Debug)]
pub enum EvError {
    ZeroDivision,
    UnsupportedBinOps(LoxValue, Operator, LoxValue),
    UnsupportedUnaryOps(Operator, LoxValue),
}

impl LoxValue {
    pub fn is_truthy(&self) -> bool {
        match &self {
            LoxValue::LNill | LoxValue::LBoolean(false) => false,
            _ => true,
        }
    }
}

pub fn evaluate(ast: AST) -> Result<Output, EvError> {
    println!("Evaluating....");
    evaluate_expression(&ast.top)
}

pub fn evaluate_expression(expr: &Expr) -> Result<Output, EvError> {
    Ok(match expr {
        Expr::EBinary {
            left,
            operator,
            right,
        } => {
            let left = evaluate_expression(&left)?;
            let right = evaluate_expression(&right)?;

            match (&left, operator, &right) {
                // Numeric ops
                (LoxValue::LNumber(left), OAdd, LoxValue::LNumber(right)) => {
                    LoxValue::LNumber(left + right)
                }
                (LoxValue::LNumber(left), OSub, LoxValue::LNumber(right)) => {
                    LoxValue::LNumber(left - right)
                }
                (LoxValue::LNumber(left), ODiv, LoxValue::LNumber(right)) => {
                    if *right == 0.0 {
                        return Err(EvError::ZeroDivision);
                    }
                    LoxValue::LNumber(left / right)
                }
                (LoxValue::LNumber(left), OMul, LoxValue::LNumber(right)) => {
                    LoxValue::LNumber(left * right)
                }
                // String ops
                (LoxValue::LString(left), OAdd, LoxValue::LString(right)) => {
                    LoxValue::LString(format!("{left} {right}"))
                }
                // Equality works with any combination of values
                (left, OLt, right) => LoxValue::LBoolean(left < right),
                (left, OLeq, right) => LoxValue::LBoolean(left <= right),
                (left, OGt, right) => LoxValue::LBoolean(left > right),
                (left, OGeq, right) => LoxValue::LBoolean(left >= right),
                (left, OEq, right) => LoxValue::LBoolean(left == right),
                (left, ONeq, right) => LoxValue::LBoolean(left != right),
                _ => return Err(EvError::UnsupportedBinOps(left, operator.clone(), right)),
            }
        }
        Expr::EGrouping { expression } => evaluate_expression(&expression)?,
        // My AST contains numbers as strings.
        Expr::ENum { value } => Output::LNumber(value.parse().unwrap()),
        Expr::EStr { value } => Output::LString(value.clone()),
        Expr::ENil => Output::LNill,
        Expr::EBool { value } => Output::LBoolean(*value),
        Expr::EUnary { operator, right } => {
            let rv = evaluate_expression(&right)?;
            match (operator, &rv) {
                (OSub, LoxValue::LNumber(right)) => LoxValue::LNumber(-right),
                (ONot, right) => LoxValue::LBoolean(!right.is_truthy()),
                _ => return Err(EvError::UnsupportedUnaryOps(operator.clone(), rv)),
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
}
