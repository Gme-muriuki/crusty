use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

pub struct Source {
    pub contents: String,
}

impl Source {
    pub fn new(contents: impl Into<String>) -> Self {
        Self {
            contents: contents.into(),
        }
    }
}

#[derive(Debug)]
pub struct ReadError {
    msg: String,
}

impl From<io::Error> for ReadError {
    fn from(error: io::Error) -> Self {
        ReadError {
            msg: format!("{:?}", error),
        }
    }
}

pub fn read_source(filename: &str) -> Result<Source, ReadError> {
    println!("Reading source");
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => return Err(ReadError::from(e)),
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
