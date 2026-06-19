use super::ChunkView;
use crate::error::RspResult;

// 1. 编译期产出的所有权实体
#[derive(Clone, Debug)]
pub struct OwnedChunk {
    pub codes: Vec<u8>,
    pub constants: Vec<u8>,
    pub vars: Vec<u8>,
}

impl OwnedChunk {
    // 将其转化为字节流
    pub fn to_bytes(&self) -> Vec<u8> {
        let total_sz = 12 + self.codes.len() + self.constants.len() + self.vars.len();
        let mut out = Vec::with_capacity(total_sz);
        out.extend_from_slice(&(self.codes.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.codes);
        out.extend_from_slice(&(self.constants.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.constants);
        out.extend_from_slice(&(self.vars.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.vars);
        out
    }

    pub fn from_bytes(bytes: Vec<u8>) -> RspResult<Self> {
        let view = ChunkView::from_bytes(&bytes)?;
        Ok(Self {
            codes: view.codes.to_vec(),
            constants: view.constants.to_vec(),
            vars: view.vars.to_vec(),
        })
    }

    // 从所有权结构中切出只读视图
    pub fn as_view(&self) -> ChunkView<'_> {
        ChunkView {
            codes: &self.codes,
            constants: &self.constants,
            vars: &self.vars,
        }
    }
}
