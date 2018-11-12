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
        file.write_all(&self.bytes)?;
        Ok(())
    }
}
