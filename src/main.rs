#![allow(unused)]
use crate::{evaluate::evaluate, parser::parse, reader::read_source, tokenize::tokenize};

pub mod evaluate;
pub mod parser;
pub mod reader;
pub mod tokenize;

fn main() {
    let source = read_source("somefile.lox").unwrap();
    let tokens = tokenize(source).unwrap();
    let ast = parse(tokens).unwrap();
    let eval = evaluate(ast).unwrap();
}
