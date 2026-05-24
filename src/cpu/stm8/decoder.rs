use crate::cpu::stm8::asm_line::ASMLine;
use crate::cpu::stm8::instruction::Instruction;

pub struct Decoder {

}

impl Decoder {

    pub fn new() -> Decoder {
        Decoder {
            
        }
    }

    pub fn decode(&mut self, encoded_instruction: u32, next_encoded_instruction: u32, asm_line: &mut ASMLine) {

        let first_byte: u8 = (encoded_instruction >> 24) as u8;

        println!("first_byte: 0x{:02x?}", first_byte);

        match first_byte {

            // LDW X,($50,SP)
            0x1E => {
                println!("LDW_X_SP_OFFSET");
                asm_line.instruction = Instruction::LDW_X_SP_OFFSET;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;

                println!("offset: {:02x?}", asm_line.immediate);
            }

            // LOAD WORD subtype LDW_X_SP_OFFSET (0x1F)
            // LDW ($50,SP),X
            0x1F => {
                println!("LDW_SP_OFFSET_X");
                asm_line.instruction = Instruction::LDW_SP_OFFSET_X;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;

                println!("offset: {:02x?}", asm_line.immediate);
            }

            // 0x20
            0x20 => {
                println!("JRA");
                asm_line.instruction = Instruction::JRA;

                let data: i8 = ((encoded_instruction >> 16) & 0xFF) as i8;


                asm_line.jump_offset = data as i32;
            }

            // Jump if zero flag is set
            0x27 => {
                println!("JREQ");
                asm_line.instruction = Instruction::JREQ;
                asm_line.jump_offset = ((encoded_instruction >> 16) & 0xFF) as i32;
            }

            // TNZ, Test Negative or Zero
            // TNZ A
            0x4D => {
                println!("TNZ_A");
                asm_line.instruction = Instruction::TNZ_A;
            }

            // CLR, CLEAR (0x4F)
            0x4F => {
                println!("CLR_A");
                asm_line.instruction = Instruction::CLR_A;
            }

            // SUB SP, imm
            0x52 => {
                println!("SUB_SP");
                asm_line.instruction = Instruction::SUB_SP;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
            }

            // page 146
            0x58 => {
                println!("SLLW_X");
                asm_line.instruction = Instruction::SLLW_X;
            }

            0x5C => {
                println!("INCW X");
                asm_line.instruction = Instruction::INC;
                //asm_line.immediate = ((encoded_instruction >> 8) & 0x0000FFFF) as i32;
            }

            0x5F => {
                println!("CLRW_X");
                asm_line.instruction = Instruction::CLRW_X; // pm0044, page 93
                //asm_line.immediate = ((encoded_instruction >> 8) & 0x0000FFFF) as i32;
            }

            0x72 => {

                let second_byte: u8 = (encoded_instruction >> 16) as u8;

                println!("second_byte: 0x{:02x?}", second_byte);

                match second_byte {

                    0xFB => {
                        asm_line.instruction = Instruction::ADDW_X_OFFSET_SP;
                        asm_line.immediate = ((encoded_instruction >> 8) & 0xFF) as i32;
                    }

                    _ => todo!()
                }
            }

            // RET (INTERRUPT) (0x81), PM0044, page 131
            0x81 => {
                println!("RET");
                asm_line.instruction = Instruction::RET;
            }

            // INT (INTERRUPT) (0x82)
            0x82 => {
                println!("INT");
                asm_line.instruction = Instruction::INT;
                asm_line.immediate = (encoded_instruction & 0x00FFFFFF) as i32;
            }

            0x96 => {
                println!("LDW_X_SP");
                asm_line.instruction = Instruction::LDW_X_SP;
            }

            0xA3 => {
                println!("CPW_X_IMM");
                asm_line.instruction = Instruction::CPW_X_IMM;
                asm_line.immediate = ((encoded_instruction >> 8) & 0xFFFF) as i32;
            }

            // LOAD WORD (0xAE)
            0xAE => {
                println!("LDW_AE");

                println!("{:08x?}", encoded_instruction);

                asm_line.instruction = Instruction::LDW_AE;
                asm_line.immediate = ((encoded_instruction >> 8) & 0x0000FFFF) as i32;
            }

            // CALL ??????? UNDOCUMENTED
            0xCC => {
                println!("CALL_CC");
                asm_line.instruction = Instruction::CALL_CC;
                asm_line.immediate = ((encoded_instruction >> 8) & 0x0000FFFF) as i32;
            }

            // CALL (0xCD)
            0xCD => {
                println!("CALL_CD");
                asm_line.instruction = Instruction::CALL_CD;
                asm_line.immediate = ((encoded_instruction >> 8) & 0x0000FFFF) as i32;
            }

            // LDW X,(X)
            0xFE => {
                println!("LDW X,(X)");
                asm_line.instruction = Instruction::LDW_X_X;
            }

            _ => {
                todo!("Unknown opcode: {:02X?}", first_byte);
            }

        }
    }
}