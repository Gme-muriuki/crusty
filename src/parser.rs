use crate::{
    ast::{AST, Expr, Operator, Stmt},
    tokenize::{Token, TokenType, Tokens},
};

impl From<&Token> for Operator {
    fn from(tok: &Token) -> Self {
        match tok.toktype {
            TokenType::Plus => Operator::OAdd,
            TokenType::Minus => Operator::OSub,
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
            TokenType::Equal => Operator::OEq,
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
pub enum ParseError {
    SyntaxError { line: usize, msg: String },
    UnterminatedCharacter { line: usize },
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
        self.size >= self.tokens.len() || self.tokens[self.size].toktype == TokenType::EOF
    }
    fn last_token(&self) -> &Token {
        &self.tokens[self.size - 1]
    }
    fn last_lexeme(&self) -> &String {
        &self.tokens[self.size - 1].lexeme
    }
    fn consume(&mut self, toktype: TokenType, msg: &str) -> Result<(), ParseError> {
        if !self.accept(toktype.clone()) {
            Err(self.syntax_error(msg))
        } else {
            Ok(())
        }
    }

    fn syntax_error(&self, msg: impl Into<String>) -> ParseError {
        ParseError::SyntaxError {
            line: self.tokens[self.size].line,
            msg: format!("{} at {:?}", msg.into(), self.tokens[self.size].lexeme),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        if self.accept(TokenType::Number) {
            Ok(Expr::num(self.last_lexeme()))
        } else if self.accept(TokenType::String) {
            Ok(Expr::str(self.last_lexeme()))
        } else if self.accept(TokenType::LeftParen) {
            let expr = self.parse_expr()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression")?;
            Ok(Expr::grouping(expr))
        } else if self.accept(TokenType::LeftBraces) {
            let expr = self.parse_expr()?;
            self.consume(TokenType::RightBraces, "Expected '}' after expression")?;
            Ok(Expr::grouping(expr))
        } else if self.accept(TokenType::True) {
            Ok(Expr::bool(true))
        } else if self.accept(TokenType::False) {
            Ok(Expr::bool(false))
        } else if self.accept(TokenType::Nil) {
            Ok(Expr::nil())
        } else if self.accept(TokenType::Identifier) {
            Ok(Expr::variable(self.last_lexeme()))
        } else {
            Err(self.syntax_error("Expected primary"))
        }
    }
    fn parse_binary(&mut self) -> Result<Expr, ParseError> {
        let left = self.parse_unary()?;
        if self.accepts([
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Slash,
            TokenType::Star,
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::EqualEqual,
            TokenType::BangEqual,
        ]) {
            let ops = Operator::from(self.last_token());
            let right = self.parse_unary()?;
            Ok(Expr::binary(left, ops, right))
        } else {
            Ok(left)
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_assignment()
    }

    pub fn parse_top(&mut self) -> Result<AST, ParseError> {
        let top = self.parse_statements()?;
        if !self.at_end() {
            return Err(self.syntax_error("Unparsed inputs"));
        }
        Ok(AST { top })
    }
    pub fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if self.accepts([TokenType::Bang, TokenType::Minus]) {
            let ops = Operator::from(self.last_token());
            Ok(Expr::unary(ops, self.parse_unary()?))
        } else {
            self.parse_primary()
        }
    }

    // Parsing expression
    pub fn parse_statements(&mut self) -> Result<Vec<Stmt>, ParseError> {
        // parse zero or more statements until we reach the end of the file. Each statement can be a print statement, an expression statement, a variable declaration, etc.
        let mut statements = Vec::new();
        while !self.at_end() {
            statements.push(self.parse_declaration()?);
        }
        Ok(statements)
    }
    pub fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        // parse a single statement, which can be a print statement, an expression statement, a variable declaration, etc.
        if self.accept(TokenType::Print) {
            self.parse_print_statement()
        } else {
            self.parse_expression_statement()
        }
    }

    pub fn parse_declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.accept(TokenType::Var) {
            self.parse_var_declaration()
        } else {
            self.parse_statement()
        }
    }
    pub fn parse_print_statement(&mut self) -> Result<Stmt, ParseError> {
        // parse a print statement, which consists of the 'print' keyword followed by an expression and a semicolon.
        let value = self.parse_expr()?;
        self.consume(TokenType::SemiColon, "Expected ';' after value")?;
        Ok(Stmt::print(value))
    }

    pub fn parse_var_declaration(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::Identifier, "Expected a variable name");
        let name = self.last_lexeme().clone();

        let mut initializer = None;
        if self.accept(TokenType::Equal) {
            initializer = Some(self.parse_expr()?);
        }
        self.consume(
            TokenType::SemiColon,
            "Expected ';' after the variable declaration",
        );
        Ok(Stmt::var(name, initializer))
    }

    pub fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_binary()?;

        if self.accept(TokenType::Equal) {
            let value = self.parse_assignment()?;
            if let Expr::EVarDecl { name } = expr {
                return Ok(Expr::assign(name, value));
            } else {
                panic!("Invalid assignment target")
            }
        }

        Ok(expr)
    }

    pub fn parse_expression_statement(&mut self) -> Result<Stmt, ParseError> {
        // parse an expression statement, which consists of an expression followed by a semicolon.
        let value = self.parse_expr()?;
        self.consume(TokenType::SemiColon, "Expected ';' after value {value:?}");
        Ok(Stmt::expression(value))
    }
}

pub fn parse(tokens: Tokens) -> Result<AST, ParseError> {
    Ok(Parser::new(tokens).parse_top()?)
}

#[cfg(test)]
mod test {
    use crate::{reader::Source, tokenize::tokenize};

    use super::*;

    // Helper:
    fn parse_expr_string(s: &str) -> Expr {
        let source = Source::new(s);
        let tokens = tokenize(source).unwrap();
        Parser::new(tokens).parse_expr().unwrap()
    }

    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
    #[test]
    fn test_primary() {
        assert_eq!(parse_expr_string("123"), Expr::num("123"));
        assert_eq!(parse_expr_string("\"Hello\""), Expr::str("\"Hello\""));
        assert_eq!(parse_expr_string("(2)"), Expr::grouping(Expr::num("2")));
        assert_eq!(parse_expr_string("true"), Expr::bool(true));
        assert_eq!(parse_expr_string("false"), Expr::bool(false));
        assert_eq!(parse_expr_string("nil"), Expr::nil());
    }

    #[test]
    fn test_binary() {
        assert_eq!(
            parse_expr_string("1 + 2"),
            Expr::binary(Expr::num("1"), Operator::OAdd, Expr::num("2"))
        )
    }

    #[test]
    fn test_bool() {
        assert_eq!(parse_expr_string("true"), Expr::bool(true));
        assert_eq!(parse_expr_string("false"), Expr::bool(false))
    }
    #[test]
    fn test_grouping() {
        assert_eq!(
            parse_expr_string("{ 1 + 2}"),
            Expr::grouping(Expr::binary(Expr::num("1"), Operator::OAdd, Expr::num("2")))
        );
        assert_eq!(
            parse_expr_string("( 1 + 2 )"),
            Expr::grouping(Expr::binary(Expr::num("1"), Operator::OAdd, Expr::num("2")))
        )
    }
}

// I decided to throw operator precedence out of the window.....🕊️🕊️🕊️... It is a real pain...
// Maybe I'll fix this in the future, but definitely not tomorrow....
// # operator precedence has to die...
