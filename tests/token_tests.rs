#[macro_use]
extern crate lazy_static;
extern crate zeus_compiler;

#[cfg(test)]
mod token_tests {
    use zeus_compiler::instruction::Instruction;
    use zeus_compiler::token::{CompilationResult, Token};

    #[test]
    fn create_token() {
        let labels = vec![];
        let token = Token::from_value("NOOP", &labels);
        assert_eq!(token, Token::Instruction(Instruction::NOOP));
    }

    #[test]
    fn create_token_mvix() {
        let labels = vec![];
        let token = Token::from_value("MVIX", &labels);
        assert_eq!(token, Token::Instruction(Instruction::MVIX));
    }

    #[test]
    fn create_token_copy() {
        let labels = vec![];
        let token = Token::from_value("COPY", &labels);
        assert_eq!(token, Token::Instruction(Instruction::COPY));
    }

    #[test]
    fn create_token_arg() {
        let labels = vec![];
        let token = Token::from_value("123", &labels);
        assert_eq!(token, Token::Argument(123));
    }

    #[test]
    fn create_token_arg_hex_u8() {
        let labels = vec![];
        let token = Token::from_value("0xDE", &labels);
        assert_eq!(token, Token::Argument(222));
    }

    #[test]
    fn create_token_arg_hex_u16() {
        let labels = vec![];
        let token = Token::from_value("0xDEAD", &labels);
        assert_eq!(token, Token::Argument(57005));
    }

    #[test]
    fn create_token_label() {
        let labels = vec![];
        let token = Token::from_value("start:", &labels);
        assert_eq!(token, Token::Label("start".to_string()));
    }

    #[test]
    fn create_token_label_arg() {
        let labels = vec!["start".to_string()];
        let token = Token::from_value("start", &labels);
        assert_eq!(token, Token::LabelArg("start".to_string()));
    }

    #[test]
    fn create_token_invalid_label() {
        let labels = vec!["start".to_string()];
        let token = Token::from_value("invalid", &labels);
        assert_eq!(token, Token::Invalid("invalid".to_string()));
    }

    #[test]
    fn copy_token_instr() {
        let labels = vec![];
        let token = Token::from_value("CPID", &labels);
        let copied = Token::from_token(&token);
        assert_eq!(copied, Token::Instruction(Instruction::CPID));
    }

    #[test]
    fn copy_token_arg() {
        let labels = vec![];
        let token = Token::from_value("128", &labels);
        let copied = Token::from_token(&token);
        assert_eq!(copied, Token::Argument(128));
    }

    #[test]
    fn copy_token_label() {
        let labels = vec![];
        let token = Token::from_value("loop:", &labels);
        let copied = Token::from_token(&token);
        assert_eq!(copied, Token::Label("loop".to_string()));
    }

    #[test]
    fn compile_token_instr() {
        let labels = vec![];
        let token = Token::from_value("NOOP", &labels);
        let compiled = token.compile().unwrap();
        if let CompilationResult::Bytes(values) = compiled {
            assert_eq!(values, vec![0x00]);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn compile_token_instr_swiz() {
        let labels = vec![];
        let token = Token::from_value("SWIZ", &labels);
        let compiled = token.compile().unwrap();
        if let CompilationResult::Bytes(values) = compiled {
            assert_eq!(values, vec![0x19]);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn compile_token_arg_u8() {
        let labels = vec![];
        let token = Token::from_value("127", &labels);
        let compiled = token.compile().unwrap();
        if let CompilationResult::Bytes(values) = compiled {
            assert_eq!(values, vec![127, 0]);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn compile_token_arg_u16() {
        let labels = vec![];
        let token = Token::from_value("0xDEAD", &labels);
        let compiled = token.compile().unwrap();
        if let CompilationResult::Bytes(values) = compiled {
            assert_eq!(values, vec![0xAD, 0xDE]);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn compile_token_label() {
        let labels = vec![];
        let token = Token::from_value("point:", &labels);
        let compiled = token.compile().unwrap();
        if let CompilationResult::Label(label) = compiled {
            assert_eq!(label, "point");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn compile_token_label_arg() {
        let labels = vec!["here".to_string()];
        let token = Token::from_value("here", &labels);
        let compiled = token.compile().unwrap();
        if let CompilationResult::LabelArg(label) = compiled {
            assert_eq!(label, "here".to_string());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn compile_token_invalid() {
        let labels = vec![];
        let token = Token::from_value("qwerty", &labels);
        let compiled = token.compile();
        if let Err(_) = compiled {
            assert!(true);
        } else {
            assert!(false, "Error not returned for an invalid token");
        }
    }
}
