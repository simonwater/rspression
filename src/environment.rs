use crate::values::Value;
use std::borrow::Cow;
use std::collections::HashMap;

pub trait Environment {
    fn before_execute(&mut self, _vars: &[String]) -> bool {
        true
    }
    fn get(&self, name: &str) -> Option<&Value>;
    fn put(&mut self, name: Cow<'_, str>, value: Value) -> bool;
    fn extend(&mut self, iter: &mut dyn Iterator<Item = (String, Value)>);
    fn size(&self) -> usize;
}

#[derive(Clone)]
pub struct DefaultEnvironment {
    map: HashMap<String, Value>,
}

impl DefaultEnvironment {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn with_capacity(c: usize) -> Self {
        Self {
            map: HashMap::with_capacity(c),
        }
    }
}

impl Environment for DefaultEnvironment {
    fn get(&self, name: &str) -> Option<&Value> {
        self.map.get(name)
    }

    fn put(&mut self, name: Cow<'_, str>, value: Value) -> bool {
        if let Some(old_value) = self.map.get_mut(name.as_ref()) {
            *old_value = value;
        } else {
            self.map.insert(name.into_owned(), value);
        }
        true
    }

    fn extend(&mut self, iter: &mut dyn Iterator<Item = (String, Value)>) {
        self.map.extend(iter);
    }

    fn size(&self) -> usize {
        self.map.len()
    }
}

impl Default for DefaultEnvironment {
    fn default() -> Self {
        Self::new()
    }
}
