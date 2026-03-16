# Crusty: A Lox Interpreter in Rust

Crusty is a complete implementation of the Lox programming language interpreter, written in Rust. This project is based on the "Crafting Interpreters" book by Robert Nystrom, adapting the Java-based interpreter into idiomatic Rust code. The interpreter supports both interactive REPL mode and file execution, featuring a clean modular architecture with proper error handling and lexical scoping.

## Features

Crusty implements a substantial subset of the Lox language specification:

- **Variables and Scoping**: Variable declaration, assignment, and lexical scoping with block-level isolation
- **Data Types**: Numbers (64-bit floats), strings, booleans, and nil
- **Expressions**: Arithmetic, comparison, and logical operations with proper operator precedence
- **Control Flow**: Conditional statements (`if`/`else`) and loops (`while`)
- **I/O**: Print statements for output
- **Error Handling**: Comprehensive error reporting for syntax, runtime, and I/O errors
- **REPL Support**: Interactive mode for testing expressions and statements

## Architecture

The interpreter follows a traditional compiler pipeline, organized into modular components:

```
Source Code → Reader → Tokenizer → Parser → AST → Evaluator
```

### Core Modules

- **`reader.rs`**: Handles file I/O operations, reading source code into memory
- **`tokenize.rs`**: Lexical analysis - converts source text into tokens (keywords, operators, literals, identifiers)
- **`ast.rs`**: Abstract Syntax Tree definitions for expressions and statements
- **`parser.rs`**: Syntax analysis using recursive descent parsing with proper precedence handling
- **`environ.rs`**: Variable environment management with support for nested scopes
- **`evaluate.rs`**: Runtime execution engine that evaluates expressions and executes statements
- **`error.rs`**: Unified error handling system with detailed error messages and line number reporting
- **`main.rs`**: Entry point supporting both REPL and file execution modes

### Key Design Decisions

- **Pure Rust**: No external dependencies, relying solely on the standard library
- **Type Safety**: Strong typing throughout, with custom types for tokens, AST nodes, and runtime values
- **Error Propagation**: Uses Rust's `Result` type and the `?` operator for clean error handling
- **Memory Management**: Leverages Rust's ownership system and reference counting for environment scoping
- **Modular Design**: Each phase of interpretation is isolated in its own module for maintainability

## Usage

### Prerequisites

- Rust 1.70 or later (edition 2021)
- Cargo package manager

### Building

Clone the repository and build with Cargo:

```bash
git clone <repository-url>
cd crusty
cargo build --release
```

### Running

#### Interactive REPL

Run without arguments for interactive mode:

```bash
./target/release/crusty
```

In REPL mode, enter Lox expressions and statements at the `>` prompt. Use Ctrl+C to exit.

#### Execute a File

Run with a Lox source file:

```bash
./target/release/crusty program.lox
```

### Example Programs

#### Hello World

```lox
print "Hello, World!";
```

#### Variables and Arithmetic

```lox
var a = 10;
var b = 20;
print a + b;  // Prints: 30
```

#### Control Flow

```lox
var x = 10;
if (x > 5) {
    print "x is greater than 5";
} else {
    print "x is not greater than 5";
}

var i = 0;
while (i < 3) {
    print i;
    i = i + 1;
}
```

#### Scoping

```lox
var a = "global";
{
    var a = "local";
    print a;  // Prints: local
}
print a;  // Prints: global
```

#### Functions (Future Feature)

```lox
fun fibonacci(n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

print fibonacci(10);  // Would print: 55
```

## Development

### Project Structure

```
crusty/
├── Cargo.toml          # Project configuration
├── src/
│   ├── main.rs         # Entry point
│   ├── reader.rs       # File I/O
│   ├── tokenize.rs     # Lexical analysis
│   ├── tokenizer.rs    # Alternative tokenizer (unused)
│   ├── ast.rs          # AST definitions
│   ├── parser.rs       # Syntax analysis
│   ├── environ.rs      # Environment/scoping
│   ├── evaluate.rs     # Runtime evaluation
│   └── error.rs        # Error handling
├── target/             # Build artifacts
└── somefile.lox        # Example Lox program
```

### Testing

Run the test suite:

```bash
cargo test
```

The tokenizer module includes comprehensive unit tests covering various token types and edge cases.

### Code Quality

- Follows Rust idioms and best practices
- Comprehensive error handling with meaningful messages
- Modular architecture for easy maintenance and extension
- Well-documented code with inline comments

## Future Enhancements

While Crusty implements core Lox features, potential extensions include:

- Function definitions and calls
- Class-based object orientation
- Array/list data structures
- Additional control flow (for loops, break/continue)
- Enhanced standard library functions

## License

This project is open source. See LICENSE file for details.

## Acknowledgments

This implementation is heavily inspired by the "Crafting Interpreters" book by Robert Nystrom. The modular Rust architecture demonstrates how language implementation concepts translate across programming languages while leveraging Rust's unique strengths in memory safety and performance.</content>
<parameter name="filePath">c:\Users\James\Documents\Rust\interpreter\crusty\README.md