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

    // 0x1E
    // LDW X,($50,SP)
    // PM0044, page 117
    LDW_X_SP_OFFSET,

    // 0x1F
    // XLDW ($50,SP),X 
    // (0x1F) LDW, (shortoff,SP) XLDW ($50,SP),X 
    // PM0044, page 118
    LDW_SP_OFFSET_X,

    // 0x20
    JRA,

    // 0x27
    // JREQ, PM0044, page 113
    JREQ, 

    // 0x4F
    // CLEAR, PM0044, page 92
    CLR_A, 

    // 0x4D
    // TNZ, PM0044, page 155
    TNZ_A, 

    // 0x52
    // SUB, PM0044, page 151
    SUB_SP, 

    // 0x58, page 146
    // SLLW X
    SLLW_X, 

    // 0x5C
    // INC, PM0044, page 106
    INC, 

    // 0x5F
    // CLRW_X, PM0044, page 93
    CLRW_X,

    // 0x72 0xFB
    ADDW_X_OFFSET_SP,

    // 0x81
    // RET, PM0044, page 131
    RET, 

    // 0x82
    // INT, INTERRUPT
    INT,

    // 0x96
    // LDW X,SP
    LDW_X_SP,

    // 0xA3
    CPW_X_IMM,

    // 0xAE
    // LDW, LOAD
    LDW_AE, 

    // 0xCC  
    // UNDOCUMENTED!!!!!!!!
    CALL_CC, 

    // 0xCD
    // CALL, PM0044, page 88
    CALL_CD,

    // 0xFE
    LDW_X_X,

    UNDEFINED,

}

impl FromStr for Instruction {

    type Err = ();

    fn from_str(input: &str) -> Result<Instruction, Self::Err> {

        match input.to_uppercase().as_ref() {

            // 0x1E
            "LDW_X_SP_OFFSET" => Ok(Instruction::LDW_X_SP_OFFSET),
            // 0x1F
            "LDW_SP_OFFSET_X" => Ok(Instruction::LDW_SP_OFFSET_X),

            // 0x20
            "JRA" => Ok(Instruction::JRA),

            // 0x27
            "JREQ" => Ok(Instruction::JREQ),

            // 0x4D
            "TNZ_A" => Ok(Instruction::TNZ_A),

            // 0x4F
            "CLR_A" => Ok(Instruction::CLR_A),

            // 0x58
            "SLLW_X" => Ok(Instruction::SLLW_X),

            // 0x72 0xFB
            "ADDW_X_OFFSET_SP" => Ok(Instruction::ADDW_X_OFFSET_SP),

            // 0x81
            "RET" => Ok(Instruction::RET),

            // 0x82
            "INT" => Ok(Instruction::INT),

            // 0x96
            "LDW_X_SP" => Ok(Instruction::LDW_X_SP),

            // 0xA3
            "CPW_X_OFFSET" => Ok(Instruction::CPW_X_IMM),

            // 0xAE
            "LDW_AE" => Ok(Instruction::LDW_AE),

            // 0xCC, UNDOCUMENTED !!!!!!!!
            "CALL_CC" => Ok(Instruction::CALL_CC), 

            // 0xCD
            "CALL_CD" => Ok(Instruction::CALL_CD),

            // LDW_X_X
            "LDW_X_X" => Ok(Instruction::LDW_X_X),

            _ => todo!(),

        }
    }
}

impl fmt::Display for Instruction {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {

            // 0x1E
            Instruction::LDW_X_SP_OFFSET => write!(f, "LDW X,($50,SP)"),
            // 0x1F
            Instruction::LDW_SP_OFFSET_X => write!(f, "LDW ($50,SP),X"),

            // 0x20
            Instruction::JRA => write!(f, "jra"),

            // 0x27
            Instruction::JREQ => write!(f, "jreq"),

            // 0x4F
            Instruction::CLR_A => write!(f, "clr_a"),

            // 0x4D
            Instruction::TNZ_A => write!(f, "tnz_a"),

            // 0x52
            Instruction::SUB_SP => write!(f, "sub_sp"),

            // 0x5C
            Instruction::INC => write!(f, "inc"),

            // 0x5F
            Instruction::CLRW_X => write!(f, "clrw_x"),

            // 0x58, page 146
            Instruction::SLLW_X => write!(f, "sllw_x"),

            // 0x72 0xFB
            Instruction::ADDW_X_OFFSET_SP => write!(f, "ADDW_X_OFFSET_SP"),

            // 0x81
            Instruction::RET => write!(f, "ret"),

            // 0x82
            Instruction::INT => write!(f, "int"),

            // 0x96
            Instruction::LDW_X_SP => write!(f, "LDW_X_SP"),

            // 0xA3
            Instruction::CPW_X_IMM=> write!(f, "CPW_X_IMM"),

            // 0xAE
            Instruction::LDW_AE => write!(f, "ldw_ae"),

            // 0xCC
            Instruction::CALL_CC => write!(f, "call_cc"),
            // 0xCD
            Instruction::CALL_CD => write!(f, "call_cd"),

            // 0xFE, page 117
            Instruction::LDW_X_X => write!(f, "LDW_X_X"),

            Instruction::UNDEFINED => write!(f, "undefined"),

        }
        
    }
}
