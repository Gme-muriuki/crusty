use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment<V> {
    pub vars: HashMap<String, V>,
}

impl<V> Environment<V> {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn declare(&mut self, name: &str, value: V) {
        // Declare new variable...(var name = value)
        self.vars.insert(name.to_string(), value);
    }

    pub fn lookup(&self, name: &str) -> Option<&V> {
        self.vars.get(name)
    }

    pub fn assign(&mut self, value: V, name: &str) {
        // Change value of an already *existent* variable (name = value)
        if self.vars.contains_key(name) {
            self.vars.insert(name.to_string(), value);
        }
    }
}
