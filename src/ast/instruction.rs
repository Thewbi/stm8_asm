use std::str::FromStr;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub struct InstructionError {}

impl InstructionError {
    #[allow(dead_code)]
    pub fn new() -> InstructionError {
        InstructionError {}
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {

    ASR,
    ADD,

    B,
    BAL,
    BEQ,
    BGT,
    BIC,
    BL,
    BX,

    CMP,

    LDR,
    LDR_W,

    MOV,
    MOV_W,

    ORR_W,

    PUSH,

    STR,
    STR_W,
    // now: svc (Supervisor Call), earlier: swi (Software Interrupt)
    SVC,
    SWI,

    WFI,

    UNDEFINED,
}

impl FromStr for Instruction {

    type Err = ();

    fn from_str(input: &str) -> Result<Instruction, Self::Err> {

        // https://github.com/rust-lang/rfcs/issues/1988
        match input.to_uppercase().as_ref() {

            "ASR" => Ok(Instruction::ASR),
            "ADD" => Ok(Instruction::ADD),

            "B" => Ok(Instruction::B),
            "BAL" => Ok(Instruction::BAL),
            "BEQ" => Ok(Instruction::BEQ),
            "BGT" => Ok(Instruction::BGT),
            "BIC" => Ok(Instruction::BIC),
            "BL" => Ok(Instruction::BL),
            "BX" => Ok(Instruction::BX),

            "CMP" => Ok(Instruction::CMP),

            "LDR" => Ok(Instruction::LDR),
            "LDR.W" => Ok(Instruction::LDR_W),

            "MOV" => Ok(Instruction::MOV),
            "MOV.W" => Ok(Instruction::MOV_W),

            "ORR.W" => Ok(Instruction::ORR_W),

            "PUSH" => Ok(Instruction::PUSH),

            "STR" => Ok(Instruction::STR),
            "STR.W" => Ok(Instruction::STR_W),
            // now: svc (Supervisor Call), earlier: swi (Software Interrupt)
            "SVC" => Ok(Instruction::SVC),
            "SWI" => Ok(Instruction::SWI),

            "WFI" => Ok(Instruction::WFI),

            //_ => Err(()),
            _ => todo!(),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {

            Instruction::ASR => write!(f, "asr"),
            Instruction::ADD => write!(f, "add"),

            Instruction::B => write!(f, "b"),
            Instruction::BAL => write!(f, "bal"),
            Instruction::BEQ => write!(f, "beq"),
            Instruction::BGT => write!(f, "bgt"),
            Instruction::BIC => write!(f, "bic"),
            Instruction::BL => write!(f, "bl"),
            Instruction::BX => write!(f, "bx"),

            Instruction::CMP => write!(f, "cmp"),

            Instruction::LDR => write!(f, "ldr"),
            Instruction::LDR_W => write!(f, "ldr.w"),

            Instruction::MOV => write!(f, "mov"),
            Instruction::MOV_W => write!(f, "mov.w"),

            Instruction::ORR_W => write!(f, "orr.w"),

            Instruction::PUSH => write!(f, "push"),

            Instruction::STR => write!(f, "str"),
            Instruction::STR_W => write!(f, "str.w"),
            // now: svc (Supervisor Call), earlier: swi (Software Interrupt)
            Instruction::SVC => write!(f, "svc"),
            Instruction::SWI => write!(f, "swi"),

            Instruction::WFI => write!(f, "wfi"),

            Instruction::UNDEFINED => write!(f, "undefined"),

        }
        
    }
}
