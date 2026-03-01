#![allow(unused)]
use std::{
    env,
    io::{Write, stdin, stdout},
    vec,
};

use crate::{
    evaluate::evaluate,
    parser::parse,
    reader::{Source, read_source},
    tokenize::tokenize,
};

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

fn run(source: Source) -> Result<(), IError> {
    let tokens = tokenize(source)?;
    print!("{:#?}", tokens);
    let ast = parse(tokens)?;
    let eval = evaluate(ast)?;
    Ok(())
}

fn run_file(filename: &str) -> Result<(), IError> {
    let source = reader::read_source(filename)?;
    run(source)
}

fn run_prompt() {
    let mut stdout = stdout();
    let mut stdin = stdin();

    'lox: loop {
        stdout.write_all(b">").unwrap();
        stdout.flush().unwrap();
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).expect("Failed to read input");
        let source = reader::Source::new(buffer);

        match run(source) {
            Ok(_) => println!("Hexa"),
            Err(_) => eprintln!("Nah!! Nah!!"),
        }
    }
}

fn main() {
    println!("Hello, Lox");

    let args = env::args().collect::<Vec<String>>();
    println!("Args: {:#?}", args);

    if args.len() == 1 {
        run_prompt();
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => println!("Success!! It worked"),
            Err(_) => println!("Failed!!, I don't know why"),
        }
    } else {
        eprintln!("Usage: lox [filename]")
    }
}
