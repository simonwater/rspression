use crate::values::Value;
use std::collections::HashMap;

pub trait Environment {
    fn before_execute(&mut self, _vars: &[String]) -> bool {
        true
    }
    fn get(&self, name: &str) -> Option<&Value>;
    fn put(&mut self, name: String, value: Value) -> bool;
    fn extend<T: IntoIterator<Item = (String, Value)>>(&mut self, iter: T);
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

    pub fn with_capacity(c: usize) -> Self {
        Self {
            values: HashMap::with_capacity(c),
        }
    }
}

impl Environment for DefaultEnvironment {
    fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }

    fn put(&mut self, name: String, value: Value) -> bool {
        self.values.insert(name, value);
        true
    }

    fn extend<T: IntoIterator<Item = (String, Value)>>(&mut self, iter: T) {
        self.values.extend(iter);
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
