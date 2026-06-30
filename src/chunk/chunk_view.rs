use crate::error::{RspError, RspResult};
#[derive(Clone, Debug)]
pub struct ChunkView<'a> {
    pub codes: &'a [u8],
    pub constants: &'a [u8],
    pub vars: &'a [u8],
}

impl<'a> ChunkView<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> RspResult<Self> {
        let mut i = 0usize;

        let read_u32 = |b: &[u8], offset: usize| -> RspResult<usize> {
            if offset + 4 > b.len() {
                return Err(RspError::FormatError {
                    message: String::from(
                        "Byte stream length is insufficient; unable to parse the length prefix.",
                    ),
                });
            }
            let array = [b[offset], b[offset + 1], b[offset + 2], b[offset + 3]];
            Ok(u32::from_be_bytes(array) as usize)
        };

        // 1. parse codes slice
        let code_sz = read_u32(bytes, i)?;
        i += 4;
        if i + code_sz > bytes.len() {
            return Err(RspError::FormatError {
                message: String::from("Codes data block out of bounds"),
            });
        }
        let codes = &bytes[i..i + code_sz];
        i += code_sz;

        // 2. parse constants slice
        let const_sz = read_u32(bytes, i)?;
        i += 4;
        if i + const_sz > bytes.len() {
            return Err(RspError::FormatError {
                message: String::from("Constants data block out of bounds"),
            });
        }
        let constants = &bytes[i..i + const_sz];
        i += const_sz;

        // 3. parse vars slice
        let var_sz = read_u32(bytes, i)?;
        i += 4;
        if i + var_sz > bytes.len() {
            return Err(RspError::FormatError {
                message: String::from("Vars data block out of bounds"),
            });
        }
        let vars = &bytes[i..i + var_sz];

        Ok(Self {
            codes,
            constants,
            vars,
        })
    }

    pub fn get_byte_size(&self) -> usize {
        self.codes.len() + self.constants.len() + self.vars.len()
    }
}
