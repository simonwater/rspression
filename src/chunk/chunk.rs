#[derive(Clone, Debug)]
pub struct Chunk {
    pub codes: Vec<u8>,
    pub constants: Vec<u8>,
    pub vars: Vec<u8>,
}

impl Chunk {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&(self.codes.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.codes);
        out.extend_from_slice(&(self.constants.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.constants);
        out.extend_from_slice(&(self.vars.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.vars);
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut i = 0usize;
        let read_u32 = |b: &[u8]| u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as usize;
        let code_sz = read_u32(&bytes[i..i + 4]);
        i += 4;
        let codes = bytes[i..i + code_sz].to_vec();
        i += code_sz;
        let const_sz = read_u32(&bytes[i..i + 4]);
        i += 4;
        let constants = bytes[i..i + const_sz].to_vec();
        i += const_sz;
        let var_sz = read_u32(&bytes[i..i + 4]);
        i += 4;
        let vars = bytes[i..i + var_sz].to_vec();
        Self {
            codes,
            constants,
            vars,
        }
    }
}
