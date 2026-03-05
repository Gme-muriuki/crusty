use crate::ast::{AST, Expr};

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
        } => todo!(),
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
