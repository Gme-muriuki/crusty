use std::f64;

use crate::reader::Source;

#[derive(Debug, Clone, PartialEq)]
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
    Return,
    //
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub toktype: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    // literal: object
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct TError {
    error: Vec<ScanError>,
}

pub fn tokenize(source: Source) -> Result<Tokens, TError> {
    Scanner::new(&source.contents).scan_tokens()
}

#[derive(PartialEq, Debug)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    error: Vec<ScanError>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ScanError {
    UnexpectedCharacter { line: usize, ch: char },
}

impl ScanError {
    pub fn new(line: usize, ch: char) -> Self {
        Self::UnexpectedCharacter { line, ch }
    }
}

impl Scanner {
    fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            error: vec![],
        }
    }
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    pub fn error(&mut self, err: ScanError) {
        self.error.push(err);
    }

    pub fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    pub fn scan_tokens(mut self) -> Result<Tokens, TError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "", self.line, Literal::None));

        if self.error.len() == 0 {
            Ok(Tokens {
                tokens: self.tokens,
            })
        } else {
            Err(TError { error: self.error })
        }
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
            '!' => {
                let toktype = if self.is_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };

                self.add_token(toktype)
            }
            '=' => {
                let toktype = if self.is_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(toktype);
            }
            '<' => {
                let toktype = if self.is_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };

                self.add_token(toktype);
            }

            '>' => {
                let toktype = if self.is_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };

                self.add_token(toktype);
            }
            '/' => {
                if self.is_match('/') {
                    while self.peek() != '\n' && self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            // Ignore whitespaces
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            c if c.is_digit(10) => {
                self.number();
            }

            x if x.is_alphabetic() => {
                self.identifier();
            }

            ch => {
                println!("Entered this branch");
                self.error(ScanError::new(self.line, ch));
                println!("Errors: {:#?}", self.error)
            }
        }
    }

    fn lexeme(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let toktype = match &self.lexeme()[..] {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "true" => TokenType::True,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        self.add_token(toktype);
    }

    fn number(&mut self) {
        while (self.peek().is_digit(10)) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while (self.peek().is_digit(10)) {
                self.advance();
            }
        }

        let literal = Literal::Num(self.lexeme().parse().unwrap());

        self.add_token_with_literal(TokenType::Number, literal);
    }
    fn peek_next(&self) -> char {
        if self.current + 1 == self.source.len() {
            return '\0';
        };
        self.source[self.current + 1]
    }

    fn string(&mut self) {
        while (self.peek() != '"' && !self.is_at_end()) {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        // Unterminated string
        if self.is_at_end() {
            return;
        }
        self.advance();

        let value = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect::<String>();

        self.add_token_with_literal(TokenType::String, Literal::Str(value));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\x00'
        } else {
            self.source[self.current]
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;

        c
    }

    fn add_token_with_literal(&mut self, toktype: TokenType, literal: Literal) {
        self.tokens
            .push(Token::new(toktype, self.lexeme(), self.line, literal));
    }

    fn add_token(&mut self, toktype: TokenType) {
        self.add_token_with_literal(toktype, Literal::None);
    }
}

// mod test {
// #[cfg(test)]
//     use super::*;

//     #[test]
//     fn is_alive() {
//         assert_eq!(true, true)
//     }

//     #[test]
//     fn test_single_character() {
//         let mut scanner = Scanner::new("(){},.+-;*");
//         let mut tokens = scanner.scan_tokens().unwrap();

//         assert_eq!(
//             tokens.tokens,
//             vec![
//                 Token::new(TokenType::LeftParen, "(", 1, Literal::None),
//                 Token::new(TokenType::RightParen, ")", 1, Literal::None),
//                 Token::new(TokenType::LeftBraces, "{", 1, Literal::None),
//                 Token::new(TokenType::RightBraces, "}", 1, Literal::None),
//                 Token::new(TokenType::Comma, ",", 1, Literal::None),
//                 Token::new(TokenType::Dot, ".", 1, Literal::None),
//                 Token::new(TokenType::Plus, "+", 1, Literal::None),
//                 Token::new(TokenType::Minus, "-", 1, Literal::None),
//                 Token::new(TokenType::SemiColon, ";", 1, Literal::None),
//                 Token::new(TokenType::Star, "*", 1, Literal::None),
//                 Token::new(TokenType::EOF, "", 1, Literal::None),
//             ]
//         )
//     }

//     #[test]
//     fn test_double_character() {
//         let mut scanner = Scanner::new("! != < <= > >= == =");
//         let mut tokens = scanner.scan_tokens().unwrap();
//         assert_eq!(
//             tokens.tokens,
//             vec![
//                 Token::new(TokenType::Bang, "!", 1, Literal::None),
//                 Token::new(TokenType::BangEqual, "!=", 1, Literal::None),
//                 Token::new(TokenType::Less, "<", 1, Literal::None),
//                 Token::new(TokenType::LessEqual, "<=", 1, Literal::None),
//                 Token::new(TokenType::Greater, ">", 1, Literal::None),
//                 Token::new(TokenType::GreaterEqual, ">=", 1, Literal::None),
//                 Token::new(TokenType::EqualEqual, "==", 1, Literal::None),
//                 Token::new(TokenType::Equal, "=", 1, Literal::None),
//                 Token::new(TokenType::EOF, "", 1, Literal::None),
//             ]
//         )
//     }

//     #[test]
//     fn test_string() {
//         let mut scanner = Scanner::new("\"Hello\" \"world\"");
//         let tokens = scanner.scan_tokens().unwrap();
//         assert_eq!(
//             tokens.tokens,
//             vec![
//                 Token::new(
//                     TokenType::String,
//                     "\"Hello\"",
//                     1,
//                     Literal::Str("Hello".to_string())
//                 ),
//                 Token::new(
//                     TokenType::String,
//                     "\"world\"",
//                     1,
//                     Literal::Str("world".to_string())
//                 ),
//                 Token::new(TokenType::EOF, "", 1, Literal::None)
//             ]
//         )
//     }
//     #[test]
//     fn test_number() {
//         let mut scanner = Scanner::new("1234 231.23");
//         let tokens = scanner.scan_tokens().unwrap();
//         assert_eq!(
//             tokens.tokens,
//             vec![
//                 Token::new(TokenType::Number, "1234", 1, Literal::Num(1234.0)),
//                 Token::new(TokenType::Number, "231.23", 1, Literal::Num(231.23)),
//                 Token::new(TokenType::EOF, "", 1, Literal::None)
//             ]
//         )
//     }

//     #[test]
//     fn test_keywords() {
//         let mut scanner = Scanner::new(
//             "class and if while else false for fun nil or print return super this true var ",
//         );
//         let tokens = scanner.scan_tokens().unwrap();
//         assert_eq!(
//             tokens.tokens,
//             vec![
//                 Token::new(TokenType::Class, "class", 1, Literal::None),
//                 Token::new(TokenType::And, "and", 1, Literal::None),
//                 Token::new(TokenType::If, "if", 1, Literal::None),
//                 Token::new(TokenType::While, "while", 1, Literal::None),
//                 Token::new(TokenType::Else, "else", 1, Literal::None),
//                 Token::new(TokenType::False, "false", 1, Literal::None),
//                 Token::new(TokenType::For, "for", 1, Literal::None),
//                 Token::new(TokenType::Fun, "fun", 1, Literal::None),
//                 Token::new(TokenType::Nil, "nil", 1, Literal::None),
//                 Token::new(TokenType::Or, "or", 1, Literal::None),
//                 Token::new(TokenType::Print, "print", 1, Literal::None),
//                 Token::new(TokenType::Return, "return", 1, Literal::None),
//                 Token::new(TokenType::Super, "super", 1, Literal::None),
//                 Token::new(TokenType::This, "this", 1, Literal::None),
//                 Token::new(TokenType::True, "true", 1, Literal::None),
//                 Token::new(TokenType::Var, "var", 1, Literal::None),
//                 Token::new(TokenType::EOF, "", 1, Literal::None),
//             ]
//         )
//     }
// }
