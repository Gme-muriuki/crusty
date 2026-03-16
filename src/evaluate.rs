//! # Evaluation Module
//!
//! This module implements the runtime execution engine for the Lox interpreter.
//! It evaluates expressions and executes statements, managing the global environment
//! and handling control flow. The evaluator implements Lox's runtime semantics
//! including operator evaluation, variable lookup/assignment, and statement execution.

use std::{fmt::Display, rc::Rc};

use crate::{
    ast::{
        AST, Expr,
        Operator::{self, *},
        Statements,
    },
    environ::Environment,
};

/// Type alias for runtime values produced by evaluation.
pub type Output = LoxValue;

/// Type alias for the environment storing Lox values.
pub type Env = Environment<LoxValue>;

/// The main interpreter that executes Lox programs.
///
/// Maintains the top-level environment and coordinates the evaluation
/// of statements and expressions.
///
/// # Examples
/// ```rust
/// use crusty::evaluate::Interpreter;
/// use crusty::ast::AST;
///
/// let mut interpreter = Interpreter::new();
/// // interpreter.evaluate(ast); // Would evaluate an AST
/// ```
#[derive(Debug, Clone)]
pub struct Interpreter {
    /// The global environment for variable storage
    top_level: Rc<Env>,
}

impl Interpreter {
    /// Creates a new interpreter with an empty global environment.
    pub fn new() -> Self {
        Self {
            top_level: Environment::new(None),
        }
    }

    /// Evaluates an AST by executing all top-level statements.
    ///
    /// # Arguments
    /// * `ast` - The abstract syntax tree to evaluate
    ///
    /// # Returns
    /// * `Ok(())` on successful execution, `Err(EvaluateError)` on runtime error
    pub fn evaluate(&mut self, ast: AST) -> Result<(), EvaluateError> {
        execute_statements(ast.top, self.top_level.clone())?;

        Ok(())
    }
}

/// Runtime values in the Lox language.
///
/// Lox has four value types: nil, booleans, numbers, and strings.
/// Numbers are stored as 64-bit floats.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum LoxValue {
    /// The nil value (Lox's null/undefined)
    LNill,
    /// Boolean values
    LBoolean(bool),
    /// Numeric values (stored as f64)
    LNumber(f64),
    /// String values
    LString(String),
}

impl Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxValue::LNill => f.write_str("nil"),
            LoxValue::LBoolean(value) => f.write_str(&format!("{value}")),
            LoxValue::LNumber(value) => f.write_str(&format!("{value}")),
            LoxValue::LString(value) => f.write_str(value),
        }
    }
}

#[derive(Debug)]
pub enum EvaluateError {
    ZeroDivision,
    UnsupportedBinOps(LoxValue, Operator, LoxValue),
    UnsupportedUnaryOps(Operator, LoxValue),
    NotFound(String),
}

impl LoxValue {
    pub fn is_truthy(&self) -> bool {
        match &self {
            LoxValue::LNill | LoxValue::LBoolean(false) => false,
            _ => true,
        }
    }
}

pub fn evaluate(ast: AST) -> Result<(), EvaluateError> {
    println!("Evaluating....");
    let environ = Environment::new(None);
    execute_statements(ast.top, environ)?;
    Ok(())
}

pub fn evaluate_expression(expr: &Expr, environ: Rc<Env>) -> Result<Output, EvaluateError> {
    Ok(match expr {
        Expr::EBinary {
            left,
            operator,
            right,
        } => {
            let left = evaluate_expression(&left, environ.clone())?;
            let right = evaluate_expression(&right, environ.clone())?;

            match (&left, operator, &right) {
                // Numeric ops
                (LoxValue::LNumber(left), OAdd, LoxValue::LNumber(right)) => {
                    LoxValue::LNumber(left + right)
                }
                (LoxValue::LNumber(left), OSub, LoxValue::LNumber(right)) => {
                    LoxValue::LNumber(left - right)
                }
                (LoxValue::LNumber(left), ODiv, LoxValue::LNumber(right)) => {
                    if *right == 0.0 {
                        return Err(EvaluateError::ZeroDivision);
                    }
                    LoxValue::LNumber(left / right)
                }
                (LoxValue::LNumber(left), OMul, LoxValue::LNumber(right)) => {
                    LoxValue::LNumber(left * right)
                }
                // String ops
                (LoxValue::LString(left), OAdd, LoxValue::LString(right)) => {
                    LoxValue::LString(format!("{left} {right}"))
                }
                // Equality works with any combination of values
                (left, OLt, right) => LoxValue::LBoolean(left < right),
                (left, OLeq, right) => LoxValue::LBoolean(left <= right),
                (left, OGt, right) => LoxValue::LBoolean(left > right),
                (left, OGeq, right) => LoxValue::LBoolean(left >= right),
                (left, OEq, right) => LoxValue::LBoolean(left == right),
                (left, ONeq, right) => LoxValue::LBoolean(left != right),
                _ => {
                    return Err(EvaluateError::UnsupportedBinOps(
                        left,
                        operator.clone(),
                        right,
                    ));
                }
            }
        }
        Expr::EGrouping { expression } => evaluate_expression(&expression, environ)?,
        Expr::ENum { value } => Output::LNumber(value.parse().unwrap()),
        Expr::EStr { value } => Output::LString(value.clone()),
        Expr::ENil => Output::LNill,
        Expr::EBool { value } => Output::LBoolean(*value),
        Expr::EUnary { operator, right } => {
            let rv = evaluate_expression(&right, environ)?;
            match (operator, &rv) {
                (OSub, LoxValue::LNumber(right)) => LoxValue::LNumber(-right),
                (ONot, right) => LoxValue::LBoolean(!right.is_truthy()),
                _ => return Err(EvaluateError::UnsupportedUnaryOps(operator.clone(), rv)),
            }
        }

        // !FIXME!!
        Expr::EVariable { name } => {
            if let Some(value) = environ.lookup(name) {
                value
            } else {
                return Err(EvaluateError::NotFound(name.clone()));
            }
        }

        // !FIXME!!
        Expr::EAssign { name, value } => {
            let value = evaluate_expression(&*value, environ.clone())?;
            if let Some(value) = environ.assign(value.clone(), name) {
                value
            } else {
                return Err(EvaluateError::NotFound(name.clone()));
            }
        }
    })
}

pub fn execute_statements(
    statements: Vec<Statements>,
    environ: Rc<Env>,
) -> Result<(), EvaluateError> {
    // Evaluate a sequence of statements, which can include print statements, variable declarations, etc.
    for stmt in statements.iter() {
        execute_statement(stmt, environ.clone())?
    }
    Ok(())
}

pub fn execute_statement(statement: &Statements, environ: Rc<Env>) -> Result<(), EvaluateError> {
    // Evaluate a single statement, which can be a print statement, a variable declaration, etc.
    match statement {
        Statements::SPrint { expression } => {
            let value = evaluate_expression(expression, environ)?;
            println!("{}", value);
        }
        Statements::SExpression { expression } => {
            evaluate_expression(expression, environ)?;
        }
        Statements::SVar { name, initializer } => {
            let value = match initializer {
                Some(value) => evaluate_expression(value, environ.clone())?,
                None => LoxValue::LNill,
            };

            environ.declare(name, value);
        }
        Statements::SBlock { statements } => {
            let new_env = Environment::new(Some(environ));
            execute_statements(statements.clone(), new_env)?;
        }
        Statements::SWhile {
            condition,
            consequence,
        } => {
            while evaluate_expression(condition, environ.clone())?.is_truthy() {
                execute_statement(consequence, environ.clone())?;
            }
        }
        Statements::SIf {
            condition,
            consequence,
            alternative,
        } => {
            if evaluate_expression(condition, environ.clone())?.is_truthy() {
                execute_statement(consequence, environ.clone())?;
            } else if let Some(alternative) = alternative {
                execute_statement(alternative, environ.clone())?;
            }
        }
    }

    Ok(()) // statements do not produce values.
}

#[cfg(test)]
mod test {
    use crate::evaluate::Interpreter;
    use crate::reader::Source;

    #[test]
    fn test_evaluate() {
        let source = Source::new(
            r#"
              print "Hello, World!";
              var x = 10;
              print x + 5;
          "#
            .to_string(),
        );

        let ast = crate::parser::parse(crate::tokenize(source).unwrap()).unwrap();
        let mut interpreter = Interpreter::new();
        assert!(interpreter.evaluate(ast).is_ok());
    }
    #[test]
    fn is_alive() {
        assert_eq!(true, true)
    }
}
