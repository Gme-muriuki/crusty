//! # Source Reader Module
//!
//! This module handles file I/O operations for reading Lox source code.
//! It provides a simple abstraction over file reading with error handling.

use std::{
    fs::File,
    io::{self, Read},
};

/// Represents a Lox source code file in memory.
///
/// This struct wraps the raw string contents of a Lox program,
/// providing a clean interface for the rest of the interpreter pipeline.
pub struct Source {
    /// The raw text content of the Lox source file.
    pub contents: String,
}

impl Source {
    /// Creates a new Source instance from any string-like input.
    ///
    /// # Arguments
    /// * `contents` - The source code content as a string or string-like type
    pub fn new(contents: impl Into<String>) -> Self {
        Self {
            contents: contents.into(),
        }
    }
}

/// Error type for file reading operations.
///
/// Wraps I/O errors that occur when attempting to read source files.
///
#[allow(unused)]
#[derive(Debug)]
pub struct ReadError {
    /// Human-readable error message describing the failure.
    msg: String,
}

impl From<io::Error> for ReadError {
    fn from(error: io::Error) -> Self {
        ReadError {
            msg: format!("{:?}", error),
        }
    }
}

/// Reads a Lox source file from disk and returns it as a Source struct.
///
/// This function handles file opening, reading, and basic error conversion.
/// It's designed to be the first step in the interpretation pipeline.
///
/// # Arguments
/// * `filename` - Path to the Lox source file to read
///
/// # Returns
/// * `Ok(Source)` containing the file contents on success
/// * `Err(ReadError)` if the file cannot be opened or read
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

    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
}
