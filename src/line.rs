use ops::Op;
use util::ByteVec;

pub struct Line {
    pub op: Op,
    pub args: Vec<String>
}

impl Line {
    pub fn new(op: String, args: Vec<String>) -> Line {
        Line { op: Op::from_string(&op), args }
    }

    pub fn compile_single_byte_arg(&mut self, bytes: &mut Vec<u8>) {
        bytes.push(self.op.opcode);
        bytes.push(self.args[0].parse::<u8>().unwrap());
    }

    pub fn compile_two_bytes_arg(&mut self, bytes: &mut Vec<u8>) {
        bytes.push(self.op.opcode);
        let addr = u16::from_str_radix(&self.args[0], 16).unwrap().as_u8_vec();
        bytes.extend(addr);
    }

    pub fn compile_n_addr(&mut self, addr_num: usize, bytes: &mut Vec<u8>) {
        bytes.push(self.op.opcode);

        for n in 0..addr_num {
            let addr = u16::from_str_radix(&self.args[n], 16).unwrap().as_u8_vec();
            bytes.extend(addr);
        }
    }

    pub fn compile_u8_u16(&mut self) -> Result<Vec<u8>, String> {
        let mut result = vec!();
        result.push(self.op.opcode);

        let arg = self.args[0].parse::<u8>().unwrap();
        result.push(arg);

        let addr = u16::from_str_radix(&self.args[1], 16).unwrap().as_u8_vec();
        result.extend(addr);

        Ok(result)
    }

    pub fn save_label(&mut self) {

    }

    pub fn to_compiled(&mut self) -> Result<Vec<u8>, String> {
        let mut result = vec!();
        let name: &str = &self.op.name.to_owned();

        match name {
            "NOOP" => result.push(self.op.opcode),

            "MVIX" => self.compile_single_byte_arg(&mut result),
            "MVIY" => self.compile_single_byte_arg(&mut result),
            "MVIT" => self.compile_single_byte_arg(&mut result),

            "MVAX" => self.compile_n_addr(1, &mut result),
            "MVAY" => self.compile_n_addr(1, &mut result),
            "MVAT" => self.compile_n_addr(1, &mut result),

            "MVXA" => self.compile_two_bytes_arg(&mut result),
            "MVYA" => self.compile_two_bytes_arg(&mut result),
            "MVTA" => self.compile_two_bytes_arg(&mut result),
            "MVPA" => self.compile_two_bytes_arg(&mut result),

            "ADDX" => self.compile_single_byte_arg(&mut result),
            "ADDY" => self.compile_single_byte_arg(&mut result),
            "ADDT" => self.compile_single_byte_arg(&mut result),
            "SUBX" => self.compile_single_byte_arg(&mut result),
            "SUBY" => self.compile_single_byte_arg(&mut result),
            "SUBT" => self.compile_single_byte_arg(&mut result),

            "COPY" => result.extend(self.compile_u8_u16().unwrap()),
            "CPID" => self.compile_n_addr(2, &mut result),
            "CPIR" => self.compile_n_addr(2, &mut result),

            "ADDI" => self.compile_n_addr(3, &mut result),
            "SUBI" => self.compile_n_addr(3, &mut result),
            "MULI" => self.compile_n_addr(3, &mut result),
            "DIVI" => self.compile_n_addr(3, &mut result),
            "MODI" => self.compile_n_addr(3, &mut result),
            "SWIZ" => self.compile_n_addr(3, &mut result),
            "ANDI" => self.compile_n_addr(3, &mut result),
            "ORLI" => self.compile_n_addr(3, &mut result),
            "XORI" => self.compile_n_addr(3, &mut result),
            "NEGI" => self.compile_n_addr(2, &mut result),
            "SHLI" => self.compile_n_addr(2, &mut result),
            "SHRI" => self.compile_n_addr(2, &mut result),

            "EQLS" => self.compile_n_addr(2, &mut result),
            "GRTR" => self.compile_n_addr(2, &mut result),
            "LESS" => self.compile_n_addr(2, &mut result),
            "JUMP" => self.compile_two_bytes_arg(&mut result),
            "TJMP" => self.compile_two_bytes_arg(&mut result),
            "FJMP" => self.compile_two_bytes_arg(&mut result),
            "RJMP" => result.extend(self.compile_u8_u16().unwrap()),
            "IJMP" => self.compile_two_bytes_arg(&mut result),

            "BANK" => self.compile_single_byte_arg(&mut result),
            "RAND" => self.compile_n_addr(2, &mut result),
            "WAIT" => result.push(self.op.opcode),
            "CLRS" => result.push(self.op.opcode),

            "LABEL" => self.save_label(),
            _ => panic!("Unknown instruction: {}", name)
        }

        Ok(result)
    }
}
