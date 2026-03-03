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
pub struct PError {}

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
    fn expect(&mut self, toktype: TokenType, msg: &str) {
        if !self.accept(toktype.clone()) {
            panic!("Syntax error: {msg}");
        }
    }

    fn parse_primary(&mut self) -> Expr {
        if self.accept(TokenType::Number) {
            Expr::num(self.last_lexeme())
        } else if self.accept(TokenType::String) {
            Expr::str(self.last_lexeme())
        } else if self.accept(TokenType::LeftParen) {
            let expr = self.parse_expr();
            self.expect(TokenType::RightParen, "Expected ')' after expression");
            Expr::grouping(expr)
        } else if self.accept(TokenType::LeftBraces) {
            let expr = self.parse_expr();
            self.expect(TokenType::RightBraces, "Expected '}' after expression");
            Expr::grouping(expr)
        } else if self.accept(TokenType::True) {
            Expr::bool(true)
        } else if self.accept(TokenType::False) {
            Expr::bool(false)
        } else if self.accept(TokenType::Nil) {
            Expr::nil()
        } else {
            panic!("Syntax error");
        }
    }
    fn parse_expr(&mut self) -> Expr {
        let left = self.parse_primary();
        if self.accepts([
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Slash,
            TokenType::Star,
        ]) {
            let ops = Operator::from(self.last_token());
            let right = self.parse_primary();
            Expr::binary(left, ops, right)
        } else {
            left
        }
    }
    pub fn parse_top(&mut self) -> Result<AST, PError> {
        Ok(AST {
            top: Some(self.parse_expr()),
        })
    }
}

pub fn parse(tokens: Tokens) -> Result<AST, PError> {
    println!("Parsing");
    Ok(Parser::new(tokens).parse_top().unwrap())
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
                top: Some(Expr::num("123"))
            }
        );
        assert_eq!(
            parse_string("\"Hello\""),
            AST {
                top: Some(Expr::str("\"Hello\""))
            }
        )
    }

    #[test]
    fn test_binary() {
        assert_eq!(
            parse_string("1 + 2"),
            AST {
                top: Some(Expr::binary(Expr::num("1"), Operator::OAdd, Expr::num("2")))
            }
        )
    }

    #[test]
    fn test_bool() {
        assert_eq!(
            parse_string("true"),
            AST {
                top: Some(Expr::bool(true))
            }
        );
        assert_eq!(
            parse_string("false"),
            AST {
                top: Some(Expr::bool(false))
            }
        )
    }
    #[test]
    fn test_grouping() {
        assert_eq!(
            parse_string("{ 1 + 2}"),
            AST {
                top: Some(Expr::grouping(Expr::binary(
                    Expr::num("1"),
                    Operator::OAdd,
                    Expr::num("2")
                )))
            }
        );
        assert_eq!(
            parse_string("( 1 + 2 )"),
            AST {
                top: Some(Expr::grouping(Expr::binary(
                    Expr::num("1"),
                    Operator::OAdd,
                    Expr::num("2")
                )))
            }
        )
    }
    #[test]
    fn test_nil() {
        assert_eq!(
            parse_string(" "),
            AST {
                top: Some(Expr::nil())
            }
        )
    }
}
