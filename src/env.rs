use std::collections::HashMap;
use crate::values::Value;

pub trait Environment {
    fn define(&mut self, name: String, value: Value);
    fn get(&self, name: &str) -> Option<&Value>;
    fn assign(&mut self, name: &str, value: Value) -> bool;
    fn get_at(&self, distance: usize, name: &str) -> Option<&Value>;
    fn assign_at(&mut self, distance: usize, name: &str, value: Value) -> bool;
}

#[derive(Default)]
pub struct DefaultEnvironment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<dyn Environment>>,
}

impl DefaultEnvironment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_enclosing(enclosing: Box<dyn Environment>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }
}

impl Environment for DefaultEnvironment {
    fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    fn get(&self, name: &str) -> Option<&Value> {
        if let Some(value) = self.values.get(name) {
            Some(value)
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get(name)
        } else {
            None
        }
    }

    fn assign(&mut self, name: &str, value: Value) -> bool {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            true
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value)
        } else {
            false
        }
    }

    fn get_at(&self, distance: usize, name: &str) -> Option<&Value> {
        if distance == 0 {
            self.values.get(name)
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get_at(distance - 1, name)
        } else {
            None
        }
    }

    fn assign_at(&mut self, distance: usize, name: &str, value: Value) -> bool {
        if distance == 0 {
            self.values.insert(name.to_string(), value);
            true
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign_at(distance - 1, name, value)
        } else {
            false
        }
    }
}
