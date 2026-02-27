use crate::reader::Source;

pub struct Tokens {}

#[derive(Debug)]
pub struct TError {}

pub fn tokenize(source: Source) -> Result<Tokens, TError> {
    println!("Tokenize");

    Ok(Tokens {})
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
}
