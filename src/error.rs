#![allow(unused)]
//! # Error Handling Module
//!
//! This module provides unified error handling across all phases of the interpreter.
//! It defines a top-level InterpreterError enum that can represent errors from
//! reading, tokenization, parsing, or evaluation. Error conversion traits allow
//! automatic propagation of errors up the call stack.

use std::{
    env,
    io::{Write, stdin, stdout},
    vec,
};

use crate::{
    evaluate::{self, Interpreter, evaluate},
    parser::{self, parse},
    reader::{self, Source, read_source},
    tokenize::{self, TokenizeError, tokenize},
};

/// Top-level error type encompassing all possible interpreter errors.
///
/// This enum unifies errors from different phases of interpretation,
/// allowing consistent error handling and reporting throughout the codebase.
#[derive(Debug)]
pub enum InterpreterError {
    /// File I/O errors during source reading
    Reader(reader::ReadError),
    /// Lexical analysis errors during tokenization
    Tokenizer(tokenize::TokenizeError),
    /// Syntax analysis errors during parsing
    Parser(parser::ParseError),
    /// Runtime errors during evaluation
    Evaluator(evaluate::EvaluateError),
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

impl From<evaluate::EvaluateError> for InterpreterError {
    fn from(error: evaluate::EvaluateError) -> Self {
        InterpreterError::Evaluator(error)
    }
}

/// Reports an interpreter error to stderr with appropriate formatting.
///
/// This function provides user-friendly error messages by pattern matching
/// on the error type and extracting relevant information like line numbers
/// and descriptive messages.
///
/// # Arguments
/// * `error` - The interpreter error to report
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
            evaluate::EvaluateError::ZeroDivision => eprintln!("Division by zero"),
            evaluate::EvaluateError::UnsupportedBinOps(left, operator, right) => {
                eprintln!("Unsupported operation: {left:?} {operator:?} {right:?}")
            }
            evaluate::EvaluateError::UnsupportedUnaryOps(operator, left) => {
                eprintln!("Unsupported operation: {operator:?} {left:?}")
            }
            evaluate::EvaluateError::NotFound(name) => {
                eprintln!("{name} not found")
            }
        },
    }
}
