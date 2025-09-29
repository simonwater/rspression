use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn write_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn read_byte(&self, offset: usize) -> Option<u8> {
        self.code.get(offset).copied()
    }

    pub fn read_constant(&self, index: usize) -> Option<&Value> {
        self.constants.get(index)
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
