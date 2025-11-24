use crate::value::Value;
use std::collections::HashMap;

// A simple environment using a HashMap.
// For proper scoping, we'd later need a chain of environments (parent pointer).
#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    // Define a new variable or re-assign an existing one.
    pub fn assign(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    // Get the value of a variable.
    pub fn get(&self, name: &str) -> Result<Value, String> {
        self.values
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Undefined variable '{}'", name))
    }
}
