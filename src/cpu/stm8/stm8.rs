use crate::cpu::stm8::decoder::Decoder;
use crate::cpu::stm8::instruction::Instruction;
use crate::cpu::stm8::register::Register;
use std::collections::HashMap;
use crate::cpu::stm8::asm_line::ASMLine;
use crate::cpu::mem_access::read_byte;
use crate::cpu::mem_access::write_byte;
use crate::cpu::mem_access::write_halfword;
use crate::cpu::mem_access::read_word_be;
use crate::cpu::mem_access::read_halfword_be;

pub const START_ADDRESS: u32 = 0x00008000;

pub struct STM8 {

    pub debug: bool,

    pub halt: bool,

    pub accumulator: i8,
    pub x_index: u16,
    pub y_index: u16,
    pub stack_pointer: u16,
    pub program_counter: u32,

    pub memory_block_map: HashMap<u32, Vec<u8>>,

    // flags (page 15, 16)
    pub overflow_bit: bool, // V
    pub zero_bit: bool, // Z
    pub negative_bit: bool, // N
    pub carry_bit: bool, // C
    pub half_carry_bit: bool, // H - set if there is a carry from bit 4 to 5 cleared otherwise

    pub i0: bool,
    pub i1: bool,

    // create decoder
//        let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();
       // let mut decoder: Decoder = Decoder::new();

    pub decoder: Decoder

}

impl STM8 {

    pub fn new() -> STM8 {
        STM8 {

            //debug: false,
            debug: true,

            halt: false,

            accumulator: 0,
            x_index: 0,
            y_index: 0,

            // The 16-bit Stack Pointer provides access to a 64K-level Stack.
            //
            // After an MCU reset the Stack Pointer is set to its upper limit value. It is then decremented
            // after data has been pushed onto the stack and incremented after data is popped from the
            // stack. When the lower limit is exceeded, the stack pointer wraps around to the stack upper
            // limit. The previously stored information is then overwritten, and therefore lost.
            //
            // The data space is 16-Mbyte and linear. As the stack must be located in section 0 and as
            // data access outside section 0/1 can be managed only with LDF instructions, frequently used
            // data should be located in section 0 to get the optimum code efficiency.
            //
            // STM8S103F2 STM8S103F3 STM8S103K3 DataSheet, Memory and register map, page 31
            // SP is initialized to the value 0x3FF (and has access to 513 bytes ?!? before the rest of the RAM begins)
            //
            //stack_pointer: 1024*64,
            stack_pointer: 0x03FF,

            program_counter: 0,

            memory_block_map: HashMap::new(),

            overflow_bit: false, // V
            zero_bit: false, // Z
            negative_bit: false, // N
            carry_bit: false, // C
            half_carry_bit: false, // H

            i0: false,
            i1: false,

            decoder: Decoder::new()
        }
    }

    pub fn push_return_address_to_stack(&mut self) {

        // stack
        // TODO: Push the current PC+2 onto the stack as two byte (Lo then Hi)
        let temp_pc = self.program_counter + 2;

        println!("Pushing return-address to stack. Stack-Pointer: {:08x?} Value: 0x{:04x?}", self.stack_pointer, temp_pc);

        let temp_pc_lo:u8 = ((temp_pc >> 0) & 0xFF) as u8;
        let temp_pc_hi:u8 = ((temp_pc >> 8) & 0xFF) as u8;

        println!("Pushing to stack. Stack-Pointer: {:08x?} Value: 0x{:02x?}", self.stack_pointer, temp_pc_lo);

        write_byte(&mut self.memory_block_map, self.stack_pointer as u32, temp_pc_lo);
        self.stack_pointer = self.stack_pointer - 1;

        println!("Pushing to stack. Stack-Pointer: {:08x?} Value: 0x{:02x?}", self.stack_pointer, temp_pc_hi);

        write_byte(&mut self.memory_block_map, self.stack_pointer as u32, temp_pc_hi);
        self.stack_pointer = self.stack_pointer - 1;
    }

    pub fn push_word_address_to_stack(&mut self, data: u16) {

        let temp_pc_lo:u8 = ((data >> 0) & 0xFF) as u8;
        let temp_pc_hi:u8 = ((data >> 8) & 0xFF) as u8;

        write_byte(&mut self.memory_block_map, self.stack_pointer as u32, temp_pc_lo);
        self.stack_pointer = self.stack_pointer - 1;
        write_byte(&mut self.memory_block_map, self.stack_pointer as u32, temp_pc_hi);
        self.stack_pointer = self.stack_pointer - 1;
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.program_counter = pc;
    }

    pub fn set_value_register(&mut self, reg: Register, value: i32) {

        match reg {

            Register::Accumulator => {
                self.accumulator = value as i8;
            }

            Register::XIndex => {
                self.x_index = value as u16;
            }

            Register::YIndex => {
                self.y_index = value as u16;
            }

            Register::StackPointer => {
                self.stack_pointer = value as u16;
            }

            Register::ProgramCounter => {
                self.program_counter = value as u32;
            }

            _ => {

            }

        }

    }

    pub fn get_value_register(&mut self, reg: Register) -> i32 {

        match reg {

            Register::Accumulator => {
                self.accumulator as i32
            }

            Register::XIndex => {
                self.x_index as i32
            }

            Register::YIndex => {
                self.y_index as i32
            }

            Register::StackPointer => {
                self.stack_pointer as i32
            }

            Register::ProgramCounter => {
                self.program_counter as i32
            }

            _ => {
                0
            }

        }
    }

    pub fn step(&mut self) {

        if self.debug {
            println!("");
            println!("----------------------------------------------------------------------");
        }

        // even in Thumb or Thumb-2 mode, some instructions still need 32 bit to be decoded.
        // Here, load an entire word to decode such cases
        let instruction: u32 = read_word_be(&self.memory_block_map, self.program_counter);
        let next_instruction: u32 = read_word_be(&self.memory_block_map, self.program_counter+4);

        if self.debug {
            // DEBUG
            println!("Address: 0x{:04x?}, Instruction: 0x{:08x?}, Next Instruction: 0x{:08x?}", self.program_counter, instruction, next_instruction);
        }

        // decode machine code to ASMLine object
        let mut asm_line: ASMLine = ASMLine::new();

//        thumb_decoder.decode(thumb_instruction, next_thumb_instruction, &mut asm_line);

        self.decoder.decode(instruction, next_instruction, &mut asm_line);

        if self.debug {
            // DEBUG
            println!("self.program_counter: {:04X?}", self.program_counter);
            println!("asm_line.byte_count: {:04X?}", asm_line.byte_count);
        }

        // execute the decoded instruction
        self.execute(&asm_line);
    }

    pub fn execute(&mut self, asm_line: &ASMLine) {

        // ignore empty lines, preprocessor instructions or assembler instructions
        if asm_line.instruction == Instruction::UNDEFINED {
            return;
        }

        if self.debug {
            println!("[CortexM4::execute()] asm_line: {}", asm_line.to_string());
        }

        // determine size of instruction
        let mut _pc_increment: i32 = 1;
        match asm_line.instruction {

            // 0x90, 0x58
            Instruction::SLLW_Y => { _pc_increment = 2 }

            // 0x08
            Instruction::SLL_SP_OFFSET => { _pc_increment = 2 }
            // 0x09
            Instruction::RLC_SP_OFFSET => { _pc_increment = 2 }
            // 0x0C
            Instruction::INC_SP_OFFSET => { _pc_increment = 2 }
            // 0x0F
            Instruction::CLR_SP_OFFSET => { _pc_increment = 2 }

            // 0x13
            Instruction::CPW_X_SP_OFFSET => { _pc_increment = 2 }

            // 0x1C
            Instruction::ADDW_X_IMM => { _pc_increment = 3 }

            Instruction::MOV => { _pc_increment = 4 }

            Instruction::JRA => { _pc_increment = 2 }

            // 0x24
            Instruction::JRNC => { _pc_increment = 2 }

            // 0x25
            Instruction::JRC => { _pc_increment = 2 }

            // 0x26
            Instruction::JRNE => { _pc_increment = 2 }

            // 0x27
            Instruction::JREQ => { _pc_increment = 2 }

            // 0x28
            Instruction::JRNV => { _pc_increment = 2 }

            // 0x90 0x28
            Instruction::JRNH => { _pc_increment = 3 }
            // 0x90 0x29, page 67
            Instruction::JRH => { _pc_increment = 3 }

            // 0x29
            Instruction::JRV => { _pc_increment = 2 }

            // 0x2A
            Instruction::JRPL => { _pc_increment = 2 }

            // 0x2B
            Instruction::JRMI => { _pc_increment = 2 }

            // 0x2E
            Instruction::JRSGE => { _pc_increment = 2 }

            // 0x39
            Instruction::RLC_SHORTMEM => { _pc_increment = 2 }

            // 0x4B
            Instruction::PUSH => { _pc_increment = 2 }

            // 0x51
            Instruction::EXGW => { _pc_increment = 1 }

            Instruction::ADDW_SP_IMM => { _pc_increment = 2 }

            Instruction::LDW_X_SP_OFFSET => { _pc_increment = 2 }
            Instruction::LDW_SP_OFFSET_X => { _pc_increment = 2 }

            // 0x72
            Instruction::ADDW_Y_OFFSET_SP => { _pc_increment = 3 }
            Instruction::ADDW_X_OFFSET_SP => { _pc_increment = 3 }
            Instruction::BTJT => { _pc_increment = 5 }

            Instruction::BSET => { _pc_increment = 4 }

            Instruction::SUB_SP => { _pc_increment = 2 }

            Instruction::SUBW_X_IMM => { _pc_increment = 3 }

            Instruction::CPW_X_IMM => { _pc_increment = 3 }
            Instruction::CPW_Y_IMM => { _pc_increment = 4 }

            Instruction::AND_A => { _pc_increment = 2 }

            Instruction::LD_A => { _pc_increment = 3; },
            Instruction::LD_A_LONGMEM => { _pc_increment = 3; },
            Instruction::LDW_AE => { _pc_increment = 3 }

            Instruction::LDW_Y_IMM => { _pc_increment = 4 }

            // 0x90 0x94
            Instruction::LDW_SP_Y => { _pc_increment = 2 }

            // 0x90 0x96
            Instruction::LDW_Y_SP => { _pc_increment = 2 }
            Instruction::LD_A_OFFSET_SP => { _pc_increment = 2 }

            // 0x90, 0x2C bb
            Instruction::JRNM => { _pc_increment = 3 }

            // 0x90, 0x5C
            Instruction::INCW_Y => { _pc_increment = 2 }

            // 0x90, 0xE3
            Instruction::CPW_X_SHORTOFF_Y => { _pc_increment = 3 }

            Instruction::LD_A_Y => { _pc_increment = 2 }

            Instruction::LDW_Y_Y => { _pc_increment = 2 }
            Instruction::LDW_Y_SP_OFFSET => { _pc_increment = 2 }
            Instruction::LDW_OFFSET_SP_Y => { _pc_increment = 2 }

            // 0x98
            Instruction::RCF => { _pc_increment = 1 }

            // A0
            Instruction::SUB_A_IMM => { _pc_increment = 2 }

            // A1
            Instruction::CP_A_IMM => { _pc_increment = 2 }

            // A5
            Instruction::BCP => { _pc_increment = 2 }

            // A6
            Instruction::LD_A_IMM => { _pc_increment = 2 }

            // AA
            Instruction::OR_A => { _pc_increment = 2 }
            // AB
            Instruction::ADD_A => { _pc_increment = 2 }

            // 0xCE
            Instruction::LDW_X_IMM => { _pc_increment = 3 }
            // 0xCF
            Instruction::LDW_IMM_X => { _pc_increment = 3 }

            // 0xF6
            Instruction::LD_A_MEMORY_X => { _pc_increment = 1 }

            _ => { _pc_increment = 1 }

        }

        // increment PC
        let pc:u32 = self.get_value_register(Register::ProgramCounter) as u32;
        let temp_pc: i32 = pc as i32 + _pc_increment;
        self.set_value_register(Register::ProgramCounter, temp_pc.try_into().unwrap());

        // execute operation
        match asm_line.instruction {

            // 0x01, page 139
            Instruction::RRWA_X => {

                let a_bit: u8 = self.accumulator as u8 & 0x01;
                let index_bit: u8 = (self.x_index as u16 & 0x0001) as u8;

                self.accumulator = ((self.accumulator >> 1) as u8 | index_bit) as i8;
                self.x_index = (self.x_index >> 1) | (a_bit as u16) << 15;

                // flags
                self.negative_bit = (self.x_index as i16) < 0;
                self.zero_bit = self.x_index == 0;
            }

            // 0x08
            Instruction::SLL_SP_OFFSET => {

                if self.debug {
                    println!("SLL subtype: SLL_SP_OFFSET");
                }

                let address = self.stack_pointer as i16 + asm_line.immediate as i16;
                let mut data: u16 = read_halfword_be(&mut self.memory_block_map, address as u32);
                data = data << 1;
                write_halfword(&mut self.memory_block_map, address as u32, data as u16);
            }

            // 0x09
            Instruction::RLC_SP_OFFSET => {
                if self.debug {
                    println!("RLC subtype: RLC_SP_OFFSET");
                }

                let address = self.stack_pointer as i16 + asm_line.immediate as i16;
                let mut data: u8 = read_byte(&mut self.memory_block_map, address as u32);

                self.carry_bit = ((data >> 7) & 1) == 1;

                // TODO: shift left logial with carry!!!!!
                if self.debug {
                    println!("data: {:02X?}", data);
                }
                data = data << 1;
                if self.debug {
                    println!("data: {:02X?}", data);
                }

                write_byte(&mut self.memory_block_map, address as u32, data);

                // flags
                // TODO: overflog V flag!!!!
                self.negative_bit = (data as i16) < 0;
                self.zero_bit = data == 0;
            }

            // 0x0C
            // INC ($10,SP), page 106
            Instruction::INC_SP_OFFSET => {
                if self.debug {
                    println!("INC subtype: INC_SP_OFFSET");
                }

                let address = self.stack_pointer as i16 + asm_line.immediate as i16;
                let mut data: u8 = read_byte(&mut self.memory_block_map, address as u32);

                self.carry_bit = ((data >> 7) & 1) == 1;

                if self.debug {
                    println!("data: {:02X?}", data);
                }
                data = data + 1;
                if self.debug {
                    println!("data: {:02X?}", data);
                }

                write_byte(&mut self.memory_block_map, address as u32, data);

                // flags
                // TODO: overflog V flag!!!!
                self.negative_bit = (data as i16) < 0;
                self.zero_bit = data == 0;
            }

            // 0x0F
            //
            // CLR dst
            // CLR ($10,SP)
            Instruction::CLR_SP_OFFSET => {
                if self.debug {
                    println!("LDW subtype: CLR_SP_OFFSET");
                    println!("asm_line.immediate: 0x{:02X?}", asm_line.immediate);
                }

                let address = self.stack_pointer as i16 + asm_line.immediate as i16;
                write_byte(&mut self.memory_block_map, address as u32, 0);

                if self.debug {
                    println!("address: {:04x?} ({:?}), data={:?}", address, address, 0);
                }

                // flags
                self.negative_bit = false;
                self.zero_bit = true;
            }

            // 0x1C
            //
            // dst <= dst + src
            // ADDW X,#$1000
            Instruction::ADDW_X_IMM => {

                let result = self.x_index as i16 + asm_line.immediate as i16;
                self.x_index = result as u16;

                // flags

                // TODO
                // V-flag
                self.overflow_bit = (result > 32767) || (-32768 < result);

                self.negative_bit = result < 0;
                self.zero_bit = result == 0;

                // TODO
                // C-flag
            }

            // 0x13, page 95
            //
            // CPW dst,src
            // cpw x, (0x01, sp)
            // CPW X, ($10, SP)
            Instruction::CPW_X_SP_OFFSET => {
                if self.debug {
                    println!("LDW subtype: CPW_X_SP_OFFSET");
                }

                let address:u16 = self.stack_pointer + asm_line.immediate as u16;
                let data: u16 = read_halfword_be(&mut self.memory_block_map, address as u32);
                if self.debug {
                    println!("data: {:?}", data);
                }

                // dest - src
                let result: i32 = self.x_index as i32 - data as i32;
                if self.debug {
                    println!("result: {:?} = {:?} - {:?}", result, self.x_index as i32, data);
                }

                // TODO
                // V-flag
                self.overflow_bit = (result > 32767) || (-32768 < result);

                self.negative_bit = result < 0;
                self.zero_bit = result == 0;

                // TODO
                // C-flag
            }

            // 0x16
            //
            // LDW Y,($50,SP)
            // LDW dst,src
            Instruction::LDW_Y_SP_OFFSET => {
                if self.debug {
                    println!("LDW subtype: LDW_Y_SP_OFFSET");
                }

                // add offset to value stored in current SP
                let address:u16 = self.stack_pointer + asm_line.immediate as u16;

                let data: u16 = read_halfword_be(&mut self.memory_block_map, address as u32);

                if self.debug {
                    println!("data: {:?}", data);
                }

                self.y_index = data;
            }

            // 0x17
            // page 117
            //
            // LDW dst, src
            Instruction::LDW_OFFSET_SP_Y => {
                if self.debug {
                    println!("LDW subtype: LDW_OFFSET_SP_Y");
                }

                // add offset to value stored in current SP
                let address:u16 = self.stack_pointer + asm_line.immediate as u16;

                //let data: u16 = read_halfword_be(&mut self.memory_block_map, address as u32);
                write_halfword(&mut self.memory_block_map, address as u32, self.y_index);
            }

            // 0x1D
            Instruction::SUBW_X_IMM => {
                if self.debug {
                    println!("SUBW X IMMEDIATE");
                }

                let temp: i16 = self.x_index as i16 - asm_line.immediate as i16;

                self.x_index = temp as u16;
            }

            // 0x1E
            // LOAD Word (1E) from memory relative to Stackpointer into X register
            //
            // ldw destination, source
            Instruction::LDW_X_SP_OFFSET => {
                if self.debug {
                    println!("LDW subtype: LDW_X_SP_OFFSET");
                }

                // add offset to value stored in current SP
                let address:u16 = self.stack_pointer + asm_line.immediate as u16;

                let data: u16 = read_halfword_be(&mut self.memory_block_map, address as u32);

                if self.debug {
                    println!("data: {:?}", data);
                }

                self.x_index = data;
            }

            // 0x1F
            // LOAD Word (1F) from X register to memory relative to Stackpointer
            //
            // ldw destination, source
            // ldw	(0x19, sp), x
            //
            // PM0044, page 118
            Instruction::LDW_SP_OFFSET_X => {
                if self.debug {
                    println!("LDW subtype: LDW_SP_OFFSET_X");
                }

                // add offset to value stored in current SP
                let address:u16 = self.stack_pointer + asm_line.immediate as u16;

                if self.debug {
                    println!("address: {:08x?}", address);
                }

                write_halfword(&mut self.memory_block_map, address as u32, self.x_index as u16);
            }

            // JRxx Conditional Jump Relative Instruction

            // 0x20, page 112
            //
            // JRA dst
            // Jump Relative Always
            //
            // PC = PC + lgth
            // PC <= PC + dst, if Condition is True
            Instruction::JRA => {
                if self.debug {
                    println!("JRA");
                }

                let offset:i32 = asm_line.jump_offset as i32;

                if self.debug {
                    println!("BEFORE> program_counter {:08x?}", self.program_counter);
                    println!("program_counter {:02x?}", offset);
                }

                // push return address onto stack
//                self.push_return_address_to_stack();

                // perform jump
                self.program_counter = (self.program_counter as i32 + offset as i32) as u32;

                if self.debug {
                    println!("AFTER > program_counter {:08x?}", self.program_counter);
                }
            }

            // 0x24
            Instruction::JRNC => {
                if self.debug {
                    println!("JRNC, Jump if carry_bit (C) NOT set");
                }

                if self.carry_bit == false {

                    // push return address onto stack
//                    self.push_return_address_to_stack();

                    // perform jump
                    self.program_counter = self.program_counter + asm_line.jump_offset as u32;
                }

                if self.debug {
                    println!("JRNC, self.program_counter: {:08x?}", self.program_counter);
                }
            }

            // 0x25
            Instruction::JRC => {
                if self.debug {
                    println!("JRC, Jump if carry_bit (C) set");
                }

                if self.carry_bit == true {

                    // push return address onto stack
//                    self.push_return_address_to_stack();

                    let result = self.program_counter as i64 + asm_line.jump_offset as i64;

                    // perform jump
                    self.program_counter = result as u32;
                }

                if self.debug {
                    println!("JRC, self.program_counter: {:08x?}", self.program_counter);
                }
            }

            // 0x26
            Instruction::JRNE => {
                if self.debug {
                    println!("JRNE, Jump if zero-flag (Z) NOT set");
                }

                if self.zero_bit == false {

                    // push return address onto stack
//                    self.push_return_address_to_stack();

                    let temp: i32 = self.program_counter as i32 + ((asm_line.jump_offset as i8) as i32);

                    // perform jump
                    self.program_counter = temp as u32;

                    println!("JRNE - PERFORMS JUMP - self.program_counter: {:08x?}", self.program_counter);
                }

                if self.debug {
                    println!("JRNE, self.program_counter: {:08x?}", self.program_counter);
                }
            }

            // 0x27
            // Jump if Zero (if zero flag is set, jump to label), PM0044, page 113
            Instruction::JREQ => {
                if self.debug {
                    println!("JREQ, Jump if zero-flag set");
                }

                // PC <= PC + dst, if Condition is True

                //if CC.Z = 1
                //then PC ← PC + 2 + rr
                //else PC ← PC + 2

                if self.debug {
                    println!("JREQ, self.program_counter: {:08x?}", self.program_counter);
                }

                //self.program_counter = self.program_counter + 2;

                //println!("JREQ, self.program_counter: {:08x?}", self.program_counter);

                if self.zero_bit {

                    // push return address onto stack
//                    self.push_return_address_to_stack();

                    // perform jump
                    self.program_counter = self.program_counter + asm_line.jump_offset as u32;
                }

                if self.debug {
                    println!("JREQ, self.program_counter: {:08x?}", self.program_counter);
                }
            }

            // 0x28, page 68 (Error in datasheet! Datasheet says: if CC.C = 0 whereas it should say if CC.V = 0)
            //
            // if CC.V = 0
            // then PC ← PC + 2+ rr
            // else PC ← PC + 2
            Instruction::JRNV => {
                if self.debug {
                    println!("JRNV, Jump if v-flag (overflow) not set");
                }

                if self.overflow_bit == false {
                    self.program_counter = self.program_counter + asm_line.jump_offset as u32;
                }

                if self.debug {
                    println!("JREQ, self.program_counter: {:08x?}", self.program_counter);
                }
            }

            // 0x29
            //
            // V = 1
            Instruction::JRV => {
                if self.debug {
                    println!("JRV, Jump if v-flag (overflow) set");
                }

                if self.overflow_bit == true {
                    self.program_counter = self.program_counter + asm_line.jump_offset as u32;
                }

                if self.debug {
                    println!("JREQ, self.program_counter: {:08x?}", self.program_counter);
                }
            }

            // 0x90 0x29
            // Jump if Zero (if zero flag is set, jump to label), PM0044, page 113
            Instruction::JRH => {
                if self.debug {
                    println!("JRH, Jump if half-carry-flag set");
                }

                // PC <= PC + dst, if Condition is True

                // if CC.H = 1
                // then PC ← PC + 2+ rr
                // else PC ← PC + 2

                if self.debug {
                    println!("JRH, self.program_counter: {:08x?}", self.program_counter);
                }

                if self.half_carry_bit == true {
                    // perform jump
                    self.program_counter = self.program_counter + asm_line.jump_offset as u32;
                }

                if self.debug {
                    println!("JRH, self.program_counter: {:08x?}", self.program_counter);
                }
            }

            // 0x2A
            //
            // Jump if N = 0 (plus)
            Instruction::JRPL => {
                if self.debug {
                    println!("JRPL, positive ( >= 0, N flag is not set )");
                }

                if self.negative_bit == false {

                    // push return address onto stack
//                    self.push_return_address_to_stack();

                    // perform jump
                    self.program_counter = self.program_counter + asm_line.jump_offset as u32;
                }
            }

            // 0x2B
            //
            // if CC.N = 1
            // then PC ← PC + 2+ rr
            // else PC ← PC + 2
            Instruction::JRMI => {
                if self.negative_bit == true {
                    // perform jump
                    self.program_counter = self.program_counter + asm_line.jump_offset as u32;
                }
            }

            // 0x2E
            // Signed Greater or Equal ( >= ), (N XOR V) = 0
            // JRSGE, PM0044, page 113
            Instruction::JRSGE => {
                if self.debug {
                    println!("JRSGE, Jump if Greater or Equal ( >= )");
                }

                if self.negative_bit ^ self.overflow_bit {

                    // push return address onto stack
//                    self.push_return_address_to_stack();

                    // perform jump
                    self.program_counter = self.program_counter + asm_line.jump_offset as u32;
                }

                if self.debug {
                    println!("JREQ, self.program_counter: {:08x?}", self.program_counter);
                }
            }

            // 0x35
            // MOV, MOV dst,src
            Instruction::MOV => {
                if self.debug {
                    println!("MOV, MOV");
                }

                let address:u16 = asm_line.jump_offset as u16;

                if self.debug {
                    println!("address: {:08x?}", address);
                }

                write_halfword(&mut self.memory_block_map, address as u32, asm_line.immediate as u16);
            }

            // 0x39
            // RLC_SHORTMEM, page 134
            // Rotate Left Logical through Carry
            Instruction::RLC_SHORTMEM => {
                if self.debug {
                    println!("RLC_SHORTMEM");
                }

                // This instruction shifts all bits of the register or memory, one place to the left, through the Carry bit.
                // Bit 0 of the result is a copy of the CC.C value before the operation.

                let temp_carry: u8 = if self.carry_bit { 1u8 } else { 0u8 };

                let mut value: u8 = read_byte(&self.memory_block_map, asm_line.immediate as u32);

                let msb: u8 = value & 0x80;

                value = (((value << 1) as u8) | temp_carry) as u8;

                write_byte(&mut self.memory_block_map, asm_line.immediate as u32, value);

                // flags
                self.negative_bit = (self.x_index as i16) < 0;
                self.zero_bit = self.x_index == 0;
                // carry bit
                if msb > 0 {
                    self.carry_bit = true;
                }
            }

            // 0x49
            // RLC_A, page 134
            // Rotate Left Logical through Carry
            Instruction::RLC_A => {
                if self.debug {
                    println!("RLC_A");
                }

                // The destination is either a memory byte or a register.
                // This instruction is compact, and does not affect any register when used with RAM variables.
                // This instruction shifts all bits of the register or memory, one place
                // to the left, through the Carry bit.
                // Bit 0 of the result is a copy of the CC.C value before the operation.

                let msb: u8 = self.accumulator as u8 & 0x80;

                let temp_carry: u8 = if self.carry_bit { 1u8 } else { 0u8 };
                self.accumulator = (((self.accumulator << 1) as u8) | temp_carry) as i8;

                if msb > 0 {
                    self.carry_bit = true;
                }
            }

            // 0x4B
            // PUSH
            Instruction::PUSH => {
                if self.debug {
                    println!("PUSH");
                }

                write_byte(&mut self.memory_block_map, self.stack_pointer as u32, asm_line.immediate as u8);
                self.stack_pointer = self.stack_pointer - 1;
            }

            // 0x4D
            // TNZ A, Test not Zero A (0x4D)
            Instruction::TNZ_A => {
                if self.debug {
                    println!("TNZ subtype: A");
                }

                // flags
                self.negative_bit = self.accumulator < 0;
                self.zero_bit = self.accumulator == 0;
            }

            // 0x4F
            // CLR, CLEAR (0x4F)
            Instruction::CLR_A => {
                if self.debug {
                    println!("CLR subtype: A");
                }

                // registers
                self.accumulator = 0x00;

                // flags
                self.negative_bit = false;
                self.zero_bit = true;
            }

            // 0x51, page 104
            Instruction::EXGW => {
                if self.debug {
                    println!("EXGW");
                }

                // exchange x and y
                let temp = self.x_index;
                self.x_index = self.y_index;
                self.y_index = temp;
            }

            // 0x58
            // (0x58), page 146
            Instruction::SLLW_X => {
                if self.debug {
                    println!("SLLW_X");
                }

                // logical (= shifts in a zero) left shift of the x register by a fixed amount of 1

                // carry bit is determined before shift
                self.carry_bit = ((self.x_index >> 15) & 1) == 1;

                // peform the shift
                self.x_index = self.x_index << 1;

                // flags
                self.negative_bit = ((self.x_index >> 15) & 1) == 1;
                self.zero_bit = self.x_index == 0;

                // todo learn about the overflow flag
                //self.overflow =
            }

            // 0x5B
            // ADDW SP,#$9
            //
            // ADDW dst,src
            // dst <= dst + src
            Instruction::ADDW_SP_IMM => {
                if self.debug {
                    println!("ADDW_SP_IMM");
                }

                self.stack_pointer = (self.stack_pointer as i32 + asm_line.immediate) as u16;
            }

            // 0x5C
            // INC, PM0044, page 106 (0x5C)
            Instruction::INC => {
                if self.debug {
                    println!("INCW X");
                }

                // todo learn about the overflow flag
                //self.overflow =

                //self.x_index = self.x_index + asm_line.immediate as u16;
                self.x_index = self.x_index + 1;

                if self.debug {
                    println!("X: {:04x?}", self.x_index);
                    println!("X: {:04x?}", self.x_index);
                }

                // TODO: flags!
                self.negative_bit = (self.x_index as i16) < 0;
                self.zero_bit = self.x_index == 0;
            }

            // 0x5D
            // TNZW, PM0044, page 74 (0x5D)
            //
            // Test word for negative or zero
            Instruction::TNZW => {
                if self.debug {
                    println!("TNZW X");
                }

                // flags
                self.negative_bit = (self.x_index as i16) < 0;
                self.zero_bit = self.x_index == 0;
            }

            // 0x5F
            // CLRW X, Clear X (0x5F)
            Instruction::CLRW_X => {
                if self.debug {
                    println!("CLRW_X");
                }

                self.x_index = 0;

                //let imm: i32 = asm_line.immediate;

                // let imm_lo:u8 = ((imm >> 0) & 0xFF) as u8;
                // let imm_hi:u8 = ((imm >> 8) & 0xFF) as u8;

                //write_byte(&mut self.memory_block_map, asm_line.immediate as u32, 0x00);

                // flags
                self.negative_bit = false;
                self.zero_bit = true;
            }

            // 0x52
            // SUB SP, PM0044, page 151
            Instruction::SUB_SP => {
                if self.debug {
                    println!("SUB_SP");
                }

                if self.debug {
                    println!("BEFORE> self.stack_pointer: {:08x?}", self.stack_pointer);
                }

                self.stack_pointer = self.stack_pointer - asm_line.immediate as u16;

                if self.debug {
                    println!("BEFORE> self.stack_pointer: {:08x?}", self.stack_pointer);
                }

                // flags
                // no flags are affected by the SUB variant that uses SP and an immediate (see table "Instruction overview")
            }

            // 0x72
            //
            // Add word without carry
            // page 64, 78
            //
            // ADDW dst,src
            // ADDW X,($10,SP)
            Instruction::ADDW_X_OFFSET_SP => {
                if self.debug {
                    println!("ADDW_X_OFFSET_SP");
                }

                // X <-- X + M(SP+shortoff)

                let address = self.stack_pointer + asm_line.immediate as u16;

                let data = read_halfword_be(&self.memory_block_map, address as u32);

                if self.debug {
                    println!("{:08x}", data);
                }

                if self.debug {
                    println!("{:08X?} ({:?}) = {:?} + {:?}", (self.x_index + data), (self.x_index + data), self.x_index, data);
                }

                self.x_index = self.x_index + data;


                // TODO!!!!!
                // // flags
                // V ⇒ (A15.M15 + M15.R15 + R15.A15) ⊕ (A14.M14 + M14.R14 + R14.A14) Set if the signed operation generates an overflow, cleared otherwise.
                // H ⇒ X7.M7 + M7.R7 + R7.X7 Set if a carry occurred from bit 7 of the result, cleared otherwise.
                // N ⇒ R15 Set if bit 15 of the result is set (negative value), cleared otherwise.
                // Z ⇒ R15.R14.R13.R12.R11.R10.R9.R8.R7.R6.R5.R4.R3.R2.R1.R0 Set if the result is zero (0x0000), cleared otherwise.
                // C ⇒ X15.M15 + M15.R15 + R15.X15 Set if a carry occurred from bit 15 of the result, cleared otherwise.
            }

            // 0x72
            Instruction::ADDW_Y_OFFSET_SP => {
                if self.debug {
                    println!("ADDW_Y_OFFSET_SP");
                }

                // Y <-- Y + M(SP+shortoff)

                let address = self.stack_pointer + asm_line.immediate as u16;

                let data = read_halfword_be(&self.memory_block_map, address as u32);

                if self.debug {
                    println!("{:08x}", data);
                    println!("{:08X?} ({:?}) = {:?} + {:?}", (self.x_index + data), (self.x_index + data), self.x_index, data);
                }

                self.y_index = self.y_index + data;
            }

            // 0x72 - Bit Set
            //
            // BSET dst,#pos pos = [0..7]
            // dst <= dst OR (2**pos)
            Instruction::BSET => {
                if self.debug {
                    println!("BSET");
                }

                if self.debug {
                    println!("immediate: {:04x?} ({:?})", asm_line.immediate, asm_line.immediate);
                    println!("n: {:?}", asm_line.jump_offset);
                }

                let mut value: u8 = read_byte(&self.memory_block_map, asm_line.immediate as u32);

                value = value | (2 * asm_line.jump_offset) as u8;

                write_byte(&mut self.memory_block_map, asm_line.immediate as u32, value);
            }

            // 0x72 0x0n, page 87
            //
            // BTJT - Bit Test and Jump if True
            Instruction::BTJT => {

                // Read the destination byte
                let destination_byte: u8 = read_byte(&self.memory_block_map, asm_line.immediate as u32);

                // test the corresponding bit (bit position)
                let bit_test: bool = destination_byte & (1 << asm_line.bit_pos) > 0;

                // jump to ’rel’ label if the bit is true (1),
                if bit_test {
                    self.program_counter = (self.program_counter as i32 + asm_line.jump_offset as i32) as u32;
                }

            }

            // 0x7B, page 114
            //
            // Load the destination BYTE with the source BYTE. (BYTE NOT WORD!)
            //
            // LD dst,src
            // LD A,($12,SP)
            Instruction::LD_A_OFFSET_SP => {

                let address = self.stack_pointer + asm_line.immediate as u16;

                //let data = read_halfword_be(&self.memory_block_map, address as u32);
                let data = read_byte(&self.memory_block_map, address as u32);

                if self.debug {
                    //println!("address: data: {:08x}", address, address, data);
                    println!("address: {:04x?} ({:?}), data={:?}", address, address, data);
                }

                //println!("{:08X?} ({:?}) = {:?} + {:?}", (self.x_index + data), (self.x_index + data), self.x_index, data);

                self.accumulator = data as i8;

                // flags
                self.negative_bit = self.accumulator < 0;
                self.zero_bit = self.accumulator == 0;
            }

            // 0x7D, page 155
            //
            // TNZ dst
            // {N, Z} = Test(dst)
            Instruction::TNZ_X => {
                if self.debug {
                    println!("TNZ_X");
                }

                // flags
                self.negative_bit = (self.x_index as i16) < 0;
                self.zero_bit = self.x_index == 0;
            }

            // 0x81
            // RET (0x81), PM0044, page 131
            Instruction::RET => {
                if self.debug {
                    println!("RET");
                }

                // PCH <- M(++SP)
                // PCL <- M(++SP)

                if self.debug {
                    println!("self.stack_pointer: {:08x?}", self.stack_pointer);
                }

                self.stack_pointer = self.stack_pointer + 1;
                if self.debug {
                    println!("self.stack_pointer: {:08x?}", self.stack_pointer);
                }
                let temp_pc_hi = read_byte(&mut self.memory_block_map, self.stack_pointer as u32);
                if self.debug {
                    println!("pc.hi: {:08x?}", temp_pc_hi);
                }

                self.stack_pointer = self.stack_pointer + 1;
                if self.debug {
                    println!("self.stack_pointer: {:08x?}", self.stack_pointer);
                }
                let temp_pc_lo = read_byte(&mut self.memory_block_map, self.stack_pointer as u32);
                if self.debug {
                    println!("pc.lo: {:08x?}", temp_pc_lo);
                }

                let temp_pc:u16 = ((temp_pc_hi as u16) << 8) | ((temp_pc_lo as u16) << 0);

                if self.debug {
                    println!("RET, jumping back to address: {:08x?}", temp_pc);
                }

                self.program_counter = temp_pc as u32;
            }

            // 0x82
            // INT, INTERRUPT (0x82)
            Instruction::INT => {
                if self.debug {
                    println!("INT");
                    println!("INT, jumping to address: {:06x?}", asm_line.immediate);
                }

                self.program_counter = asm_line.immediate as u32;
            }

            // 0x89, page 70, 129
            Instruction::PUSHW => {
                if self.debug {
                    println!("PUSHW X");
                }

                self.push_word_address_to_stack(self.x_index as u16);
            }

            // 0x8E
            Instruction::HALT => {
                if self.debug {
                    println!("HALT");
                }
                self.halt = true;
            }

            // 0x8F
            Instruction::WFI => {
                if self.debug {
                    println!("WFI");
                }
                self.halt = true;
            }

            // JRNH, page 68
            //
            // if CC.H = 0
            // then PC ← PC + 2 + rr
            // else PC ← PC + 2
            Instruction::JRNH => {
                if self.debug {
                    println!("JRNH");
                }
                if self.half_carry_bit == false {

                    // push return address onto stack
//                    self.push_return_address_to_stack();

                    // perform jump
                    self.program_counter = self.program_counter + asm_line.jump_offset as u32;
                }
            }

            // JRNM, page 68
            //
            // Jump if Interrupts are not masked
            //
            // if I0 AND I1 = 0
            // then PC ← PC + 2 + rr
            // else PC ← PC + 2
            Instruction::JRNM => {
                if self.debug {
                    println!("JRNM");
                }
                if self.i0 == false && self.i1 == false {

                    // push return address onto stack
//                    self.push_return_address_to_stack();

                    // perform jump
                    self.program_counter = self.program_counter + asm_line.jump_offset as u32;
                }
            }

            // 0x90 0x96
            //
            // LDW Y,SP
            // PM0044 page 118
            //
            // LDW dst,src
            // LDW Y,SP  , page 118
            //
            // LDW dst,src
            // ldw destination Y, source SP
            Instruction::LDW_Y_SP => {
                if self.debug {
                    println!("LDW_Y_SP");
                }

                // LDW <dst>,<src>
                // LDW Y,SP

                if self.debug {
                    println!("SP: {:08x?}", self.stack_pointer);
                }

                self.y_index = self.stack_pointer;
            }

            // 0x90, 0x01, page 139
            Instruction::RRWA_Y => {

                let a_bit: u8 = self.accumulator as u8 & 0x01;
                let index_bit: u8 = (self.y_index as u16 & 0x0001) as u8;

                self.accumulator = ((self.accumulator >> 1) as u8 | index_bit) as i8;
                self.x_index = (self.y_index >> 1) | (a_bit as u16) << 15;

                // flags
                self.negative_bit = (self.y_index as i16) < 0;
                self.zero_bit = self.y_index == 0;
            }

            // 0x90, 0x58, page 146
            Instruction::SLLW_Y => {
                if self.debug {
                    println!("SLLW_Y");
                }

                self.carry_bit = ((self.y_index >> 15) & 1) == 1;

                // peform the shift
                self.y_index = self.y_index << 1;

                // flags
                self.negative_bit = ((self.y_index >> 15) & 1) == 1;
                self.zero_bit = self.y_index == 0;
            }

            // 0x90, 0x5C, page 107
            Instruction::INCW_Y => {
                if self.debug {
                    println!("INCW_Y");
                }

                self.y_index = self.y_index + 1;
            }

            // 0x90, 0x7D, page 155
            //
            // TNZ dst
            // {N, Z} = Test(dst)
            Instruction::TNZ_Y => {
                if self.debug {
                    println!("TNZ_Y");
                }

                // flags
                self.negative_bit = (self.y_index as i16) < 0;
                self.zero_bit = self.y_index == 0;
            }

            // 0x90 0xA3
            Instruction::CPW_Y_IMM=> {
                if self.debug {
                    println!("CPW_Y_IMM");
                }

                // The source byte is subtracted from the destination byte and the result is
                // lost. However, N, Z, C flags of Condition Code (CC) register are updated
                // according to the result. The destination is an index register, and the source
                // is a memory or data word. This instruction generally is used just before a
                // conditional jump instruction.

                let result: i32 = self.y_index as i32 - asm_line.immediate;
                if self.debug {
                    println!("result: {:?} = {:?} - {:?}", result, self.y_index as i32, asm_line.immediate);
                }

                self.negative_bit = result < 0;
                self.zero_bit = result == 0;
            }

            // 0x90 0xAE
            Instruction::LDW_Y_IMM => {
                if self.debug {
                    println!("LDW_Y_IMM");
                }

                //let result: i32 = self.y_index as i32 - asm_line.immediate;
                //println!("result: {:?} = {:?} - {:?}", result, self.y_index as i32, asm_line.immediate);

                self.y_index = asm_line.immediate as u16;

                self.negative_bit = (self.y_index as i16) < 0;
                self.zero_bit = self.y_index == 0;
            }

            // 0x90, 0xE3
            // CPW X,($10,Y), page 95
            Instruction::CPW_X_SHORTOFF_Y => {
                if self.debug {
                    println!("LDW_X_SHORTOFF_Y");
                }

                // Compare word with immediate

                let address = self.y_index + asm_line.immediate as u16;

                let data = read_halfword_be(&self.memory_block_map, address as u32);

                let result: i32 = self.x_index as i32 - data as i32;
                if self.debug {
                    println!("result: {:?} = {:?} - {:?}", result, self.x_index as i32, data);
                }

                // todo! flag V (overflow)

                // flags
                self.negative_bit = result < 0;
                self.zero_bit = result == 0;

                // todo! flag C (carry)
            }

            // 0x90 0xF6
            // LD dst,src
            // LD A, Y
            Instruction::LD_A_Y => {
                if self.debug {
                    println!("LD_A_(Y)");
                }

                let data = read_halfword_be(&self.memory_block_map, self.y_index as u32);
                if self.debug {
                    println!("LDW_A_(Y) loaded value: {:0?}", data);
                }
                self.accumulator = data as i8;

                if self.debug {
                    println!("{:?}", data as u8 as char);
                }

                //self.accumulator = self.y_index as i8;

                // flags
                self.negative_bit = self.accumulator < 0;
                self.zero_bit = self.accumulator == 0;
            }

            // 0x90 0xFE
            Instruction::LDW_Y_Y => {
                if self.debug {
                    println!("LDW_Y_(Y)");
                }

                let data = read_halfword_be(&self.memory_block_map, self.y_index as u32);
                if self.debug {
                    println!("LDW_Y_(Y) loaded value: {:0?}", data);
                }
                self.y_index = data;

                // flags
                self.negative_bit = self.y_index < 0;
                self.zero_bit = self.y_index == 0;
            }

            // 0x90 0x94, page 118
            //
            // LDW dst,src
            // LDW SP,Y
            Instruction::LDW_SP_Y => {
                if self.debug {
                    println!("LDW_SP_Y");
                }

                self.stack_pointer = self.y_index;

                // flags
                self.negative_bit = self.accumulator < 0;
                self.zero_bit = self.accumulator == 0;
            }

            // 0x93, page 118
            //
            // LDW dst,src
            // LDW X,Y
            Instruction::LDW_X_Y => {
                if self.debug {
                    println!("LDW_X_Y");
                }

                self.x_index = self.y_index;

                // flags
                self.negative_bit = self.accumulator < 0;
                self.zero_bit = self.accumulator == 0;
            }

            // 0x94
            //
            // LDW dst,src
            // LDW SP,X
            Instruction::LDW_SP_X => {
                if self.debug {
                    println!("LDW_SP_X");
                }

                self.stack_pointer = self.x_index;

                // flags
                self.negative_bit = self.accumulator < 0;
                self.zero_bit = self.accumulator == 0;
            }

            // 0x95
            //
            // LD dst,src
            // LD_XH_A
            Instruction::LD_XH_A => {
                if self.debug {
                    println!("LD_XH_A");
                }

                // LDW <dst>,<src>
                // LDW XH,A

                //println!("SP: {:08x?}", self.stack_pointer);

                //self.x_index = self.stack_pointer;

                self.accumulator = ((self.x_index >> 8) & 0xFF) as i8;

                // flags
                self.negative_bit = self.accumulator < 0;
                self.zero_bit = self.accumulator == 0;

                // add offset to value stored in current SP
                //let address:u16 = self.stack_pointer;

                //write_halfword(&mut self.memory_block_map, address as u32, self.x_index as u16);
            }

            // 0x96
            // ldw destination X, source SP
            Instruction::LDW_X_SP => {
                if self.debug {
                println!("LDW_X_SP");
                }

                // LDW <dst>,<src>
                // LDW X,SP

                if self.debug {
                println!("SP: {:08x?}", self.stack_pointer);
                }

                self.x_index = self.stack_pointer;

                // add offset to value stored in current SP
                //let address:u16 = self.stack_pointer;

                //write_halfword(&mut self.memory_block_map, address as u32, self.x_index as u16);
            }

            // 0x97
            //
            // LD dst, src
            // LD_XL_A
            Instruction::LD_XL_A => {
                if self.debug {
                    println!("LD_XL_A");
                }

                // LDW <dst>,<src>
                // LDW XL,A

                //println!("SP: {:08x?}", self.stack_pointer);

                //self.x_index = self.stack_pointer;

                //self.accumulator = (self.x_index & 0xFF) as i8;
                self.x_index = self.x_index | (self.accumulator as u8) as u16;

                // flags
                self.negative_bit = (self.x_index as i16) < 0;
                self.zero_bit = self.x_index == 0;

                // add offset to value stored in current SP
                //let address:u16 = self.stack_pointer;

                //write_halfword(&mut self.memory_block_map, address as u32, self.x_index as u16);
            }

            // 0x98, page 130
            //
            // Reset C(arry) Flag
            // Clear the carry flag of the Condition Code (CC) register. May be used as a boolean user controlled flags
            Instruction::RCF => {
                if self.debug {
                    println!("RCF");
                }
                self.carry_bit = false; // C == carry bit
            }

            // 0x99, page 142
            //
            // Set C(arry) Flag
            // Set the carry flag of the Condition Code (CC) register. It may be used as user controlled flag.
            Instruction::SCF => {
                if self.debug {
                    println!("SCF");
                }
                self.carry_bit = true;
            }

            // 0x9A, page 70
            //
            // Reset interrupt mask / Disable interrupts
            Instruction::RIM => {
                if self.debug {
                    println!("RIM");
                }
                self.i0 = false; // ???????? Does the CPU do this or not? Datasheet is ambiguous!
                self.i1 = false;
            }

            // 0x9B, page 71
            //
            // Set interrupt mask / Disable interrupts
            Instruction::SIM => {
                if self.debug {
                    println!("SIM");
                }
                self.i0 = true;
                self.i1 = true;
            }

            // 0x9C, page 71
            //
            // CC.V ← 0
            Instruction::RVF => {
                if self.debug {
                    println!("RVF");
                }
                self.overflow_bit = false; // overflow_bit == V
            }

            // A0
            //
            // SUB A,src
            //
            // A <= A - src
            Instruction::SUB_A_IMM => {
                if self.debug {
                    println!("SUB_A_IMM");
                }

                let result: i32 = asm_line.immediate;
                //println!("result: {:?}", result);

                // let temp = self.accumulator;
                // self.accumulator = (self.accumulator & ((asm_line.immediate & 0xFF) as i8)) as i8;
                // if self.debug {
                //     println!("self.accumulator: {:?} = {:?} & {:?}", self.accumulator, temp, result);
                // }

                self.accumulator = self.accumulator - result as i8;

                // flags
                // TODO: V
                self.negative_bit = self.accumulator < 0;
                self.zero_bit = self.accumulator == 0;
                // TODO C
            }

            // 0xA1
            //
            // CP dst,src
            // CP A,#10
            Instruction::CP_A_IMM => {
                if self.debug {
                    println!("CP subtype A_IMM");
                }

                // Compare word with immediate

                let result: i32 = self.accumulator as i32 - asm_line.immediate as i8 as i32;
                if self.debug {
                    println!("result: {:?} = {:?} - {:?}", result, self.accumulator as i32, asm_line.immediate);
                }

                // todo! flag V (overflow)

                // flags
                self.negative_bit = result < 0;
                self.zero_bit = result == 0;

                // todo! flag C (carry)
                //self.carry_bit = ((self.x_index >> 15) & 1) == 1;
                //self.carry_bit = true;
                //self.carry_bit = false;

                // bit 9 of a 8 bit operation contains the carry
                self.carry_bit = ((result >> 8) & 0x1) == 1;
            }

            // 0xA3
            Instruction::CPW_X_IMM => {
                if self.debug {
                    println!("CPW subtype: X_IMM");
                }

                // Compare word with immediate

                let result: i32 = self.x_index as i32 - asm_line.immediate;
                if self.debug {
                    println!("result: {:?} = {:?} - {:?}", result, self.x_index as i32, asm_line.immediate);
                }

                // todo! flag V (overflow)

                // flags
                self.negative_bit = result < 0;
                self.zero_bit = result == 0;

                // todo! flag C (carry)

            }

            // 0xA4
            Instruction::AND_A => {
                if self.debug {
                    println!("AND_A");
                }

                let result: i32 = asm_line.immediate;
                //println!("result: {:?}", result);

                let temp = self.accumulator;
                self.accumulator = (self.accumulator & ((asm_line.immediate & 0xFF) as i8)) as i8;
                if self.debug {
                    println!("self.accumulator: {:?} = {:?} & {:?}", self.accumulator, temp, result);
                }

                // flags
                self.negative_bit = self.accumulator < 0;
                self.zero_bit = self.accumulator == 0;
            }

            // 0xA5
            //
            // Logical Bit Compare, page 0x81
            //
            // BCP A,src
            // {N, Z} <= A AND src
            //
            // The source byte, is ANDed to the contents of the accumulator. The result is
            // lost but condition flags N and Z are updated accordingly. The source is a
            // memory or data byte. This instruction can be used to perform bit tests on A.
            Instruction::BCP => {
                if self.debug {
                    println!("BCP");
                }

                let result: u8 = self.accumulator as u8 & asm_line.immediate as u8;
                println!("result: {:02x?}", result);

                // flags
                self.negative_bit = (result as i8) < 0;
                self.zero_bit = result == 0;
            }

            // A6
            Instruction::LD_A_IMM => {
                if self.debug {
                    println!("LD_A_IMM");
                }

                let result: i32 = asm_line.immediate;
                //println!("result: {:?}", result);

                // let temp = self.accumulator;
                // self.accumulator = (self.accumulator & ((asm_line.immediate & 0xFF) as i8)) as i8;
                // if self.debug {
                //     println!("self.accumulator: {:?} = {:?} & {:?}", self.accumulator, temp, result);
                // }

                self.accumulator = result as i8;

                // flags
                self.negative_bit = self.accumulator < 0;
                self.zero_bit = self.accumulator == 0;
            }

            // 0xAA, page 125
            //
            // OR_A
            //
            // OR A,src
            // A <= A OR src
            Instruction::OR_A => {
                if self.debug {
                    println!("OR_A");
                }

                self.accumulator = self.accumulator | asm_line.immediate as i8;

                // flags
                self.negative_bit = self.accumulator < 0;
                self.zero_bit = self.accumulator == 0;

            }

            // 0xAB, page 77
            //
            // ADD_A
            //
            // ADD A,src
            // A <= A ADD src
            Instruction::ADD_A => {
                if self.debug {
                    println!("ADD_A");
                }

                let accum_lo: i8 = self.accumulator & 0x0f;
                let imm_lo: i8 = (asm_line.immediate & 0x0f) as i8;
                let result = accum_lo + imm_lo;

                let temp_result: i16 = self.accumulator as i16 + asm_line.immediate as i16;

                self.accumulator = temp_result as i8;

                // flags
                // TODO V

                self.negative_bit = self.accumulator < 0;
                self.zero_bit = self.accumulator == 0;
                self.carry_bit = ((self.accumulator >> 7) & 1) == 1;
                // TODO H
                self.half_carry_bit = result > 0x0f;
            }

            // 0xAE
            // LOAD (0xAE)
            Instruction::LDW_AE => {
                if self.debug {
                    println!("LDW subtype: AE");
                }

                self.x_index = asm_line.immediate as u16;

            }

            // 0xB7, page 115
            //
            // LD dst,src
            // LD #$15, A
            //
            // dst <= src
            Instruction::LD_IMM_A => {
                if self.debug {
                    println!("LD_IMM_A");
                }

                let address = asm_line.immediate as u32;

                write_byte(&mut self.memory_block_map, address, self.accumulator as u8);
            }

            // 0xC6
            //
            // LD dst, src
            // LD A, $5000
            Instruction::LD_A => {
                if self.debug {
                    println!("LD_A");
                }

                self.accumulator = asm_line.immediate as i8;

                // 5230 == UART1 Status Register
                if asm_line.immediate == 0x5230 {
                    //self.accumulator = 0x80;
                    self.accumulator = 0b10000000u8 as i8;
                }

                // flags
                self.negative_bit = self.accumulator < 0;
                self.zero_bit = self.accumulator == 0;
            }

            // 0xC7
            // LD $5000,A
            // LD dst,src
            Instruction::LD_A_LONGMEM => {
                if self.debug {
                    println!("LD_A_LONGMEM");
                }

                let src:u16 = self.accumulator as u16;
                let dst:u16 = asm_line.immediate as u16;

                // 0x5231 == UART1 Data Register
                if dst == 0x5231 {
                    println!("UART-DR: Writing: Address: {:04x?} Data: {:04x?} Data: {:?}", dst, src, (src as u8 as char));
                }

                write_halfword(&mut self.memory_block_map, dst as u32, src);

                // flags
                self.negative_bit = asm_line.immediate < 0;
                self.zero_bit = asm_line.immediate == 0;
            }

            // CALL (0xCD), PM
            // 0xCC // 0xCD
            Instruction::CALL_CC |
            Instruction::CALL_CD => {
                if self.debug {
                    println!("CALL subtype: CD");
                    println!("CALL subtype: CD, jumping to address: {:06x?}", asm_line.immediate);
                }

                self.push_return_address_to_stack();

                // registers
                self.program_counter = asm_line.immediate as u32;

                if self.debug {
                    println!("CALL subtype: CD, program_counter: {:06x?}", self.program_counter);
                }
            }

            // 0xCE
            //
            // LDW dst,src
            // LDW X,$5000
            Instruction::LDW_X_IMM => {
                if self.debug {
                    println!("LDW_X_IMM");
                }

                let data = read_halfword_be(&mut self.memory_block_map, asm_line.immediate as u32);
                self.x_index = data;

                // flags
                self.negative_bit = self.x_index < 0;
                self.zero_bit = self.x_index == 0;
            }

            // 0xCF
            //
            // LDW dst,src
            // LDW $5000,X
            Instruction::LDW_IMM_X => {
                if self.debug {
                    println!("LDW_IMM_X");
                }

                write_halfword(&mut self.memory_block_map, asm_line.immediate as u32, self.x_index as u16);

                // flags
                self.negative_bit = asm_line.immediate < 0;
                self.zero_bit = asm_line.immediate == 0;
            }

            // 0xF6
            //
            // page 114
            //
            // LD dst,src
            // LD A,(X)
            //
            // Load BYTE!!!!
            Instruction::LD_A_MEMORY_X => {
                if self.debug {
                    println!("LD_A_MEMORY_X");
                }

                //let data = read_halfword_be(&self.memory_block_map, self.x_index as u32);
                let data = read_byte(&self.memory_block_map, self.x_index as u32);

                if self.debug {
                    println!("LD_A_(X) loaded value: {:02x?}", data);
                }

                self.accumulator = data as i8;
            }

            // 0xFE
            // indexed mode
            //
            // Load value from memory at address stored in register X and then store value into register X.
            //
            // The LDW X, (X) instruction in STM8 assembly is a pointer-dereference operation
            // that loads a 16-bit word from the memory address stored in X into register X.
            // It uses the Indexed Addressing Mode with a zero offset.
            // It functions similarly to X = * (uint16_t *)X in C.
            //
            // Take the value of X. Interpret it as an address. Load 16 bit out of memory at
            // that address. Store the loaded value into X.
            Instruction::LDW_X_X => {
                if self.debug {
                    println!("LDW_X_(X)");
                }

                if self.debug {
                    println!("LDW_X_(X) addr: {:08x?}", self.x_index);
                }

                let data = read_halfword_be(&self.memory_block_map, self.x_index as u32);

                if self.debug {
                    println!("LDW_X_(X) loaded value: {:0?}", data);
                }

                self.x_index = data;

                // flags
                self.negative_bit = (self.x_index as i16) < 0;
                self.zero_bit = self.x_index == 0;
            }

            // Load Word, page 117
            //
            // LDW dst,src
            // dst <= src
            //
            // ee01 ldw x,(1,x)
            Instruction::LDW_X_IMM_X => {
                if self.debug {
                    println!("LDW_X_IMM_X");
                }

                let address:i32 = self.x_index as i32 + asm_line.immediate as i32;

                if self.debug {
                    println!("LDW_X_IMM_X addr: {:08x?}", address);
                }

                let data = read_halfword_be(&self.memory_block_map, address as u32);

                if self.debug {
                    println!("LDW_X_IMM_X loaded value: {:0?}", data);
                }

                self.x_index = data;

                // flags
                self.negative_bit = (self.x_index as i16) < 0;
                self.zero_bit = self.x_index == 0;
            }

            // 0xF8, page 160
            Instruction::XOR_A_X => {
                if self.debug {
                    println!("XOR_A_X");
                }

                // The source byte, is logically XORed with the contents of the accumulator and the result is stored in the accumulator.
                // The source is a memory or data byte.

                let data = read_byte(&self.memory_block_map, self.x_index as u32);
                self.accumulator = (self.accumulator as u8 ^ data as u8) as i8;

                // flags
                self.negative_bit = self.accumulator < 0;
                self.zero_bit = self.accumulator == 0;
            }

            Instruction::UNDEFINED => todo!(),
        }

    }

    pub(crate) fn halt(&self) -> bool {
        self.halt
    }

}