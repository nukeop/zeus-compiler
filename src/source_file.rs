use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use instruction::{ArgType, Instruction};
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
        let lines: Vec<&str> = self.content
        .as_ref()
        .unwrap()
        .split("\n")
        .filter(|line| !line.to_string().starts_with("#"))
        .collect();

        for line in lines {
            let mut line_tokens = line
            .trim()
            .split(" ")
            .filter(|token| token.len() > 0);

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

    pub fn validate_instruction_args(&self, instr: &Instruction, index: usize) -> Result<(), String> {
        let arg_type = instr.get_arg_type();
        Ok(())
    }

    pub fn validate_correctness(&self) -> Result<(), String> {
        if self.tokens.is_empty() {
            return Err("No tokens, tokenize the source first".to_string());
        }
        Ok(())
    }
}
