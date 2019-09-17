extern crate zeus_compiler;

#[cfg(test)]
mod source_file_tests {
    use zeus_compiler::source_file::SourceFile;
    use zeus_compiler::instruction::Instruction;
    use zeus_compiler::token::Token;

    #[test]
    fn tokenize_basic() {
        let mut sf = SourceFile::new();
        sf.content = Some("abc\ndef\nghi".to_string());

        let result = sf.tokenize().unwrap();
        assert_eq!(result, ());
    }

    #[test]
    fn tokenize_instruction_noop() {
        let mut sf = SourceFile::new();
        sf.content = Some("NOOP".to_string());
        let result = sf.tokenize().unwrap();
        assert_eq!(result, ());
        assert_eq!(
            sf.tokens,
            vec![Token::Instruction(Instruction::NOOP)]
        );
    }

    #[test]
    fn tokenize_instruction_copy() {
        let mut sf = SourceFile::new();
        sf.content = Some("COPY".to_string());
        let result = sf.tokenize().unwrap();
        assert_eq!(result, ());
        assert_eq!(
            sf.tokens,
            vec![Token::Instruction(Instruction::COPY)]
        );
    }

    #[test]
    fn tokenize_instruction_with_argument() {
        let mut sf = SourceFile::new();
        sf.content = Some("MVIX 15".to_string());
        let result = sf.tokenize().unwrap();
        assert_eq!(result, ());
        assert_eq!(
            sf.tokens,
            vec![
            Token::Instruction(Instruction::MVIX),
            Token::Argument(15)
            ]
        );
    }

    #[test]
    fn tokenize_instruction_with_two_arguments() {
        let mut sf = SourceFile::new();
        sf.content = Some("COPY 15 0100".to_string());
        let result = sf.tokenize().unwrap();
        assert_eq!(result, ());
        assert_eq!(
            sf.tokens,
            vec![
            Token::Instruction(Instruction::COPY),
            Token::Argument(15),
            Token::Argument(100)
            ]
        );
    }

    #[test]
    fn tokenize_label() {
        let mut sf = SourceFile::new();
        sf.content = Some("start:".to_string());
        let result = sf.tokenize().unwrap();
        assert_eq!(result, ());
        assert_eq!(
            sf.tokens,
            vec![Token::Label("start".to_string())]
        );
    }

    #[test]
    fn tokenize_jump() {
        let mut sf = SourceFile::new();
        sf.content = Some("start:\nJUMP start".to_string());
        let result = sf.tokenize().unwrap();
        assert_eq!(result, ());
        assert_eq!(
            sf.tokens,
            vec![
            Token::Label("start".to_string()),
            Token::Instruction(Instruction::JUMP),
            Token::LabelArg("start".to_string())
            ]
        );
    }

    #[test]
    fn tokenize_complex_source() {
        let mut sf = SourceFile::new();
        sf.content=Some("
        begin:
        MVIX 20
        MVIY 50
        Copy 1 1030
        test:
        cpid 1234 1236
        ".to_string());
        let result = sf.tokenize().unwrap();
        assert_eq!(result, ());
        assert_eq!(sf.tokens, vec![
            Token::Label("begin".to_string()),
            Token::Instruction(Instruction::MVIX),
            Token::Argument(20),
            Token::Instruction(Instruction::MVIY),
            Token::Argument(50),
            Token::Instruction(Instruction::COPY),
            Token::Argument(1),
            Token::Argument(1030),
            Token::Label("test".to_string()),
            Token::Instruction(Instruction::CPID),
            Token::Argument(1234),
            Token::Argument(1236)
            ]);
    }

    #[test]
    fn validate_correctness_no_tokens() {
        let sf = SourceFile::new();
        let result = sf.validate_correctness();
        match result {
            Ok(_) => assert!(false, "Error not returned"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn validate_correctness_one_instr() {
        let mut sf = SourceFile::new();
        sf.content = Some("MVIX start".to_string());
        sf.tokenize().unwrap();
        let result = sf.validate_correctness();
        if let Err(_) = result {
            assert!(true);
        } else {
            assert!(false);
        }
    }
}
