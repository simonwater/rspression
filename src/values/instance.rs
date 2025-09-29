use crate::values::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    pub fields: std::collections::HashMap<String, Value>,
}

impl Instance {
    pub fn new() -> Self {
        Self {
            fields: std::collections::HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Value {
        self.fields.get(name).cloned().unwrap_or(Value::Null)
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.fields.insert(name, value);
    }
}
