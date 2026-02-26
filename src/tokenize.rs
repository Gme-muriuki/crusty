use crate::reader::Source;

pub type Tokens = ();
pub type TError = ();

pub fn tokenize(source: Source) -> Result<Tokens, TError> {
    println!("Tokenize");
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
