extern crate zeus_compiler;

#[cfg(test)]
mod instruction_tests {
    use zeus_compiler::instruction::{ArgType, Instruction};

    #[test]
    fn from_string_noop() {
        assert_eq!(Instruction::from_string("NOOP"), Ok(Instruction::NOOP));
    }

    #[test]
    fn from_string_copy() {
        assert_eq!(Instruction::from_string("COPY"), Ok(Instruction::COPY));
    }

    #[test]
    fn from_string_jump() {
        assert_eq!(Instruction::from_string("JUMP"), Ok(Instruction::JUMP));
    }

    #[test]
    fn from_string_invalid() {
        assert_eq!(Instruction::from_string("abc"), Err("Invalid instruction"));
    }

    #[test]
    fn to_string_noop() {
        assert_eq!(Instruction::NOOP.to_string(), "NOOP");
    }

    #[test]
    fn to_string_mvix() {
        assert_eq!(Instruction::MVIX.to_string(), "MVIX");
    }

    #[test]
    fn to_string_copy() {
        assert_eq!(Instruction::COPY.to_string(), "COPY");
    }

    #[test]
    fn arg_type_noop() {
        assert_eq!(Instruction::NOOP.get_arg_type(), ArgType::None);
    }

    #[test]
    fn arg_type_mviy() {
        assert_eq!(Instruction::MVIY.get_arg_type(), ArgType::U8);
    }

    #[test]
    fn arg_type_cpid() {
        assert_eq!(Instruction::CPID.get_arg_type(), ArgType::U16U16);
    }
}
