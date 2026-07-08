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

    // 0x01
    RRWA_X,

    // 0x08
    // SLL ($15,SP)
    // The instruction reads the byte stored in memory at the offset,
    // shifts its bits one position to the left, and stores the new value back to the same address.
    SLL_SP_OFFSET,

    // 0x09
    // RLC ($10,SP), page 134
    RLC_SP_OFFSET,

    // 0x0C
    // INC ($10,SP)
    INC_SP_OFFSET,

    // 0x0F
    // CLR dst
    // CLR ($10,SP)
    CLR_SP_OFFSET,

    // 0x10
    // SUB A,($10,SP), page 151
    SUB_A_OFFSET_SP,

    // 0x1C
    ADDW_X_IMM,

    // 0x13
    CPW_X_SP_OFFSET,

    // 0x16
    LDW_Y_SP_OFFSET,

    // 0x17
    LDW_OFFSET_SP_Y,

    // 0x18
    XOR_A_OFFSET_SP,

    // 0x1A, OR A,($10,SP), page 125
    OR_A_OFFSET_SP,
    
    // 0x1D
    SUBW_X_IMM,

    // 0x1E
    // LDW X,($50,SP)
    // PM0044, page 117
    LDW_X_SP_OFFSET,

    // 0x1F
    // XLDW ($50,SP),X
    // (0x1F) LDW, (shortoff,SP) XLDW ($50,SP),X
    // PM0044, page 118
    LDW_SP_OFFSET_X,

    // 0x20, JRA, page 112
    JRA,

    // 0x24
    JRNC,

    // 0x25
    JRC,

    // 0x26
    JRNE,

    // 0x27
    // JREQ, PM0044, page 113
    JREQ,

    // 0x28, page 68
    JRNV,

    // 0x29, page 68
    JRV,

    // 0x2A
    // JRPL, PM0044, page 113
    JRPL,

    // 0x2B, page 113
    JRMI,

    // 0x2E
    JRSGE,

    // 0x2F
    JRSLT,

    // 0x34
    SRL_SHORTMEM,

    // 0x35
    // MOV, page 119
    MOV,

    // 0x36, page 137
    RRC_SHORTMEM,

    // 0x39
    RLC_SHORTMEM,

    // 0x41
    // CPL, Logical 1’s Complement of the accumulator
    CPL_A,

    // 0x48
    SLL_A,

    // 0x49
    RLC_A,

    // 0x4A, page 99
    DEC_A,

    // 0x4B
    // PUSH, PM0044, page 70
    PUSH,

    // 0x4D
    // TNZ, PM0044, page 155
    TNZ_A,

    // 0x4F
    // CLEAR, PM0044, page 92
    CLR_A,

    // 0x51
    // EXGW, PM0044, page 104
    EXGW,

    // 0x52
    // SUB, PM0044, page 151
    SUB_SP,

    // 0x55
    // MOV longmem, longmem
    MOV_LONGMEM_LONGMEM,

    // 0x58, page 146
    // SLLW X
    SLLW_X,

    // 0x59, rlcw page 135
    RLCW,

    // 0x5B
    ADDW_SP_IMM,

    // 0x5D
    // TNZW, PM0044, page 156
    TNZW,

    // 0x5F
    // CLRW_X, PM0044, page 93
    CLRW_X,

    // 0x6B, page 115
    LD_OFFSET_SP_A,

    // 0x72, bitpos
    // Bit Test and Jump if True
    // page 87
    BTJT,

    // 0xE4, page 79
    AND_A_X_OFFSET,

    // A9, page 76
    ADC_A_IMM,

    // 0x72 0x5F, page 92
    CLR_LONGMEM,

    // 0x72 0xA9, page 78
    //ADC_A_IMM,
    ADDW_Y_IMM,

    // 0x72 0xF9
    ADDW_Y_OFFSET_SP,

    // 0x72 0xFB
    ADDW_X_OFFSET_SP,

    // 0x72, 0x1a
    BSET,

    // 0x72, 0x1b
    BRES,

    // 0x7B
    //LD A,($12,SP)
    LD_A_OFFSET_SP,

    // 0x7D
    TNZ_X,

    // 0x81
    // RET, PM0044, page 131
    RET,

    // 0x82
    // INT, INTERRUPT
    INT,

    // 0x84
    POP_A,

    // 0x85
    POPW,

    // 0x87, page 132, Far Return from subroutine
    RETF,

    // 0x88, page 128, PUSH A, Push into the Stack
    PUSH_A,

    // 0x89
    PUSHW,

    // 0x8D,
    CALLF_EXTMEM,

    // 0x8E
    HALT,

    // 0x8F
    WFI,

    // 0x90, 0x01
    RRWA_Y,

    // LDW SP,Y
    // 0x90, 0x94
    LDW_SP_Y,

    // LDW Y,SP
    // 0x90 0x96
    LDW_Y_SP,

    // 0x90 0xAE, page 118
    // LDW, Y, IMM
    LDW_Y_IMM,

    // 0x90 0xCE, page 118
    LDW_Y_IMM_LONGMEM,

    // 0x90, 0xE3
    // CPW X,($10,Y)
    CPW_X_SHORTOFF_Y,

    // 0x90, 0x28
    JRNH,

    // 0x90, 0x29
    JRH,

    // 0x90 0x2C bb
    JRNM,

    // 0x90, 0x58
    SLLW_Y,

    // 0x5C
    INCW,

    // 0x90, 0x5C
    INCW_Y,

    // 0x90, 0x7D
    TNZ_Y,

    // 0x90, 0x93, page 118
    LDW_Y_X,

    // 0x90, 0x96
    // LDW X,SP
    LDW_X_SP,

    // 0x90, 0xB3, page 96
    CPW_Y_SHORTMEM,

    // 0x90, 0xE8
    XOR_A_OFFSET_Y,

    // 0x90, 0xEE, page 118
    LDW_Y_SHORTOFF_Y,

    // CPW_Y_IMM
    // 0x90 0xA3
    CPW_Y_IMM,

    // 0x93
    LDW_X_Y,

    // 0x94
    LDW_SP_X,

    // 0x95
    LD_XH_A,

    // 0x97
    LD_XL_A,

    // 0x98
    RCF,

    // 0x99
    SCF,

    // 0x9A, page 70, Reset interrupt mask / Interrupt enable
    RIM,

    // 0x9B, page 71, Set interrupt mask / Disable interrupts
    SIM,

    // 0x9C
    RVF,

    // 0x9D, page 124
    NOP,

    // A0
    SUB_A_IMM,

    // A1
    // CP A,#$10
    CP_A_IMM,

    // 0xA3
    CPW_X_IMM,

    // 0xA4
    AND_A,

    // 0xA5, page 81
    BCP_A_IMM,

    // 0xA6
    LD_A_IMM,

    // 0xAA
    OR_A,

    // 0xAB
    ADD_A,

    // 0xAD
    CALLR,

    // 0xAE
    // LDW, LOAD
    LDW_AE,

    // 0xB6
    LD_A_SHORTMEM,

    // 0xB7
    LD_IMM_A,

    // 0xBE XX, page 117
    LDW_X_IMM_SHORTMEM,

    // 0xBF XX, page 117
    LDW_IMM_SHORTMEM_X,

    // 0xC6, page 114
    LD_A,

    // 0xC7, page 115
    LD_A_LONGMEM,

    // 0xC8, page 160
    XOR_A_LONGMEM,

    // 0xCC
    // UNDOCUMENTED!!!!!!!!
    CALL_CC,

    // 0xCD
    // CALL, PM0044, page 88
    CALL_CD,

    // 0xCE, page 117
    LDW_X_IMM,

    // 0xCF
    LDW_IMM_X,

    // 0xE1,page 94
    CP_A_OFFSET_X,

    // 0xE6, page 114, LD A,($50,X)
    LD_A_OFFSET_X,

    // 0xE7, page 115, LD ($50,X),A
    LD_X_OFFSET_A,

    // 0xF1, page 94
    CP_A_X_MEMORY,

    // 0xF6, page 114
    // LD A,(X)
    LD_A_X_MEMORY,

    // 0xF7, LD (X),A, page 115
    //LD_X_MEMORY_A,

    // 0xFE
    //LDW_X_X,

    // 0x90 0xF6, page 114
    LD_A_Y,

    // 0xEE, page 117
    // LDW X,($50,X)
    LDW_X_IMM_X,

    // 0xF7, page 115
    LD_X_MEMORY_A,

    // 0xFE, page 117
    LDW_X_X_MEMORY,

    // 0x90, 0xFE, page 118
    LDW_Y_Y_MEMORY,

    // 0xBC, page 116
    LDF_A_EXTMEM,

    // 0x92 0xAF
    LDF_A_X_OFFSET,

    // 0xF8
    XOR_A_X,

    UNDEFINED,

}

impl FromStr for Instruction {

    type Err = ();

    fn from_str(input: &str) -> Result<Instruction, Self::Err> {

        match input.to_uppercase().as_ref() {

            // 0x13
            "CPW_X_SP_OFFSET" => Ok(Instruction::CPW_X_SP_OFFSET),

            // 0x16
            "LDW_Y_SP_OFFSET" => Ok(Instruction::LDW_Y_SP_OFFSET),

            // 0x1E
            "LDW_X_SP_OFFSET" => Ok(Instruction::LDW_X_SP_OFFSET),
            // 0x1F
            "LDW_SP_OFFSET_X" => Ok(Instruction::LDW_SP_OFFSET_X),

            // 0x20
            "JRA" => Ok(Instruction::JRA),

            // 0x26
            "JRNE" => Ok(Instruction::JRNE),

            // 0x27
            "JREQ" => Ok(Instruction::JREQ),

            // 0x4B
            "PUSH" => Ok(Instruction::PUSH),

            // 0x49
            "RLC_A" => Ok(Instruction::RLC_A),

            // 0x4D
            "TNZ_A" => Ok(Instruction::TNZ_A),

            // 0x4F
            "CLR_A" => Ok(Instruction::CLR_A),

            // 0x58
            "SLLW_X" => Ok(Instruction::SLLW_X),

            // 0x72 0xFB
            "ADDW_X_OFFSET_SP" => Ok(Instruction::ADDW_X_OFFSET_SP),

            // 0x72
            "BSET" => Ok(Instruction::BSET),

            // 0x7B
            "LD_A_OFFSET_SP" => Ok(Instruction::LD_A_OFFSET_SP),

            // 0x7D
            "TNZ_X" => Ok(Instruction::TNZ_X),

            // 0x81
            "RET" => Ok(Instruction::RET),

            // 0x82
            "INT" => Ok(Instruction::INT),

            // 0x95
            "LD_XH_A" => Ok(Instruction::LD_XH_A),

            // 0x96
            "LDW_X_SP" => Ok(Instruction::LDW_X_SP),

            // 0x97
            "LD_XL_A" => Ok(Instruction::LD_XL_A),

            // 0xA3
            "CPW_X_OFFSET" => Ok(Instruction::CPW_X_IMM),

            // 0xA4
            "AND_A" => Ok(Instruction::AND_A),

            "BCP_A_IMM" => Ok(Instruction::BCP_A_IMM),

            // 0xAE
            "LDW_AE" => Ok(Instruction::LDW_AE),

            // 0xC6
            "LDW_A" => Ok(Instruction::LD_A),

            // 0xC7
            "LDW_A_LONGMEM" => Ok(Instruction::LD_A_LONGMEM),

            // 0xCC, UNDOCUMENTED !!!!!!!!
            "CALL_CC" => Ok(Instruction::CALL_CC),

            // 0xCD
            "CALL_CD" => Ok(Instruction::CALL_CD),

            // LDW_X_X
            //"LDW_X_X" => Ok(Instruction::LDW_X_X),

            _ => todo!(),

        }
    }
}

impl fmt::Display for Instruction {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {

            // 0x01
            Instruction::RRWA_X => write!(f, "rrwa"),

            // 0x08
            Instruction::SLL_SP_OFFSET => write!(f, "sll"),
            // 0x09
            Instruction::RLC_SP_OFFSET => write!(f, "rlc"),
            // 0x0C
            Instruction::INC_SP_OFFSET => write!(f, "inc"),
            // 0x0F
            Instruction::CLR_SP_OFFSET => write!(f, "clr"),

            // 0x10
            Instruction::SUB_A_OFFSET_SP => write!(f, "sub"),

            // 0x1C
            Instruction::ADDW_X_IMM => write!(f, "addw"),

            // 0x13
            Instruction::CPW_X_SP_OFFSET => write!(f, "cpw"),

            // 0x16, LDW Y,($50,SP)
            Instruction::LDW_Y_SP_OFFSET => write!(f, "ldw"),
            // 0x17, LDW ($50,SP),Y
            Instruction::LDW_OFFSET_SP_Y => write!(f, "ldw"),

            // 0x18, XOR A,($10,SP)
            Instruction::XOR_A_OFFSET_SP => write!(f, "xor"),

            // 0x1A, OR A,($10,SP), page 125
            Instruction::OR_A_OFFSET_SP => write!(f, "or"),

            // 0x1D
            Instruction::SUBW_X_IMM => write!(f, "subw"),

            // 0x1E, LDW X,($50,SP)
            Instruction::LDW_X_SP_OFFSET => write!(f, "ldw"),
            // 0x1F, LDW ($50,SP),X
            Instruction::LDW_SP_OFFSET_X => write!(f, "ldw"),

            // 0x20
            Instruction::JRA => write!(f, "jra"),

            // 0x24, JRNC
            Instruction::JRNC => write!(f, "jrnc"),

            // 0x25, jump if carry flag is true (also called NOT EQUAL)
            Instruction::JRC => write!(f, "jrc"),

            // 0x26, page 68, jump if zero flag is false (also called NOT EQUAL)
            Instruction::JRNE => write!(f, "jrne"),

            // 0x27
            Instruction::JREQ => write!(f, "jreq"),

            // 0x90 0x2C bb
            Instruction::JRNM => write!(f, "jrnm"),

            // 0x28
            Instruction::JRNV => write!(f, "jrnv"),
            // 0x29
            Instruction::JRV => write!(f, "jrv"),

            // 0x2A
            Instruction::JRPL => write!(f, "jrpl"),
            // 0x2B
            Instruction::JRMI => write!(f, "jrmi"),

            // 0x2E
            Instruction::JRSGE => write!(f, "jrsge"),

            // 0x2F
            Instruction::JRSLT => write!(f, "jrslt"),

            // 0x34
            Instruction::SRL_SHORTMEM => write!(f, "srl"),

            // 0x35
            Instruction::MOV => write!(f, "mov"),

            // 0x36
            Instruction::RRC_SHORTMEM => write!(f, "rrc"),

            // 0x39
            Instruction::RLC_SHORTMEM => write!(f, "rlc"),

            // 0x43
            Instruction::CPL_A => write!(f, "cpl"), // Logical 1’s Complement of accumulator

            // 0x48
            Instruction::SLL_A => write!(f, "sll"),

            // 0x4A
            Instruction::DEC_A => write!(f, "dec"),

            // 0x4B, page 128
            Instruction::PUSH => write!(f, "push"),

            // 0x49, page 134
            Instruction::RLC_A => write!(f, "rlc"),

            // 0x4D
            Instruction::TNZ_A => write!(f, "tnz"),

            // 0x4F
            Instruction::CLR_A => write!(f, "clr"),

            // 0x51
            Instruction::EXGW => write!(f, "exgw"),

            // 0x52, page 151
            Instruction::SUB_SP => write!(f, "sub"),

            // 0x55
            Instruction::MOV_LONGMEM_LONGMEM => write!(f, "mov"),

            // 0x59
            Instruction::RLCW => write!(f, "rlcw"),

            // 0x5B
            Instruction::ADDW_SP_IMM => write!(f, "addw"),

            // 0x5C
            Instruction::INCW => write!(f, "incw"),

            // 90 5C
            Instruction::INCW_Y => write!(f, "incw"),

            // 0x5D
            Instruction::TNZW => write!(f, "tnzw"),

            // 0x5F
            Instruction::CLRW_X => write!(f, "clrw"),

            // 0x58, page 146
            Instruction::SLLW_X => write!(f, "sllw"),

            // 0x6B, page 115
            Instruction::LD_OFFSET_SP_A => write!(f, "ld"),

            // 0x72, bitpos
            // page 87
            Instruction::BTJT => write!(f, "btjt"),

            // 0x72 0x5F, page 92
            Instruction::CLR_LONGMEM => write!(f, "clr"),

            // 0x72 0xA9, page 78
            Instruction::ADDW_Y_IMM => write!(f, "addw"),

            // 0x72 0xF9
            Instruction::ADDW_Y_OFFSET_SP => write!(f, "addw"),

            // 0x72 0xFB
            Instruction::ADDW_X_OFFSET_SP => write!(f, "addw"),

            // 0x72, 0x1a
            Instruction::BSET => write!(f, "bset"),

            // 0x72, 0x1b
            Instruction::BRES => write!(f, "bres"),

            // 0x7B
            Instruction::LD_A_OFFSET_SP => write!(f, "ld"),

            // 0x7D
            Instruction::TNZ_X => write!(f, "tnz"),

            // 0x81
            Instruction::RET => write!(f, "ret"),

            // 0x82
            Instruction::INT => write!(f, "int"),

            // 0x84
            Instruction::POP_A => write!(f, "pop"),

            // 0x85
            Instruction::POPW => write!(f, "popw"),

            // 0x87, page 132, Far Return from subroutine
            Instruction::RETF => write!(f, "retf"),

            // 0x88
            Instruction::PUSH_A => write!(f, "push"),

            // 0x89
            Instruction::PUSHW => write!(f, "pushw"),

            // 0x8D
            Instruction::CALLF_EXTMEM => write!(f, "CALLF_EXTMEM"),

            // 0x8E
            Instruction::HALT => write!(f, "halt"),

            // 0x8F
            Instruction::WFI => write!(f, "wfi"),

            // 0x90, 0x01
            Instruction::RRWA_Y => write!(f, "rrwa"),

            // 0x90, 0x28, page 68
            Instruction::JRNH => write!(f, "jrnh"),

            // 0x90, 0x29
            Instruction::JRH => write!(f, "jrh"),

            // 0x90, 0x58
            Instruction::SLLW_Y => write!(f, "sllw"),

            // 0x90, 0x7D
            Instruction::TNZ_Y => write!(f, "tnz"),

            // 0x90, 0x93, page 118
            Instruction::LDW_Y_X => write!(f, "ldw"),

            // 0x90, 0xA3
            Instruction::CPW_Y_IMM => write!(f, "cpw"),

            // 0x90, 0xAE
            Instruction::LDW_Y_IMM => write!(f, "ldw"),

            // 0x90, 0xB3
            Instruction::CPW_Y_SHORTMEM => write!(f, "cpw"),

            // 0x90, 0xCE
            Instruction::LDW_Y_IMM_LONGMEM => write!(f, "ldw"),

            // 0x90, 0xE3, page 95
            Instruction::CPW_X_SHORTOFF_Y => write!(f, "cpw"),

            // 0x90, 0xE8, page 160
            Instruction::XOR_A_OFFSET_Y => write!(f, "xor"),

            // 0x90, 0xEE, page 118
            Instruction::LDW_Y_SHORTOFF_Y => write!(f, "ldw"),

            // 0x90, 0xF6, page 114
            Instruction::LD_A_Y => write!(f, "ld"),

            // 0x90
            Instruction::LDW_Y_SP => write!(f, "ldw"),

            // 0xBC, page 116
            Instruction::LDF_A_EXTMEM => write!(f, "ldf"),

            // 0x92 0xBC, page 116
            Instruction::LDF_A_LONGPTR_E => write!(f, "ldf"),

            // 0x92 0xAF
            Instruction::LDF_A_X_OFFSET => write!(f, "ldf"),

            // 0x94
            Instruction::LDW_SP_X => write!(f, "ldw"),

            // 0x90 0x94
            Instruction::LDW_SP_Y => write!(f, "ldw"),

            // 0x93
            Instruction::LDW_X_Y => write!(f, "ldw"),

            // 0x95
            Instruction::LD_XH_A => write!(f, "ld"),

            // 0x96
            Instruction::LDW_X_SP => write!(f, "ldw"),

            // 0x97
            Instruction::LD_XL_A => write!(f, "ld"),

            // 0x98
            Instruction::RCF => write!(f, "rcf"),

            // 0x99
            Instruction::SCF => write!(f, "scf"),

            // 0x9A
            Instruction::RIM => write!(f, "rim"),

            // 0x9B
            Instruction::SIM => write!(f, "sim"),

            // 0x9C
            Instruction::RVF => write!(f, "rvf"),

            // 0x9D
            Instruction::NOP => write!(f, "nop"),

            // 0xA0
            Instruction::SUB_A_IMM => write!(f, "sub"),

            // 0xA1
            Instruction::CP_A_IMM => write!(f, "cp"),

            // 0xA3
            Instruction::CPW_X_IMM => write!(f, "cpw"),

            // 0xA4
            Instruction::AND_A => write!(f, "and"),

            // 0xA5
            Instruction::BCP_A_IMM => write!(f, "bcp"),

            // 0xA6
            // LD A,#$55
            Instruction::LD_A_IMM => write!(f, "ld"),

            // 0xA9, page 76
            Instruction::ADC_A_IMM => write!(f, "adc"),

            // 0xAA
            Instruction::OR_A => write!(f, "or"),
            // 0xAB
            Instruction::ADD_A => write!(f, "add"),

            // 0xAD
            Instruction::CALLR => write!(f, "CALLR"),

            // 0xAE, page 117, load word into X
            Instruction::LDW_AE => write!(f, "ldw"),

            // 0xB6, page 114
            Instruction::LD_A_SHORTMEM => write!(f, "ld"),

            // 0xB7
            Instruction::LD_IMM_A => write!(f, "ld"),

            // 0xBE, page 117
            Instruction::LDW_X_IMM_SHORTMEM => write!(f, "ldw"),

            // 0xBF, page 117
            Instruction::LDW_IMM_SHORTMEM_X => write!(f, "ldw"),

            // 0xC6
            Instruction::LD_A => write!(f, "ld_a"),
            // 0xC7
            Instruction::LD_A_LONGMEM => write!(f, "ld"),
            // 0xC8, page 160
            Instruction::XOR_A_LONGMEM => write!(f, "xor"),
            // 0xCC
            Instruction::CALL_CC => write!(f, "call_cc"),
            // 0xCD
            Instruction::CALL_CD => write!(f, "call_cd"),

            // 0xCE
            Instruction::LDW_X_IMM => write!(f, "ldw"),
            // 0xCF
            Instruction::LDW_IMM_X => write!(f, "ldw"),

            // 0xE1, page 94, CP A,($10,X)
            Instruction::CP_A_OFFSET_X => write!(f, "cp"),

            // 0xE4, page 79
            Instruction::AND_A_X_OFFSET => write!(f, "and"),

            // 0xE6, page 114, LD A,($50,X)
            Instruction::LD_A_OFFSET_X => write!(f, "ld"),

            // 0xE7, page 115, LD ($50,X),A
            Instruction::LD_X_OFFSET_A => write!(f, "ld"),

            // 0xEE, page 117
            Instruction::LDW_X_IMM_X => write!(f, "ldw"),

            // 0xF1, page 94
            Instruction::CP_A_X_MEMORY => write!(f, "cp"),

            // 0xF6, page 114
            Instruction::LD_A_X_MEMORY => write!(f, "ld"),
            // 0xF7, page 115
            Instruction::LD_X_MEMORY_A => write!(f, "ld"),

            //// 0xFA, page 114
            //Instruction::LD_A_MEMORY_X => write!(f, "ld a, (x)"),

            // 0xFE, page 117
            Instruction::LDW_X_X_MEMORY => write!(f, "ldw"),

            // 0x90, 0xFE, page 118
            Instruction::LDW_Y_Y_MEMORY => write!(f, "ldw"),

            // 0xF8
            Instruction::XOR_A_X => write!(f, "xor"),

            Instruction::UNDEFINED => write!(f, "undefined"),

        }

    }
}
