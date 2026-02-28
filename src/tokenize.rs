use crate::reader::Source;
use std::collections::HashMap;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Token {
    toktype: TokenType,
    lexeme: String,
    literal: Literal,
    // literal: object
    line: usize,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Str(String),
    Num(f64),
    None,
}

impl Token {
    pub fn new(
        toktype: TokenType,
        lexeme: impl Into<String>,
        line: usize,
        literal: Literal,
    ) -> Self {
        Self {
            toktype,
            lexeme: lexeme.into(),
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

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    pub fn scan_tokens(&mut self) -> Result<Tokens, TError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "", self.line, Literal::None));

        Ok(Tokens {
            tokens: self.tokens.clone(),
        })
    }
    pub fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBraces),
            '}' => self.add_token(TokenType::RightBraces),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            '*' => self.add_token(TokenType::Star),
            ';' => self.add_token(TokenType::SemiColon),
            _ => (),
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;

        c
    }

    fn add_token_with_literal(&mut self, toktype: TokenType, literal: Literal) {
        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();
        self.tokens
            .push(Token::new(toktype, text, self.line, literal));
    }

    fn add_token(&mut self, toktype: TokenType) {
        self.add_token_with_literal(toktype, Literal::None);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
}
