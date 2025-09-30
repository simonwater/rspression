use std::collections::HashMap;

use crate::values::Value;

#[derive(Clone, Debug)]
pub struct ConstantPool {
    constants: Vec<Value>,
    index_map: HashMap<String, usize>,
}

impl ConstantPool {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
            index_map: HashMap::new(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut constants = Vec::new();
        let mut i = 0usize;
        while i < bytes.len() {
            let tag = bytes[i];
            i += 1;
            let value = match tag {
                1 => {
                    // Integer
                    let b = &bytes[i..i + 4];
                    i += 4;
                    let v = i32::from_be_bytes([b[0], b[1], b[2], b[3]]);
                    Value::Integer(v)
                }
                4 => {
                    // Double
                    let b = &bytes[i..i + 8];
                    i += 8;
                    let v = f64::from_bits(u64::from_be_bytes([
                        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
                    ]));
                    Value::Double(v)
                }
                5 => {
                    // String
                    let len_b = &bytes[i..i + 2];
                    i += 2;
                    let len = u16::from_be_bytes([len_b[0], len_b[1]]) as usize;
                    let s_b = &bytes[i..i + len];
                    i += len;
                    let s = String::from_utf8(s_b.to_vec()).expect("utf8");
                    Value::String(s)
                }
                _ => panic!("unsupported constant tag: {}", tag),
            };
            constants.push(value);
        }
        Self {
            constants,
            index_map: HashMap::new(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // Match Java ByteBuffer default big-endian encoding
        let mut out = Vec::new();
        for v in &self.constants {
            match v {
                Value::Integer(i) => {
                    out.push(v.type_code());
                    out.extend_from_slice(&i.to_be_bytes());
                }
                Value::Double(d) => {
                    out.push(v.type_code());
                    out.extend_from_slice(&d.to_bits().to_be_bytes());
                }
                Value::String(s) => {
                    out.push(v.type_code());
                    let b = s.as_bytes();
                    assert!(b.len() <= u16::MAX as usize);
                    out.extend_from_slice(&(b.len() as u16).to_be_bytes());
                    out.extend_from_slice(b);
                }
                _ => panic!("unsupported constant type in pool"),
            }
        }
        out
    }

    pub fn add_const(&mut self, v: Value) -> usize {
        let key = v.to_string();
        if let Some(idx) = self.index_map.get(&key).copied() {
            return idx;
        }
        match v {
            Value::Integer(_) | Value::Double(_) | Value::String(_) | Value::Boolean(_) => {}
            _ => panic!("unsupported constant value type: {:?}", v.type_code()),
        }
        self.constants.push(v);
        let idx = self.constants.len() - 1;
        self.index_map.insert(key, idx);
        idx
    }

    pub fn read_const(&self, index: usize) -> &Value {
        &self.constants[index]
    }

    pub fn all(&self) -> &Vec<Value> {
        &self.constants
    }
}
