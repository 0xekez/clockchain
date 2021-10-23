use crate::post;

impl post::Instruction {
    pub fn assemble(self) -> [u8; 5] {
        let opcode: u8 = self.opcode as u8;
        let bytes = self.immediate.to_le_bytes();
        [opcode, bytes[0], bytes[1], bytes[2], bytes[3]]
    }
}

impl post::Program {
    pub fn assemble(self) -> Vec<u8> {
        self.instructions
            .into_iter()
            .map(|i| i.assemble())
            .flatten()
            .collect()
    }
}
