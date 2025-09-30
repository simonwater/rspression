use bitvec::prelude::*;

use crate::{values::Value, vm::OpCode};

use super::Chunk;
use super::pool::ConstantPool;

pub struct ChunkWriter {
    code: Vec<u8>,
    pool: ConstantPool,
    is_var_const: BitVec<u8, Msb0>,
}

impl ChunkWriter {
    pub fn new() -> Self {
        Self {
            code: Vec::with_capacity(1024),
            pool: ConstantPool::new(),
            is_var_const: BitVec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.code.clear();
        self.pool = ConstantPool::new();
        self.is_var_const.clear();
    }

    pub fn flush(&mut self) -> Chunk {
        let codes = std::mem::take(&mut self.code);
        let constants = self.pool.to_bytes();
        let vars = self.is_var_const.as_raw_slice().to_vec();
        Chunk {
            codes,
            constants,
            vars,
        }
    }

    pub fn write_byte(&mut self, v: u8) {
        self.code.push(v);
    }
    pub fn write_short(&mut self, v: i16) {
        self.code.extend_from_slice(&v.to_be_bytes());
    }
    pub fn write_int(&mut self, v: i32) {
        self.code.extend_from_slice(&v.to_be_bytes());
    }
    pub fn update_int(&mut self, index: usize, v: i32) {
        self.code[index..index + 4].copy_from_slice(&v.to_be_bytes());
    }
    pub fn write_code(&mut self, op: OpCode) {
        self.write_byte(op as u8);
    }
    pub fn add_constant(&mut self, v: Value) -> usize {
        self.pool.add_const(v)
    }
    pub fn set_variables(&mut self, vars: &[String]) {
        let n = vars.len();
        self.is_var_const.resize(n, false);
        for var in vars {
            let idx = self.pool.add_const(Value::String(var.clone()));
            if idx >= self.is_var_const.len() {
                self.is_var_const.resize(idx + 1, false);
            }
            self.is_var_const.set(idx, true);
        }
    }
    pub fn position(&self) -> usize {
        self.code.len()
    }
}
