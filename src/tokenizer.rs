use std::{
    char,
    iter::{Peekable, Scan},
    ops::Range,
    str::CharIndices,
};

type Chars<'a> = Peekable<CharIndices<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single character tokens
    TLeftParen,
    TRightParen,
    TLeftBraces,
    TRightBraces,
    TComma,
    TDot,
    TMinus,
    TPlus,
    TSemiColon,
    TSlash,
    TStar,
    // One or Two Character Token
    TBang,
    TBangEqual,
    TEqual,
    TEqualEqual,
    TGreater,
    TGreaterEqual,
    TLess,
    TLessEqual,
    // Literals
    TIdentifier,
    TString,
    TNumber,
    TFloat,
    // Keywords
    TClass,
    TAnd,
    TElse,
    TFalse,
    TFun,
    TFor,
    TIf,
    TNil,
    TOr,
    TWhile,
    TPrint,
    TSuper,
    TThis,
    TTrue,
    TVar,
    TReturn,
    //
    TEof,
    TIgnore,
}

use TokenType::*;

use crate::reader::Source;

#[derive(Debug)]
pub struct TError {
    error: Vec<ScanError>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ScanError {
    UnexpectedCharacter { line: usize, ch: char },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub toktype: TokenType,
    // I could or maybe should change this so that I don't copy the lexeme but use it as a part of the source.
    // It is doable, although I'll dig big holes for myself, and spend some significant amount of time on it, and I need to also translate the "Building programming Language interpreter C++ code into Rust".
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(toktype: TokenType, lexeme: impl Into<String>, line: usize) -> Self {
        Self {
            toktype,
            lexeme: lexeme.into(),
            line,
        }
    }
}

#[derive(Debug)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}

pub fn accepts(
    chars: &mut Chars,
    toktype: TokenType,
    start: usize,
) -> Option<(TokenType, Range<usize>)> {
    let (n, ch) = chars.next()?;
    Some((toktype, start..n + 1))
}

pub fn map_keywords(lexeme: &str) -> TokenType {
    match lexeme {
        "and" => TAnd,
        "class" => TClass,
        "if" => TIf,
        "else" => TElse,
        "false" => TFalse,
        "for" => TFor,
        "fun" => TFun,
        "nil" => TNil,
        "or" => TOr,
        "print" => TPrint,
        "true" => TTrue,
        "return" => TReturn,
        "super" => TSuper,
        "this" => TThis,
        "var" => TVar,
        "while" => TWhile,
        _ => TIdentifier,
    }
}

pub fn scan_tokens(lexeme: String) -> Result<Tokens, TError> {
    let mut chars = lexeme.char_indices().peekable();
    let mut result = Vec::new();
    let mut line = 1;
    while let Some((mut toktype, range)) = scan_token(&mut chars) {
        if toktype == TIgnore {
            continue;
        }
        let lex = &lexeme[range];
        if toktype == TIdentifier {
            toktype = map_keywords(&lexeme);
        }
        result.push(Token::new(toktype, lex, line));
    }
    result.push(Token::new(TEof, "", line));
    Ok(Tokens { tokens: result })
}

pub fn scan_token(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    scan_simple_symbols(chars)
        .or_else(|| scan_string(chars))
        .or_else(|| scan_numbers(chars))
        .or_else(|| scan_identifier(chars))
        .or_else(|| scan_compare_symbols(chars))
        .or_else(|| ignore_whitespace(chars))
}

fn scan_simple_symbols(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    match ch {
        '+' => accepts(chars, TPlus, start),
        '-' => accepts(chars, TMinus, start),
        '*' => accepts(chars, TStar, start),
        '}' => accepts(chars, TRightBraces, start),
        '{' => accepts(chars, TLeftBraces, start),
        '(' => accepts(chars, TLeftParen, start),
        ')' => accepts(chars, TRightParen, start),
        ';' => accepts(chars, TSemiColon, start),
        ',' => accepts(chars, TComma, start),
        '.' => accepts(chars, TDot, start),
        _ => None,
    }
}

fn scan_compare_symbols(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    match ch {
        '<' => {
            _ = chars.next();
            if peek(chars, '=') {
                accepts(chars, TLessEqual, start)
            } else {
                Some((TLess, start..start + 1))
            }
        }
        '>' => {
            _ = chars.next();
            if peek(chars, '=') {
                accepts(chars, TGreaterEqual, start)
            } else {
                Some((TGreater, start..start + 1))
            }
        }
        '=' => {
            _ = chars.next();
            if peek(chars, '=') {
                accepts(chars, TEqualEqual, start)
            } else {
                Some((TEqual, start..start + 1))
            }
        }
        '!' => {
            _ = chars.next();
            if peek(chars, '=') {
                accepts(chars, TBangEqual, start)
            } else {
                Some((TBang, start..start + 1))
            }
        }
        '/' => {
            _ = chars.next();
            if peek(chars, '/') {
                let mut end = start;
                while let Some(&(idx, char)) = chars.peek() {
                    end = idx;
                    if ch == '\n' {
                        break;
                    }
                }
                Some((TIgnore, start..end))
            } else {
                Some((TSlash, start..start + 1))
            }
        }
        _ => None,
    }
}

fn scan_numbers(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    let mut end = start;
    if ch.is_digit(10) {
        while let Some(&(idx, ch)) = chars.peek() {
            if ch.is_digit(10) {
                end = idx;
                chars.next().unwrap();
            } else {
                break;
            }
        }
        if peek(chars, '.') {
            chars.next().unwrap();
            while let Some(&(idx, ch)) = chars.peek() {
                if ch.is_digit(10) {
                    end = idx;
                    chars.next().unwrap();
                } else {
                    break;
                }
            }
        }

        Some((TNumber, start..end + 1))
    } else {
        None
    }
}

fn scan_string(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    let mut end = start + 1;
    if ch == '"' {
        _ = chars.next().unwrap();
        while let Some(&(idx, ch)) = chars.peek() {
            end = idx;
            chars.next().unwrap();
            if ch == '"' {
                break;
            }
        }
        Some((TString, start..end + 1))
    } else {
        None
    }
}

fn scan_identifier(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    let mut end = start + 1;
    if ch.is_alphabetic() || ch == '_' {
        while let Some(&(size, ch)) = chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                end = size;
                _ = chars.next();
            } else {
                break;
            }
        }
        Some((TIdentifier, start..end + 1))
    } else {
        None
    }
}

fn peek(chars: &mut Chars, ch: char) -> bool {
    if let Some(&(_, c)) = chars.peek() {
        ch == c
    } else {
        false
    }
}

fn ignore_whitespace(chars: &mut Chars) -> Option<(TokenType, Range<usize>)> {
    let &(start, ch) = chars.peek()?;
    let mut end = start + 1;
    if ch.is_whitespace() {
        while let Some(&(size, ch)) = chars.peek() {
            if ch.is_whitespace() {
                end = size;
                _ = chars.next();
            } else {
                break;
            }
        }
    }

    Some((TIgnore, start..end + 1))
}

pub fn tokenize(source: Source) -> Result<Tokens, TError> {
    println!("Tokenizing..");
    scan_tokens(source.contents)
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
        let mut tokens = scan_tokens("(){},.+-;*/".to_string()).unwrap();

        assert_eq!(
            tokens.tokens,
            vec![
                Token::new(TLeftParen, "(", 1),
                Token::new(TRightParen, ")", 1),
                Token::new(TLeftBraces, "{", 1),
                Token::new(TRightBraces, "}", 1),
                Token::new(TComma, ",", 1),
                Token::new(TDot, ".", 1),
                Token::new(TPlus, "+", 1),
                Token::new(TMinus, "-", 1),
                Token::new(TSemiColon, ";", 1),
                Token::new(TStar, "*", 1),
                Token::new(TSlash, "/", 1),
                Token::new(TEof, "", 1),
            ]
        )
    }

    #[test]
    fn test_double_character() {
        let mut tokens = scan_tokens("! != < <= > >= == =".to_string()).unwrap();
        assert_eq!(
            tokens.tokens,
            vec![
                Token::new(TBang, "!", 1),
                Token::new(TBangEqual, "!=", 1),
                Token::new(TLess, "<", 1),
                Token::new(TLessEqual, "<=", 1),
                Token::new(TGreater, ">", 1),
                Token::new(TGreaterEqual, ">=", 1),
                Token::new(TEqualEqual, "==", 1),
                Token::new(TEqual, "=", 1),
                Token::new(TEof, "", 1),
            ]
        )
    }

    #[test]
    fn test_string() {
        let mut tokens = scan_tokens("\"Hello\" \"world\"".to_string()).unwrap();
        assert_eq!(
            tokens.tokens,
            vec![
                Token::new(TString, "\"Hello\"", 1,),
                Token::new(TString, "\"world\"", 1,),
                Token::new(TEof, "", 1)
            ]
        )
    }
    #[test]
    fn test_number() {
        let mut tokens = scan_tokens("1234 231.23".to_string()).unwrap();
        assert_eq!(
            tokens.tokens,
            vec![
                Token::new(TNumber, "1234", 1),
                Token::new(TNumber, "231.23", 1),
                Token::new(TEof, "", 1)
            ]
        )
    }

    #[ignore = "not yet"]
    fn test_keywords() {
        let mut tokens = scan_tokens(
            "class and if while else false for fun nil or print return super this true var "
                .to_string(),
        )
        .unwrap();
        assert_eq!(
            tokens.tokens,
            vec![
                Token::new(TClass, "class", 1),
                Token::new(TAnd, "and", 1),
                Token::new(TIf, "if", 1),
                Token::new(TWhile, "while", 1),
                Token::new(TElse, "else", 1),
                Token::new(TFalse, "false", 1),
                Token::new(TFor, "for", 1),
                Token::new(TFun, "fun", 1),
                Token::new(TNil, "nil", 1),
                Token::new(TOr, "or", 1),
                Token::new(TPrint, "print", 1),
                Token::new(TReturn, "return", 1),
                Token::new(TSuper, "super", 1),
                Token::new(TThis, "this", 1),
                Token::new(TTrue, "true", 1),
                Token::new(TVar, "var", 1),
                Token::new(TEof, "", 1),
            ]
        )
    }
}
