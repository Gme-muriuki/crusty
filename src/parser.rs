use crate::tokenize::Tokens;

pub struct AST {}

#[derive(Debug)]
pub struct PError {}

pub fn parse(tokens: Tokens) -> Result<AST, PError> {
    println!("Parsing");
    Ok(AST {})
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
}
