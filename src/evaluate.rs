use crate::ast::{AST, Expr, Operator::*};

pub type Output = LoxValue;

#[derive(Debug)]
pub enum LoxValue {
    LNill,
    LBoolean(bool),
    LNumber(f64),
    LString(String),
}
#[derive(Debug)]
pub struct EvError {}

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
            match (left, operator, right) {
                (LoxValue::LNumber(left), OAdd, LoxValue::LNumber(right)) => {
                    LoxValue::LNumber(left + right)
                }
                (LoxValue::LNumber(left), OSub, LoxValue::LNumber(right)) => {
                    LoxValue::LNumber(left - right)
                }
                (LoxValue::LNumber(left), ODiv, LoxValue::LNumber(right)) => {
                    if right == 0.0 || left == 0.0 {
                        panic!("Cannot divide by 0")
                    }
                    LoxValue::LNumber(left / right)
                }
                (LoxValue::LNumber(left), OMul, LoxValue::LNumber(right)) => {
                    LoxValue::LNumber(left * right)
                }
                (LoxValue::LString(left), OAdd, LoxValue::LString(right)) => {
                    LoxValue::LString(format!("{left} {right}"))
                }
                _ => panic!("Unsupported operation"),
            }
        }
        Expr::EGrouping { expression } => todo!(),
        Expr::ENum { value } => Output::LNumber(value.parse().unwrap()),
        Expr::EStr { value } => Output::LString(value.clone()),
        Expr::ENil => Output::LNill,
        Expr::EBool { value } => Output::LBoolean(*value),
        Expr::EUnary { operator, right } => todo!(),
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
