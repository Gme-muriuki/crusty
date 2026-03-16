//! # Tokenizer Module
//!
//! This module implements lexical analysis for the Lox language.
//! It converts raw source code into a stream of tokens that can be
//! parsed into an abstract syntax tree. The tokenizer handles keywords,
//! operators, literals, and identifiers while tracking line numbers
//! for error reporting.

use std::f64;

use crate::reader::Source;

/// Represents all possible token types in the Lox language.
///
/// This enum covers single-character tokens, compound operators,
/// literals, keywords, and the end-of-file marker.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single character tokens
    /// Left parenthesis `(`
    LeftParen,
    /// Right parenthesis `)`
    RightParen,
    /// Left brace `{`
    LeftBraces,
    /// Right brace `}`
    RightBraces,
    /// Comma `,`
    Comma,
    /// Dot `.`
    Dot,
    /// Minus `-`
    Minus,
    /// Plus `+`
    Plus,
    /// Semicolon `;`
    SemiColon,
    /// Slash `/`
    Slash,
    /// Star `*`
    Star,
    // One or Two Character Token
    /// Logical not `!`
    Bang,
    /// Not equal `!=`
    BangEqual,
    /// Assignment `=`
    Equal,
    /// Equal `==`
    EqualEqual,
    /// Greater than `>`
    Greater,
    /// Greater than or equal `>=`
    GreaterEqual,
    /// Less than `<`
    Less,
    /// Less than or equal `<=`
    LessEqual,
    // Literals
    /// Variable or function identifier
    Identifier,
    /// String literal
    String,
    /// Integer number
    Number,
    /// Floating point number
    Float,
    // Keywords
    /// Class keyword
    Class,
    /// Logical and
    And,
    /// Else keyword
    Else,
    /// False boolean literal
    False,
    /// Function keyword
    Fun,
    /// For loop keyword
    For,
    /// If keyword
    If,
    /// Nil literal
    Nil,
    /// Logical or
    Or,
    /// While loop keyword
    While,
    /// Print statement keyword
    Print,
    /// Super keyword (for inheritance)
    Super,
    /// This keyword
    This,
    /// True boolean literal
    True,
    /// Variable declaration keyword
    Var,
    /// Return keyword
    Return,
    /// End of file marker
    EOF,
}

/// A single token produced by the lexical analyzer.
///
/// Tokens contain the token type, the original lexeme (text),
/// any associated literal value, and the line number for error reporting.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The type of this token
    pub toktype: TokenType,
    /// The original text that was tokenized
    pub lexeme: String,
    /// Literal value for number/string tokens, None for others
    pub literal: Literal,
    /// Line number where this token appears in the source
    pub line: usize,
}

/// Represents literal values that can be associated with tokens.
///
/// Only string and number tokens carry literal values; other tokens
/// use the None variant.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// String literal value
    Str(String),
    /// Numeric literal value (always stored as f64)
    Num(f64),
    /// No literal value (for non-literal tokens)
    None,
}

impl Token {
    /// Creates a new token with the specified properties.
    ///
    /// # Arguments
    /// * `toktype` - The type of token
    /// * `lexeme` - The original source text
    /// * `line` - Line number in source
    /// * `literal` - Associated literal value (if any)
    ///
    /// # Returns
    /// * A new Token instance
    ///
    /// # Examples
    /// ```rust
    /// use crusty::tokenize::{Token, TokenType, Literal};
    ///
    /// let token = Token::new(
    ///     TokenType::Number,
    ///     "42",
    ///     1,
    ///     Literal::Num(42.0)
    /// );
    /// ```
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

    /// Converts the token to a human-readable string representation.
    ///
    /// Useful for debugging and token stream inspection.
    pub fn to_string(self) -> String {
        return format!("{:#?} {} {:#?}", self.toktype, self.lexeme, self.literal);
    }
}

/// A collection of tokens representing a complete token stream.
///
/// This is the output of the tokenization process and input to the parser.
#[derive(Debug, PartialEq)]
pub struct Tokens {
    /// The sequence of tokens from the source code
    pub tokens: Vec<Token>,
}

/// Error type for tokenization failures.
///
/// Contains a list of scanning errors that occurred during tokenization.
#[allow(unused)]
#[derive(Debug)]
pub struct TokenizeError {
    /// Collection of individual scanning errors
    error: Vec<ScanError>,
}

/// Main tokenization function that converts source code to tokens.
///
/// This function creates a scanner and processes the entire source,
/// returning either a complete token stream or a list of errors.
///
/// # Arguments
/// * `source` - The source code to tokenize
///
/// # Returns
/// * `Ok(Tokens)` containing the token stream on success
/// * `Err(TokenizeError)` with scanning errors on failure
pub fn tokenize(source: Source) -> Result<Tokens, TokenizeError> {
    Scanner::new(&source.contents).scan_tokens()
}

#[derive(PartialEq, Debug)]
pub struct Scanner {
    /// Source code as a vector of characters for easy indexing
    source: Vec<char>,
    /// Accumulated tokens from the scan
    tokens: Vec<Token>,
    /// Start index of the current token being scanned
    start: usize,
    /// Current position in the source
    current: usize,
    /// Current line number for error reporting
    line: usize,
    /// Any scanning errors encountered
    error: Vec<ScanError>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ScanError {
    /// An unexpected character was encountered
    UnexpectedCharacter {
        /// Line where the error occurred
        line: usize,
        /// The unexpected character
        ch: char
    },
}

impl ScanError {
    pub fn new(line: usize, ch: char) -> Self {
        Self::UnexpectedCharacter { line, ch }
    }
}

impl Scanner {
    /// Creates a new scanner for the given source code.
    ///
    /// # Arguments
    /// * `source` - The source code string to scan
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

    /// Checks if the scanner has reached the end of the source.
    ///
    /// # Returns
    /// * `true` if at end of source, `false` otherwise
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Records a scanning error.
    ///
    /// # Arguments
    /// * `err` - The error to record
    pub fn error(&mut self, err: ScanError) {
        self.error.push(err);
    }

    /// Checks if the next character matches the expected one, consuming it if so.
    ///
    /// Used for handling compound tokens like `!=`, `==`, etc.
    ///
    /// # Arguments
    /// * `expected` - The character to match against
    ///
    /// # Returns
    /// * `true` if the character matched and was consumed, `false` otherwise
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

    /// Scans all tokens from the source code.
    ///
    /// This is the main scanning loop that processes the entire source,
    /// calling `scan_token()` for each character until the end is reached.
    ///
    /// # Returns
    /// * `Ok(Tokens)` with the complete token stream on success
    /// * `Err(TokenizeError)` if scanning errors were encountered
    pub fn scan_tokens(mut self) -> Result<Tokens, TokenizeError> {
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
            Err(TokenizeError { error: self.error })
        }
    }
    /// Scans a single token starting from the current position.
    ///
    /// This method examines the current character and determines what kind
    /// of token it represents, potentially consuming additional characters
    /// for compound tokens, strings, numbers, or identifiers.
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

    /// Extracts the current lexeme (substring) being scanned.
    ///
    /// # Returns
    /// * The string slice from start to current position
    fn lexeme(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }

    /// Scans an identifier or keyword token.
    ///
    /// Consumes alphanumeric characters and underscores, then checks
    /// if the lexeme matches any reserved keywords.
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

    /// Scans a numeric literal token.
    ///
    /// Handles both integers and floating-point numbers, consuming
    /// digits and an optional decimal point.
    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let literal = Literal::Num(self.lexeme().parse().unwrap());

        self.add_token_with_literal(TokenType::Number, literal);
    }
    /// Peeks at the character two positions ahead.
    ///
    /// Used for lookahead in number parsing (checking for decimal point).
    ///
    /// # Returns
    /// * The next-next character, or null character if at end
    fn peek_next(&self) -> char {
        if self.current + 1 == self.source.len() {
            return '\0';
        };
        self.source[self.current + 1]
    }

    /// Scans a string literal token.
    ///
    /// Consumes characters until the closing quote, handling multiline strings
    /// and tracking line numbers. Currently doesn't handle escape sequences.
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
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

    /// Peeks at the next character without consuming it.
    ///
    /// # Returns
    /// * The current character, or null character if at end
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\x00'
        } else {
            self.source[self.current]
        }
    }

    /// Consumes and returns the current character, advancing the scanner.
    ///
    /// # Returns
    /// * The character that was consumed
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;

        c
    }

    /// Adds a token with a literal value to the token stream.
    ///
    /// # Arguments
    /// * `toktype` - The type of token
    /// * `literal` - The literal value to associate
    fn add_token_with_literal(&mut self, toktype: TokenType, literal: Literal) {
        self.tokens
            .push(Token::new(toktype, self.lexeme(), self.line, literal));
    }

    /// Adds a token without a literal value to the token stream.
    ///
    /// # Arguments
    /// * `toktype` - The type of token
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

    #[test]
    fn test_single_character() {
        let scanner = Scanner::new("(){},.+-;*");
        let tokens = scanner.scan_tokens().unwrap();

        assert_eq!(
            tokens.tokens,
            vec![
                Token::new(TokenType::LeftParen, "(", 1, Literal::None),
                Token::new(TokenType::RightParen, ")", 1, Literal::None),
                Token::new(TokenType::LeftBraces, "{", 1, Literal::None),
                Token::new(TokenType::RightBraces, "}", 1, Literal::None),
                Token::new(TokenType::Comma, ",", 1, Literal::None),
                Token::new(TokenType::Dot, ".", 1, Literal::None),
                Token::new(TokenType::Plus, "+", 1, Literal::None),
                Token::new(TokenType::Minus, "-", 1, Literal::None),
                Token::new(TokenType::SemiColon, ";", 1, Literal::None),
                Token::new(TokenType::Star, "*", 1, Literal::None),
                Token::new(TokenType::EOF, "", 1, Literal::None),
            ]
        )
    }

    #[test]
    fn test_double_character() {
        let scanner = Scanner::new("! != < <= > >= == =");
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(
            tokens.tokens,
            vec![
                Token::new(TokenType::Bang, "!", 1, Literal::None),
                Token::new(TokenType::BangEqual, "!=", 1, Literal::None),
                Token::new(TokenType::Less, "<", 1, Literal::None),
                Token::new(TokenType::LessEqual, "<=", 1, Literal::None),
                Token::new(TokenType::Greater, ">", 1, Literal::None),
                Token::new(TokenType::GreaterEqual, ">=", 1, Literal::None),
                Token::new(TokenType::EqualEqual, "==", 1, Literal::None),
                Token::new(TokenType::Equal, "=", 1, Literal::None),
                Token::new(TokenType::EOF, "", 1, Literal::None),
            ]
        )
    }

    #[test]
    fn test_string() {
        let scanner = Scanner::new("\"Hello\" \"world\"");
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(
            tokens.tokens,
            vec![
                Token::new(
                    TokenType::String,
                    "\"Hello\"",
                    1,
                    Literal::Str("Hello".to_string())
                ),
                Token::new(
                    TokenType::String,
                    "\"world\"",
                    1,
                    Literal::Str("world".to_string())
                ),
                Token::new(TokenType::EOF, "", 1, Literal::None)
            ]
        )
    }
    #[test]
    fn test_number() {
        let scanner = Scanner::new("1234 231.23");
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(
            tokens.tokens,
            vec![
                Token::new(TokenType::Number, "1234", 1, Literal::Num(1234.0)),
                Token::new(TokenType::Number, "231.23", 1, Literal::Num(231.23)),
                Token::new(TokenType::EOF, "", 1, Literal::None)
            ]
        )
    }

    #[test]
    fn test_keywords() {
        let scanner = Scanner::new(
            "class and if while else false for fun nil or print return super this true var ",
        );
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(
            tokens.tokens,
            vec![
                Token::new(TokenType::Class, "class", 1, Literal::None),
                Token::new(TokenType::And, "and", 1, Literal::None),
                Token::new(TokenType::If, "if", 1, Literal::None),
                Token::new(TokenType::While, "while", 1, Literal::None),
                Token::new(TokenType::Else, "else", 1, Literal::None),
                Token::new(TokenType::False, "false", 1, Literal::None),
                Token::new(TokenType::For, "for", 1, Literal::None),
                Token::new(TokenType::Fun, "fun", 1, Literal::None),
                Token::new(TokenType::Nil, "nil", 1, Literal::None),
                Token::new(TokenType::Or, "or", 1, Literal::None),
                Token::new(TokenType::Print, "print", 1, Literal::None),
                Token::new(TokenType::Return, "return", 1, Literal::None),
                Token::new(TokenType::Super, "super", 1, Literal::None),
                Token::new(TokenType::This, "this", 1, Literal::None),
                Token::new(TokenType::True, "true", 1, Literal::None),
                Token::new(TokenType::Var, "var", 1, Literal::None),
                Token::new(TokenType::EOF, "", 1, Literal::None),
            ]
        )
    }
}
