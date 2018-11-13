use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

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

    pub fn compile_two_addr(&mut self, bytes: &mut Vec<u8>) {
        bytes.push(self.op.opcode);
        let addr1 = u16::from_str_radix(&self.args[0], 16).unwrap().as_u8_vec();
        let addr2 = u16::from_str_radix(&self.args[1], 16).unwrap().as_u8_vec();
        bytes.extend(addr1);
        bytes.extend(addr2);
    }

    pub fn to_compiled(&mut self) -> Result<Vec<u8>, String> {
        let mut result = vec!();
        let name: &str = &self.op.name.to_owned();
        
        match name {
            "MVIY" => self.compile_single_byte_arg(&mut result),
            "MVIT" => self.compile_single_byte_arg(&mut result),
            "MVYA" => self.compile_two_bytes_arg(&mut result),
            "COPY" => result.extend(self.compile_copy().unwrap()),
            "CPIR" => self.compile_two_addr(&mut result),
            "CPID" => self.compile_two_addr(&mut result),
            "ADDX" => self.compile_single_byte_arg(&mut result),
            "ADDY" => self.compile_single_byte_arg(&mut result),
            "NEGI" => self.compile_two_addr(&mut result),
            "EQLS" => self.compile_two_addr(&mut result),
            "JUMP" => self.compile_two_bytes_arg(&mut result),
            "FJMP" => self.compile_two_bytes_arg(&mut result),
            "WAIT" => result.push(self.op.opcode),
            "CLRS" => result.push(self.op.opcode),
            _ => panic!("Unknown instruction: {}", name)
        }

        

        Ok(result)
    }

    pub fn compile_copy(&mut self) -> Result<Vec<u8>, String> {
        let mut result = vec!();
        result.push(self.op.opcode);

        let arg = self.args[0].parse::<u8>().unwrap();
        result.push(arg);

        let addr = u16::from_str_radix(&self.args[1], 16).unwrap().as_u8_vec();
        result.extend(addr);
        
        Ok(result)
    }
}

pub struct SourceFile {
    path: Option<String>,
    pub lines: Option<Vec<Line>>
}

impl SourceFile {
    pub fn new() -> SourceFile {
        SourceFile {
            path: None,
            lines: None
        }
    }

    pub fn load(&mut self, path: String) -> Result<(), io::Error> {
        self.path = Some(path.to_owned());
        let file = File::open(path).expect("Source file not found");

        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).expect("Could not
        read source file");
        let lines: Vec<&str> = contents.split("\n").collect();

        let mut lines_parsed = vec!();
        for line in lines {
            let mut tokens = line.split(" ");
            let op = tokens.next().unwrap().to_string();

            if (op == "") {
                continue;
            }

            lines_parsed.push(Line::new(op, tokens.map(|s| s.to_string()).collect()));
        }
        self.lines = Some(lines_parsed);
        
        Ok(())
    }
}
