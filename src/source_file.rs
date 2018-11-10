use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use ops::Op;

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
        let name: &str = &self.op.name;
        match name {
            "COPY" => {
                result.push(self.op.opcode);
                result.push(self.op.opcode);
                
            },
            "ADDX" => result.push(self.op.opcode),
            "WAIT" => result.push(self.op.opcode),
            _ => result.push(0x00)
                
        }

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
