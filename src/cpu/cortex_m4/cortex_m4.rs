use crate::ast::{self, instruction::Instruction};
use crate::ast::register::Register;
use std::collections::HashMap;
use crate::ast::asm_line::ASMLine;
use crate::encoder::thumb::thumb_decoder::ThumbDecoder;
use crate::cpu::mem_access::read_byte;
use crate::cpu::mem_access::read_halfword_le;
use crate::cpu::mem_access::read_word_le;

pub const START_ADDRESS: u32 = 0x00800000;

pub struct CortexM4 {
    pub halt: bool,
    pub reg_file: [i32; 16],
    pub memory_blocks: HashMap<u32, Vec<u8>>,
    pub zero_bit: bool,
    pub negative_bit: bool
}

impl CortexM4 {

    pub fn new() -> CortexM4 {
        CortexM4 {
            halt: false,
            reg_file: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            memory_blocks: HashMap::new(),
            zero_bit: false,
            negative_bit: false
        }
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.set_value_register(Register::PC, pc as i32);
    }

    pub fn set_value_register(&mut self, reg: Register, value: i32) {
        self.reg_file[Register::to_index(reg)] = value;
    }

    pub fn get_value_register(&mut self, reg: Register) -> i32 {
        self.reg_file[Register::to_index(reg)]
    }

    pub fn step(&mut self) {

        println!("");
        println!("----------------------------------------------------------------------");

        let pc:u32 = self.get_value_register(Register::PC) as u32;

        // even in Thumb or Thumb-2 mode, some instructions still need 32 bit to be decoded.
        // Here, load an entire word to decode such cases
        let thumb_instruction: u16 = read_halfword_le(&self.memory_blocks, pc);
        let next_thumb_instruction: u16 = read_halfword_le(&self.memory_blocks, pc + 2);

        // DEBUG
        println!("Address: 0x{:02x?}, Instruction: 0x{:02x?}", pc, thumb_instruction);

        // create decoder
        let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();

        // decode machine code to ASMLine object
        let mut asm_line: ASMLine = ASMLine::new();
        thumb_decoder.decode(thumb_instruction, next_thumb_instruction, &mut asm_line);

        // execute the decoded instruction
        self.execute(&asm_line);
    }

    pub fn execute(&mut self, asm_line: &ast::asm_line::ASMLine) {

        // ignore empty lines, preprocessor instructions or assembler instructions
        if asm_line.instruction == Instruction::UNDEFINED {
            return;
        }

        println!("[CortexM4::execute()] asm_line: {}", asm_line.to_string());

        // determine size of instruction
        let mut _pc_increment: i32 = 2;
        match asm_line.instruction {
            Instruction::BIC => {
                _pc_increment = 4; // wide instruction is 4 byte always even in thumb
            },
            Instruction::BL => {
                _pc_increment = asm_line.immediate + 4;
            },
            Instruction::LDR_W => {
                _pc_increment = 4; // wide instruction is 4 byte always even in thumb
            },
            Instruction::MOV_W => {
                _pc_increment = 4; // wide instruction is 4 byte always even in thumb
            },
            Instruction::ORR_W => {
                _pc_increment = 4; // wide instruction is 4 byte always even in thumb
            },
            Instruction::STR_W => {
                _pc_increment = 4; // wide instruction is 4 byte always even in thumb
            },
            _ => { _pc_increment = 2 }
        }

        // increment PC
        let pc:u32 = self.get_value_register(Register::PC) as u32;
        let temp_pc: i32 = pc as i32 + _pc_increment;
        self.set_value_register(Register::PC, temp_pc.try_into().unwrap());

        match asm_line.instruction {

            Instruction::ASR => todo!(),

            Instruction::ADD => {
                println!("ADD");

                if asm_line.reg2 == Register::UNDEFINED || asm_line.reg3 == Register::UNDEFINED {

                    let register_value = self.get_value_register(asm_line.reg1);

                    println!("{:02x?} = {:02x?} + {:02x?}", register_value + asm_line.immediate, register_value, asm_line.immediate);

                    self.set_value_register(asm_line.reg1, register_value + asm_line.immediate);

                } else {

                    let reg2_val = self.get_value_register(asm_line.reg2);
                    let reg3_val = self.get_value_register(asm_line.reg3);

                    println!("{:02x?} = {:02x?} + {:02x?}", reg2_val + reg3_val, reg2_val, reg3_val);

                    self.set_value_register(asm_line.reg1, reg2_val + reg3_val);
                }

                println!("ADD");
            },

            Instruction::B => todo!(),

            Instruction::BAL => {
                println!("BAL");

                let temp_pc: i32 = pc as i32 + asm_line.jump_offset + 4; // +4 ??????
                println!("BAL target address: {:02x?}", temp_pc);

                self.set_value_register(Register::PC, temp_pc.try_into().unwrap());
            },

            Instruction::BEQ =>  {
                println!("BEQ");

                if self.zero_bit {
                    let temp_pc: i32 = pc as i32 + asm_line.jump_offset + 4; // +4 ??????
                    println!("BAL target address: {:02x?}", temp_pc);

                    self.set_value_register(Register::PC, temp_pc.try_into().unwrap());
                }
            },

            Instruction::BGT => {
                println!("BGT");
            },

            Instruction::BIC => {
                //pc_increment = 4; // wide instruction is 4 byte always even in thumb
            },

            Instruction::BL => {
                //pc_increment = asm_line.immediate + 4;
            },

            Instruction::BX => {
                println!("BX");
            },

            Instruction::CMP => {
                // A8.8.38 CMP (register), A8-370 - https://developer.arm.com/documentation/ddi0406/cb/Application-Level-Architecture/Instruction-Details/Alphabetical-list-of-instructions/CMP--register-

                println!("CMP");

                let reg1_val = self.get_value_register(asm_line.reg1);
                let reg2_val = self.get_value_register(asm_line.reg2);

                let result:i32 = reg1_val - reg2_val;

                println!("CMP: {:02x} = {:02x} - {:02x}", result, reg1_val, reg2_val);

                self.zero_bit = result == 0;
                self.negative_bit = result < 0;
            },

            Instruction::LDR => {
                println!("LDR");

                // should be the PC register
                let mut reg2_val = self.get_value_register(asm_line.reg2);
                println!("PC: {:02x?}", reg2_val);

                // word align ???? is this required???
                if reg2_val % 4 != 0 {
                    reg2_val = ( (reg2_val + 4) >> 2 ) << 2;
                    println!("reg2_val: {:02x?}", reg2_val);
                }

                println!("imm: {:02x?}", asm_line.immediate);

                let address: u32 = (reg2_val + asm_line.immediate) as u32;
                println!("address: {:02x?}", address);

                let val: i32 = read_word_le(&self.memory_blocks, address) as i32;
                println!("value from address: {:02x?}", val);

                self.set_value_register(asm_line.reg1, val);
            },

            Instruction::LDR_W => {
                // println!("LDR.W");
                //pc_increment = 4; // wide instruction is 4 byte always even in thumb
            },

            Instruction::MOV => {
                println!("MOV");
                self.set_value_register(asm_line.reg1, asm_line.immediate);
            },

            Instruction::MOV_W => {
                println!("MOV_W");
                self.set_value_register(asm_line.reg1, asm_line.immediate);
                //pc_increment = 4; // wide instruction is 4 byte always even in thumb
            },

            Instruction::ORR_W => {
                // println!("ORR.W");
                //pc_increment = 4; // wide instruction is 4 byte always even in thumb
            },

            Instruction::PUSH => {
                //println!("PUSH");
            },

            Instruction::STR => {
                println!("STR");
            },
            Instruction::STR_W => {
                // println!("STR.W");
                //pc_increment = 4; // wide instruction is 4 byte always even in thumb
            },

            // SWI = Software Interrupt old term for SVC supervisor call
            //
            // https://developer.arm.com/documentation/dui0056/d/handling-processor-exceptions/swi-handlers/calling-swis-from-an-application
            //
            // Die Rolle von R7: Bevor Sie SWI 0 aufrufen, legen Sie die ID des gewünschten Systemaufrufs in R7 ab.
            // Die Nummer 1 steht beispielsweise für exit (Programm beenden), die Nummer 4 für write (Text ausgeben) [1].
            Instruction::SWI | Instruction::SVC => {

                match asm_line.immediate {

                    // SWI 0 - expects type of Linux OS operation in register r7
                    0 => {

                        println!("Executing SWI 0 - Linux OS call");

                        let register_r7_value = self.get_value_register(Register::R7);
                        match register_r7_value {

                            0 | 1 => {
                                //println!("Halting the CPU");
                                // the supervisor call 0 halts the CPU
                                self.halt = true;
                            }

                            4 => {
                                //println!("Printing fixed length string");

                                // 0 == stdin, 1 == stdout, 2 == stderr
                                // stdin: keyboard input into the terminal is forwarded to the process via stdin
                                // stdout: output from the process to the terminal over stdin-stream is forwarded to the monitor
                                // stderr: output from the process to the terminal over stderr-stream is forwarded to the monitor
                                let file_descriptor = self.get_value_register(Register::R0);
                                let string_addr = self.get_value_register(Register::R1);
                                let string_len = self.get_value_register(Register::R2);

                                //println!("Addr: {:02x?}, Len: {:02x?}", string_addr, string_len);

                                match file_descriptor {
                                    1 => print!("[stdout>> "),
                                    2 => print!("[stdout>> "),
                                    _ => print!("[UNKNOWN>> ")
                                }

                                for i in 0..string_len {
                                    print!("{}", read_byte(&self.memory_blocks, (string_addr + i) as u32) as char);
                                }
                                println!("");
                            }

                            _ => {
                                println!("Unknown Linux OS system call / supervisor call / software interrupt {:02x}!", register_r7_value);
                            }

                        }

                    }
                    _ => {
                        todo!()
                    }
                }
            },

            Instruction::WFI => todo!(),

            Instruction::UNDEFINED => todo!(),

        }


    }

    pub(crate) fn halt(&self) -> bool {
        self.halt
    }

}