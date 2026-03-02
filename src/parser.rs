use crate::{
    ast::AST,
    tokenize::{Token, TokenType, Tokens},
};

#[derive(Debug)]
struct Parser {
    // Parsing involves left-to-right scan over tokens. Sometimes peeking ahead if necessary.!!
    tokens: Vec<Token>,
    size: usize,
}
#[derive(Debug)]
pub struct PError {}

impl Parser {
    pub fn accept(&mut self, toktype: TokenType) -> bool {
        if !self.at_end() && self.tokens[self.size].toktype == toktype {
            self.size += 1;
            true
        } else {
            false
        }
    }

    fn at_end(&self) -> bool {
        self.size >= self.tokens.len()
    }
    fn last(&self) -> &Token {
        &self.tokens[self.size - 1]
    }
}

pub fn parse(tokens: Tokens) -> Result<AST, PError> {
    println!("Parsing");
    Ok(AST { top: None })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
}
