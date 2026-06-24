use crate::{RspError, RspResult, functions::Callable};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub enum Value {
    Integer(i32),
    Double(f64),
    String(Rc<str>),
    Boolean(bool),
    Object(Rc<RefCell<HashMap<String, Value>>>),
    Function(Rc<dyn Callable>),
    Null,
}

impl Value {
    pub fn type_code(&self) -> u8 {
        match self {
            Value::Integer(_) => 1,
            Value::Double(_) => 4,
            Value::String(_) => 5,
            Value::Boolean(_) => 6,
            Value::Object(_) => 7,
            Value::Null => 8,
            Value::Function(_) => 9,
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

    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Value::Function(_))
    }

    pub fn as_double(&self) -> f64 {
        match self {
            Value::Integer(i) => *i as f64,
            Value::Double(d) => *d,
            _ => 0.0,
        }
    }

    pub fn to_double(&self) -> RspResult<f64> {
        match self {
            Value::Integer(i) => Ok(*i as f64),
            Value::Double(d) => Ok(*d),
            _ => Err(RspError::TypeError {
                message: format!("Expected type is Double, but got: {:?}", self),
            }),
        }
    }

    pub fn as_integer(&self) -> i32 {
        match self {
            Value::Integer(i) => *i,
            Value::Double(d) => *d as i32,
            _ => 0,
        }
    }

    pub fn to_integer(&self) -> RspResult<i32> {
        match self {
            Value::Integer(i) => Ok(*i),
            Value::Double(d) => Ok(*d as i32),
            _ => Err(RspError::TypeError {
                message: format!("Expected type is Integer, but got: {:?}", self),
            }),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Value::String(s) => s,
            _ => "",
        }
    }

    pub fn to_str(&self) -> RspResult<&str> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(RspError::TypeError {
                message: format!("Expected type is String, but got: {:?}", self),
            }),
        }
    }

    pub fn as_boolean(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            _ => false,
        }
    }

    pub fn to_boolean(&self) -> RspResult<bool> {
        match self {
            Value::Boolean(b) => Ok(*b),
            _ => Err(RspError::TypeError {
                message: format!("Expected type is Boolean, but got: {:?}", self),
            }),
        }
    }

    pub fn as_object(&self) -> Option<Rc<RefCell<HashMap<String, Value>>>> {
        match self {
            Value::Object(o) => Some(o.clone()),
            _ => None,
        }
    }

    pub fn to_object(&self) -> RspResult<Rc<RefCell<HashMap<String, Value>>>> {
        match self {
            Value::Object(o) => Ok(o.clone()),
            _ => Err(RspError::TypeError {
                message: format!("Expected type is Object, but got: {:?}", self),
            }),
        }
    }

    pub fn as_function(&self) -> Option<Rc<dyn Callable>> {
        match self {
            Value::Function(func) => Some(func.clone()),
            _ => None,
        }
    }

    pub fn to_function(&self) -> RspResult<Rc<dyn Callable>> {
        match self {
            Value::Function(func) => Ok(func.clone()),
            _ => Err(RspError::TypeError {
                message: format!("Expected type is Function, but got: {:?}", self),
            }),
        }
    }

    pub fn equals(&self, other: &Value) -> bool {
        self == other
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Double(a), Value::Double(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => Rc::ptr_eq(a, b),
            (Value::Function(a), Value::Function(b)) => Rc::ptr_eq(a, b),
            (Value::Null, Value::Null) => true,
            _ => false, // 类型不同，一律不相等
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(v) => f.debug_tuple("Integer").field(v).finish(),
            Value::Double(v) => f.debug_tuple("Double").field(v).finish(),
            Value::String(v) => f.debug_tuple("String").field(v).finish(),
            Value::Boolean(v) => f.debug_tuple("Boolean").field(v).finish(),
            Value::Object(v) => {
                if let Ok(map) = v.try_borrow() {
                    f.debug_tuple("Object").field(&*map).finish()
                } else {
                    f.write_str("Object(<borrowed>)")
                }
            }
            Value::Function(_) => f.write_str("Function"),
            Value::Null => f.write_str("Null"),
        }
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
            Value::Object(_) => write!(f, "<Object>"),
            Value::Null => write!(f, "null"),
            Value::Function(_) => write!(f, "<function>"),
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
        Value::String(Rc::from(value))
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(Rc::from(value))
    }
}

impl From<&String> for Value {
    fn from(value: &String) -> Self {
        Value::String(Rc::from(value.as_ref()))
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl<T: Callable + 'static> From<T> for Value {
    fn from(value: T) -> Self {
        Value::Function(Rc::new(value))
    }
}
