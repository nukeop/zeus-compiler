use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub struct Line {
    pub op: String,
    pub args: Vec<String>
}

impl Line {
    pub fn new(op: String, args: Vec<String>) -> Line {
        Line { op, args }
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
