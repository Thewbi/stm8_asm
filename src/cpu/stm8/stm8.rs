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
    pub halt: bool,

    pub accumulator: i8,
    pub x_index: u16,
    pub y_index: u16,
    pub stack_pointer: u16,
    pub program_counter: u32,

    pub memory_blocks: HashMap<u32, Vec<u8>>,

    // flags (page 15, 16)
    pub zero_bit: bool,
    pub negative_bit: bool,
    pub carry_bit: bool,


    // create decoder
//        let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();
       // let mut decoder: Decoder = Decoder::new();

       pub decoder: Decoder
}

impl STM8 {

    pub fn new() -> STM8 {
        STM8 {

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

            memory_blocks: HashMap::new(),
            zero_bit: false,
            negative_bit: false,
            carry_bit: false,

            decoder: Decoder::new()
        }
    }

    pub fn set_pc(&mut self, pc: u32) {
        //self.set_value_register(Register::PC, pc as i32);
        self.program_counter = pc;
    }

    pub fn set_value_register(&mut self, reg: Register, value: i32) {
        //self.reg_file[Register::to_index(reg)] = value;

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
        //self.reg_file[Register::to_index(reg)]

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

        println!("");
        println!("----------------------------------------------------------------------");

        //let pc:u32 = self.get_value_register(Register::PC) as u32;

        // even in Thumb or Thumb-2 mode, some instructions still need 32 bit to be decoded.
        // Here, load an entire word to decode such cases
        let instruction: u32 = read_word_be(&self.memory_blocks, self.program_counter);
        let next_instruction: u32 = read_word_be(&self.memory_blocks, self.program_counter+4);

        // DEBUG
        println!("Address: 0x{:02x?}, Instruction: 0x{:08x?}, Next Instruction: 0x{:08x?}", self.program_counter, instruction, next_instruction);

        // decode machine code to ASMLine object
        let mut asm_line: ASMLine = ASMLine::new();
//        thumb_decoder.decode(thumb_instruction, next_thumb_instruction, &mut asm_line);
        self.decoder.decode(instruction, next_instruction, &mut asm_line);

        // DEBUG
        println!("self.program_counter: {:04X?}", self.program_counter);
        println!("asm_line.byte_count: {:04X?}", asm_line.byte_count);
        //if asm_line.byte_count == 0x38 {
        if self.program_counter == 0x8065 {
            println!("test");
        }
        // execute the decoded instruction
        self.execute(&asm_line);

    }

    pub fn execute(&mut self, asm_line: &ASMLine) {

        // ignore empty lines, preprocessor instructions or assembler instructions
        if asm_line.instruction == Instruction::UNDEFINED {
            return;
        }

        println!("[CortexM4::execute()] asm_line: {}", asm_line.to_string());

        // determine size of instruction
        let mut _pc_increment: i32 = 1;
        match asm_line.instruction {

            Instruction::SLLW_Y => { _pc_increment = 2 }

            Instruction::SLL_SP_OFFSET => { _pc_increment = 2 }

            Instruction::RLC_SP_OFFSET => { _pc_increment = 2 }

            Instruction::JRA => { _pc_increment = 2 }

            Instruction::JREQ => { _pc_increment = 2 }

            Instruction::ADDW_SP_IMM => { _pc_increment = 2 }

            Instruction::LDW_X_SP_OFFSET => { _pc_increment = 2 }
            Instruction::LDW_SP_OFFSET_X => { _pc_increment = 2 }

            Instruction::ADDW_Y_OFFSET_SP => { _pc_increment = 3 }
            Instruction::ADDW_X_OFFSET_SP => { _pc_increment = 3 }

            Instruction::SUB_SP => { _pc_increment = 2 }

            Instruction::CPW_X_IMM => { _pc_increment = 3 }
            Instruction::CPW_Y_IMM => { _pc_increment = 4 }

            Instruction::LDW_AE => { _pc_increment = 3 }

            Instruction::LDW_Y_SP => { _pc_increment = 2 }

            Instruction::LDW_Y_Y => { _pc_increment = 2 }
            Instruction::LDW_Y_SP_OFFSET => { _pc_increment = 2 }
            Instruction::LDW_OFFSET_SP_Y => { _pc_increment = 2 }

            _ => { _pc_increment = 1 }

        }

        // increment PC
        let pc:u32 = self.get_value_register(Register::ProgramCounter) as u32;
        let temp_pc: i32 = pc as i32 + _pc_increment;
        self.set_value_register(Register::ProgramCounter, temp_pc.try_into().unwrap());

        // execute operation
        match asm_line.instruction {

            // 0x08
            Instruction::SLL_SP_OFFSET => {
                println!("SLL subtype: SLL_SP_OFFSET");

                let address = self.stack_pointer as i16 + asm_line.immediate as i16;
                let mut data: u16 = read_halfword_be(&mut self.memory_blocks, address as u32);
                data = data << 1;
                write_halfword(&mut self.memory_blocks, address as u32, data as u16);
            }

            // 0x09
            Instruction::RLC_SP_OFFSET => {
                println!("RLC subtype: RLC_SP_OFFSET");

                let address = self.stack_pointer as i16 + asm_line.immediate as i16;
                let mut data: u8 = read_byte(&mut self.memory_blocks, address as u32);

                self.carry_bit = ((data >> 7) & 1) == 1;

                // TODO: shift left logial with carry!!!!!
                println!("data: {:02X?}", data);
                data = data << 1;
                println!("data: {:02X?}", data);

                write_byte(&mut self.memory_blocks, address as u32, data);

                // flags
                self.negative_bit = (data as i16) < 0;
                self.zero_bit = data == 0;
            }

            // 0x16
            //
            // LDW Y,($50,SP)
            // LDW dst,src
            Instruction::LDW_Y_SP_OFFSET => {
                println!("LDW subtype: LDW_Y_SP_OFFSET");

                // add offset to value stored in current SP
                let address:u16 = self.stack_pointer + asm_line.immediate as u16;

                let data: u16 = read_halfword_be(&mut self.memory_blocks, address as u32);

                println!("data: {:?}", data);

                self.y_index = data;
            }

            // 0x17
            // page 117
            //
            // LDW dst, src
            Instruction::LDW_OFFSET_SP_Y => {
                println!("LDW subtype: LDW_OFFSET_SP_Y");

                // add offset to value stored in current SP
                let address:u16 = self.stack_pointer + asm_line.immediate as u16;

                //let data: u16 = read_halfword_be(&mut self.memory_blocks, address as u32);
                write_halfword(&mut self.memory_blocks, address as u32, self.y_index);
            }

            // 0x1E
            // LOAD Word (1E) from memory relative to Stackpointer into X register
            //
            // ldw destination, source
            Instruction::LDW_X_SP_OFFSET => {
                println!("LDW subtype: LDW_X_SP_OFFSET");

                // add offset to value stored in current SP
                let address:u16 = self.stack_pointer + asm_line.immediate as u16;

                let data: u16 = read_halfword_be(&mut self.memory_blocks, address as u32);

                println!("data: {:?}", data);

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
                println!("LDW subtype: LDW_SP_OFFSET_X");

                // add offset to value stored in current SP
                let address:u16 = self.stack_pointer + asm_line.immediate as u16;

                println!("address: {:08x?}", address);

                write_halfword(&mut self.memory_blocks, address as u32, self.x_index as u16);
            }

            // JRxx Conditional Jump Relative Instruction

            // 0x20
            Instruction::JRA => {
                println!("JRA");

                let offset:i32 = asm_line.jump_offset as i32;

                println!("BEFORE> program_counter {:08x?}", self.program_counter);
                println!("program_counter {:02x?}", offset);

                self.program_counter = (self.program_counter as i32 + offset as i32) as u32;

                println!("AFTER > program_counter {:08x?}", self.program_counter);
            }

            // 0x27
            // Jump if Zero (if zero flag is set, jump to label), PM0044, page 113
            Instruction::JREQ => {

                println!("JREQ, Jump if zero-flag set");

                // PC <= PC + dst, if Condition is True

                //if CC.Z = 1
                //then PC ← PC + 2 + rr
                //else PC ← PC + 2

                println!("JREQ, self.program_counter: {:08x?}", self.program_counter);

                //self.program_counter = self.program_counter + 2;

                //println!("JREQ, self.program_counter: {:08x?}", self.program_counter);

                if self.zero_bit {
                    self.program_counter = self.program_counter + asm_line.jump_offset as u32;
                }

                println!("JREQ, self.program_counter: {:08x?}", self.program_counter);
            }

            // 0x4F
            // CLR, CLEAR (0x4F)
            Instruction::CLR_A => {
                println!("CLR subtype: A");

                // registers
                self.accumulator = 0x00;

                // flags
                self.negative_bit = false;
                self.zero_bit = true;
            }

            // 0x4D
            // TNZ A, Test not Zero A (0x4D)
            Instruction::TNZ_A => {
                println!("TNZ subtype: A");

                // flags
                self.negative_bit = self.accumulator < 0;
                self.zero_bit = self.accumulator == 0;
            }

            // 0x58
            // (0x58), page 146
            Instruction::SLLW_X => {
                println!("SLLW_X");

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
                println!("ADDW_SP_IMM");
                self.stack_pointer = (self.stack_pointer as i32 + asm_line.immediate) as u16;
            }

            // 0x5C
            // INC, PM0044, page 106 (0x5C)
            Instruction::INC => {
                println!("INCW X");



                // todo learn about the overflow flag
                //self.overflow =

                //self.x_index = self.x_index + asm_line.immediate as u16;
                self.x_index = self.x_index + 1;

                println!("X: {:04x?}", self.x_index);
                println!("X: {:04x?}", self.x_index);

                // TODO: flags!
                self.negative_bit = (self.x_index as i16) < 0;
                self.zero_bit = self.x_index == 0;
            }

            // 0x5F
            // CLRW X, Clear X (0x5F)
            Instruction::CLRW_X => {
                println!("CLRW_X");

                self.x_index = 0;

                //let imm: i32 = asm_line.immediate;

                // let imm_lo:u8 = ((imm >> 0) & 0xFF) as u8;
                // let imm_hi:u8 = ((imm >> 8) & 0xFF) as u8;

                //write_byte(&mut self.memory_blocks, asm_line.immediate as u32, 0x00);

                // flags
                self.negative_bit = false;
                self.zero_bit = true;
            }

            // 0x52
            // SUB SP, PM0044, page 151
            Instruction::SUB_SP => {
                println!("SUB_SP");

                println!("BEFORE> self.stack_pointer: {:08x?}", self.stack_pointer);

                self.stack_pointer = self.stack_pointer - asm_line.immediate as u16;

                println!("BEFORE> self.stack_pointer: {:08x?}", self.stack_pointer);

                // flags
                // no flags are affected by the SUB variant that uses SP and an immediate (see table "Instruction overview")
            }

            // 0x72
            // Add word without carry
            // ADDW X,($10,SP)
            // page 64, 78
            Instruction::ADDW_X_OFFSET_SP => {
                println!("ADDW_X_OFFSET_SP");

                // X <-- X + M(SP+shortoff)

                let address = self.stack_pointer + asm_line.immediate as u16;

                let data = read_halfword_be(&self.memory_blocks, address as u32);

                println!("{:08x}", data);

                println!("{:08X?} ({:?}) = {:?} + {:?}", (self.x_index + data), (self.x_index + data), self.x_index, data);

                self.x_index = self.x_index + data;
            }

            Instruction::ADDW_Y_OFFSET_SP => {
                println!("ADDW_Y_OFFSET_SP");

                // Y <-- Y + M(SP+shortoff)

                let address = self.stack_pointer + asm_line.immediate as u16;

                let data = read_halfword_be(&self.memory_blocks, address as u32);

                println!("{:08x}", data);

                println!("{:08X?} ({:?}) = {:?} + {:?}", (self.x_index + data), (self.x_index + data), self.x_index, data);

                self.y_index = self.y_index + data;
            }

            // 0x81
            // RET (0x81), PM0044, page 131
            Instruction::RET => {
                println!("RET");

                // PCH <- M(++SP)
                // PCL <- M(++SP)

                println!("self.stack_pointer: {:08x?}", self.stack_pointer);

                self.stack_pointer = self.stack_pointer + 1;
                println!("self.stack_pointer: {:08x?}", self.stack_pointer);
                let temp_pc_hi = read_byte(&mut self.memory_blocks, self.stack_pointer as u32);
                println!("pc.hi: {:08x?}", temp_pc_hi);

                self.stack_pointer = self.stack_pointer + 1;
                println!("self.stack_pointer: {:08x?}", self.stack_pointer);
                let temp_pc_lo = read_byte(&mut self.memory_blocks, self.stack_pointer as u32);
                println!("pc.lo: {:08x?}", temp_pc_lo);

                let temp_pc:u16 = ((temp_pc_hi as u16) << 8) | ((temp_pc_lo  as u16) << 0);

                println!("RET, jumping back to address: {:08x?}", temp_pc);

                self.program_counter = temp_pc as u32;
            }

            // 0x82
            // INT, INTERRUPT (0x82)
            Instruction::INT => {
                println!("INT");
                println!("INT, jumping to address: {:06x?}", asm_line.immediate);

                self.program_counter = asm_line.immediate as u32;
            }

            // 8E
            Instruction::HALT => {
                println!("HALT");
            }

            // 8F
            Instruction::WFI => {
                println!("WFI");
            }

            // LDW Y,SP
            // 0x90 0x96
            //
            // LDW dst,src
            // ldw destination Y, source SP
            Instruction::LDW_Y_SP => {
                println!("LDW_Y_SP");

                // LDW <dst>,<src>
                // LDW Y,SP

                println!("SP: {:08x?}", self.stack_pointer);

                self.stack_pointer = self.y_index;
            }

            // 0x90, 0x58, page 146
            Instruction::SLLW_Y => {
                println!("SLLW_Y");

                self.carry_bit = ((self.y_index >> 15) & 1) == 1;

                // peform the shift
                self.y_index = self.y_index << 1;

                // flags
                self.negative_bit = ((self.y_index >> 15) & 1) == 1;
                self.zero_bit = self.y_index == 0;
            }

            // 0x90, 0x5C, page 107
            Instruction::INCW_Y => {
                println!("INCW_Y");

                self.y_index = self.y_index + 1;
            }

            // 0x90 0xA3
            Instruction::CPW_Y_IMM=> {
                println!("CPW_Y_IMM");

                // The source byte is subtracted from the destination byte and the result is
                // lost. However, N, Z, C flags of Condition Code (CC) register are updated
                // according to the result. The destination is an index register, and the source
                // is a memory or data word. This instruction generally is used just before a
                // conditional jump instruction.

                let result: i32 = self.y_index as i32 - asm_line.immediate;
                println!("result: {:?} = {:?} - {:?}", result, self.y_index as i32, asm_line.immediate);

                self.negative_bit = result < 0;
                self.zero_bit = result == 0;
            }

            // 0x90 0xFE
            Instruction::LDW_Y_Y => {
                println!("LDW_Y_(Y)");

                let data = read_halfword_be(&self.memory_blocks, self.y_index as u32);
                println!("LDW_Y_(Y) loaded value: {:0?}", data);
                self.y_index = data;
            }

            // 0x96
            // ldw destination X, source SP
            Instruction::LDW_X_SP => {
                println!("LDW_X_SP");

                // LDW <dst>,<src>
                // LDW X,SP

                println!("SP: {:08x?}", self.stack_pointer);

                self.x_index = self.stack_pointer;

                // add offset to value stored in current SP
                //let address:u16 = self.stack_pointer;

                //write_halfword(&mut self.memory_blocks, address as u32, self.x_index as u16);
            }

            // 0xA3
            Instruction::CPW_X_IMM => {
                println!("CPW subtype: X_IMM");

                // Compare word with immediate

                let result: i32 = self.x_index as i32 - asm_line.immediate;
                println!("result: {:?} = {:?} - {:?}", result, self.x_index as i32, asm_line.immediate);

                self.negative_bit = result < 0;
                self.zero_bit = result == 0;
            }

            // 0xAE
            // LOAD (0xAE)
            Instruction::LDW_AE => {
                println!("LDW subtype: AE");

                self.x_index = asm_line.immediate as u16;

            }

            // CALL (0xCD), PM
            // 0xCC // 0xCD
            Instruction::CALL_CC |
            Instruction::CALL_CD => {
                println!("CALL subtype: CD");
                println!("CALL subtype: CD, jumping to address: {:06x?}", asm_line.immediate);

                // stack
                // TODO: Push the current PC+2 onto the stack as two byte (Lo then Hi)
                let temp_pc = self.program_counter + 2;

                let temp_pc_lo:u8 = ((temp_pc >> 0) & 0xFF) as u8;
                let temp_pc_hi:u8 = ((temp_pc >> 8) & 0xFF) as u8;

                write_byte(&mut self.memory_blocks, self.stack_pointer as u32, temp_pc_lo);
                self.stack_pointer = self.stack_pointer - 1;
                write_byte(&mut self.memory_blocks, self.stack_pointer as u32, temp_pc_hi);
                self.stack_pointer = self.stack_pointer - 1;

                // registers
                self.program_counter = asm_line.immediate as u32;

                println!("CALL subtype: CD, program_counter: {:06x?}", self.program_counter);
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
                println!("LDW_X_(X)");

                println!("LDW_X_(X) addr: {:08x?}", self.x_index);

                let data = read_halfword_be(&self.memory_blocks, self.x_index as u32);

                println!("LDW_X_(X) loaded value: {:0?}", data);

                self.x_index = data;
            }

            Instruction::UNDEFINED => todo!(),
        }

    }

    pub(crate) fn halt(&self) -> bool {
        self.halt
    }

}