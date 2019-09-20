extern crate zeus_compiler;

#[cfg(test)]
mod integration_tests {
    use zeus_compiler::source_file::SourceFile;
    use zeus_compiler::instruction::Instruction;
    use zeus_compiler::token::Token;

    #[test]
    fn tokenize_example_intro() {
        let mut sf = SourceFile::new();
        sf.load("./examples/intro.source".to_string()).unwrap();
        let result = sf.tokenize().unwrap();
        assert_eq!(result, ());
        assert_eq!(sf.tokens, vec![
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(112),
            Token::ArgumentU16(0),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(80),
            Token::ArgumentU16(0x0002),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(119),
            Token::ArgumentU16(0x0004),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(21),
            Token::ArgumentU16(0x0006),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(119),
            Token::ArgumentU16(0x0008),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(1),
            Token::ArgumentU16(0x000A),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(7),
            Token::ArgumentU16(0x000C),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(112),
            Token::ArgumentU16(0x000E),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(80),
            Token::ArgumentU16(0x0010),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(119),
            Token::ArgumentU16(0x0012),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(21),
            Token::ArgumentU16(0x00014),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(119),
            Token::ArgumentU16(0x0016),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(1),
            Token::ArgumentU16(0x018),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(7),
            Token::ArgumentU16(0x001A),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(165),
            Token::ArgumentU16(0x001E),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(181),
            Token::ArgumentU16(0x0020),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(172),
            Token::ArgumentU16(0x0022),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(165),
            Token::ArgumentU16(0x0024),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(1),
            Token::ArgumentU16(0x0026),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(128),
            Token::ArgumentU16(0x001F),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(128),
            Token::ArgumentU16(0x0021),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(128),
            Token::ArgumentU16(0x0023),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(128),
            Token::ArgumentU16(0x0025),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(192),
            Token::ArgumentU16(0x0027),
            Token::Instruction(Instruction::WAIT),
            Token::Instruction(Instruction::WAIT),
            Token::Instruction(Instruction::WAIT),
            Token::Instruction(Instruction::WAIT),
            Token::Instruction(Instruction::MVIY),
            Token::ArgumentU8(0),
            Token::Instruction(Instruction::CPIR),
            Token::ArgumentU16(0x0000),
            Token::ArgumentU16(0x0300),
            Token::Instruction(Instruction::NEGI),
            Token::ArgumentU16(0x0300),
            Token::ArgumentU16(0x0300),
            Token::Instruction(Instruction::CPID),
            Token::ArgumentU16(0x0300),
            Token::ArgumentU16(0x0000),
            Token::Instruction(Instruction::ADDY),
            Token::ArgumentU8(1),
            Token::Instruction(Instruction::MVYA),
            Token::ArgumentU16(0x0300),
            Token::Instruction(Instruction::COPY),
            Token::ArgumentU8(40),
            Token::ArgumentU16(0x0301),
            Token::Instruction(Instruction::EQLS),
            Token::ArgumentU16(0x0300),
            Token::ArgumentU16(0x0301),
            Token::Instruction(Instruction::FJMP),
            Token::ArgumentU16(0x2066),
            Token::Instruction(Instruction::MVIY),
            Token::ArgumentU8(0),
            Token::Instruction(Instruction::MVIT),
            Token::ArgumentU8(0),
            Token::Instruction(Instruction::WAIT),
            Token::Instruction(Instruction::WAIT),
            Token::Instruction(Instruction::WAIT),
            Token::Instruction(Instruction::WAIT),
            Token::Instruction(Instruction::CLRS),
            Token::Instruction(Instruction::JUMP),
            Token::ArgumentU16(0x2000)
            ]);
        }
    }
