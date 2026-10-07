use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DefineMode {
    INTERFACE,
    DEFINITION,
    NONE,
}

impl fmt::Display for DefineMode {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DefineMode::INTERFACE => write!(f, "INTERFACE"),
            DefineMode::DEFINITION => write!(f, "DEFINITION"),
            DefineMode::NONE => write!(f, "NONE"),
        }
    }
}