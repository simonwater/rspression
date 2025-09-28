use std::collections::HashMap;
use crate::value::Value;
use crate::error::LoxResult;

pub trait Environment {
    fn before_execute(&mut self, _vars: &[String]) -> LoxResult<bool> {
        Ok(true)
    }
    
    fn get(&self, name: &str) -> LoxResult<Value>;
    fn get_or_default(&self, name: &str, default: Value) -> LoxResult<Value>;
    fn put(&mut self, name: String, value: Value) -> LoxResult<()>;
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
    fn get(&self, name: &str) -> LoxResult<Value> {
        Ok(self.values.get(name).cloned().unwrap_or(Value::Null))
    }
    
    fn get_or_default(&self, name: &str, default: Value) -> LoxResult<Value> {
        Ok(self.values.get(name).cloned().unwrap_or(default))
    }
    
    fn put(&mut self, name: String, value: Value) -> LoxResult<()> {
        self.values.insert(name, value);
        Ok(())
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
