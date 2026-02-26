use crate::tokenize::Tokens;

pub type AST = ();

pub fn parse(tokens: Tokens) -> AST {
    println!("Parsing");
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
}
