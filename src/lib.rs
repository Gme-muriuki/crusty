//! # Crusty Lox Interpreter
//!
//! A complete implementation of the Lox programming language interpreter in Rust,
//! based on Robert Nystrom's "Crafting Interpreters" book.
//!
//! ## Features
//!
//! - Full Lox language support (variables, control flow, expressions)
//! - Interactive REPL mode
//! - File execution
//! - Comprehensive error reporting
//! - Lexical scoping
//!
//! ## Example
//!
//! ```rust,ignore
//! use crusty::{Interpreter, Source};
//!
//! let mut interpreter = Interpreter::new();
//! let source = Source::new(r#"
//!     var x = 42;
//!     print x;
//! "#.to_string());
//!
//! // interpreter.evaluate(source); // Would execute the code
//! ```

// Re-export public API for library usage
pub use ast::*;
pub use environ::*;
pub use error::*;
pub use evaluate::*;
pub use parser::*;
pub use reader::*;
pub use tokenize::*;

// Include all modules
pub mod ast;
pub mod environ;
pub mod error;
pub mod evaluate;
pub mod parser;
pub mod reader;
pub mod tokenize;
pub mod tokenizer;