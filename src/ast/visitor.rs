/*
use pest::{iterators::Pair};
use crate::Rule; // this use statement will only work if you are compiling main.rs_asm
use crate::ast::instruction::Instruction;

use std::str::FromStr;

use crate::ast::asm_line::ASMLine;
use crate::ast::register::Register;

pub struct Visitor {
    pub asm_lines: Vec<ASMLine>,
    pub current_asm_line: ASMLine
}

impl Visitor {

    pub fn new() -> Visitor {
        Visitor { 
            asm_lines: Vec::new(),
            current_asm_line: ASMLine::new(),
        }
    }

    pub fn enter_asm_line(&mut self, pair: &mut Pair<'_, Rule>) {
        println!("[enter] asm_line: {}", pair.to_string());
    }
    pub fn exit_asm_line(&mut self, pair: &mut Pair<'_, Rule>) {
        println!("[exit ] asm_line: {}", pair.to_string());
        self.asm_lines.push(self.current_asm_line.clone());
        self.current_asm_line.clear();
    }

    pub fn enter_arm_register(&mut self, pair: &mut Pair<'_, Rule>) {
        println!("[enter] arm_register: {}", pair.to_string());

        match self.current_asm_line.param_idx {
            0 => {
                self.current_asm_line.reg1 = Register::from_str(pair.as_str()).expect(&format!("Cannot convert: {}", pair.to_string()));
                self.current_asm_line.param_idx = self.current_asm_line.param_idx + 1;
            }
            1 => {
                self.current_asm_line.reg2 = Register::from_str(pair.as_str()).expect(&format!("Cannot convert: {}", pair.to_string()));
                self.current_asm_line.param_idx = self.current_asm_line.param_idx + 1;
            }
            2 => {
                self.current_asm_line.reg3 = Register::from_str(pair.as_str()).expect(&format!("Cannot convert: {}", pair.to_string()));
                self.current_asm_line.param_idx = self.current_asm_line.param_idx + 1;
            }
            _ => todo!()
        }
    }
    pub fn exit_arm_register(&mut self, pair: &mut Pair<'_, Rule>) {
        println!("[exit ] arm_register: {}", pair.to_string());
    }

    pub fn enter_immediate(&mut self, pair: &mut Pair<'_, Rule>) {
        println!("[enter] immediate: {}", pair.to_string());

        let first_off: &str = &pair.to_string()[1..pair.to_string().len()];

        println!("[enter] first_off: {}", first_off);

        self.current_asm_line.immediate = first_off.parse::<i32>().unwrap();
    }
    pub fn exit_immediate(&mut self, pair: &mut Pair<'_, Rule>) {
        println!("[exit ] immediate: {}", pair.to_string());
    }

    pub fn enter_opcode(&mut self, pair: &mut Pair<'_, Rule>) {
        println!("[enter] opcode: {}", pair.to_string());
        self.current_asm_line.instruction = Instruction::from_str(&pair.to_string()).expect("Cannot decode instruction!");
    }
    pub fn exit_opcode(&mut self, pair: &mut Pair<'_, Rule>) {
        println!("[exit ] opcode: {}", pair.to_string());
    }

    pub fn enter_label(&mut self, pair: &mut Pair<'_, Rule>) {
        println!("[enter] label: {}", pair.to_string());
        self.current_asm_line.label = pair.to_string();
    }
    pub fn exit_label(&mut self, pair: &mut Pair<'_, Rule>) {
        println!("[exit ] label: {}", pair.to_string());
    }

    pub fn enter_label_target(&mut self, pair: &mut Pair<'_, Rule>) {
        println!("[enter] label_target: {}", pair.to_string());
        self.current_asm_line.label_target = pair.to_string();
    }
    pub fn exit_label_target(&mut self, pair: &mut Pair<'_, Rule>) {
        println!("[exit ] label_target: {}", pair.to_string());
    }

}
*/