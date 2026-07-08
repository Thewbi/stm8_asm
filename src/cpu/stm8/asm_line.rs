use std::fmt;
use String;

use crate::cpu::stm8::instruction::Instruction;
use crate::cpu::stm8::register::Register;

#[derive(Clone, Debug)]
pub struct ASMLine {
    pub byte_count: u32,
    pub instruction: Instruction,
    pub reg1: Register,
    pub reg2: Register,
    pub reg3: Register,
    pub immediate: i32,
    pub immediate_2: i32,
    pub label: String,
    pub label_target: String,
    pub jump_offset: i32,
    pub bit_pos: u8,
}

impl ASMLine {

    pub fn new() -> ASMLine {
        ASMLine {
            byte_count: 0,
            instruction: Instruction::UNDEFINED,
            reg1: Register::UNDEFINED,
            reg2: Register::UNDEFINED,
            reg3: Register::UNDEFINED,
            immediate: 0,
            immediate_2: 2,
            label: String::new(),
            label_target: String::new(),
            jump_offset: 0,
            bit_pos: 0,
        }
    }

    pub fn _clear(&mut self) {
        self.byte_count = 0;
        self.instruction = Instruction::UNDEFINED;
        self.reg1 = Register::UNDEFINED;
        self.reg2 = Register::UNDEFINED;
        self.reg3 = Register::UNDEFINED;
        self.immediate = 0;
        self.immediate_2 = 0;
        self.label = String::new();
        self.label_target = String::new();
        self.jump_offset = 0;
        self.bit_pos = 0;
    }

}

impl fmt::Display for ASMLine {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        if self.instruction == Instruction::UNDEFINED {
            return Ok(());
        }

        match self.instruction {

            //
            // ADC, page 75
            //

            //
            // ADD, page 77
            //

            //
            // ADW, page 78
            //

            // 1C MS LS, page 78, ADDW X,#$1000, 
            Instruction::ADDW_X_IMM => write!(
                f,
                "{} adw x, #{:04x}",
                self.label, self.immediate
            ),

            //
            // AND, page 79
            //

            // 0xA4 (AND)
            Instruction::AND_A => write!(
                f,
                "{} and a, #{}",
                self.label, self.immediate
            ),

            // 0xE4 (AND), page 79
            Instruction::AND_A_X_OFFSET => write!(
                f,
                "{} and a, (#{}, x)",
                self.label, self.immediate
            ),

            // BCCM, page 80

            // BCP, page 81

            // BCP_A_IMM, page 81
            Instruction::BCP_A_IMM => write!(
                f,
                "{} {} a, 0x{:02x}",
                self.label, self.instruction, self.immediate
            ),

            // BCPL, page 82

            // BREAK, page 83

            // BRES, page 84
            // BSET, page 85
            // BTJF, page 86
            // BTJT, page 87
            // CALL, page 88
            // CALLF, page 89
            // CALLR, page 90
            // CCF, page 91
            // CLR, page 92
            
            //
            // CLRW, page 93
            //

            Instruction::CLRW_X => write!(
                f,
                "{} clrw x",
                self.label
            ),

            //
            // CP, page 94
            //

            // 0xE1, page 94
            Instruction::CP_A_OFFSET_X => write!(
                f,
                "{} {} a, (0x{:04x}, x)",
                self.label, self.instruction, self.immediate
            ),

            // 0xF1, page 94
            Instruction::CP_A_X_MEMORY => write!(
                f,
                "{} {} a, (x)",
                self.label, self.instruction
            ),

            //
            // CPW, page 95
            //

            Instruction::CPW_X_IMM => write!(
                f,
                "{} {} x, (0x{:04x})",
                self.label, self.instruction, self.immediate
            ),

            // 0x90, 0xB3, page 96
            Instruction::CPW_Y_SHORTMEM => write!(
                f,
                "{} {} y, (0x{:02x})",
                self.label, self.instruction, self.immediate
            ),

            // CPL, page 97

            // 0x43, page 97
            Instruction::CPL_A => write!(
                f,
                "{} cpl a",
                self.label
            ),

            // CPLW, page 98
            // DEC, page 99
            // DECW, page 100
            // DIV, page 101
            // DIVW, page 102
            // EXG, page 103
            // EXGW, page 104
            // HALT, page 105

            //
            // INC, page 106
            //
            
            //
            // INCW, page 107
            //

            // 0x5C, INCW, PM0044, page 107
            Instruction::INCW => write!(
                f,
                "{} {} x",
                self.label, self.instruction
            ),

            // 90 5C, INCW_Y, PM0044, page 107
            Instruction::INCW_Y => write!(
                f,
                "{} {} y",
                self.label, self.instruction
            ),

            // INT, page 108
            // IRET, page 109
            // JP, page 110
            // JPF, page 111

            //
            // JRA, page 112
            //

            // 20 XX, JRA, PM0044, page 112
            Instruction::JRA => write!(
                f,
                "{} {} 0x{:02x} == {} (Jump Relative Always)",
                self.label, self.instruction, self.jump_offset as u8, self.jump_offset
            ),

            //
            // JRxx, page 113
            //

            Instruction::JRNE => write!(
                f,
                "{} {} 0x{:02x} == {}dec",
                self.label, self.instruction, self.jump_offset, self.jump_offset as i8
            ),

            // 0x25, jump if carry flag is true (also called NOT EQUAL), page 113
            Instruction::JRC => write!(
                f,
                "{} {} 0x{:02x} == {}dec",
                self.label, self.instruction, self.jump_offset, self.jump_offset as i8
            ),

            //
            // LD, page 114
            //

            // 0x7B
            // LD A,($12,SP)
            Instruction::LD_A_OFFSET_SP => write!(
                f,
                "{} {} a, ({}, sp)",
                self.label, self.instruction, self.immediate
            ),

            // 0x90, 0xF6, page 114
            Instruction::LD_A_Y => write!(
                f,
                "{} {} a, (y)",
                self.label, self.instruction
            ),

            // 0xB6, page 114
            Instruction::LD_A_SHORTMEM => write!(
                f,
                "{} {} a, (0x{:02x})",
                self.label, self.instruction, self.immediate
            ),

            // 0xB7
            Instruction::LD_IMM_A => write!(
                f,
                "{} {} 0x{:02x}, a ",
                self.label, self.instruction, self.immediate
            ),

            // 0xE6, page 114, LD A,($50,X)
            Instruction::LD_A_OFFSET_X => write!(
                f,
                "{} {} a, ({:02x}, x)",
                self.label, self.instruction, self.immediate
            ),

            // 0xE7, page 115
            Instruction::LD_X_OFFSET_A => write!(
                f,
                "{} {} ({:02x}, x), a",
                self.label, self.instruction, self.immediate
            ),

            // 0xF7, page 115
            Instruction::LD_X_MEMORY_A => write!(
                f,
                "{} {} (x), a",
                self.label, self.instruction
            ),

            //
            // LDF, page 116
            //

            //
            // LDW, page 117
            //

            // 0x90, 0x96
            // LDW X,SP
            Instruction::LDW_X_SP => write!(
                f,
                "{} ldw x, sp",
                self.label
            ),

            // 0x1E
            // LDW X,($50,SP)
            // PM0044, page 117
            Instruction::LDW_X_SP_OFFSET => write!(
                f,
                "{} {} X, (0x{:02x}, SP)",
                self.label, self.instruction, self.immediate
            ),

            // 0x1F, LDW ($50,SP),X
            Instruction::LDW_SP_OFFSET_X => write!(
                f,
                "{} {} (0x{:02x}, SP), X",
                self.label, self.instruction, self.immediate
            ),

            // 0xBE, page 117
            Instruction::LDW_X_IMM_SHORTMEM => write!(
                f,
                "{} {} x, (0x{:02x})",
                self.label, self.instruction, self.immediate
            ),

            // 0x90, 0x93, page 118
            Instruction::LDW_Y_X => write!(
                f,
                "{} {} y, x",
                self.label, self.instruction
            ),

            // 0x90, 0xEE, page 118
            Instruction::LDW_Y_SHORTOFF_Y => write!(
                f,
                "{} {} y, (0x{:02x}, y)",
                self.label, self.instruction, self.immediate
            ),

            // 0x93
            Instruction::LDW_X_Y => write!(
                f,
                "{} {} x, y",
                self.label, self.instruction
            ),

            // page 117
            // LDW $5000,X
            // 0xCF MS LS
            Instruction::LDW_IMM_X => write!(
                f,
                "{} {} {:04x}, x",
                self.label, self.instruction, self.immediate
            ),

            // 0xFE, page 117
            Instruction::LDW_X_X_MEMORY => write!(
                f,
                "{} {} x, (x)",
                self.label, self.instruction
            ),

            //
            // MOV, page 119
            //

            // 0x35
            Instruction::MOV => write!(
                f,
                "{} mov 0x{:04x}, 0x{:02x}",
                self.label, self.jump_offset, self.immediate 
            ),

            // 0x55
            Instruction::MOV_LONGMEM_LONGMEM => write!(
                f,
                "{} mov 0x{:04x}, 0x{:04x}",
                self.label, self.immediate_2, self.immediate
            ),

            // MUL, page 120
            // NEG, page 121
            // NEGW, page 123

            //
            // NOP, page 124
            //

            Instruction::NOP => write!(
                f,
                "{} nop",
                self.label
            ),

            //
            // OR, page 125
            //

            // 0xAA (OR_A)
            Instruction::OR_A => write!(
                f,
                "{} or a, #{}",
                self.label, self.immediate
            ),

            // 0x1A, OR A,($10,SP), page 125
            Instruction::OR_A_OFFSET_SP => write!(
                f,
                "{} or a, (#{}, SP)",
                self.label, self.immediate
            ),

            //
            // POP, page 126
            //

            Instruction::POP_A => write!(
                f,
                "{} pop a",
                self.label
            ),

            //
            // POPW, page 127
            //

            // 0x85, page 127
            Instruction::POPW => write!(
                f,
                "{} popw x",
                self.label
            ),

            //
            // PUSH, page 128
            //

            // 0x4B, page 128
            Instruction::PUSH => write!(
                f,
                "{} push #0x{:02x}",
                self.label, self.immediate
            ),

            Instruction::PUSH_A => write!(
                f,
                "{} push a",
                self.label
            ),

            // PUSHW, page 129

            Instruction::PUSHW => write!(
                f,
                "{} pushw x",
                self.label
            ),

            // Instruction::PUSHW_X => write!(
            //     f,
            //     "{} pushw y",
            //     self.label
            // ),

            // RCF, page 130
            // RET, page 131

            //
            // RETF, page 132
            //

            Instruction::RETF => write!(
                f,
                "{} retf",
                self.label
            ),

            // RIM, page 133
            // RLC, page 134

            //
            // RLCW, page 135
            //

            // 0x59
            Instruction::RLCW => write!(
                f,
                "{} rlcw x",
                self.label
            ),
            
            // RLWA, page 136

            //
            // RRC, page 137
            //

            // 0x36
            Instruction::RRC_SHORTMEM => write!(
                f,
                "{} rrc 0x{:02x} // Rotate Right Logical through Carry",
                self.label, self.immediate
            ),

            // RRCW, page 138

            //
            // RRWA, page 139
            //

            // 0x01
            Instruction::RRWA_X => write!(
                f,
                "{} rrwa x, a",
                self.label
            ),

            //
            // RVF, page 140
            //

            Instruction::RVF => write!(
                f,
                "{} rvf",
                self.label
            ),

            // SBC, page  141
            // SCF, page 142
            // SIM, page 143

            //
            // SLL/SLA, page 144
            //

            // 0x48
            Instruction::SLL_A => write!{
                f,
                "{} sll a",
                self.label
            },

            //
            // SLLW/SLAW, page 146
            //

            Instruction::SLLW_X => write!(
                f,
                "{} sllw x",
                self.label
            ),

            // SRA, page 147
            // SRAW, page 148
            // SRL, page 149
            // SRLW, page 150

            //
            // SUB, page 151
            //

            // 0x10
            // SUB A,($10,SP), page 151
            Instruction::SUB_A_OFFSET_SP => write!(
                f,
                "{} sub a, (0x{:02x}, sp)",
                self.label, self.immediate
            ),

            // 0x52 XX, page 151
            Instruction::SUB_SP => write!(
                f,
                "{} sub sp, 0x{:02x}",
                self.label, self.immediate
            ),

            //
            // SUBW, page 152
            //

            // SWAP, page 153
            // SWAPW, page 154
            // TNZ., page 155
            // TNZW, page 156
            // TRAP, page 157
            // WFE, page 158
            // WFI, page 159

            //
            // XOR, page 160
            //

            // 0xC8, page 160
            Instruction::XOR_A_LONGMEM => write!(
                f,
                "{} {} 0x{:04x} == {}",
                self.label, self.instruction, self.immediate as u16, self.immediate as u16
            ),

            // JREQ (0x27), PM0044, page 113
            Instruction::JREQ => write!(
                f,
                "{} {} 0x{:02x} == {} (Jump relative if equal (Z-Bit == 1))",
                self.label, self.instruction, self.jump_offset as u8, self.jump_offset as i8
            ),

            // TNZ (0x4D), PM0044, page 155
            Instruction::TNZ_A => write!(
                f,
                "{} {}",
                self.label, self.instruction
            ),

            // CLEAR (0x4F), PM0044, page 92
            Instruction::CLR_A => write!(
                f,
                "{} clr a",
                self.label
            ),

            // 0x72, 0x1a (BSET)
            Instruction::BSET => write!(
                f,
                "{} bset {}, #{}",
                self.label, self.immediate, self.jump_offset
            ),

            // 0x72, 0x1b (BRES)
            Instruction::BRES => write!(
                f,
                "{} bres {}, #{}",
                self.label, self.immediate, self.jump_offset
            ),

            // RET (0x81), PM0044, page 131
            Instruction::RET => write!(
                f,
                "{} {}",
                self.label, self.instruction
            ),

            // INT, INTERRUPT (0x82),
            Instruction::INT => write!(
                f,
                "{} {} #{:08x}",
                self.label, self.instruction, self.immediate
            ),

            // 0x90 0xAE (LDW, Y, IMM)
            Instruction::LDW_Y_IMM => write!(
                f,
                "{} ldw y, #{:04x?}",
                self.label, self.immediate
            ),

            

            // 0xBF (LDW $50,X), page 117
            Instruction::LDW_IMM_SHORTMEM_X => write!(
                f,
                "{} ldw (0x{:02x}), x",
                self.label, self.immediate
            ),

            // 0xCE
            Instruction::LDW_X_IMM => write!(
                f,
                "{} ldw x, 0x{:04x}",
                self.label, self.immediate
            ),

            // 0xEE, page 117
            Instruction::LDW_X_IMM_X => write!(
                f,
                "{} ldw x, (0x{:02x}, x)",
                self.label, self.immediate
            ),

            // 0x94 (LDW SP,X)
            Instruction::LDW_SP_X => write!(
                f,
                "{} ldw sp, x",
                self.label
            ),

            // 0xA6 (LD_A_IMM)
            Instruction::LD_A_IMM => write!(
                f,
                "{} ld a, #{}",
                self.label, self.immediate
            ),

            

            //
            // LDW
            //

            // 0xAE (LOAD), page 117
            Instruction::LDW_AE => write!(
                f,
                "{} {} #{}",
                self.label, self.instruction, self.immediate
            ),

            // // 0xAE, page 117
            // Instruction::LDW_AE => write!(
            //     f,
            //     "{} ldw x, #{:04x?} ({:?})dec",
            //     self.label, self.immediate, self.immediate
            // ),

            // 0x90 0xCE, page 118
            Instruction::LDW_Y_IMM_LONGMEM => write!(
                f,
                "{} {} y, #{:04x}",
                self.label, self.instruction, self.immediate
            ),

            // 0xC6 (LD_A)
            Instruction::LD_A => write!(
                f,
                "{} ld a, #0x{:04x}",
                self.label, self.immediate
            ),

            // 0xF6, page 114
            Instruction::LD_A_X_MEMORY => write!(
                f,
                "{} ld a, (x)",
                self.label
            ),

            // 0xC7 (LD_A_LONGMEM)
            Instruction::LD_A_LONGMEM => write!(
                f,
                "{} ld #0x{:04x}, a",
                self.label, self.immediate
            ),

            // 0xCD (CALL), PM0044, page 88
            Instruction::CALL_CD => write!(
                f,
                "{} {} #{}",
                self.label, self.instruction, self.immediate
            ),

            // default
            _ => write!(
                f,
                "{} {:?} {:?}, {:?}",
                self.label, self.instruction, self.reg1, self.reg2
            ),

        }
    }
}