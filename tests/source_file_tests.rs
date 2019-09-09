extern crate zeus_compiler;

#[cfg(test)]
mod source_file_tests {
    use zeus_compiler::source_file::SourceFile;
    use zeus_compiler::instruction::Instruction;
    use zeus_compiler::token::Token;

    #[test]
    fn tokenize_basic() {
        let mut sf: SourceFile = SourceFile::new();
        sf.content = Some("abc\ndef\nghi".to_string());

        let result = sf.tokenize().unwrap();
        assert_eq!(result, ());
    }

    #[test]
    fn tokenize_instruction_noop() {
        let mut sf: SourceFile = SourceFile::new();
        sf.content = Some("NOOP".to_string());
        let result = sf.tokenize().unwrap();
        assert_eq!(result, ());
        assert_eq!(
            sf.tokens.unwrap(),
            vec![Token::Instruction(Instruction::NOOP)]
        );
    }

    #[test]
    fn tokenize_instruction_with_argument() {
        let mut sf: SourceFile = SourceFile::new();
        sf.content = Some("MVIX 15".to_string());
        let result = sf.tokenize().unwrap();
        assert_eq!(result, ());
        assert_eq!(
            sf.tokens.unwrap(),
            vec![
            Token::Instruction(Instruction::MVIX),
            Token::Argument(15)
            ]
        );
    }

    #[test]
    fn tokenize_instruction_with_two_arguments() {
        let mut sf: SourceFile = SourceFile::new();
        sf.content = Some("COPY 15 0100".to_string());
        let result = sf.tokenize().unwrap();
        assert_eq!(result, ());
        assert_eq!(
            sf.tokens.unwrap(),
            vec![
            Token::Instruction(Instruction::COPY),
            Token::Argument(15),
            Token::Argument(100)
            ]
        );
    }

    #[test]
    fn tokenize_label() {
        let mut sf: SourceFile = SourceFile::new();
        sf.content = Some("start:".to_string());
        let result = sf.tokenize().unwrap();
        assert_eq!(result, ());
        assert_eq!(
            sf.tokens.unwrap(),
            vec![Token::Label("start".to_string())]
        );
    }

    #[test]
    fn tokenize_jump() {
        let mut sf: SourceFile = SourceFile::new();
        sf.content = Some("JUMP start".to_string());
        let result = sf.tokenize().unwrap();
        assert_eq!(result, ());
        assert_eq!(
            sf.tokens.unwrap(),
            vec![
            Token::Instruction(Instruction::JUMP),
            Token::LabelArg("start".to_string())
            ]
        );
    }
}