use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use token::{CompilationResult, Token};

pub struct SourceFile {
    path: Option<String>,
    pub content: Option<String>,
    pub labels: Vec<String>,
    pub tokens: Vec<Token>
}

pub struct JumpDestination {
    pub label: String,
    pub address: u16
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

    pub fn validate_correctness(&self) -> Result<(), String> {
        if self.tokens.is_empty() {
            return Err("No tokens, tokenize the source first".to_string());
        }
        for token in &self.tokens {
            if !token.is_valid() {
                return Err(format!("Invalid source code token: {}", token))
            }
        }

        Ok(())
    }

    pub fn compile(&self) -> Result<Vec<u8>, String>{
        if self.tokens.is_empty() {
            return Err("No tokens, tokenize the source first".to_string());
        }

        let mut program: Vec<u8> = vec![];
        let mut labels: Vec<JumpDestination> = vec![];
        let mut current_pointer: u16 = 0x2000;

        for token in &self.tokens {
            let compiled_token: CompilationResult = token.compile()?;
            match compiled_token {
                CompilationResult::Bytes(values) => {
                    let compiled_token_values: Vec<u8> = values.iter().cloned().collect();
                    current_pointer.wrapping_add(compiled_token_values.len() as u16);
                    program.extend(compiled_token_values);
                },
                CompilationResult::Label(label) => {
                    labels.push(JumpDestination::new(label, current_pointer));
                },
                CompilationResult::LabelArg(labelArg) => {
                    
                }
            }
        }

        Ok(program)
    }
}

impl JumpDestination {
    pub fn new(label: String, address: u16) -> JumpDestination {
        JumpDestination { label, address }
    }
}
