use std::{
    fs::File,
    io::{Error, Read},
    path::PathBuf,
};

pub struct Source {}

#[derive(Debug)]
pub struct RError {}

pub fn read_source(filename: &str) -> Result<Source, RError> {
    println!("Reading source");

    Ok(Source {})
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
}
