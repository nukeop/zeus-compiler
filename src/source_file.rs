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

    pub fn to_compiled(&mut self) -> Result<Vec<u8>, String> {
        let mut result = vec!();
        let name: &str = &self.op.name.to_owned();
        
        match name {
            "MVIT" => {
                result.push(self.op.opcode);
                result.push(self.args[0].parse::<u8>().unwrap());
            },
            "COPY" => {
                let compiled = self.compile_copy().unwrap();
                for byte in compiled {
                    result.push(byte);
                }
            },
            "ADDX" => {
                result.push(self.op.opcode);
                result.push(self.args[0].parse::<u8>().unwrap());
            },
            "FJMP" => {
                let compiled = self.compile_fjmp().unwrap();
                for byte in compiled {
                    result.push(byte);
                }
            },
            "WAIT" => result.push(self.op.opcode),
            _ => result.push(0x00)
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

    pub fn compile_fjmp(&mut self) -> Result<Vec<u8>, String> {
        let mut result = vec!();
        result.push(self.op.opcode);
        result.extend(self.args[0].parse::<u16>().unwrap().as_u8_vec());
        
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
        let lines = contents.split("\n");

        self.lines = Some(lines.map(|s| {
            let mut tokens = s.split(" ");
            let op = tokens.next().unwrap().to_string();
            Line::new(op, tokens.map(|s| s.to_string()).collect())
        }).collect());
        Ok(())
    }
}
