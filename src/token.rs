use std::fmt;
use instruction::Instruction;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Instruction(Instruction),
    Argument(u16),
    Label(String),
    LabelArg(String),
    Invalid(String)
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
                return Token::Argument(parsed);
            }
        }

        let argument = value.parse::<u16>();
        if let Ok(parsed) = argument {
            return Token::Argument(parsed);
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
            Token::Argument(arg) => Token::Argument(*arg),
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
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Instruction(instr) => write!(f, "Token::Instruction({})", instr),
            Token::Argument(arg) => write!(f, "Token::Argument({})", arg),
            Token::Label(label) => write!(f, "Token::Label({})", label),
            Token::LabelArg(arg) => write!(f, "Token::LabelArg({})", arg),
            Token::Invalid(str) => write!(f, "Token::Invalid({})", str)
        }
    }
}
