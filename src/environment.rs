use crate::values::Value;
use std::collections::HashMap;

pub trait Environment {
    fn before_execute(&mut self, _vars: &[String]) -> bool {
        true
    }

    fn get(&self, name: &str) -> Option<&Value>;
    fn get_or_default<'a>(&'a self, name: &str, default: &'a Value) -> Option<&'a Value>;
    fn put(&mut self, name: String, value: Value) -> bool;
    fn size(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct DefaultEnvironment {
    values: HashMap<String, Value>,
}

impl DefaultEnvironment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}

impl Environment for DefaultEnvironment {
    fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }

    fn get_or_default<'a>(&'a self, name: &str, default: &'a Value) -> Option<&'a Value> {
        Some(self.values.get(name).unwrap_or(default))
    }

    fn put(&mut self, name: String, value: Value) -> bool {
        self.values.insert(name, value);
        true
    }

    fn size(&self) -> usize {
        self.values.len()
    }
}

impl Default for DefaultEnvironment {
    fn default() -> Self {
        Self::new()
    }
}
