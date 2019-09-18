pub trait ByteVec {
    fn as_u8_vec(&self) -> Vec<u8>;
}

impl ByteVec for u16 {
    fn as_u8_vec(&self) -> Vec<u8> {
        let low = *self & 0xFF;
        let  hi = (*self & 0xFF00) >> 8;
        vec![low as u8, hi as u8]
    }
}
