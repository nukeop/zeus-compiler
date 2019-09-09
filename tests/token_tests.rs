#[macro_use]
extern crate lazy_static;
extern crate zeus_compiler;

#[cfg(test)]
mod source_file_tests {
    use zeus_compiler::instruction::Instruction;
    use zeus_compiler::token::Token;

    lazy_static!{
        static ref labels: Vec<&'static str> = vec![];
    }

    #[test]
    fn create_token() {
        let token = Token::from_value("NOOP", &labels);
        assert_eq!(token, Token::Instruction(Instruction::NOOP));
    }

    #[test]
    fn create_token_mvix() {
        let token = Token::from_value("MVIX", &labels);
        assert_eq!(token, Token::Instruction(Instruction::MVIX));
    }

    #[test]
    fn create_token_label_instr() {
        let token = Token::from_value("LABEL", &labels);
        assert_eq!(token, Token::Instruction(Instruction::LABEL));
    }

    #[test]
    fn create_token_arg() {
        let token = Token::from_value("123", &labels);
        assert_eq!(token, Token::Argument(123));
    }

    #[test]
    fn create_token_arg_hex_u8() {
        let token = Token::from_value("DE", &labels);
        assert_eq!(token, Token::Argument(222));
    }

    #[test]
    fn create_token_arg_hex_u16() {
        let token = Token::from_value("DEAD", &labels);
        assert_eq!(token, Token::Argument(57005));
    }

    #[test]
    fn create_token_label() {
        let token = Token::from_value("start:", &labels);
        assert_eq!(token, Token::Label("start".to_string()));
    }
}
