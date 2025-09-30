use std::collections::HashMap;
use crate::values::Value;

pub trait Callable {
    fn call(&self, arguments: Vec<Value>) -> Value;
    fn arity(&self) -> usize;
}

pub struct Function {
    pub name: String,
    pub arity: usize,
    pub body: fn(Vec<Value>) -> Value,
}

impl Callable for Function {
    fn call(&self, arguments: Vec<Value>) -> Value {
        if arguments.len() != self.arity {
            panic!("Expected {} arguments but got {}", self.arity, arguments.len());
        }
        (self.body)(arguments)
    }

    fn arity(&self) -> usize {
        self.arity
    }
}

pub struct FunctionManager {
    functions: HashMap<String, Box<dyn Callable>>,
}

impl FunctionManager {
    pub fn new() -> Self {
        let mut manager = Self {
            functions: HashMap::new(),
        };
        manager.register_builtins();
        manager
    }

    pub fn register(&mut self, name: String, callable: Box<dyn Callable>) {
        self.functions.insert(name, callable);
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Callable>> {
        self.functions.get(name)
    }

    fn register_builtins(&mut self) {
        // Register built-in functions
        self.register("clock".to_string(), Box::new(ClockFunction));
        self.register("abs".to_string(), Box::new(AbsFunction));
    }
}

// Built-in functions
pub struct ClockFunction;

impl Callable for ClockFunction {
    fn call(&self, _arguments: Vec<Value>) -> Value {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        Value::Double(duration)
    }

    fn arity(&self) -> usize {
        0
    }
}

pub struct AbsFunction;

impl Callable for AbsFunction {
    fn call(&self, arguments: Vec<Value>) -> Value {
        if let Some(value) = arguments.get(0) {
            match value {
                Value::Integer(i) => Value::Integer(i.abs()),
                Value::Double(d) => Value::Double(d.abs()),
                _ => Value::Null,
            }
        } else {
            Value::Null
        }
    }

    fn arity(&self) -> usize {
        1
    }
}
