#![allow(unused)]
use crate::{evaluate::evaluate, parser::parse, reader::read_source, tokenize::tokenize};

pub mod evaluate;
pub mod parser;
pub mod reader;
pub mod tokenize;

#[derive(Debug)]
pub enum IError {
    Reader(reader::RError),
    Tokenizer(tokenize::TError),
    Parser(parser::PError),
    Evaluator(evaluate::EvError),
}

impl From<reader::RError> for IError {
    fn from(error: reader::RError) -> Self {
        IError::Reader(error)
    }
}

impl From<tokenize::TError> for IError {
    fn from(error: tokenize::TError) -> Self {
        IError::Tokenizer(error)
    }
}

impl From<parser::PError> for IError {
    fn from(error: parser::PError) -> Self {
        IError::Parser(error)
    }
}

impl From<evaluate::EvError> for IError {
    fn from(error: evaluate::EvError) -> Self {
        IError::Evaluator(error)
    }
}

fn run() -> Result<(), IError> {
    let source = read_source("somefile.lox").unwrap();
    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let eval = evaluate(ast).unwrap();
    Ok(())
}

fn main() {
    println!("Hello, Lox");

    match run() {
        Ok(_) => println!("Success!! It worked"),
        Err(_) => println!("Failed!!, I don't know why"),
    }
}
