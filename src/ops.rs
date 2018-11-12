pub struct Op {
    pub name: String,
    pub opcode: u8
}

impl Op {
    pub fn from_string(name: &str) -> Op {
        let opcode = match name {
            "MVIX" => 0x01,
            "MVIY" => 0x02,
            "MVIT" => 0x03,
            "MVYA" => 0x08,
            "ADDX" => 0x0B,
            "ADDY" => 0x0C,
            "ADDT" => 0x0D,
            "COPY" => 0x11,
            "CPID" => 0x12,
            "CPIR" => 0x13,
            "NEGI" => 0x1D,
            "EQLS" => 0x20,
            "JUMP" => 0x23,
            "FJMP" => 0x25,
            "WAIT" => 0x29,
            "CLRS" => 0x2A,
            _ => panic!("Unknown instruction: {}", name)
        };

        Op { name: name.to_string(), opcode }
    }
}
