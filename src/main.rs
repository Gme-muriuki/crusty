use crate::{evaluate::evaluate, parser::parse, reader::read_source, tokenize::tokenize};

pub mod evaluate;
pub mod parser;
pub mod reader;
pub mod tokenize;
pub mod types;


fn main() {
    evaluate();
    parse();
    read_source();
    tokenize();
    println!("Hello, world!");
}

