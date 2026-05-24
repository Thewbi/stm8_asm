use std::fmt;
use String;

use crate::ast::instruction::Instruction;
use crate::ast::register::Register;

#[derive(Clone, Debug)]
pub struct ASMLine {
    pub instruction: Instruction,
    pub _param_idx: u32,
    pub reg1: Register,
    pub reg2: Register,
    pub reg3: Register,
    pub immediate: i32,
    pub label: String,
    pub label_target: String,
    pub jump_offset: i32,
    pub update_flags: bool,
}

impl ASMLine {

    pub fn new() -> ASMLine {
        ASMLine {
            instruction: Instruction::UNDEFINED,
            _param_idx: 0,
            reg1: Register::UNDEFINED,
            reg2: Register::UNDEFINED,
            reg3: Register::UNDEFINED,
            immediate: 0,
            label: String::new(),
            label_target: String::new(),
            jump_offset: 0,
            update_flags: false,
        }
    }

    pub fn _clear(&mut self) {
        self.instruction = Instruction::UNDEFINED;
        self._param_idx = 0;
        self.reg1 = Register::UNDEFINED;
        self.reg2 = Register::UNDEFINED;
        self.reg3 = Register::UNDEFINED;
        self.immediate = 0;
        self.label = String::new();
        self.label_target = String::new();
        self.jump_offset = 0;
        self.update_flags = false;
    }

}

impl fmt::Display for ASMLine {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        if self.instruction == Instruction::UNDEFINED {
            return Ok(());
        }

        match self.instruction {
            Instruction::ASR => write!(
                f,
                "{} {} {}, {}, #{}",
                self.label, self.instruction, self.reg1, self.reg2, self.immediate
            ),
            Instruction::ADD => {
                if self.update_flags {
                    write!(
                        f,
                        "{} {}s {}, #{}",
                        self.label, self.instruction, self.reg1, self.immediate
                    )
                } else {
                    write!(
                        f,
                        "{} {} {}, {}, {}",
                        self.label, self.instruction, self.reg1, self.reg2, self.reg3
                    )
                }
            }

            Instruction::B => write!(f, "{} {} {}", self.label, self.instruction, self.immediate),
            Instruction::BAL => {
                if self.label_target.len() != 0 {
                    write!(
                        f,
                        "{} {} {}",
                        self.label, self.instruction, self.label_target
                    )
                } else {
                    write!(
                        f,
                        "{} {} {}",
                        self.label, self.instruction, self.jump_offset
                    )
                }
            }
            Instruction::BL => write!(f, "{} {} {}", self.label, self.instruction, self.immediate),
            Instruction::BEQ => write!(
                f,
                "{} {} {}/#{}",
                self.label, self.instruction, self.label_target, self.jump_offset
            ),
            Instruction::BGT => {
                if self.label_target.len() != 0 {
                    write!(
                        f,
                        "{} {} {}",
                        self.label, self.instruction, self.label_target
                    )
                } else {
                    write!(
                        f,
                        "{} {} {}",
                        self.label, self.instruction, self.jump_offset
                    )
                }
            },
            Instruction::BIC => {
                write!(f, "{} {} {}, [{}, #0x{:02x}]", self.label, self.instruction, self.reg2, self.reg1, self.immediate)
            },
            Instruction::BX => write!(f, "{} {} {}", self.label, self.instruction, self.reg1),

            Instruction::CMP => write!(
                f,
                "{} {} {}, {}, #{}",
                self.label, self.instruction, self.reg1, self.reg2, self.immediate
            ),

            Instruction::LDR => 
                write!(f, "{} {} {} {} #{}", self.label, self.instruction, self.reg1, self.reg2, self.immediate),
            Instruction::LDR_W => 
                write!(f, "{} {} {}, [{}, #0x{:02x}]", self.label, self.instruction, self.reg2, self.reg1, self.immediate),

            Instruction::MOV => {
                if self.reg2 == Register::UNDEFINED {
                    write!(
                        f,
                        "{} {} {}, #0x{:02x}",
                        self.label, self.instruction, self.reg1, self.immediate
                    )
                } else {
                    write!(
                        f,
                        "{} {} {}, #{}",
                        self.label, self.instruction, self.reg1, self.reg2
                    )
                }
            },
            Instruction::MOV_W => {
                write!(
                    f,
                    "{} {} {}, #0x{:02x}",
                    self.label, self.instruction, self.reg1, self.immediate
                )
            },

            Instruction::ORR_W => 
                write!(f, "{} {} {}, [{}, #0x{:02x}]", self.label, self.instruction, self.reg2, self.reg1, self.immediate),

            Instruction::STR => 
                write!(f, "{} {} {} {} #{}", self.label, self.instruction, self.reg1, self.reg2, self.immediate),
            Instruction::STR_W => 
                write!(f, "{} {} {}, [{}, #0x{:02x}]", self.label, self.instruction, self.reg2, self.reg1, self.immediate),
            // now: svc (Supervisor Call), earlier: swi (Software Interrupt)
            Instruction::SVC => {
                write!(f, "{} {} #{}", self.label, self.instruction, self.immediate)
            }
            Instruction::SWI => {
                write!(f, "{} {} #{}", self.label, self.instruction, self.immediate)
            }

            Instruction::WFI => write!(f, "{} {}", self.label, self.instruction),

            _ => write!(
                f,
                "{} {:?} {:?}, {:?}",
                self.label, self.instruction, self.reg1, self.reg2
            ),
        }
    }
}
