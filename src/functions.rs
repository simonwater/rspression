use crate::Environment;
use crate::RspError;
use crate::RspResult;
use crate::values::Value;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Callable {
    fn call(&self, arguments: Vec<Value>, env: &mut dyn Environment) -> RspResult<Value>;
}

pub enum _Arity {
    Fixed(usize),         // 固定参数，如 IF(c, t, e) 是 Fixed(3)
    Variadic,             // 变长参数，如 SUM(...)
    MinMax(usize, usize), // 范围变长，如 COUNTIF 最少1个最多2个
}

pub struct FunctionManager {
    functions: HashMap<String, Rc<dyn Callable>>,
}

impl FunctionManager {
    pub fn new() -> Self {
        let mut manager = Self {
            functions: HashMap::new(),
        };
        manager.register_builtins();
        manager
    }

    pub fn register(&mut self, name: String, callable: Rc<dyn Callable>) {
        self.functions.insert(name, callable);
    }

    pub fn get(&self, name: &str) -> Option<Rc<dyn Callable>> {
        self.functions.get(name).map(Rc::clone)
    }

    fn register_builtins(&mut self) {
        // Register built-in functions
        self.register("clock".to_string(), Rc::new(ClockFunction));
        self.register("abs".to_string(), Rc::new(AbsFunction));
    }
}

// Built-in functions
pub struct ClockFunction;

impl Callable for ClockFunction {
    fn call(&self, _arguments: Vec<Value>, _: &mut dyn Environment) -> RspResult<Value> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        Ok(Value::Double(duration))
    }
}

pub struct AbsFunction;

impl Callable for AbsFunction {
    fn call(&self, arguments: Vec<Value>, _: &mut dyn Environment) -> RspResult<Value> {
        if let Some(value) = arguments.get(0) {
            match value {
                Value::Integer(i) => Ok(Value::from(i.abs())),
                Value::Double(d) => Ok(Value::from(d.abs())),
                _ => Err(RspError::CallableError {
                    message: format!("Value: {} can not call abs function", value),
                }),
            }
        } else {
            return Err(RspError::CallableError {
                message: format!("abs function must take 1 argument"),
            });
        }
    }
}
