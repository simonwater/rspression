use crate::{values::Value, vm::OpCode};

use super::pool::ConstantPool;

pub struct ChunkReader<'a> {
    code: &'a [u8],
    ip: usize,
    const_pool: ConstantPool,
    _vars_bits: Vec<u8>,
}

impl<'a> ChunkReader<'a> {
    pub fn new(code: &'a [u8], constants: &'a [u8], vars: &'a [u8]) -> Self {
        Self {
            code,
            ip: 0,
            const_pool: ConstantPool::from_bytes(constants),
            _vars_bits: vars.to_vec(),
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
