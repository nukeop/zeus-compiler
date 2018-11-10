pub struct Op {
    pub name: String,
    pub opcode: u8
}

impl Op {
    pub fn from_string(name: &str) -> Op {
        let opcode = match name {
            "ADDX" => 0x0B,
            "ADDY" => 0x0C,
            "ADDT" => 0x0D,
            "COPY" => 0x11,
            "WAIT" => 0x29,
            _ => 0x00
        };

        Op { name: name.to_string(), opcode }
    }
}
