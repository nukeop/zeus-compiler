use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub struct SourceFile {
    path: Option<String>,
    lines: Option<Vec<String>>
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
        let mut file = File::open(path).expect("Source file not found");

        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).expect("Could not
        read source `file");

        self.lines = Some(vec!(contents));
        Ok(())
    }
}
