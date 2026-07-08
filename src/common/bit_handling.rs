// https://levelup.gitconnected.com/learning-rust-rolling-bits-53b6b3b20d02
pub fn rol_u8(value: u8, shift: u8) -> u8 {
    (value << shift) | (value >> (8 - shift))
}

pub fn rol_u16(value: u16, shift: u16) -> u16 {
    (value << shift) | (value >> (16 - shift))
}