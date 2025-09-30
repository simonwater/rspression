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

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.fields.get(name)
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.fields.insert(name, value);
    }
}
