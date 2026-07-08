use std::{fmt::Write, num::ParseIntError};

// convert a string with hex digits into a vector of those bytes
// deserializes a hex byte string
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

// serializes a bytes array to a hex string
pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02X}", b).unwrap();
    }
    s
}