use std::{fs::File, io::Read, path::PathBuf};

pub type Source = String;

pub fn read_source(filename: &str) -> Result<Source, String> {
    let mut file = File::open(filename).unwrap();
    let mut buffer = String::new();
    _ = file.read_to_string(&mut buffer).unwrap();

    Ok(buffer)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
}
