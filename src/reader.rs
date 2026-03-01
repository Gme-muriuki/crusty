use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

pub struct Source {
    pub contents: String,
}

impl Source {
    pub fn new(contents: String) -> Self {
        Self { contents }
    }
}

#[derive(Debug)]
pub struct RError {
    msg: String,
}

impl From<io::Error> for RError {
    fn from(error: io::Error) -> Self {
        RError {
            msg: format!("{:?}", error),
        }
    }
}

pub fn read_source(filename: &str) -> Result<Source, RError> {
    println!("Reading source");
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => return Err(RError::from(e)),
    };

    let mut contents = String::new();
    _ = file.read_to_string(&mut contents);

    Ok(Source { contents })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
}
