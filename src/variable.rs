pub enum Variable {
    Byte(u8),
    Char(char),
    Short(u16),
    Int(u32),
    Float(f32),
    Reference(usize),
    ReturnAddress(usize),

    Long(u64),
    Double(f64),
}
