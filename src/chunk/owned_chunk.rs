use super::ChunkView;
use crate::error::RspResult;

// 编译期产出的所有权实体
#[derive(Clone, Debug)]
pub struct OwnedChunk {
    bytes: Vec<u8>,
}

impl OwnedChunk {
    // 将其转化为字节流
    pub fn to_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    // 从所有权结构中切出只读视图
    pub fn try_to_view(&self) -> RspResult<ChunkView<'_>> {
        ChunkView::from_bytes(&self.bytes)
    }
}
