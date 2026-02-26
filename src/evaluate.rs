use crate::parser::AST;

pub type Output = ();

pub fn evaluate(ast: AST) -> Output {
    println!("Evaluating");
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
