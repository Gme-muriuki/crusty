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
        if self.vars.contains_key(name) {
            self.vars.get(name)
        } else {
            //TODO... I think here I can do sth like: throw new RuntimeError(name,"Undefined variable '" + name.lexeme + "'.");
            None
        }
    }

    pub fn assign(&mut self, value: V, name: &str) {
        // Change value of an already *existent* variable (name = value)
        if self.vars.contains_key(name) {
            self.vars.insert(name.to_string(), value);
        }
        //TODO... Also here..  throw new RuntimeError(name,"Undefined variable '" + name.lexeme + "'.");
    }
}
