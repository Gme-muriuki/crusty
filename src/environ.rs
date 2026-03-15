use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Environment<V> {
    pub parent: Option<Rc<Environment<V>>>,
    pub vars: RefCell<HashMap<String, V>>,
}

impl<V: Clone> Environment<V> {
    pub fn new(parent: Option<Rc<Environment<V>>>) -> Rc<Self> {
        Rc::new(Self {
            parent,
            vars: RefCell::new(HashMap::new()),
        })
    }

    pub fn declare(&self, name: &str, value: V) -> Option<V> {
        // Declare new variable...(var name = value)
        self.vars.borrow_mut().insert(name.to_string(), value)
    }

    pub fn lookup(&self, name: &str) -> Option<V> {
        Some(self.vars.borrow().get(name)?.clone())
    }

    pub fn assign(&self, value: V, name: &str) {
        if self.vars.borrow_mut().contains_key(name) {
            self.vars.borrow_mut().insert(name.into(), value);
        }
    }
}
