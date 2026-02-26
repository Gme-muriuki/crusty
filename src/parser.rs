use crate::tokenize::Tokens;

pub type AST = ();
pub type PError = ();

pub fn parse(tokens: Tokens) -> Result<AST, PError> {
    println!("Parsing");
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
