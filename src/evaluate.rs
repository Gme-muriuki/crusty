use crate::parser::AST;

pub type Output = ();
pub type EvError = ();

pub fn evaluate(ast: AST) -> Result<Output, EvError> {
    println!("Evaluating");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
}
