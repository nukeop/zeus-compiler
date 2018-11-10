use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Program {
    pub bytes: Vec<u8>
}

impl Program {
    pub fn new() -> Program {
        Program { bytes: vec!() }
    }

    pub fn to_file(&mut self, filename: String) -> io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(&self.bytes)?;
        Ok(())
    }
}
