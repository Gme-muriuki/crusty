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

pub fn evaluate(_ast: AST) -> Result<Output, EvError> {
    println!("Evaluating....");
    Ok(Output::LNill)
}

pub fn evaluate_expression(expr: &Expr) -> Result<Output, EvError> {
    Ok(match expr {
        Expr::EBinary {
            left,
            operator,
            right,
        } => todo!(),
        Expr::EGrouping { expression } => todo!(),
        Expr::ENum { value } => todo!(),
        Expr::EStr { value } => todo!(),
        Expr::ENil => todo!(),
        Expr::EBool { value } => todo!(),
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
