use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// A type alias for a reference-counted, mutable environment.
// Rc allows multiple owners (e.g., child scopes pointing to a parent).
// RefCell allows mutable borrowing even with multiple owners.
pub type EnvironmentRef = Rc<RefCell<Environment>>;

// The Environment struct now holds an optional parent pointer.
#[derive(Debug, PartialEq)]
pub struct Environment {
    parent: Option<EnvironmentRef>,
    pub values: HashMap<String, Value>, // Made public for direct mutation after finding environment
}

impl Environment {
    /// Creates a new, top-level (global) environment.
    pub fn new() -> EnvironmentRef {
        Rc::new(RefCell::new(Environment {
            parent: None,
            values: HashMap::new(),
        }))
    }

    /// Creates a new, enclosed environment that points to a parent.
    pub fn new_enclosed(parent: &EnvironmentRef) -> EnvironmentRef {
        Rc::new(RefCell::new(Environment {
            parent: Some(Rc::clone(parent)),
            values: HashMap::new(),
        }))
    }

    /// Defines or re-assigns a variable in the *current* scope.
    /// This allows for variable shadowing.
    pub fn assign(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    /// Gets a variable's value, searching recursively up through parent scopes.
    pub fn get(&self, name: &str) -> Result<Value, String> {
        // Try to get from the current scope first.
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        }

        // If not found, try the parent scope.
        if let Some(parent_ref) = &self.parent {
            return parent_ref.borrow().get(name);
        }

        // If not found in any scope, it's an error.
        Err(format!("Undefined variable '{}'", name))
    }

    /// Finds the EnvironmentRef where a variable is defined, searching recursively up through parent scopes.
    /// Returns None if the variable is not found in any scope.
    pub fn find_environment(rc_self: &EnvironmentRef, name: &str) -> Option<EnvironmentRef> {
        {
            // Borrow the RefCell immutably first to check for key existence.
            let self_borrow = rc_self.borrow();
            if self_borrow.values.contains_key(name) {
                return Some(Rc::clone(rc_self));
            }
        } // The immutable borrow is dropped here.

        // If not found in current, check parent.
        if let Some(parent_ref) = &rc_self.borrow().parent {
            return Environment::find_environment(parent_ref, name);
        }
        None
    }
}
