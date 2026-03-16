#![allow(unused)]
//! # Crusty Interpreter Main Module
//!
//! This module serves as the entry point for the Crusty Lox interpreter.
//! It handles command-line arguments, sets up the REPL (Read-Eval-Print Loop)
//! for interactive use, and coordinates the execution of Lox programs from files.
//!
//! The interpreter follows a pipeline: source reading → tokenization → parsing → evaluation.

use std::{
    env,
    io::{Write, stdin, stdout},
    vec,
};

use crate::{
    error::{InterpreterError, report_error},
    evaluate::{Interpreter, evaluate},
    parser::parse,
    reader::{Source, read_source},
    tokenize::tokenize,
};

pub mod ast;
pub mod environ;
pub mod error;
pub mod evaluate;
pub mod parser;
pub mod reader;
pub mod tokenize;
pub mod tokenizer;

// Tokenizer alternative impl
// fn run_interpreter(source: Source, interpreter: &mut Interpreter) -> Result<(), InterpreterError> {
//     let tokens = tokenize(source).unwrap();
//     println!("Tokens: {:#?}", tokens);
//     // let ast = parse(tokens)?;
//     // interpreter.evaluate(ast)?;
//     Ok(())
// }

/// Executes the full interpretation pipeline: tokenization, parsing, and evaluation.
///
/// This is the main entry point for running Lox code. It coordinates the three
/// main phases of interpretation and handles any errors that occur during processing.
///
/// # Arguments
/// * `source` - The source code to interpret
/// * `interpreter` - The interpreter instance to use
///
/// # Returns
/// * `Ok(())` on successful execution
/// * `Err(InterpreterError)` if any phase fails
///
/// # Examples
/// ```rust,ignore
/// use crusty::{Source, Interpreter, InterpreterError};
///
/// let source = Source::new("print 42;".to_string());
/// let mut interpreter = Interpreter::new();
/// assert!(run_interpreter(source, &mut interpreter).is_ok());
/// ```
fn run_interpreter(source: Source, interpreter: &mut Interpreter) -> Result<(), InterpreterError> {
    let tokens = tokenize(source)?;
    let ast = parse(tokens)?;
    interpreter.evaluate(ast)?;
    Ok(())
}

/// Runs a complete Lox program from source code.
///
/// Creates a new interpreter instance and executes the source through the full pipeline.
fn run(source: Source) -> Result<(), InterpreterError> {
    let mut interp = Interpreter::new();
    run_interpreter(source, &mut interp)
}

/// Executes a Lox program from a file.
///
/// Reads the file contents and runs the program using the standard interpretation pipeline.
fn run_file(filename: &str) -> Result<(), InterpreterError> {
    let source = reader::read_source(filename)?;
    run(source)
}

/// Runs the interactive REPL (Read-Eval-Print Loop).
///
/// Provides an interactive prompt where users can enter Lox expressions and statements.
/// Each line is processed immediately, with results or errors displayed to the user.
fn run_prompt() {
    let mut stdout = stdout();
    let mut stdin = stdin();
    let mut interpreter = Interpreter::new();
    'lox: loop {
        stdout.write_all(b"> ").unwrap();
        stdout.flush().unwrap();
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).expect("Failed to read input");
        let source = reader::Source::new(buffer);

        match run_interpreter(source, &mut interpreter) {
            Ok(_) => println!("Program run successfully"),
            Err(e) => {
                eprintln!(
                    "I'm sorry you encountered that... (fork and create a PR to fix it...) ..."
                );
                eprintln!("{:#?}", e)
            }
        }
    }
}

/// Main entry point for the Crusty interpreter.
///
/// Handles command-line arguments to determine execution mode:
/// - No arguments: Start interactive REPL
/// - One argument: Execute the specified Lox file
/// - Multiple arguments: Display usage error
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
