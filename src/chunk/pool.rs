use std::collections::HashMap;

use crate::values::Value;

#[derive(Clone)]
pub struct ConstantPool {
    constants: Vec<Value>,
    index_map: HashMap<String, usize>,
    byte_size: usize,
}

impl ConstantPool {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
            index_map: HashMap::new(),
            byte_size: 0,
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
                    let s = str::from_utf8(s_b).expect("utf8");
                    Value::from(s)
                }
                _ => panic!("unsupported constant tag: {}", tag),
            };
            constants.push(value);
        }

        Self {
            constants,
            index_map: HashMap::new(),
            byte_size: bytes.len(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.byte_size);
        self.append_bytes_to_buf(&mut out);
        out
    }

    pub fn append_bytes_to_buf(&self, buf: &mut Vec<u8>) {
        buf.reserve(self.byte_size);
        for v in &self.constants {
            match v {
                Value::Integer(i) => {
                    buf.push(v.type_code());
                    buf.extend_from_slice(&i.to_be_bytes());
                }
                Value::Double(d) => {
                    buf.push(v.type_code());
                    buf.extend_from_slice(&d.to_bits().to_be_bytes());
                }
                Value::String(s) => {
                    buf.push(v.type_code());
                    let b = s.as_bytes();
                    assert!(b.len() <= u16::MAX as usize);
                    buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
                    buf.extend_from_slice(b);
                }
                _ => panic!("unsupported constant type in pool"),
            }
        }
    }

    pub fn add_const(&mut self, v: Value) -> usize {
        let size = match &v {
            Value::Integer(_) => 5,
            Value::Double(_) => 9,
            Value::String(s) => s.as_bytes().len() + 3, // type: 1, len: 2
            Value::Boolean(_) => 2,
            _ => panic!("unsupported constant value type: {:?}", v.type_code()),
        };

        let key = v.to_string();
        if let Some(idx) = self.index_map.get(&key).copied() {
            return idx;
        }
        self.byte_size += size;

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

    pub fn size(&self) -> usize {
        self.byte_size
    }
}
