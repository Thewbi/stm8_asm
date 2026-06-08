use crate::cpu::stm8::asm_line::ASMLine;
use crate::cpu::stm8::instruction::Instruction;

pub struct Decoder {
    pub byte_count: u32
}

impl Decoder {

    pub fn new() -> Decoder {
        Decoder {
            byte_count: 0
        }
    }

    pub fn decode(&mut self, encoded_instruction: u32, next_encoded_instruction: u32, asm_line: &mut ASMLine, program_counter: u32) {

        let first_byte: u8 = (encoded_instruction >> 24) as u8;

        //println!("first_byte: 0x{:02x?}", first_byte);

        match first_byte {

            // RRWA X
            // page 139
            0x01 => {
                asm_line.instruction = Instruction::RRWA_X;
                self.byte_count = self.byte_count + 1;
            }

            // SLL ($15,SP)
            0x08 => {
                //println!("SLL_SP_OFFSET");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::SLL_SP_OFFSET;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // RLC ($10,SP)
            0x09 => {
                //println!("RLC_SP_OFFSET");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::RLC_SP_OFFSET;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // INC ($10,SP)
            0x0C => {
                //println!("INC_SP_OFFSET");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::INC_SP_OFFSET;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // CLR ($10,SP)
            0x0F => {
                //println!("CLR_SP_OFFSET");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::CLR_SP_OFFSET;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // SUB A,($10,SP), page 151
            0x10 => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::SUB_A_OFFSET_SP;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // CPW_X_SP_OFFSET
            // CPW X,($10,SP)
            0x13 => {
                //println!("CPW X,($10,SP)");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::CPW_X_SP_OFFSET;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // LDW Y,($50,SP)
            0x16 => {
                //println!("LDW Y,($50,SP)");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LDW_Y_SP_OFFSET;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            0x17 => {
                //println!("LDW ($50,SP),Y");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LDW_OFFSET_SP_Y;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            0x18 => {
                //println!("LDW ($50,SP),Y");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::XOR_A_OFFSET_SP;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // OR_A_OFFSET_SP
            0x1A => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::OR_A_OFFSET_SP;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // SUBW X,#$5500
            0x1C => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::ADDW_X_IMM;
                asm_line.immediate = ((encoded_instruction >> 8) & 0xFFFF) as i32;

                self.byte_count = self.byte_count + 3;
            }

            // SUBW X,#$5500
            0x1D => {
                //println!("SUBW X,#$5500");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::SUBW_X_IMM;
                asm_line.immediate = ((encoded_instruction >> 8) & 0xFFFF) as i32;

                self.byte_count = self.byte_count + 3;
            }

            // LDW X,($50,SP)
            0x1E => {
                //println!("LDW_X_SP_OFFSET");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LDW_X_SP_OFFSET;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;

                //println!("offset: {:02x?}", asm_line.immediate);
                self.byte_count = self.byte_count + 2;
            }

            // LOAD WORD subtype LDW_X_SP_OFFSET (0x1F)
            // LDW ($50,SP),X
            0x1F => {
                //println!("LDW_SP_OFFSET_X");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LDW_SP_OFFSET_X;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;

                //println!("offset: {:02x?}", asm_line.immediate);
                self.byte_count = self.byte_count + 2;
            }

            // 0x20, JRA or also JRT (seem to perform the same operation)
            0x20 => {
                //println!("JRA/JRT");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::JRA;

                let data: i8 = ((encoded_instruction >> 16) & 0xFF) as i8;

                asm_line.jump_offset = data as i32;

                self.byte_count = self.byte_count + 2;
            }

            // 0x24, JRNC, page 67
            0x24 => {
                //println!("JRNC");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::JRNC;
                asm_line.jump_offset = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // 0x25
            0x25 => {
                //println!("JRC");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::JRC;
                asm_line.jump_offset = ((encoded_instruction >> 16) & 0xFF) as i8 as i32;
                self.byte_count = self.byte_count + 2;
            }

            // 0x26, page 113
            // opcode (1 Byte), offset xx (1 byte)
            0x26 => {
                //println!("JRNE");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::JRNE;
                asm_line.jump_offset = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // Jump if zero flag is set
            0x27 => {
                //println!("JREQ");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::JREQ;
                asm_line.jump_offset = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // page 68
            0x28 => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::JRNV;
                asm_line.jump_offset = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // page 68
            0x29 => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::JRV;
                asm_line.jump_offset = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // JRPL
            0x2A => {
                //println!("JRPL");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::JRPL;
                asm_line.jump_offset = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // JRMI, page 113
            0x2B => {
                //println!("JRMI");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::JRMI;
                asm_line.jump_offset = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // JRSGE
            0x2E => {
                //println!("JRSGE");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::JRSGE;
                asm_line.jump_offset = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // JRSLT
            0x2F => {
                //println!("JRSLT");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::JRSLT;
                asm_line.jump_offset = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // 0x34
            //Instruction::SRL_SHORTMEM => write!(f, "srl"),
            0x34 => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::SRL_SHORTMEM;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // mov data
            0x35 => {
                //println!("MOV");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::MOV;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                asm_line.jump_offset = ((encoded_instruction >> 0) & 0xFFFF) as i32;
                self.byte_count = self.byte_count + 4;
            }

            // 0x36
            //Instruction::RRC_SHORTMEM => write!(f, "srl"),
            0x36 => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::RRC_SHORTMEM;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // RLC Shortmem, page 134
            0x39 => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::RLC_SHORTMEM;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // 0x43
            0x43 => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::CPL_A; // Logical 1’s Complement of accumulator
                self.byte_count = self.byte_count + 1;
            }

            // 0x48
            0x48 => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::SLL_A;
                self.byte_count = self.byte_count + 1;
            }

            // RLC A
            // Rotate Left Logical through Carry, page 134
            0x49 => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::RLC_A;
                self.byte_count = self.byte_count + 1;
            }

            // DEC_A
            0x4A => {
                //println!("PUSH");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::DEC_A;
                self.byte_count = self.byte_count + 1;
            }

            // PUSH
            0x4B => {
                //println!("PUSH");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::PUSH;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 1;
            }

            // TNZ, Test Negative or Zero
            // TNZ A
            0x4D => {
                //println!("TNZ_A");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::TNZ_A;
                self.byte_count = self.byte_count + 1;
            }

            // CLR, CLEAR (0x4F)
            0x4F => {
                //println!("CLR_A");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::CLR_A;
                self.byte_count = self.byte_count + 1;
            }

            // EXGW, page 104, Exchange Index register contents
            0x51 => {
                //println!("EXGW");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::EXGW;
                self.byte_count = self.byte_count + 1;
            }

            // SUB SP, imm
            0x52 => {
                //println!("SUB_SP");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::SUB_SP;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // page 146
            0x58 => {
                //println!("SLLW_X");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::SLLW_X;
                self.byte_count = self.byte_count + 1;
            }

            // page 146
            0x59 => {
                //println!("RLCW");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::RLCW;
                self.byte_count = self.byte_count + 1;
            }

            // page 78
            0x5B => {
                //println!("ADDW SP,#$9");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::ADDW_SP_IMM;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // page 156
            0x5D => {
                //println!("TNZW X");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::TNZW;
                self.byte_count = self.byte_count + 1;
            }

            0x5C => {
                //println!("INCW X");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::INCW;
                self.byte_count = self.byte_count + 1;
            }

            0x5F => {
                //println!("CLRW_X");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::CLRW_X; // pm0044, page 93
                self.byte_count = self.byte_count + 1;
            }

            // pm0044, page 115
            0x6B => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LD_OFFSET_SP_A; // pm0044, page 115
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // 0xF1
            // CP, page 94
            0xF1 => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::CP_A_X_MEMORY;
                self.byte_count = self.byte_count + 1;
            }

            // BSET
            0x72 => {

                asm_line.byte_count = self.byte_count;

                let second_byte: u8 = (encoded_instruction >> 16) as u8;

                //println!("second_byte: 0x{:02x?}", second_byte);

                // if second_byte >> 4 == 0x01 {

                //     asm_line.instruction = Instruction::BSET;
                //     asm_line.immediate = ((encoded_instruction >> 0) & 0xFFFF) as i32;
                //     asm_line.jump_offset = ((encoded_instruction >> 16) & 0x0F) as i32;

                //     // TODO: verify correctness using real hardware
                //     // NOT SURE IF THIS IS times 2 (x2) or power of two (^2) datasheet uses ambiguous symbols.

                //     //asm_line.jump_offset = asm_line.jump_offset.ilog2() as i32;
                //     asm_line.jump_offset = asm_line.jump_offset / 2 as i32;
                //     self.byte_count = self.byte_count + 4;

                // } else 
                
                if second_byte >> 4 == 0x00 {

                    // TODO: How does the decoder tell BTJT apart from BTJF?
                    // The encoding is the same according to the PM0044, page 86 and page 87

                    // BTJT - Bit Test and Jump if True, page 86
                    // BTJF - Bit Test and Jump if False, page 87

                    asm_line.instruction = Instruction::BTJT;
                    
                    //
                    // Bit-Mask (1 Byte)
                    //

                    asm_line.bit_pos = ((encoded_instruction >> 16) & 0x0F) as u8;
                    // convert n into pos (bit-position in destination byte)
                    asm_line.bit_pos = asm_line.bit_pos / 2 ; //+ 1;

                    //
                    // Memory address of byte to check (2 Byte)
                    //

                    // address of byte to load from memory and check a bit in
                    asm_line.immediate = ((encoded_instruction >> 0) & 0xFFFF) as i32;

                    //
                    // Jump Offset if bit is set (1 Byte)
                    //

                    // one byte pc-relative jump offset
                    asm_line.jump_offset = ((next_encoded_instruction >> 24) & 0xFF) as i32;

                    // TODO: verify correctness using real hardware
                    // NOT SURE IF THIS IS times 2 (x2) or power of two (^2) datasheet uses ambiguous symbols.

                    //asm_line.jump_offset = asm_line.jump_offset.ilog2() as i32;
                    //asm_line.jump_offset = asm_line.jump_offset / 2 as i32;
                    self.byte_count = self.byte_count + 5;

                } else {

                    match second_byte {

                        0x12 | 0x1a => {
                            asm_line.instruction = Instruction::BSET;
                            asm_line.immediate = ((encoded_instruction >> 0) & 0xFFFF) as i32;
                            asm_line.jump_offset = ((encoded_instruction >> 16) & 0x0F) as i32;

                            // TODO: verify correctness using real hardware
                            // NOT SURE IF THIS IS times 2 (x2) or power of two (^2) datasheet uses ambiguous symbols.

                            //asm_line.jump_offset = asm_line.jump_offset.ilog2() as i32;
                            asm_line.jump_offset = asm_line.jump_offset / 2 as i32;
                            self.byte_count = self.byte_count + 4;
                        }

                        0x1b => {
                            asm_line.instruction = Instruction::BRES;
                            asm_line.immediate = ((encoded_instruction >> 0) & 0xFFFF) as i32;
                            asm_line.jump_offset = ((encoded_instruction >> 16) & 0x0F) as i32;

                            // TODO: verify correctness using real hardware
                            // NOT SURE IF THIS IS times 2 (x2) or power of two (^2) datasheet uses ambiguous symbols.

                            //asm_line.jump_offset = asm_line.jump_offset.ilog2() as i32;
                            asm_line.jump_offset = asm_line.jump_offset / 2 as i32;
                            self.byte_count = self.byte_count + 4;
                        }

                        0x5f => {
                            asm_line.instruction = Instruction::CLR_LONGMEM;
                            asm_line.immediate = ((encoded_instruction >> 0) & 0xFFFF) as i32;
                            self.byte_count = self.byte_count + 4;
                        }

                        0xA9 => {
                            asm_line.instruction = Instruction::ADDW_Y_IMM;
                            asm_line.immediate = ((encoded_instruction >> 0) & 0xFFFF) as i32;
                            self.byte_count = self.byte_count + 4;
                        }

                        0xF9 => {
                            asm_line.instruction = Instruction::ADDW_Y_OFFSET_SP;
                            asm_line.immediate = ((encoded_instruction >> 8) & 0xFF) as i32;
                            self.byte_count = self.byte_count + 2;
                        }

                        0xFB => {
                            asm_line.instruction = Instruction::ADDW_X_OFFSET_SP;
                            asm_line.immediate = ((encoded_instruction >> 8) & 0xFF) as i32;
                            self.byte_count = self.byte_count + 2;
                        }

                        _ => {
                            println!("Unknown second_byte: {:02x?}", second_byte);
                            todo!();
                        }
                    }

                }
            }

            0x7B => {
                //println!("LD A,($50,SP)");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LD_A_OFFSET_SP;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            0x7D => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::TNZ_X;
                self.byte_count = self.byte_count + 1;
            }

            // RET (INTERRUPT) (0x81), PM0044, page 131
            0x81 => {
                //println!("RET");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::RET;
                self.byte_count = self.byte_count + 1;
            }

            // INT (INTERRUPT) (0x82)
            0x82 => {
                //println!("INT");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::INT;
                asm_line.immediate = (encoded_instruction & 0x00FFFFFF) as i32;
                self.byte_count = self.byte_count + 4;
            }

            // POP_A
            0x84 => {
                //println!("POP A");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::POP_A;
                self.byte_count = self.byte_count + 1;
            }

            // 0x87, page 132, Far Return from subroutine
            0x87 => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::RETF;
                self.byte_count = self.byte_count + 1;
            }

            // 0x88, page 128, PUSH A to the stack
            0x88 => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::PUSH_A;
                self.byte_count = self.byte_count + 1;
            }

            // PUSHW (0x89)
            0x89 => {
                //println!("PUSHW");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::PUSHW;
                asm_line.immediate = (encoded_instruction & 0x00FFFFFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            0x8D => {
                //CALLF_EXTMEM
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::CALLF_EXTMEM;
                asm_line.immediate = (encoded_instruction & 0x00FFFFFF) as i32;
                self.byte_count = self.byte_count + 4;
            }

            0x8E => {
                //println!("HALT");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::HALT;
                self.byte_count = self.byte_count + 1;
            }

            0x8F => {
                //println!("WFI");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::WFI;
                self.byte_count = self.byte_count + 1;
            }

            0x90 => {

                asm_line.byte_count = self.byte_count;

                let second_byte: u8 = (encoded_instruction >> 16) as u8;

                //println!("second_byte: 0x{:02x?}", second_byte);

                match second_byte {

                    // RRWA Y
                    // page 139
                    0x01 => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::RRWA_Y;
                        self.byte_count = self.byte_count + 1;
                    }

                    // page
                    0x28 => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::JRNH;
                        self.byte_count = self.byte_count + 1;
                    }

                    // page 67
                    0x29 => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::JRH;
                        self.byte_count = self.byte_count + 1;
                    }

                    // page 67
                    0x2C => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::JRNM;
                        asm_line.jump_offset = ((encoded_instruction >> 8) & 0xFF) as i32;
                        self.byte_count = self.byte_count + 3;
                    }

                    // SLLW/SLAW Shift Left Logical Word/Shift Left Arithmetic Word
                    // page 146
                    0x58 => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::SLLW_Y;
                        self.byte_count = self.byte_count + 1;
                    }

                    // 90 5C, INCW, page 107
                    0x5C => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::INCW_Y;
                        self.byte_count = self.byte_count + 1;
                    }

                    0x7D => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::TNZ_Y;
                        self.byte_count = self.byte_count + 1;
                    }

                    0x93 => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::LDW_Y_X;
                        self.byte_count = self.byte_count + 2;
                    }

                    0x94 => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::LDW_SP_Y;
                        self.byte_count = self.byte_count + 2;
                    }

                    0x96 => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::LDW_Y_SP;
                        self.byte_count = self.byte_count + 1;
                    }

                    0xA3 => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::CPW_Y_IMM;
                        asm_line.immediate = ((encoded_instruction >> 0) & 0xFFFF) as i32;
                        self.byte_count = self.byte_count + 3;
                    }

                    0xAE => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::LDW_Y_IMM;
                        asm_line.immediate = ((encoded_instruction >> 0) & 0xFFFF) as i32;
                        self.byte_count = self.byte_count + 4;
                    }

                    0xB3 => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::CPW_Y_SHORTMEM;
                        asm_line.immediate = ((encoded_instruction >> 8) & 0xFF) as i32;
                        self.byte_count = self.byte_count + 4;
                    }

                    0xCE => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::LDW_Y_IMM_LONGMEM;
                        asm_line.immediate = ((encoded_instruction >> 0) & 0xFFFF) as i32;
                        self.byte_count = self.byte_count + 4;
                    }

                    0xE3 => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::CPW_X_SHORTOFF_Y;
                        asm_line.immediate = ((encoded_instruction >> 8) & 0xFF) as i32;
                        self.byte_count = self.byte_count + 3;
                    }

                    0xE8 => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::XOR_A_OFFSET_Y;
                        asm_line.immediate = ((encoded_instruction >> 8) & 0xFF) as i32;
                        self.byte_count = self.byte_count + 3;
                    }

                    0xEE => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::LDW_Y_SHORTOFF_Y;
                        asm_line.immediate = ((encoded_instruction >> 8) & 0xFF) as i32;
                        self.byte_count = self.byte_count + 3;
                    }

                    // page 114
                    0xF6 => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::LD_A_Y;
                        self.byte_count = self.byte_count + 2;
                    }

                    // 0x90, 0xFE, LDW Y,(Y)
                    0xFE => {
                        asm_line.byte_count = self.byte_count;
                        asm_line.instruction = Instruction::LDW_Y_Y_MEMORY;
                        self.byte_count = self.byte_count + 1;
                    }

                    _ => todo!()
                }
            }

            0x93 => {
                //println!("LDW X,Y");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LDW_X_Y;
                self.byte_count = self.byte_count + 1;
            }

            0x94 => {
                //println!("LDW SP,X");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LDW_SP_X;
                self.byte_count = self.byte_count + 1;
            }

            0x95 => {
                //println!("LD XH,A");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LD_XH_A;
                self.byte_count = self.byte_count + 1;
            }

            0x96 => {
                //println!("LDW_X_SP");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LDW_X_SP;
                self.byte_count = self.byte_count + 1;
            }

            0x97 => {
                //println!("LD XL,A");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LD_XL_A;
                self.byte_count = self.byte_count + 1;
            }

            0x98 => {
                //println!("RCF");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::RCF;
                self.byte_count = self.byte_count + 1;
            }

            0x99 => {
                //println!("SCF");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::SCF;
                self.byte_count = self.byte_count + 1;
            }

            0x9A => {
                //println!("RIM");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::RIM;
                self.byte_count = self.byte_count + 1;
            }

            0x9B => {
                //println!("SIM");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::SIM;
                self.byte_count = self.byte_count + 1;
            }

            0x9C => {
                // CC.V  ← 0
                //println!("RVF");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::RVF;
                self.byte_count = self.byte_count + 1;
            }

            0x9D => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::NOP;
                self.byte_count = self.byte_count + 1;
            }

            0xA0 => {
                //println!("SUB_A_IMM");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::SUB_A_IMM;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            0xA1 => {
                //println!("CP_A_IMM"); // BYTE compare (immediate is byte)
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::CP_A_IMM;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            0xA3 => {
                //println!("CPW_X_IMM"); // WORD compare (immediate is word)
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::CPW_X_IMM;
                asm_line.immediate = ((encoded_instruction >> 8) & 0xFFFF) as i32;
                self.byte_count = self.byte_count + 3;
            }

            0xA4 => {
                //println!("AND_A");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::AND_A;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            0xA5 => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::BCP_A_IMM;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            0xA6 => {
                //println!("LD_A_IMM");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LD_A_IMM;
                asm_line.immediate = ((encoded_instruction >> 16) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            0xA9 => {
                asm_line.instruction = Instruction::ADC_A_IMM;
                asm_line.immediate = ((encoded_instruction >> 8) & 0xFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // OR A (0xAA)
            0xAA => {
                //println!("OR_A");
                asm_line.byte_count = self.byte_count;
                //println!("{:08x?}", encoded_instruction);
                asm_line.instruction = Instruction::OR_A;
                asm_line.immediate = ((encoded_instruction >> 16) & 0x0000FF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // ADD A (0xAB), page 77
            0xAB => {
                //println!("ADD_A");
                asm_line.byte_count = self.byte_count;
                //println!("{:08x?}", encoded_instruction);
                asm_line.instruction = Instruction::ADD_A;
                asm_line.immediate = ((encoded_instruction >> 16) & 0x0000FF) as i32;
                self.byte_count = self.byte_count + 2;
            }
            
            // CALLR, CALL Subroutine Relative, page 90
            //
            // Instruction::CALLR => write!(f, "CALLR"),
            0xAD => {
                //println!("CALLR");
                asm_line.byte_count = self.byte_count;

                //println!("{:08x?}", encoded_instruction);

                asm_line.instruction = Instruction::CALLR;
                asm_line.immediate = ((encoded_instruction >> 16) & 0x000000FF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // LOAD WORD (0xAE), page 117
            0xAE => {
                //println!("LDW_AE");
                asm_line.byte_count = self.byte_count;

                //println!("{:08x?}", encoded_instruction);

                asm_line.instruction = Instruction::LDW_AE;
                asm_line.immediate = ((encoded_instruction >> 8) & 0x0000FFFF) as i32;
                self.byte_count = self.byte_count + 3;
            }

            0xB6 => {
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LD_A_SHORTMEM;
                asm_line.immediate = ((encoded_instruction >> 16) & 0x0000FF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            0xB7 => {
                //println!("LD_IMM_A");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LD_IMM_A;
                asm_line.immediate = ((encoded_instruction >> 16) & 0x0000FF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            0xBE => {
                //println!("LDW_X_IMM_SHORTMEM");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LDW_X_IMM_SHORTMEM;
                asm_line.immediate = ((encoded_instruction >> 16) & 0x0000FF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // page 117
            0xBF => {
                //println!("LDW_IMM_SHORTMEM_X");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LDW_IMM_SHORTMEM_X;
                asm_line.immediate = ((encoded_instruction >> 16) & 0x0000FF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // 0xC6
            0xC6 => {
                //println!("LD_A");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LD_A;
                asm_line.immediate = ((encoded_instruction >> 8) & 0x0000FFFF) as i32;
                self.byte_count = self.byte_count + 3;
            }

            // 0xC7
            // Instruction::LD_A_LONGMEM => {
            0xC7 => {
                //println!("LD_A_LONGMEM");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LD_A_LONGMEM;
                asm_line.immediate = ((encoded_instruction >> 8) & 0x0000FFFF) as i32;
                self.byte_count = self.byte_count + 3;
            }

            // 0xC8, page 160
            0xC8 => {
                //println!("XOR_A_LONGMEM");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::XOR_A_LONGMEM;
                asm_line.immediate = ((encoded_instruction >> 8) & 0x0000FFFF) as i32;
                self.byte_count = self.byte_count + 3;
            }

            // CALL ??????? UNDOCUMENTED
            0xCC => {
                //println!("CALL_CC");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::CALL_CC;
                asm_line.immediate = ((encoded_instruction >> 8) & 0x0000FFFF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // CALL (0xCD), page 88
            0xCD => {
                //println!("CALL_CD");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::CALL_CD;
                asm_line.immediate = ((encoded_instruction >> 8) & 0x0000FFFF) as i32;
                self.byte_count = self.byte_count + 3;
            }

            // LDW_IMM_X (0xCE)
            0xCE => {
                //println!("LDW_X_IMM");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LDW_X_IMM;
                asm_line.immediate = ((encoded_instruction >> 8) & 0x0000FFFF) as i32;
                self.byte_count = self.byte_count + 3;
            }

            // LDW_IMM_X (0xCF)
            0xCF => {
                //println!("LDW_IMM_X");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LDW_IMM_X;
                asm_line.immediate = ((encoded_instruction >> 8) & 0x0000FFFF) as i32;
                self.byte_count = self.byte_count + 3;
            }

            // 0xE1, page 94, CP A,($10,X)
            0xE1 => {
                asm_line.instruction = Instruction::CP_A_OFFSET_X;
                asm_line.immediate = ((encoded_instruction >> 16) & 0x000000FF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // 0xE6, page 114, LD A,($50,X)
            0xE6 => {
                asm_line.instruction = Instruction::LD_A_OFFSET_X;
                asm_line.immediate = ((encoded_instruction >> 16) & 0x000000FF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            0xEE => {
                asm_line.instruction = Instruction::LDW_X_IMM_X;
                asm_line.immediate = ((encoded_instruction >> 16) & 0x000000FF) as i32;
                self.byte_count = self.byte_count + 2;
            }

            // LD A,(X)
            0xF6 => {
                //println!("LD A,(X)");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LD_A_X_MEMORY;
                self.byte_count = self.byte_count + 2;
            }

            // LD (X),A
            0xF7 => {
                //println!("LD (X),A");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LD_X_MEMORY_A;
                self.byte_count = self.byte_count + 2;
            }

            // LDW X,(X), page 117
            0xFE => {
                //println!("LDW X,(X)");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::LDW_X_X_MEMORY;
                self.byte_count = self.byte_count + 1;
            }

            // XOR_A_X, page 160
            0xF8 => {
                //println!("LDW X,(X)");
                asm_line.byte_count = self.byte_count;
                asm_line.instruction = Instruction::XOR_A_X;
                self.byte_count = self.byte_count + 1;
            }

            _ => {
                todo!("Unknown opcode: 0x{:02X}. ProgramCounter (PC): 0x{:08X}, Encoded bytes: 0x{:08X} 0x{:08X}", first_byte, program_counter, encoded_instruction, next_encoded_instruction);
            
                // PM0044, page 110
                //next:     421  0029 e402          	and	a,(2,x)
                //C:\aaa_se\Project_THZ\stvd\Debug\stm8s_gpio.ls  Zeile 60
            }

        }
    }
}