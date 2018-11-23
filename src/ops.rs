pub struct Op {
    pub name: String,
    pub opcode: u8
}

impl Op {
    pub fn from_string(name: &str) -> Op {
        let opcode = match name {
            "NOOP" => 0,
            "MVIX" => 0x01,
            "MVIY" => 0x02,
            "MVIT" => 0x03,
            "MVAX" => 0x04,
            "MVAY" => 0x05,
            "MVAT" => 0x06,
            "MVXA" => 0x07,
            "MVYA" => 0x08,
            "MVTA" => 0x09,
            "MVPA" => 0x0A,
            "ADDX" => 0x0B,
            "ADDY" => 0x0C,
            "ADDT" => 0x0D,
            "SUBX" => 0x0E,
            "SUBY" => 0x0F,
            "SUBT" => 0x10,
            "COPY" => 0x11,
            "CPID" => 0x12,
            "CPIR" => 0x13,
            "ADDI" => 0x14,
            "SUBI" => 0x15,
            "MULI" => 0x16,
            "DIVI" => 0x17,
            "MODI" => 0x18,
            "SWIZ" => 0x19,
            "ANDI" => 0x1A,
            "ORLI" => 0x1B,
            "XORI" => 0x1C,
            "NEGI" => 0x1D,
            "SHLI" => 0x1E,
            "SHRI" => 0x1F,
            "EQLS" => 0x20,
            "GRTR" => 0x21,
            "LESS" => 0x22,
            "JUMP" => 0x23,
            "TJMP" => 0x24,
            "FJMP" => 0x25,
            "RJMP" => 0x26,
            "IJMP" => 0x27,
            "BANK" => 0x28,
            "RAND" => 0x29,
            "WAIT" => 0x2A,
            "CLRS" => 0x2B,
            _ => panic!("Unknown instruction: {}", name)
        };

        Op { name: name.to_string(), opcode }
    }
}
