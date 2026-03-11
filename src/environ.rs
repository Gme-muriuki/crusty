use std::collections::HashMap;

#[derive(Debug)]
struct Environment<V> {
    vars: HashMap<String, V>,
}

impl<V> Environment<V> {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn declare(&mut self, name: &str, value: V) {
        // Declare new variable...(var name = value)
        todo!()
    }

    pub fn lookup(&self, name: &str) {
        // Lookup of a variable... ()
        todo!()
    }

    pub fn assign(&self, value: V, name: &str) {
        // Change value of an already *existent* variable (name = value)
        todo!()
    }
}
