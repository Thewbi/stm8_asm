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
    pub label: String,
    pub label_target: String,
    pub jump_offset: i32,
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
            label: String::new(),
            label_target: String::new(),
            jump_offset: 0,
        }
    }

    pub fn _clear(&mut self) {
        self.byte_count = 0;
        self.instruction = Instruction::UNDEFINED;
        self.reg1 = Register::UNDEFINED;
        self.reg2 = Register::UNDEFINED;
        self.reg3 = Register::UNDEFINED;
        self.immediate = 0;
        self.label = String::new();
        self.label_target = String::new();
        self.jump_offset = 0;
    }

}

impl fmt::Display for ASMLine {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        if self.instruction == Instruction::UNDEFINED {
            return Ok(());
        }

        match self.instruction {

            // JREQ (0x27), PM0044, page 113
            Instruction::TNZ_A => write!(
                f,
                "{} {}",
                self.label, self.instruction
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
                "{} {}",
                self.label, self.instruction
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
                "{} {} #{}",
                self.label, self.instruction, self.immediate
            ),

            // LOAD (0xAE),
            Instruction::LDW_AE => write!(
                f,
                "{} {} #{}",
                self.label, self.instruction, self.immediate
            ),

            // CALL (0xCD), PM0044, page 88
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