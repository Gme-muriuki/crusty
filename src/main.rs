#![allow(unused)]
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

// Tokenizer main impl
fn run_interpreter(source: Source, interpreter: &mut Interpreter) -> Result<(), InterpreterError> {
    let tokens = tokenize(source)?;
    let ast = parse(tokens)?;
    interpreter.evaluate(ast)?;
    Ok(())
}
fn run(source: Source) -> Result<(), InterpreterError> {
    let mut interp = Interpreter::new();
    run_interpreter(source, &mut interp)
}

fn run_file(filename: &str) -> Result<(), InterpreterError> {
    let source = reader::read_source(filename)?;
    run(source)
}

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
