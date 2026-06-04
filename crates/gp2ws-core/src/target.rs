pub const DATA_BASE: usize = 0x63254; // +406100
pub const CODE_BASE: usize = 0x78254;

#[derive(Clone, Copy, Debug)]
pub enum Target {
    Data(usize),   // IDA address of a data word
    Code(usize),   // IDA address of an instruction; operand is at +1
    Direct(usize), // already a file offset (legacy old-editor offsets)
}

impl Target {
    /// file offset before calibration delta is applied
    pub fn base_offset(&self) -> usize {
        match *self {
            Target::Data(ida) => ida + DATA_BASE,
            Target::Code(ida) => ida + 1 + CODE_BASE,
            Target::Direct(off) => off,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn data_offset() {
        assert_eq!(Target::Data(0xD53DC).base_offset(), 1279536);
    }
    #[test]
    fn direct_offset() {
        assert_eq!(Target::Direct(1282824).base_offset(), 1282824);
    }
    #[test]
    fn code_operand_after_opcode() {
        // DF slope: instruction IDA 0x1682D, operand file 0x8EA82
        assert_eq!(Target::Code(0x1682D).base_offset(), 0x8EA82);
    }
}
