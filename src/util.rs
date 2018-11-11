pub trait ByteVec {
    fn as_u8_vec(&mut self) -> Vec<u8>;
}

impl ByteVec for u16 {
    fn as_u8_vec(&mut self) -> Vec<u8> {
        let low = *self & 0xFF;
        let  hi = (*self & 0xFF00) >> 8;
        vec![hi as u8, low as u8]
    }
}
