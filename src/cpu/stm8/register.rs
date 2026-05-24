use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
pub struct RegisterError {}

impl RegisterError {
    pub fn new() -> RegisterError {
        RegisterError {}
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Register {
    Accumulator,
    XIndex,
    YIndex,
    StackPointer,
    ProgramCounter,
    UNDEFINED
}

impl Register {

    /*
    pub fn from_u16(input: u16) -> Result<Register, RegisterError> {
        // https://github.com/rust-lang/rfcs/issues/1988
        match input {
            0 => Ok(Register::R0),
            1 => Ok(Register::R1),
            2 => Ok(Register::R2),
            3 => Ok(Register::R3),
            4 => Ok(Register::R4),
            5 => Ok(Register::R5),
            6 => Ok(Register::R6),
            7 => Ok(Register::R7),
            8 => Ok(Register::R8),
            9 => Ok(Register::R9),
            10 => Ok(Register::R10),

            11 => Ok(Register::R11),
            //"FP" => Ok(Register::FP),

            12 => Ok(Register::R12),
            //"IP" => Ok(Register::IP),

            13 => Ok(Register::R13),
            //"SP" => Ok(Register::SP),

            14 => Ok(Register::R14),
            //"LR" => Ok(Register::LR),

            15 => Ok(Register::R15),
            //"PC" => Ok(Register::PC),

            _ => Err(RegisterError::new()),
        }
    } 
    */

    /*
    pub fn to_index(reg: Register) -> usize {
        // https://github.com/rust-lang/rfcs/issues/1988
        match reg {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
            Register::R4 => 4,
            Register::R5 => 5,
            Register::R6 => 6,
            Register::R7 => 7,
            Register::R8 => 8,
            Register::R9 => 9,
            Register::R10 => 10,
            
            Register::R11 => 11,
            Register::FP => 11,
            
            Register::R12 => 12,
            Register::IP => 12,
            
            Register::R13 => 13,
            Register::SP => 13,

            Register::R14 => 14,
            Register::LR => 14,

            Register::R15 => 15,
            Register::PC => 15,

            _ => 0xFF,
        }
    }
    */
}

impl FromStr for Register {
    type Err = ();

    fn from_str(input: &str) -> Result<Register, Self::Err> {

        // PM0044, page 13
        match input.to_uppercase().as_ref() {
            "A" => Ok(Register::Accumulator),
            "X" => Ok(Register::XIndex),
            "Y" => Ok(Register::YIndex),
            "SP" => Ok(Register::StackPointer),
            "PC" => Ok(Register::ProgramCounter),

            _ => Err(()),
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            Register::Accumulator => write!(f, "A"),
            Register::XIndex => write!(f, "X"),
            Register::YIndex => write!(f, "Y"),
            Register::StackPointer => write!(f, "SP"),
            Register::ProgramCounter => write!(f, "PC"),

            Register::UNDEFINED => write!(f, "undefined"),
        }
        
    }
}