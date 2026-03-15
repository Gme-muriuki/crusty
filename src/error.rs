#![allow(unused)]
use std::{
    env,
    io::{Write, stdin, stdout},
    vec,
};

use crate::{
    evaluate::{self, Interpreter, evaluate},
    parser::{self, parse},
    reader::{self, Source, read_source},
    tokenize::{self, tokenize},
};

#[derive(Debug)]
pub enum InterpreterError {
    Reader(reader::ReadError),
    Tokenizer(tokenize::TokenizeError),
    Parser(parser::ParseError),
    Evaluator(evaluate::EvError),
}

impl From<reader::ReadError> for InterpreterError {
    fn from(error: reader::ReadError) -> Self {
        InterpreterError::Reader(error)
    }
}

impl From<tokenize::TokenizeError> for InterpreterError {
    fn from(error: tokenize::TokenizeError) -> Self {
        InterpreterError::Tokenizer(error)
    }
}

impl From<parser::ParseError> for InterpreterError {
    fn from(error: parser::ParseError) -> Self {
        InterpreterError::Parser(error)
    }
}

impl From<evaluate::EvError> for InterpreterError {
    fn from(error: evaluate::EvError) -> Self {
        InterpreterError::Evaluator(error)
    }
}

pub fn report_error(error: InterpreterError) {
    match error {
        InterpreterError::Reader(rerror) => {
            eprintln!("Failed to read file: {:#?}", rerror);
        }
        InterpreterError::Tokenizer(terror) => {
            eprintln!("Failed to tokenize: {:#?}", terror);
        }
        InterpreterError::Parser(perror) => match perror {
            parser::ParseError::SyntaxError { line, msg } => {
                eprintln!("Line: {line} Unexpected character {msg}")
            }
            parser::ParseError::UnterminatedCharacter { line } => {
                eprintln!("Line: {line}: Unterminated string");
            }
        },
        InterpreterError::Evaluator(ev_error) => match ev_error {
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
