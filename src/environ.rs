//! # Environment Module
//!
//! This module implements lexical scoping for variable storage and lookup.
//! It provides an Environment struct that manages variable bindings with
//! support for nested scopes through parent-child relationships.
//! Variables are stored in hash maps with reference counting for shared access.

use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// A lexical environment for variable storage and scoping.
///
/// Environments form a tree structure where child environments
/// can access variables from their parents, implementing lexical scoping.
/// Uses Rc<RefCell<>> for shared mutable access to variable bindings.
#[derive(Debug, Clone)]
pub struct Environment<V> {
    /// Optional parent environment for scope chaining
    pub parent: Option<Rc<Environment<V>>>,
    /// Variable bindings in this scope
    pub vars: RefCell<HashMap<String, V>>,
}

impl<V: Clone> Environment<V> {
    /// Creates a new environment with an optional parent.
    ///
    /// # Arguments
    /// * `parent` - Parent environment for scope lookup, or None for global scope
    ///
    /// # Returns
    /// * A new Rc-wrapped Environment
    pub fn new(parent: Option<Rc<Environment<V>>>) -> Rc<Self> {
        Rc::new(Self {
            parent,
            vars: RefCell::new(HashMap::new()),
        })
    }

    /// Declares a new variable in the current scope.
    ///
    /// # Arguments
    /// * `name` - Variable name
    /// * `value` - Variable value
    ///
    /// # Returns
    /// * Previous value if variable was already declared, None otherwise
    pub fn declare(&self, name: &str, value: V) -> Option<V> {
        // Declare new variable...(var name = value)
        self.vars.borrow_mut().insert(name.to_string(), value)
    }

    /// Looks up a variable by name, searching parent scopes if necessary.
    ///
    /// # Arguments
    /// * `name` - Variable name to look up
    ///
    /// # Returns
    /// * Some(value) if found, None if not found in any scope
    ///
    /// # Examples
    /// ```rust
    /// use crusty::environ::Environment;
    ///
    /// let env = Environment::new(None);
    /// env.declare("x", 42);
    /// assert_eq!(env.lookup("x"), Some(42));
    /// assert_eq!(env.lookup("y"), None);
    /// ```
    pub fn lookup(&self, name: &str) -> Option<V> {
        // Lookup value of a variable might not exist.
        // TODO... Must check parent.
        if let Some(value) = self.vars.borrow().get(name) {
            Some(value.clone())
        } else if let Some(ref parent) = self.parent {
            parent.lookup(name)
        } else {
            return None;
        }
    }

    /// Assigns a new value to an existing variable, searching parent scopes.
    ///
    /// # Arguments
    /// * `value` - New value to assign
    /// * `name` - Variable name
    ///
    /// # Returns
    /// * Some(value) if assignment succeeded, None if variable not found
    pub fn assign(&self, value: V, name: &str) -> Option<V> {
        if self.vars.borrow_mut().contains_key(name) {
            self.vars.borrow_mut().insert(name.into(), value.clone());
            Some(value)
        } else if let Some(ref parent) = self.parent {
            parent.assign(value, name)
        } else {
            None
        }
    }
}
