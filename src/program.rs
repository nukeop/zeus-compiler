use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Version {
    major: u8,
    minor: u8,
    patch: u8
}

impl Version {
    pub fn new(major: u8, minor: u8, patch: u8) -> Version {
        Version { major, minor, patch }
    }
}

pub struct ZeusHeader {
    magic: [u8; 4],
    version: Version,
    zero: [u8; 25]
}

impl ZeusHeader {
    pub fn new(version: Version) -> ZeusHeader {
        ZeusHeader {
            magic: *b"ZEUS",
            version,
            zero: [0; 25]
        }
    }

    pub fn as_slice(&mut self) -> [u8; 32] {
        let mut slice = [0; 32];
        for i in 0..self.magic.len() {
            slice[i] = self.magic[i];
        }
        slice[self.magic.len() + 1] = self.version.major;
        slice[self.magic.len() + 2] = self.version.minor;
        slice[self.magic.len() + 3] = self.version.patch;
        
        slice
    }
}

pub struct Program {
    pub header: ZeusHeader,
    pub bytes: Vec<u8>
}

impl Program {
    pub fn new(version: Version) -> Program {
        Program {
            header: ZeusHeader::new(version),
            bytes: vec!()
        }
    }

    pub fn to_file(&mut self, filename: String) -> io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(&self.header.as_slice())?;
        file.write_all(&self.bytes)?;
        Ok(())
    }
}
