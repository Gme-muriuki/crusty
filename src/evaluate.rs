use crate::ast::AST;

pub struct Output {}

#[derive(Debug)]
pub struct EvError {}

pub fn evaluate(ast: AST) -> Result<Output, EvError> {
    println!("Evaluating");
    Ok(Output {})
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
}
