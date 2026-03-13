#![allow(unused)]
use std::{
    env,
    io::{Write, stdin, stdout},
    vec,
};

use crate::{
    evaluate::{Interpreter, evaluate},
    parser::parse,
    reader::{Source, read_source},
    tokenize::tokenize,
};

pub mod ast;
pub mod environ;
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

fn report_error(error: IError) {
    match error {
        IError::Reader(rerror) => {
            eprintln!("Failed to read file: {:#?}", rerror);
        }
        IError::Tokenizer(terror) => {
            eprintln!("Failed to tokenize: {:#?}", terror);
        }
        IError::Parser(perror) => match perror {
            parser::PError::SyntaxError { line, msg } => {
                eprintln!("Line: {line} Unexpected character {msg}")
            }
            parser::PError::UnterminatedCharacter { line } => {
                eprintln!("Line: {line}: Unterminated string");
            }
        },
        IError::Evaluator(ev_error) => match ev_error {
            evaluate::EvError::ZeroDivision => eprintln!("Division by zero"),
            evaluate::EvError::UnsupportedBinOps(left, operator, right) => {
                eprintln!("Unsupported operation: {left:?} {operator:?} {right:?}")
            }
            evaluate::EvError::UnsupportedUnaryOps(operator, left) => {
                eprintln!("Unsupported operation: {operator:?} {left:?}")
            }
        },
    }
}

fn run_interpreter(source: Source, interpreter: &mut Interpreter) -> Result<(), IError> {
    let tokens = tokenize(source)?;
    let ast = parse(tokens)?;
    interpreter.evaluate(ast)?;
    Ok(())
}
fn run(source: Source) -> Result<(), IError> {
    let mut interp = Interpreter::new();
    run_interpreter(source, &mut interp)
}

fn run_file(filename: &str) -> Result<(), IError> {
    let source = reader::read_source(filename)?;
    run(source)
}

fn run_prompt() {
    let mut stdout = stdout();
    let mut stdin = stdin();
    let mut interpreter = Interpreter::new();
    'lox: loop {
        stdout.write_all(b">").unwrap();
        stdout.flush().unwrap();
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).expect("Failed to read input");
        let source = reader::Source::new(buffer);

        match run_interpreter(source, &mut interpreter) {
            Ok(_) => println!("Hexa"),
            Err(e) => eprintln!("{:#?}", e),
        }
    }
}

fn main() {
    println!("Hello, Lox");

    ast::main();

    let args = env::args().collect::<Vec<String>>();
    println!("Args: {:#?}", args);

    if args.len() == 1 {
        run_prompt();
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => println!("Success!! It worked"),
            Err(err) => report_error(err),
        }
    } else {
        eprintln!("Usage: lox [filename]")
    }
}
