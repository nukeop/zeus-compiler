extern crate zeus_compiler;

#[cfg(test)]
mod source_file_tests {
    use zeus_compiler::instruction::Instruction;

    #[test]
    fn from_string_test() {
        assert_eq!(Instruction::from_string("NOOP"), Ok(Instruction::NOOP));
    }

    #[test]
    fn from_string_invalid() {
        assert_eq!(Instruction::from_string("abc"), Err("Invalid instruction"));
    }
}
