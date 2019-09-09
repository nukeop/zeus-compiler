use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use token::Token;

pub struct SourceFile {
    path: Option<String>,
    pub content: Option<String>,
    pub labels: Vec<String>,
    pub tokens: Vec<Token>
}

impl SourceFile {
    pub fn new() -> SourceFile {
        SourceFile {
            path: None,
            content: None,
            labels: vec![],
            tokens: vec![]
        }
    }

    pub fn load(&mut self, path: String) -> Result<(), io::Error> {
        self.path = Some(path.to_owned());
        let file = File::open(path).expect("Source file not found");

        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).expect("Could not
        read source file");
        self.content = Some(contents);
        Ok(())
    }

    pub fn tokenize(&mut self) -> Result<(), String> {
        let lines: Vec<&str> = self.content.as_ref().unwrap().split("\n").collect();

        for line in lines {
            let mut line_tokens = line.split(" ");
            for value in line_tokens {
                let parsed_token = Token::from_value(value, &self.labels);
                if let Token::Label(_) = parsed_token {
                    self.labels.push(value.to_string().replace(":", ""))
                }
                self.tokens.push(parsed_token);
            }
        }

        Ok(())
    }
}
