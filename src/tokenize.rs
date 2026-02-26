use crate::reader::Source;

pub type Tokens = ();

pub fn tokenize(source: Source) -> Result<Tokens, String> {
    println!("Tokenize");
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
