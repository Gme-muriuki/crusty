use crate::{
    ast::{AST, Expr, Operator},
    tokenize::{Token, TokenType, Tokens},
};

impl From<&Token> for Operator {
    fn from(tok: &Token) -> Self {
        match tok.toktype {
            TokenType::Plus => Operator::OAdd,
            TokenType::Minus => Operator::ODiv,
            TokenType::Star => Operator::OMul,
            TokenType::Slash => Operator::ODiv,
            TokenType::Less => Operator::OLt,
            TokenType::LessEqual => Operator::OLeq,
            TokenType::Greater => Operator::OGt,
            TokenType::GreaterEqual => Operator::OGeq,
            TokenType::EqualEqual => Operator::OEq,
            TokenType::BangEqual => Operator::ONeq,
            TokenType::And => Operator::OAnd,
            TokenType::Or => Operator::OOr,
            TokenType::Bang => Operator::ONot,
            _ => panic!("Not an operator {:?}", tok.toktype),
        }
    }
}

#[derive(Debug)]
struct Parser {
    // Parsing involves left-to-right scan over tokens. Sometimes peeking ahead if necessary.!!
    tokens: Vec<Token>,
    size: usize,
}
#[derive(Debug)]
pub enum PError {
    SyntaxError { line: usize, msg: String },
}

impl Parser {
    pub fn new(tokens: Tokens) -> Self {
        Self {
            tokens: tokens.tokens,
            size: 0,
        }
    }
    pub fn accept(&mut self, toktype: TokenType) -> bool {
        if !self.at_end() && self.tokens[self.size].toktype == toktype {
            self.size += 1;
            true
        } else {
            false
        }
    }
    fn accepts<const N: usize>(&mut self, toktype: [TokenType; N]) -> bool {
        if !self.at_end() && toktype.contains(&self.tokens[self.size].toktype) {
            self.size += 1;
            true
        } else {
            false
        }
    }
    fn at_end(&self) -> bool {
        self.size >= self.tokens.len()
    }
    fn last_token(&self) -> &Token {
        &self.tokens[self.size - 1]
    }
    fn last_lexeme(&self) -> &String {
        &self.tokens[self.size - 1].lexeme
    }
    fn expect(&mut self, toktype: TokenType, msg: &str) -> Result<(), PError> {
        if !self.accept(toktype.clone()) {
            Err(self.syntax_error(msg))
        } else {
            Ok(())
        }
    }

    fn syntax_error(&self, msg: impl Into<String>) -> PError {
        PError::SyntaxError {
            line: self.tokens[self.size].line,
            msg: format!("{} at {:?}", msg.into(), self.tokens[self.size].lexeme),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, PError> {
        if self.accept(TokenType::Number) {
            Ok(Expr::num(self.last_lexeme()))
        } else if self.accept(TokenType::String) {
            Ok(Expr::str(self.last_lexeme()))
        } else if self.accept(TokenType::LeftParen) {
            let expr = self.parse_expr()?;
            self.expect(TokenType::RightParen, "Expected ')' after expression")?;
            Ok(Expr::grouping(expr))
        } else if self.accept(TokenType::LeftBraces) {
            let expr = self.parse_expr()?;
            self.expect(TokenType::RightBraces, "Expected '}' after expression")?;
            Ok(Expr::grouping(expr))
        } else if self.accept(TokenType::True) {
            Ok(Expr::bool(true))
        } else if self.accept(TokenType::False) {
            Ok(Expr::bool(false))
        } else {
            Err(self.syntax_error("Expected primary"))
        }
    }
    fn parse_expr(&mut self) -> Result<Expr, PError> {
        let left = self.parse_primary()?;
        if self.accepts([
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Slash,
            TokenType::Star,
        ]) {
            let ops = Operator::from(self.last_token());
            let right = self.parse_primary()?;
            Ok(Expr::binary(left, ops, right))
        } else {
            Ok(left)
        }
    }
    pub fn parse_top(&mut self) -> Result<AST, PError> {
        let top = self.parse_expr()?;
        if !self.at_end() {
            return Err(self.syntax_error("Unparsed inputs"));
        }
        Ok(AST { top })
    }
}

pub fn parse(tokens: Tokens) -> Result<AST, PError> {
    println!("Parsing");
    Ok(Parser::new(tokens).parse_top()?)
}

#[cfg(test)]
mod test {
    use crate::{reader::Source, tokenize::tokenize};

    use super::*;

    // Helper:
    fn parse_string(s: &str) -> AST {
        let source = Source::new(s);
        let tokens = tokenize(source).unwrap();
        parse(tokens).unwrap()
    }
    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
    #[test]
    fn test_primary() {
        assert_eq!(
            parse_string("123"),
            AST {
                top: Expr::num("123")
            }
        );
        assert_eq!(
            parse_string("\"Hello\""),
            AST {
                top: Expr::str("\"Hello\"")
            }
        );
        assert_eq!(
            parse_string("(2)"),
            AST {
                top: Expr::grouping(Expr::num("2"))
            }
        )
    }

    #[test]
    fn test_binary() {
        assert_eq!(
            parse_string("1 + 2"),
            AST {
                top: Expr::binary(Expr::num("1"), Operator::OAdd, Expr::num("2"))
            }
        )
    }

    #[test]
    fn test_bool() {
        assert_eq!(
            parse_string("true"),
            AST {
                top: Expr::bool(true)
            }
        );
        assert_eq!(
            parse_string("false"),
            AST {
                top: Expr::bool(false)
            }
        )
    }
    #[test]
    fn test_grouping() {
        assert_eq!(
            parse_string("{ 1 + 2}"),
            AST {
                top: Expr::grouping(Expr::binary(Expr::num("1"), Operator::OAdd, Expr::num("2")))
            }
        );
        assert_eq!(
            parse_string("( 1 + 2 )"),
            AST {
                top: Expr::grouping(Expr::binary(Expr::num("1"), Operator::OAdd, Expr::num("2")))
            }
        )
    }
}
