use crate::reader::Source;
use std::collections::HashMap;

#[derive(Debug)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBraces,
    RightBraces,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    // One or Two Character Token
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    Float,
    // Keywords
    Class,
    And,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    While,
    Print,
    Super,
    This,
    True,
    Var,
    //
    EOF,
}

#[derive(Debug)]
pub struct Token {
    toktype: TokenType,
    lexeme: String,
    literal: Literal,
    // literal: object
    line: usize,
}

#[derive(Debug)]
pub enum Literal {
    Str(String),
    Num(f64),
    None,
}

impl Token {
    pub fn new(toktype: TokenType, lexeme: String, line: usize, literal: Literal) -> Self {
        Self {
            toktype,
            lexeme,
            line,
            literal,
        }
    }

    pub fn to_string(self) -> String {
        return format!("{:#?} {} {:#?}", self.toktype, self.lexeme, self.literal);
    }
}

#[derive(Debug)]
pub struct Tokens {
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct TError {}

pub fn tokenize(source: Source) -> Result<Tokens, TError> {
    println!("Tokenize");

    let tokens = vec![];

    Ok(Tokens { tokens })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
}
