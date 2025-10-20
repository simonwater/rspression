use crate::values::Instance;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i32),
    Double(f64),
    String(String),
    Boolean(bool),
    Instance(Box<Instance>),
    Null,
}

impl Value {
    pub fn type_code(&self) -> u8 {
        match self {
            Value::Integer(_) => 1,
            Value::Double(_) => 4,
            Value::String(_) => 5,
            Value::Boolean(_) => 6,
            Value::Instance(_) => 7,
            Value::Null => 8,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Boolean(b) => *b,
            Value::String(s) => !s.is_empty(),
            _ => true,
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Integer(_) | Value::Double(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Value::Integer(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Value::Double(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn is_instance(&self) -> bool {
        matches!(self, Value::Instance(_))
    }

    pub fn as_double(&self) -> f64 {
        match self {
            Value::Integer(i) => *i as f64,
            Value::Double(d) => *d,
            _ => 0.0,
        }
    }

    pub fn as_integer(&self) -> i32 {
        match self {
            Value::Integer(i) => *i,
            Value::Double(d) => *d as i32,
            _ => 0,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Value::String(s) => &s,
            _ => "",
        }
    }

    pub fn as_boolean(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            _ => false,
        }
    }

    pub fn as_instance(&self) -> Option<&Instance> {
        match self {
            Value::Instance(i) => Some(i),
            _ => None,
        }
    }

    pub fn as_instance_mut(&mut self) -> Option<&mut Instance> {
        match self {
            Value::Instance(i) => Some(i),
            _ => None,
        }
    }

    pub fn equals(&self, other: &Value) -> bool {
        self == other
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Double(d) => {
                if d.fract() == 0.0 {
                    // 没有小数部分，添加 .0
                    write!(f, "{}.0", d.trunc())
                } else {
                    // 有小数部分，原样显示
                    write!(f, "{}", d)
                }
            }
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Instance(_) => write!(f, "<instance>"),
            Value::Null => write!(f, "null"),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Integer(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Double(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}
