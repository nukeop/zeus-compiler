extern crate zeus_compiler;

#[cfg(test)]
mod util_tests {
    use zeus_compiler::util::ByteVec;
    
    #[test]
    fn unpack_u16() {
        let vector = (0xFFAA).as_u8_vec();
        assert_eq!(vector[0], 0xAA);
        assert_eq!(vector[1], 0xFF);
    }

    #[test]
    fn unpack_u16_low_zero() {
        let vector = (0x9900).as_u8_vec();
        assert_eq!(vector[0], 0);
        assert_eq!(vector[1], 0x99);
    }

    #[test]
    fn unpack_u16_hi_zero() {
        let vector = (0x0064).as_u8_vec();
        assert_eq!(vector[0], 0x64);
        assert_eq!(vector[1], 0);
    }
}
