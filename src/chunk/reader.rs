use super::pool::ConstantPool;
use crate::{ChunkView, values::Value, vm::OpCode};
use bitvec::prelude::*;

pub struct ChunkReader<'a> {
    code: &'a [u8],
    ip: usize,
    const_pool: ConstantPool,
    is_var_const: BitVec<u8, Msb0>,
}

impl<'a> ChunkReader<'a> {
    pub fn from_chunk(chunk: &'a ChunkView) -> Self {
        Self {
            code: chunk.codes,
            ip: 0,
            const_pool: ConstantPool::from_bytes(chunk.constants),
            is_var_const: BitVec::<u8, Msb0>::from_slice(chunk.vars),
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        let b = self.code[self.ip];
        self.ip += 1;
        b
    }

    pub fn read_short(&mut self) -> i16 {
        let b = &self.code[self.ip..self.ip + 2];
        self.ip += 2;
        i16::from_be_bytes([b[0], b[1]])
    }

    pub fn read_int(&mut self) -> i32 {
        let b = &self.code[self.ip..self.ip + 4];
        self.ip += 4;
        i32::from_be_bytes([b[0], b[1], b[2], b[3]])
    }

    pub fn read_opcode(&mut self) -> OpCode {
        OpCode::from(self.read_byte())
    }

    pub fn read_const(&self, index: usize) -> &Value {
        self.const_pool.read_const(index)
    }

    pub fn variable_iter(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        let iter = self
            .is_var_const
            .iter()
            .by_vals()
            .enumerate()
            .filter_map(|(idx, bit)| {
                if bit {
                    let value = self.read_const(idx);
                    Some(value.as_str())
                } else {
                    None
                }
            });
        Box::new(iter)
    }

    pub fn position(&self) -> usize {
        self.ip
    }

    pub fn new_position(&mut self, p: usize) {
        self.ip = p;
    }

    pub fn code_size(&self) -> usize {
        self.code.len()
    }
}
