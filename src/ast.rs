//! # Abstract Syntax Tree Module
//!
//! This module defines the abstract syntax tree (AST) structures for the Lox language.
//! The AST represents the syntactic structure of Lox programs after parsing,
//! consisting of expressions and statements that can be evaluated by the interpreter.

use std::fmt::Display;

/// The root of the abstract syntax tree for a Lox program.
///
/// Contains a sequence of top-level statements that make up the program.
#[derive(Debug, PartialEq)]
pub struct AST {
    /// The top-level statements of the program
    pub top: Vec<Statements>,
}

/// Binary and unary operators supported in Lox expressions.
///
/// Each variant represents a different operator with its associated symbol.
#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    /// Addition operator `+`
    OAdd,
    /// Subtraction operator `-`
    OSub,
    /// Multiplication operator `*`
    OMul,
    /// Division operator `/`
    ODiv,
    /// Less than operator `<`
    OLt,
    /// Less than or equal operator `<=`
    OLeq,
    /// Greater than operator `>`
    OGt,
    /// Greater than or equal operator `>=`
    OGeq,
    /// Equality operator `==`
    OEq,
    /// Inequality operator `!=`
    ONeq,
    /// Logical not operator `!`
    ONot,
    /// Logical and operator `and`
    OAnd,
    /// Logical or operator `or`
    OOr,
}

impl Display for Operator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Operator::OAdd => "+",
            Operator::OSub => "-",
            Operator::OMul => "*",
            Operator::ODiv => "/",
            Operator::OLt => "<",
            Operator::OLeq => "<=",
            Operator::OGt => ">",
            Operator::OGeq => ">=",
            Operator::OEq => "=",
            Operator::ONeq => "!=",
            Operator::ONot => "!",
            Operator::OAnd => "and",
            Operator::OOr => "or",
        })
    }
}

/// Expression nodes in the abstract syntax tree.
///
/// Expressions evaluate to values and can be nested within other expressions or statements.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Binary operation: `left operator right`
    EBinary {
        /// Left operand expression
        left: Box<Expr>,
        /// Binary operator
        operator: Operator,
        /// Right operand expression
        right: Box<Expr>,
    },
    /// Parenthesized grouping: `(expression)`
    EGrouping {
        /// The expression inside the parentheses
        expression: Box<Expr>,
    },
    /// Numeric literal
    ENum {
        /// String representation of the number
        value: String,
    },
    /// String literal
    EStr {
        /// The string value
        value: String,
    },
    /// Nil literal
    ENil,
    /// Boolean literal
    EBool {
        /// The boolean value
        value: bool,
    },
    /// Unary operation: `operator right`
    EUnary {
        /// Unary operator
        operator: Operator,
        /// Operand expression
        right: Box<Expr>,
    },
    /// Variable reference
    EVariable {
        /// Variable name
        name: String,
    },
    /// Variable assignment: `name = value`
    EAssign {
        /// Variable name being assigned
        name: String,
        /// Expression to assign
        value: Box<Expr>,
    },
}

impl Expr {
    /// Creates a string literal expression.
    ///
    /// # Arguments
    /// * `value` - The string value
    pub fn str(value: impl Into<String>) -> Expr {
        Expr::EStr {
            value: value.into(),
        }
    }

    /// Creates a boolean literal expression.
    ///
    /// # Arguments
    /// * `value` - The boolean value
    pub fn bool(value: bool) -> Expr {
        Expr::EBool { value }
    }

    /// Creates a numeric literal expression.
    ///
    /// # Arguments
    /// * `value` - The numeric value as a string
    pub fn num(value: impl Into<String>) -> Expr {
        Expr::ENum {
            value: value.into(),
        }
    }

    /// Creates a nil literal expression.
    pub fn nil() -> Expr {
        Expr::ENil
    }

    /// Creates a binary expression.
    ///
    /// # Arguments
    /// * `left` - Left operand
    /// * `operator` - Binary operator
    /// * `right` - Right operand
    ///
    /// # Returns
    /// * A new binary expression
    ///
    /// # Examples
    /// ```rust
    /// use crusty::ast::{Expr, Operator};
    ///
    /// let expr = Expr::binary(
    ///     Expr::num("5"),
    ///     Operator::OAdd,
    ///     Expr::num("3")
    /// );
    /// // Represents: 5 + 3
    /// ```
    pub fn binary(left: Expr, operator: Operator, right: Expr) -> Expr {
        Expr::EBinary {
            left: left.into(),
            operator,
            right: right.into(),
        }
    }
    /// Creates a unary expression.
    ///
    /// # Arguments
    /// * `operator` - Unary operator
    /// * `right` - Operand expression
    pub fn unary(operator: Operator, right: Expr) -> Expr {
        Expr::EUnary {
            operator,
            right: right.into(),
        }
    }

    /// Creates a grouping expression.
    ///
    /// # Arguments
    /// * `expression` - The expression to group
    pub fn grouping(expression: Expr) -> Expr {
        Expr::EGrouping {
            expression: expression.into(),
        }
    }

    /// Creates a variable reference expression.
    ///
    /// # Arguments
    /// * `name` - Variable name
    pub fn variable(name: impl Into<String>) -> Expr {
        Expr::EVariable { name: name.into() }
    }

    /// Creates a variable assignment expression.
    ///
    /// # Arguments
    /// * `name` - Variable name
    /// * `value` - Expression to assign
    pub fn assign(name: impl Into<String>, value: Expr) -> Expr {
        Expr::EAssign {
            name: name.into(),
            value: value.into(),
        }
    }
}

pub fn fmt_expr(e: Expr) -> String {
    match e {
        Expr::EBinary {
            left,
            operator,
            right,
        } => {
            format!(
                "({} {} {})",
                fmt_ops(&operator),
                fmt_expr(*left),
                fmt_expr(*right)
            )
        }
        Expr::EGrouping { expression } => {
            format!("(group {})", fmt_expr(*expression))
        }
        Expr::EUnary { operator, right } => {
            format!("({} {})", fmt_ops(&operator), fmt_expr(*right))
        }
        Expr::ENum { value } => {
            format!("{}", value)
        }
        Expr::EStr { value } => {
            format!("{:?}", value)
        }
        Expr::ENil => "nil".to_string(),
        Expr::EBool { value } => {
            format!("{:?}", value)
        }
        Expr::EVariable { name } => format!("{name}"),
        Expr::EAssign { name, value } => format!("assign {} {}", name, fmt_expr(*value)),
    }
}

/// Statement nodes in the abstract syntax tree.
///
/// Statements perform actions and control program flow, unlike expressions which produce values.
#[derive(Debug, Clone, PartialEq)]
pub enum Statements {
    /// Print statement: `print expression;`
    SPrint {
        /// Expression to print
        expression: Expr,
    },
    /// Expression statement: `expression;`
    SExpression {
        /// Expression to evaluate (result discarded)
        expression: Expr,
    },
    /// Variable declaration: `var name = initializer;` or `var name;`
    SVar {
        /// Variable name
        name: String,
        /// Optional initializer expression
        initializer: Option<Expr>,
    },
    /// Block statement: `{ statements }`
    SBlock {
        /// Statements within the block
        statements: Vec<Statements>,
    },
    /// If statement: `if (condition) consequence else alternative`
    SIf {
        /// Condition expression
        condition: Expr,
        /// Statement to execute if condition is true
        consequence: Box<Statements>,
        /// Optional else clause
        alternative: Option<Box<Statements>>,
    },
    /// While loop: `while (condition) consequence`
    SWhile {
        /// Loop condition
        condition: Expr,
        /// Statement to execute while condition is true
        consequence: Box<Statements>,
    },
}

// constructors for statements
impl Statements {
    /// Creates a print statement.
    ///
    /// # Arguments
    /// * `expression` - Expression to evaluate and print
    pub fn print(expression: Expr) -> Statements {
        Statements::SPrint { expression }
    }

    /// Creates an expression statement.
    ///
    /// # Arguments
    /// * `expression` - Expression to evaluate
    pub fn expression(expression: Expr) -> Statements {
        Statements::SExpression { expression }
    }

    /// Creates a variable declaration statement.
    ///
    /// # Arguments
    /// * `name` - Variable name
    /// * `initializer` - Optional initializer expression
    pub fn var(name: impl Into<String>, initializer: Option<Expr>) -> Statements {
        Statements::SVar {
            name: name.into(),
            initializer,
        }
    }

    /// Creates a block statement.
    ///
    /// # Arguments
    /// * `statements` - Statements within the block
    pub fn block(statements: Vec<Statements>) -> Statements {
        Self::SBlock { statements }
    }
    /// Creates an if statement.
    ///
    /// # Arguments
    /// * `condition` - Condition expression
    /// * `consequence` - Statement to execute if true
    /// * `alternative` - Optional else statement
    pub fn if_stmt(
        condition: Expr,
        consequence: Statements,
        alternative: Option<Statements>,
    ) -> Statements {
        Self::SIf {
            condition,
            consequence: consequence.into(),
            alternative: alternative.map(|alt| Box::new(alt)),
        }
    }

    /// Creates a while statement.
    ///
    /// # Arguments
    /// * `condition` - Loop condition
    /// * `consequence` - Statement to execute in loop
    pub fn while_stmt(condition: Expr, consequence: Statements) -> Statements {
        Self::SWhile {
            condition,
            consequence: consequence.into(),
        }
    }
}

fn fmt_ops(operation: &Operator) -> &'static str {
    match operation {
        Operator::OAdd => "+",
        Operator::OSub => "-",
        Operator::OMul => "*",
        Operator::ODiv => "/",
        Operator::OLt => "<",
        Operator::OLeq => "<=",
        Operator::OGt => ">",
        Operator::OGeq => ">=",
        Operator::OEq => "==",
        Operator::ONeq => "!=",
        Operator::ONot => "!",
        Operator::OAnd => "and",
        Operator::OOr => "or",
    }
}

pub fn main() {
    println!("Inside the ast file");
    let expression = Expr::binary(
        Expr::unary(Operator::OSub, Expr::num("123.0")),
        Operator::OMul,
        Expr::grouping(Expr::num("45.67")),
    );

    println!("{}", fmt_expr(expression));
    println!("Outside the ast file");
}
