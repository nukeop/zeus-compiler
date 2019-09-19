use std::fmt;
use instruction::Instruction;
use util::ByteVec;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Instruction(Instruction),
    ArgumentU16(u16),
    ArgumentU8(u8),
    Label(String),
    LabelArg(String),
    Invalid(String)
}

pub enum CompilationResult {
    Bytes(Vec<u8>),
    Label(String),
    LabelArg(String)
}

impl Token {
    pub fn from_value(value: &str, labels: &Vec<String>) -> Token {
        let instr_result = Instruction::from_string(value);
        if let Ok(instr) = instr_result {
            return Token::Instruction(instr);
        }

        if (value.to_string().starts_with("0x")) {
            let argument_hex = u16::from_str_radix(
                value.to_string().replace("0x", "").as_str(),
                16
            );
            if let Ok(parsed) = argument_hex {
                return Token::ArgumentU16(parsed);
            }
        }

        let argument = value.parse::<u16>();
        if let Ok(parsed) = argument {
            if parsed < 256 {
                return Token::ArgumentU8(parsed as u8);
            } else {
                return Token::ArgumentU16(parsed as u16);
            }
        }

        let last_char_result = value.chars().last();
        if let Some(last_char) = last_char_result {
            if last_char == ':' {
                return Token::Label(
                    value.to_string().replace(":", "")
                );
            }
        }

        if labels.contains(&(value.to_string())) {
            return Token::LabelArg(value.to_string());
        }

        Token::Invalid(value.to_string())
    }

    pub fn from_token(token: &Token) -> Token {
        match token {
            Token::Instruction(instr) => Token::Instruction(*instr),
            Token::ArgumentU16(arg) => Token::ArgumentU16(*arg),
            Token::ArgumentU8(arg) => Token::ArgumentU8(*arg),
            Token::Label(label) => Token::Label(label.as_str().to_string()),
            Token::LabelArg(label_arg) => Token::LabelArg(label_arg.as_str().to_string()),
            Token::Invalid(content) => Token::Invalid(content.as_str().to_string())
        }
    }

    pub fn is_label(&self) -> bool {
        if let Token::Label(_) = self {
            return true
        }
        false
    }

    pub fn is_valid(&self) -> bool {
        if let Token::Invalid(_) = self {
            return false
        }
        true
    }

    pub fn compile(&self) -> Result<CompilationResult, String> {
        match self {
            Token::Instruction(instr) => Ok(CompilationResult::Bytes(instr.compile())),
            Token::ArgumentU16(arg) => Ok(CompilationResult::Bytes(arg.as_u8_vec())),
            Token::ArgumentU8(arg) => Ok(CompilationResult::Bytes(vec![*arg])),
            Token::Label(label) => Ok(CompilationResult::Label(label.as_str().to_string())),
            Token::LabelArg(label) => Ok(CompilationResult::LabelArg(label.as_str().to_string())),
            Token::Invalid(content) => Err(format!("Invalid token: {}", content).to_string()),
            _ => Err("Unrecognized token type".to_string())
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Instruction(instr) => write!(f, "Token::Instruction({})", instr),
            Token::ArgumentU16(arg) => write!(f, "Token::ArgumentU16({})", arg),
            Token::ArgumentU8(arg) => write!(f, "Token::ArgumentU8({})", arg),
            Token::Label(label) => write!(f, "Token::Label({})", label),
            Token::LabelArg(arg) => write!(f, "Token::LabelArg({})", arg),
            Token::Invalid(str) => write!(f, "Token::Invalid({})", str)
        }
    }
}
