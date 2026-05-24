//
// Thumb Decoder for ARMv7
// Official Documentation: doc/DDI0406C_d_armv7ar_arm.pdf
//

use crate::ast::{asm_line::ASMLine, instruction::{Instruction}, register::Register};

pub struct ThumbDecoder {

}

pub fn sign_extend(num: i32, bits: u32) -> i32 {
    let shift = i32::BITS - bits;
    (num << shift) >> shift
}

impl ThumbDecoder {

    pub fn new() -> ThumbDecoder {
        ThumbDecoder {
            
        }
    }

    // https://developer.arm.com/documentation/ddi0406/c/Application-Level-Architecture/Thumb-Instruction-Set-Encoding/32-bit-Thumb-instruction-encoding/Modified-immediate-constants-in-Thumb-instructions
    //
    // ThumbExpandImm_C()
    // ==================
    // (bits(32), bit) ThumbExpandImm_C(bits(12) imm12, bit carry_in)
    //
    //      if imm12<11:10> == '00' then
    // 
    //          case imm12<9:8> of
    //              when '00'
    //                  imm32 = ZeroExtend(imm12<7:0>, 32);
    //              when '01'
    //                  if imm12<7:0> == '00000000' then UNPREDICTABLE;
    //                  imm32 = '00000000' : imm12<7:0> : '00000000' : imm12<7:0>;
    //              when '10'
    //                  if imm12<7:0> == '00000000' then UNPREDICTABLE;
    //                  imm32 = imm12<7:0> : '00000000' : imm12<7:0> : '00000000';
    //              when '11'
    //                  if imm12<7:0> == '00000000' then UNPREDICTABLE;
    //                  imm32 = imm12<7:0> : imm12<7:0> : imm12<7:0> : imm12<7:0>;
    //          
    //              carry_out = carry_in;
    //      else
    //
    //          unrotated_value = ZeroExtend('1':imm12<6:0>, 32);
    //          (imm32, carry_out) = ROR_C(unrotated_value, UInt(imm12<11:7>));
    //
    //      return (imm32, carry_out);

    pub fn thumb_expand_imm_c(&mut self, imm12: u32, _carry_in: u8) -> u32 {

        let mut _res: u32 = 0;

        if ((imm12 >> 10) & 0b11) == 0b00 {

            match (imm12 >> 8) & 0b11 {

                0b00 => {
                    let imm12_7_0 = (imm12 >> 0) & 0b11111111;
                    _res = imm12_7_0;
                }

                0b01 => {
                    let imm12_7_0 = (imm12 >> 0) & 0b11111111;
                    if imm12_7_0 == 0 {
                        todo!("UNPREDICTABLE")
                    }
                    _res = ((0b00000000 as u32) << 24) | ((imm12_7_0 as u32) << 16) | ((0b00000000 as u32) << 8) | ((imm12_7_0 as u32) << 0);
                }

                0b10 => {
                    let imm12_7_0 = (imm12 >> 0) & 0b11111111;
                    if imm12_7_0 == 0 {
                        todo!("UNPREDICTABLE")
                    }
                    _res = ((imm12_7_0 as u32) << 24) | ((0b00000000 as u32) << 16) | ((imm12_7_0 as u32) << 8) | ((0b00000000 as u32) << 8);
                }

                0b11 => {
                    let imm12_7_0 = (imm12 >> 0) & 0b11111111;
                    if imm12_7_0 == 0 {
                        todo!("UNPREDICTABLE")
                    }
                    _res = ((imm12_7_0 as u32) << 24) | ((imm12_7_0 as u32) << 16) | ((imm12_7_0 as u32) << 8) | ((imm12_7_0 as u32) << 0);
                }
    
                _ => {
                    todo!()
                }
    
            }

        } else {

            let unrotated_value = (1 << 7) | (imm12  & 0b1111111);
            //println!("{}", unrotated_value);

            let rotate_amount = (imm12 >> 7) & 0b11111;
            //println!("{}", rotate_amount);

            _res = unrotated_value.rotate_right(rotate_amount);

        }

        _res
    }

    // https://class.ece.iastate.edu/cpre288/resources/docs/Thumb-2SupplementReferenceManual.pdf
    //
    // Table 3-1 Determination of instruction length
    //
    // hw1[15:11]   Function
    // 0b11100      Thumb 16-bit unconditional branch instruction, defined in all Thumb architectures.
    // 0b111xx      Thumb 32-bit instructions, defined in Thumb-2, see Instruction encoding for 32-bit Thumb instructions on page 3-12.
    // 0bxxxxx      Thumb 16-bit instructions.
    //
    pub fn decode(&mut self, encoded_instruction: u16, next_encoded_instruction: u16, asm_line: &mut ASMLine) {

        match encoded_instruction {

            encoded_instruction if (encoded_instruction >> 11) == 0b11100 => {
                self.decode_16bit(encoded_instruction, next_encoded_instruction, asm_line);
            }

            encoded_instruction if (encoded_instruction >> 13) == 0b111 => {
                self.decode_32bit(encoded_instruction, next_encoded_instruction, asm_line);
            }

            _ => {
                self.decode_16bit(encoded_instruction, next_encoded_instruction, asm_line);
            }

        }

    }

    pub fn decode_32bit(&mut self, encoded_instruction: u16, next_encoded_instruction: u16, asm_line: &mut ASMLine) {

        println!("{:02x?}", encoded_instruction);

        match encoded_instruction {

            //
            // A8.8.103 MOV (immediate) (A8-485)
            // Encoding T2
            // ARMv6T2, ARMv7
            // MOV{S}<c>.W <Rd>, #<const>
            //

            encoded_instruction if ( ( ( (encoded_instruction >> 11) & 0b11111 ) == 0b11110 ) && ( ( (encoded_instruction >> 0) & 0b1111 ) == 0b1111 ) ) => {

                println!("A8.8.103 MOV (immediate) (A8-485)    MOV S <c>.W <Rd>, #<const>");

                let i: i32 = (encoded_instruction as i32 >> 10) & 0b1;
                let _s: i32 = (encoded_instruction as i32 >> 4) & 0b1;

                let imm3: i32 = (next_encoded_instruction as i32 >> 12) & 0b111;
                let imm8: i32 = (next_encoded_instruction as i32 >> 0) & 0b1111_1111;

                // rd
                let rd_as_u16: u16 = (next_encoded_instruction >> 8) & 0b1111;
                let rd = Register::from_u16(rd_as_u16).expect("Cannot decode Rd register!");
                //println!("rd: {:?}", rd);

                let imm12: i32 = (i << 11) | (imm3 << 8) | imm8;

                let const_immediate_expanded = self.thumb_expand_imm_c(imm12 as u32, 0);

                asm_line.instruction = Instruction::MOV_W;
                asm_line.reg1 = rd;
                asm_line.immediate = const_immediate_expanded as i32;
            }

            //
            // A8.8.103 MOV (immediate)
            // Encoding T3
            // ARMv6T2, ARMv7
            // MOVW<c> <Rd>, #<imm16>
            //

            encoded_instruction if ( ( ( (encoded_instruction >> 11) & 0b11111 ) == 0b11110 ) && ( ( (encoded_instruction >> 4) & 0b111111 ) == 0b100100 ) ) => {

                println!("A8.8.103 MOV (immediate) (A8-485)    MOV S <c>.W <Rd>, #<const>");

                let i: i32 = (encoded_instruction as i32 >> 10) & 0b1;

                let imm4: i32 = (encoded_instruction as i32 >> 0) & 0b1111;
                let imm3: i32 = (next_encoded_instruction as i32 >> 12) & 0b111;
                let imm8: i32 = (next_encoded_instruction as i32 >> 0) & 0b1111_1111;

                // rd
                let rd_as_u16: u16 = (next_encoded_instruction >> 8) & 0b1111;
                let rd = Register::from_u16(rd_as_u16).expect("Cannot decode Rd register!");
                //println!("rd: {:?}", rd);

                let imm16: i32 = (imm4 << 12) | (i << 11) | (imm3 << 8) | (imm8 << 0);

                // zero extend
                let const_immediate_expanded = imm16 as u32;

                asm_line.instruction = Instruction::MOV_W;
                asm_line.reg1 = rd;
                asm_line.immediate = const_immediate_expanded as i32;
            }

            //
            // A8.8.123 ORR (immediate) (A8-517)
            //

            encoded_instruction if ( ( ( (encoded_instruction >> 11) & 0b11111 ) == 0b11110 ) && ( ( (encoded_instruction >> 5) & 0b11111 ) == 0b00010 ) ) => {

                //println!("A8.8.123 ORR (immediate)");

                // Rd register is [11:8] of next_encoded_instruction
                let rd_as_u16: u16 = ((next_encoded_instruction >> 8) & 0b1111) << 0;
                let rd = Register::from_u16(rd_as_u16).expect("Cannot decode RD register!");
                //println!("{:?}", rd);

                // Rn register is [3:0] of encoded_instruction
                let rn_as_u16: u16 = ((encoded_instruction >> 0) & 0b1111) << 0;
                let rn = Register::from_u16(rn_as_u16).expect("Cannot decode RN register!");
                //println!("{:?}", rn);

                // bit S (setflags)
                let _setflags: bool = ((encoded_instruction >> 4) & 0b1) == 1;

                // immediate value is a concatenation of the i-bit, imm3, and imm8
                //#15728640

                let const_immediate: u32 = (((encoded_instruction as u32 >> 10) & 0b1) << 11) | (((next_encoded_instruction as u32 >> 12) & 0b111) << 8) | (((next_encoded_instruction as u32 >> 0) & 0b11111111) << 0);
                //println!("{:?}", const_immediate);
                
                let const_immediate_expanded = self.thumb_expand_imm_c(const_immediate, 0);
                //println!("{:?}", const_immediate_expanded);

                asm_line.instruction = Instruction::ORR_W;
                asm_line.reg1 = rd;
                asm_line.reg2 = rn;
                asm_line.immediate = const_immediate_expanded as i32;

            }

            //
            // A8.8.204 STR (immediate, Thumb)
            //

            encoded_instruction if ( ( ( (encoded_instruction >> 4) & 0b111111111111 ) == 0b111110001100 ) ) => {

                //println!("A8.8.204 STR (immediate, Thumb)");

                // Rt register is [15:12] of next_encoded_instruction
                let rt_as_u16: u16 = ((next_encoded_instruction >> 12) & 0b1111) << 0;
                let rt = Register::from_u16(rt_as_u16).expect("Cannot decode RT register!");
                //println!("{:?}", rt);

                // Rn register is [3:0] of encoded_instruction
                let rn_as_u16: u16 = ((encoded_instruction >> 0) & 0b1111) << 0;
                let rn = Register::from_u16(rn_as_u16).expect("Cannot decode RN register!");
                //println!("{:?}", rn);

                // bit S (setflags)
                //let setflags: bool = ((encoded_instruction >> 4) & 0b1) == 1;

                // immediate value is a concatenation of the i-bit, imm3, and imm8
                //#15728640

                let const_immediate: u32 = ((next_encoded_instruction as u32 >> 0) & 0b111111111111) << 0;
                //println!("{:?}", const_immediate);
                
                //let const_immediate_expanded = self.thumb_expand_imm_c(const_immediate, 0);
                //println!("{:?}", const_immediate_expanded);

                asm_line.instruction = Instruction::STR_W;
                asm_line.reg1 = rn;
                asm_line.reg2 = rt;
                asm_line.immediate = const_immediate as i32;

            }

            //
            // A8.8.21 BIC (immediate), page A8-338   ------------- opcode: 11110[i]00001[S][Rn]0[imm3][Rd][imm8]
            // A8.8.25 BL, BLX (immediate), page A8-346 ----------- opcode: 11110[S]imm10
            //

            encoded_instruction if ((((encoded_instruction >> 11) & 0b11111) == 0b11110) && (((encoded_instruction >> 5) & 0b11111) == 0b00001)) => {
                println!("A8.8.21 BIC (immediate), page A8-338");

                //println!("A8.8.25, BL, BLX (immediate)");

                //let double_word: u32 = ((encoded_instruction as u32) << 16) | ((next_encoded_instruction << 0) as u32);
                //println!("double_word: 0x{:02x}", double_word);

                let i: i32 = (encoded_instruction as i32 >> 10) & 0b1;
                let _s: i32 = (encoded_instruction as i32 >> 4) & 0b1;

                // // I1 = NOT(J1 EOR S);
                // let j1: i32 = (next_encoded_instruction as i32 >> 13) & 1;
                // let i1: i32 = !(j1 ^ s);

                // // I2 = NOT(J2 EOR S);
                // let j2: i32 = (next_encoded_instruction as i32 >> 11) & 1;
                // let i2: i32 = !(j2 ^ s);

                let imm3: i32 = (next_encoded_instruction as i32 >> 12) & 0b111;
                let imm8: i32 = (next_encoded_instruction as i32 >> 0) & 0b1111_1111;

                let imm12: i32 = (i << 11) | (imm3 << 8) | imm8;

                let const_immediate_expanded = self.thumb_expand_imm_c(imm12 as u32, 0);

                // rd
                let rd_as_u16: u16 = (encoded_instruction >> 8) & 0b1111;
                let rd = Register::from_u16(rd_as_u16).expect("Cannot decode Rd register!");
                //println!("rd: {:?}", rd);

                // rn
                let rn_as_u16: u16 = (encoded_instruction >> 12) & 0b1111;
                let rn = Register::from_u16(rn_as_u16).expect("Cannot decode Rn register!");
                //println!("rn: {:?}", rn);
                
                // imm32 = SignExtend(S:I1:I2:imm10:imm11:’0’, 32);
                //let imm32: i32 = (s << 24) | (i1 << 23) | (i2 << 22) | (imm10 << 12) | (imm11 << 1);

                //println!("imm32: 0x{:02x}", imm32);

                // 0x0803B5BC - 0x0803B9A6 = FFFF FFFF FFFF FC16
                // 0x0803B9A6 - 0x0803B5BC = 3EA

                //let simm32 = sign_extend(imm32.try_into().unwrap(), 20);

                //println!("simm32: 0x{:02x}", simm32);

                asm_line.instruction = Instruction::BIC;
                //asm_line.immediate = sign_extend(imm32.try_into().unwrap(), 32);
                //asm_line.immediate = imm32.try_into().unwrap();
                asm_line.reg1 = rd;
                asm_line.reg2 = rn;
                asm_line.immediate = const_immediate_expanded as i32;
            }

            // 11110x | Unconditional Branch, see B on page A8-332 (doc/DDI0406C_d_armv7ar_arm.pdf)
            encoded_instruction if (encoded_instruction >> 11) == 0b11110 => {

                //println!("A8.8.25, BL, BLX (immediate)");

                //let double_word: u32 = ((encoded_instruction as u32) << 16) | ((next_encoded_instruction << 0) as u32);
                //println!("double_word: 0x{:02x}", double_word);

                let s: i32 = (encoded_instruction as i32 >> 10) & 1;

                // I1 = NOT(J1 EOR S);
                let j1: i32 = (next_encoded_instruction as i32 >> 13) & 1;
                let i1: i32 = !(j1 ^ s);

                // I2 = NOT(J2 EOR S);
                let j2: i32 = (next_encoded_instruction as i32 >> 11) & 1;
                let i2: i32 = !(j2 ^ s);

                let imm11: i32 = (next_encoded_instruction as i32 >> 0) & 0b111_1111_1111;

                let imm10: i32 = (encoded_instruction as i32 >> 0) & 0b1111111111;
                
                // imm32 = SignExtend(S:I1:I2:imm10:imm11:’0’, 32);
                let imm32: i32 = (s << 24) | (i1 << 23) | (i2 << 22) | (imm10 << 12) | (imm11 << 1);

                //println!("imm32: 0x{:02x}", imm32);

                // 0x0803B5BC - 0x0803B9A6 = FFFF FFFF FFFF FC16
                // 0x0803B9A6 - 0x0803B5BC = 3EA

                //let simm32 = sign_extend(imm32.try_into().unwrap(), 20);

                //println!("simm32: 0x{:02x}", simm32);

                asm_line.instruction = Instruction::BL;
                //asm_line.immediate = sign_extend(imm32.try_into().unwrap(), 32);
                //asm_line.immediate = imm32.try_into().unwrap();
                asm_line.immediate = imm32;

            }

            encoded_instruction if (encoded_instruction >> 11) == 0b11111 => {

                //println!("LDR.W");

                // 1111 1000 W ??? Rn Rt imm12

                // 1111 1000 1 101 [0010] [0011] [0000 1000 1000]

                let _width_specifier: i32 = (encoded_instruction as i32 >> 7) & 1;
                //println!("width_specifier W: {:?}", width_specifier);

                // rn
                let rn_as_u16: u16 = (encoded_instruction >> 0) & 0b1111;
                let rn = Register::from_u16(rn_as_u16).expect("Cannot decode RT register!");
                //println!("rn: {:?}", rn);

                // rt
                let rt_as_u16: u16 = (next_encoded_instruction >> 12) & 0b1111;
                let rt = Register::from_u16(rt_as_u16).expect("Cannot decode RT register!");
                //println!("rt: {:?}", rt);

                let imm32: i32 = (next_encoded_instruction as i32 >> 0) & 0b111111111111;
                //println!("imm32: 0x{:02x} ({:?})", imm32, imm32);

                asm_line.instruction = Instruction::LDR_W;
                asm_line.reg1 = rn;
                asm_line.reg2 = rt;
                asm_line.immediate = imm32;

            }

            _ => todo!()

        }

    }

    pub fn decode_16bit(&mut self, encoded_instruction: u16, _next_encoded_instruction: u16, asm_line: &mut ASMLine) {

        // A6.2 - 16-bit Thumb instruction encoding, page 221

        // page 221
        match encoded_instruction {

            // 00xxxx, Shift (immediate), add, subtract, move, and compare on page A6-222
            encoded_instruction if (encoded_instruction >> 14) == 0b00 => {
                println!("Shift (immediate), add, subtract, move, and compare on page A6-222");
                self.shift_add_subtract_move_compare(encoded_instruction, asm_line);
            },

            // 010000, Data-processing on page A6-223
            encoded_instruction if (encoded_instruction >> 10) == 0b010000 => {

                // DEBUG
                println!("{:02x?}", encoded_instruction);

                println!("Data-processing on page A6-223");
                // A6.3.4 Branches and miscellaneous control, page A6-233
                let opcode = (encoded_instruction >> 6) & 0b1111;

                match opcode {

                    // 0000 | Bitwise AND | AND (register) on page A8-324
                    // 0001 Bitwise Exclusive OR | EOR (register) on page A8-385
                    // 0010 Logical Shift Left LSL (register) on page A8-471
                    // 0011 Logical Shift Right LSR (register) on page A8-475
                    // 0100 Arithmetic Shift Right ASR (register) on page A8-330
                    // 0101 Add with Carry ADC (register) on page A8-300
                    // 0110 Subtract with Carry SBC (register) on page A8-595
                    // 0111 Rotate Right ROR (register) on page A8-571
                    // 1000 Test TST (register) on page A8-747
                    // 1001 Reverse Subtract from 0 RSB (immediate) on page A8-575
                    // 1010 Compare CMP (register) on page A8-370
                    0b1010 => {
                        // rm
                        let rm_as_u16: u16 = (encoded_instruction >> 0) & 0b111;
                        let rm = Register::from_u16(rm_as_u16).expect("Cannot decode RM register!");
                        println!("{:?}", rm);

                        // rn
                        let rn_as_u16: u16 = (encoded_instruction >> 3) & 0b111;
                        let rn = Register::from_u16(rn_as_u16).expect("Cannot decode RN register!");
                        println!("{:?}", rn);

                        asm_line.instruction = Instruction::CMP;
                        asm_line.reg1 = rm;
                        asm_line.reg2 = rn;
                    }
                    // 1011 Compare Negative CMN (register) on page A8-364
                    // 1100 Bitwise OR ORR (register) on page A8-519
                    // 1101 Multiply MUL on page A8-503
                    // 1110 Bitwise Bit Clear BIC (register) on page A8-340
                    // 1111 Bitwise NOT MVN (register) on page A8-507

                    _ => todo!()
                }
            },

            // 010001, Special data instructions and branch and exchange on page A6-224
            encoded_instruction if (encoded_instruction >> 10) == 0b010001 => { 
                println!("Special data instructions and branch and exchange on page A6-224");
                self.special_data_branch_exchange(encoded_instruction, asm_line);
            },

            // 01001x | Load from Literal Pool, see LDR (literal) on page A8-411 (doc/DDI0406C_d_armv7ar_arm.pdf)
            encoded_instruction if (encoded_instruction >> 11) == 0b01001 => { 
                println!("Load from Literal Pool, see LDR (literal) on page A8-411");
                self.load_from_literal_pool(encoded_instruction, asm_line);
            },

            // 0101xx | Load/store single data item on page A6-225 (doc/DDI0406C_d_armv7ar_arm.pdf)
            encoded_instruction if (encoded_instruction >> 12) == 0b0101 => { 
                println!("1. Load/store single data item on page A6-225");
                todo!()
            },
            encoded_instruction if (encoded_instruction >> 13) == 0b011 => { 
                println!("2. Load/store single data item on page A6-225");
                self.load_store_single_data_item(encoded_instruction, asm_line);
            },
            encoded_instruction if (encoded_instruction >> 13) == 0b100 => { 
                println!("3. Load/store single data item on page A6-225");
                todo!()
            },

            // 10100x | Generate PC-relative address, see ADR on page A8-320 (doc/DDI0406C_d_armv7ar_arm.pdf)
            encoded_instruction if (encoded_instruction >> 11) == 0b10100 => { 
                println!("Generate PC-relative address, see ADR on page A8-320");
                todo!()
            }

            // 10101x | Generate SP-relative address, see ADD (SP plus immediate) on page A8-314 (doc/DDI0406C_d_armv7ar_arm.pdf)
            encoded_instruction if (encoded_instruction >> 11) == 0b10101 => { 
                println!("Generate SP-relative address, see ADD (SP plus immediate) on page A8-314");
                todo!()
            }

            // 1011xx | Miscellaneous 16-bit instructions on page A6-226 (doc/DDI0406C_d_armv7ar_arm.pdf)
            encoded_instruction if (encoded_instruction >> 12) == 0b1011 => { 
                println!("Miscellaneous 16-bit instructions on page A6-226");
                self.miscellaneous(encoded_instruction, asm_line);
            }

            // 11000x | Store multiple registers, see STM (STMIA, STMEA) on page A8-665 a
            encoded_instruction if (encoded_instruction >> 11) == 0b11000 => { 
                println!("Store multiple registers, see STM (STMIA, STMEA) on page A8-665 a");
                todo!()
            }

            // 11001x | Load multiple registers, see LDM/LDMIA/LDMFD (Thumb) on page A8-397 a 
            encoded_instruction if (encoded_instruction >> 11) == 0b11001 => { 
                println!("Load multiple registers, see LDM/LDMIA/LDMFD (Thumb) on page A8-397 a");
                todo!()
            }

            // 1101xx | Conditional branch, and Supervisor Call on page A6-227 (doc/DDI0406C_d_armv7ar_arm.pdf)
            encoded_instruction if (encoded_instruction >> 12) == 0b1101 => { 
                println!("Conditional branch, and Supervisor Call on page A6-227");
                self.conditional_branch_supervisor_call(encoded_instruction, asm_line);
            }

            // 11100x | Unconditional Branch, see B on page A8-332 (doc/DDI0406C_d_armv7ar_arm.pdf)
            encoded_instruction if (encoded_instruction >> 12) == 0b1110 => { 
                println!("Unconditional Branch, see B on page A8-332");
                self.unconditional_branch(encoded_instruction, asm_line);
            }

            _ => todo!()
            
        }

    }

    // A6.2.1 Shift (immediate), add, subtract, move, and compare, page 221
    pub fn shift_add_subtract_move_compare(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {

        // let opcode:u16 = (encoded_instruction >> 11) & 0b11111;
        // println!("opcode: {}", opcode);

        // A6.2.1 Shift (immediate), add, subtract, move, and compare, page 221
        //
        // [00][Opcode][xxxxxxxxx]
        //
        // Here the opcode is checked
        match encoded_instruction {

            // 000xx Logical Shift Left LSL (immediate) on page A8-469
            encoded_instruction if ((encoded_instruction >> 11) & 0b111) == 0b000 => { 
                println!("000xx Logical Shift Left LSL (immediate) on page A8-469"); 
            },
            
            // 001xx Logical Shift Right LSR (immediate) on page A8-473
            encoded_instruction if ((encoded_instruction >> 11) & 0b111) == 0b001 => { 
                println!("001xx Logical Shift Right LSR (immediate) on page A8-473"); 
            },

            // 010xx Arithmetic Shift Right ASR (immediate) on page A8-328
            encoded_instruction if ((encoded_instruction >> 11) & 0b11111) == 0b00010 => { 
                println!("arithmetic_right_shift, A8.8.16, page 328");
                self.arithmetic_right_shift(encoded_instruction, asm_line);
            },

            // 01100 Add register ADD (register, Thumb) on page A8-308
            encoded_instruction if ((encoded_instruction >> 9) & 0b11111) == 0b01100 => { 
                println!("Add register ADD (register, Thumb) on page A8-308");
                self.add_register_add(encoded_instruction, asm_line);
            },

            // 01101 Subtract register SUB (register) on page A8-713

            // 01110 Add 3-bit immediate ADD (immediate, Thumb) on page A8-304

            // 01111 Subtract 3-bit immediate SUB (immediate, Thumb) on page A8-709

            // 100xx Move MOV (immediate) on page A8-485
            encoded_instruction if ((encoded_instruction >> 11) & 0b111) == 0b100 => { 
                println!("Move MOV (immediate) on page A8-485");
                self.move_mov(encoded_instruction, asm_line);
            },

            // 101xx Compare CMP (immediate) on page A8-368
            encoded_instruction if ((encoded_instruction >> 11) & 0b111) == 0b101 => { 
                println!("Compare CMP (immediate) on page A8-368");
                self.compare_cmp(encoded_instruction, asm_line);
            },

            // 110xx Add 8-bit immediate ADD (immediate, Thumb) on page A8-304
            encoded_instruction if ((encoded_instruction >> 10) & 0b111) == 0b111 => { 
                println!("Add 8-bit immediate ADD (immediate, Thumb) on page A8-304");
                self.add_immediate_add_encoding_t1(encoded_instruction, asm_line);
            },

            // 111xx Subtract 8-bit immediate SUB (immediate, Thumb) on page A8-709
            encoded_instruction if ((encoded_instruction >> 11) & 0b111) == 0b110 => { 
                println!("Add 8-bit immediate ADD (immediate, Thumb) on page A8-304");
                self.add_immediate_add_encoding_t2(encoded_instruction, asm_line);
            },

            _ => todo!()

        }
    }

    pub fn special_data_branch_exchange(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {

        match encoded_instruction {

            encoded_instruction if ((encoded_instruction >> 6) & 0b1111) == 0b0000 => { println!("Add Low Registers, ADD (register, Thumb) on page A8-308"); },

            encoded_instruction if ((encoded_instruction >> 6) & 0b1111) == 0b0001 => { println!("1. - Add High Registers, ADD (register, Thumb) on page A8-308"); },
            encoded_instruction if ((encoded_instruction >> 7) & 0b111) == 0b001 => { println!("2. - Add High Registers, ADD (register, Thumb) on page A8-308"); },

            encoded_instruction if ((encoded_instruction >> 8) & 0b111) == 0b01 => { println!("Compare High Registers CMP (register) on page A8-370"); },

            encoded_instruction if ((encoded_instruction >> 6) & 0b1111) == 0b1000 => { 
                println!("Move Low Registers MOV (register, Thumb) on page A8-487"); 
                self.mov_move_low_registers(encoded_instruction, asm_line);
            },

            encoded_instruction if ((encoded_instruction >> 6) & 0b1111) == 0b1001 => { println!("Move High Registers MOV (register, Thumb) on page A8-487"); },
            encoded_instruction if ((encoded_instruction >> 7) & 0b111) == 0b101 => { println!("Move High Registers MOV (register, Thumb) on page A8-487"); },

            encoded_instruction if ((encoded_instruction >> 7) & 0b111) == 0b110 => { 
                println!("Branch and Exchange BX on page A8-350");
                self.branch_and_exchange_bx(encoded_instruction, asm_line);
            },

            encoded_instruction if ((encoded_instruction >> 7) & 0b111) == 0b111 => { println!("Branch with Link and Exchange BLX (register) on page A8-348"); },

            _ => todo!()

        }

    }

    // A8.8.65 LDR (literal)
    // Load Register (literal) calculates an address from the PC value and an immediate offset, 
    // loads a word from memory, and writes it to a register. 
    // For information about memory accesses see Memory accesses on page A8-292.
    pub fn load_from_literal_pool(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {

        // rt
        let rt_as_u16: u16 = (encoded_instruction >> 8) & 0b111;
        let rt = Register::from_u16(rt_as_u16).expect("Cannot decode RT register!");
        println!("{:?}", rt);

        // immediate
        let mut immediate: i32 = (encoded_instruction & 0xFF).into();
        immediate = immediate * 4; // two zeros are postfixed. This is the same as multiplying by 4

        asm_line.instruction = Instruction::LDR;
        asm_line.reg1 = rt;
        asm_line.reg2 = Register::PC; // implicitly set the PC register
        asm_line.immediate = immediate;
    }

    pub fn load_store_single_data_item(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {

        // 681a

        let op_a: u16 = (encoded_instruction >> 12) & 0b1111;
        let op_b: u16 = (encoded_instruction >> 9) & 0b111;

        match op_a {

            0b0101 => {
                match op_b {
                    0b000 => {
                        println!("Store Register                    - STR (register) on page A8-677");
                    }
                    0b001 => {
                        println!("Store Register Halfword           - STRH (register) on page A8-703");
                    }
                    0b010 => {
                        println!("Store Register Byte               - STRB (register) on page A8-683");
                    }
                    0b011 => {
                        println!("Load Register Signed Byte         - LDRSB (register) on page A8-455");
                    }
                    0b100 => {
                        println!("Load Register                     - LDR (register, Thumb) on page A8-413");
                    }
                    0b101 => {
                        println!("Load Register Halfword            - LDRH (register) on page A8-447");
                    }
                    0b110 => {
                        println!("Load Register Byte                - LDRB (register) on page A8-423");
                    }
                    0b111 => {
                        println!("Load Register Signed Halfword     - LDRSH (register) on page A8-463");
                    }
                    _ => {
                        todo!()
                    }
                }
            }

            0b0110 => {
                match op_b {
                    //0b0xx => {
                    _op_b if((encoded_instruction >> 11) & 0b1) == 0b0 => {
                        println!("Store Register    - STR (immediate, Thumb) on page A8-673");

                        // A8.8.204 STR (immediate, Thumb)

                        // rt
                        let rt_as_u16: u16 = (encoded_instruction >> 0) & 0b111;
                        let rt = Register::from_u16(rt_as_u16).expect("Cannot decode RT register!");
                        println!("{:?}", rt);

                        // rn
                        let rn_as_u16: u16 = (encoded_instruction >> 3) & 0b111;
                        let rn = Register::from_u16(rn_as_u16).expect("Cannot decode RN register!");
                        println!("{:?}", rn);

                        // imm5
                        let mut imm5_as_u16: u16 = (encoded_instruction >> 6) & 0b11111;
                        imm5_as_u16 = imm5_as_u16 * 4; // postfix 00 which is the same as multiplying by 4

                        //let imm5 = Register::from_u16(imm5_as_u16).expect("Cannot decode IMM5!");
                        //println!("{:?}", imm5);

                        asm_line.instruction = Instruction::STR;
                        asm_line.reg1 = rt;
                        asm_line.reg2 = rn;
                        asm_line.immediate = imm5_as_u16 as i32;

                        println!("[ThumbDecoder::load_store_single_data_item()] asm_line: {}", asm_line.to_string());
                    }
                    //0b1xx => {
                    _op_b if((encoded_instruction >> 11) & 0b1) == 0b1 => {
                        println!("Load Register     - LDR (immediate, Thumb) on page A8-407");

                        // A8.8.63 LDR (immediate, Thumb)

                        // rt
                        let rt_as_u16: u16 = (encoded_instruction >> 0) & 0b111;
                        let rt = Register::from_u16(rt_as_u16).expect("Cannot decode RT register!");
                        println!("{:?}", rt);

                        // rn
                        let rn_as_u16: u16 = (encoded_instruction >> 3) & 0b111;
                        let rn = Register::from_u16(rn_as_u16).expect("Cannot decode RN register!");
                        println!("{:?}", rn);

                        // imm5
                        let imm5_as_u16: u16 = (encoded_instruction >> 6) & 0b11111;
                        //let imm5 = Register::from_u16(imm5_as_u16).expect("Cannot decode IMM5!");
                        //println!("{:?}", imm5);

                        asm_line.instruction = Instruction::LDR;
                        asm_line.reg1 = rt;
                        asm_line.reg2 = rn;
                        asm_line.immediate = imm5_as_u16 as i32;

                        println!("[ThumbDecoder::load_store_single_data_item()] asm_line: {}", asm_line.to_string());
                    }
                    _ => {
                        todo!()
                    }
                }
            }

            0b0111 => {
                match op_b {
                    //0b0xx => {
                    _op_b if((encoded_instruction >> 11) & 0b1) == 0b0 => {
                        println!("Store Register Byte   - STRB (immediate, Thumb) on page A8-679");
                    }
                    //0b1xx => {
                    _op_b if((encoded_instruction >> 11) & 0b1) == 0b1 => {
                        println!("Load Register Byte    - LDRB (immediate, Thumb) on page A8-417");
                    }
                    _ => {
                        todo!()
                    }
                }
            }

            0b1000 => {
                match op_b {
                    //0b0xx => {
                    _op_b if((encoded_instruction >> 11) & 0b1) == 0b0 => {
                        println!("Store Register Halfword   - STRH (immediate, Thumb) on page A8-699");
                    }
                    //0b1xx => {
                    _op_b if((encoded_instruction >> 11) & 0b1) == 0b1 => {
                        println!("Load Register Halfword    - LDRH (immediate, Thumb) on page A8-441");
                    }
                    _ => {
                        todo!()
                    }
                }
            }

            0b1001 => {
                match op_b {
                    //0b0xx => {
                    _op_b if((encoded_instruction >> 11) & 0b1) == 0b0 => {
                        println!("Store Register SP relative    - STR (immediate, Thumb) on page A8-673");
                    }
                    //0b1xx => {
                    _op_b if((encoded_instruction >> 11) & 0b1) == 0b1 => {
                        println!("Load Register SP relative     - LDR (immediate, Thumb) on page A8-407");
                    }
                    _ => {
                        todo!()
                    }
                }
            }

            _ => {
                todo!()
            }
        }
    }

    // page A6-226 (doc/DDI0406C_d_armv7ar_arm.pdf)
    pub fn miscellaneous(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {

        //let opcode = (encoded_instruction >> 5) & 0b1111111;

        match encoded_instruction {

            encoded_instruction if ((encoded_instruction >> 7) & 0b11111) == 0b00000 => { 
                // 00000xx, Add Immediate to SP, ADD (SP plus immediate) on page A8-314, v4T
                println!("00000xx, Add Immediate to SP, ADD (SP plus immediate) on page A8-314, v4T");
            },

            encoded_instruction if ((encoded_instruction >> 7) & 0b11111) == 0b00001 => {
                // 00001xx, Subtract Immediate from SP, SUB (SP minus immediate) on page A8-717, v4T
            }

            encoded_instruction if ((encoded_instruction >> 8) & 0b1111) == 0b0001 => {
                // 0001xxx, Compare and Branch on Zero, CBNZ, CBZ on page A8-354, v6T2
            }

            encoded_instruction if ((encoded_instruction >> 6) & 0b111111) == 0b001000 => {
                // 001000x, Signed Extend Halfword, SXTH on page A8-735, v6
            }

            encoded_instruction if ((encoded_instruction >> 6) & 0b111111) == 0b001001 => {
                // 001001x, Signed Extend Byte, SXTB on page A8-731, v6
            }

            encoded_instruction if ((encoded_instruction >> 6) & 0b111111) == 0b001010 => {
                // 001010x, Unsigned Extend Halfword, UXTH on page A8-817, v6
            }

            encoded_instruction if ((encoded_instruction >> 6) & 0b111111) == 0b001011 => {
                // 001011x, Unsigned Extend Byte, UXTB on page A8-813, v6
            }

            encoded_instruction if ((encoded_instruction >> 8) & 0b1111) == 0b0011 => {
                // 0011xxx, Compare and Branch on Zero, CBNZ, CBZ on page A8-354, v6T2, 
            }

            encoded_instruction if ((encoded_instruction >> 9) & 0b111) == 0b010 => {
                // 010xxxx, Push Multiple Registers, PUSH on page A8-539, v4T
                println!("010xxxx, Push Multiple Registers, PUSH on page A8-539, v4T");

                // 1011 0 10 1 0000 1000

                let mut registers:u16 = 0;

                // set the M bit (Corresponds to the LR register)
                let mbit: u16 = (encoded_instruction >> 8) & 0b1;
                registers = registers | mbit << 14;

                // set the register list
                registers = registers | ((encoded_instruction >> 0) & 0b11111111) << 0;

                if registers == 0 {
                    todo!("UNPREDICTABLE")
                }

                // push {r3, lr}
                let mut mask = 1u16; // assuming rightmost bit first
                for i in 0..15 {

                    let is_set:bool = registers & mask != 0;
                    if is_set {
                        if i == 14 {
                            println!("push LR register");
                        } else {
                            println!("push R{} register", i);
                        }
                    }

                    mask <<= 1; // assuming rightmost bit first
                }

                asm_line.instruction = Instruction::PUSH;
            }

            encoded_instruction if ((encoded_instruction >> 5) & 0b1111111) == 0b0110010 => {
                // 0110010, Set Endianness, SETEND on page A8-605, v6
            }

            encoded_instruction if ((encoded_instruction >> 5) & 0b1111111) == 0b0110011 => {
                // 0110011, Change Processor State, CPS (Thumb) on page B9-1964, v6
            }

            encoded_instruction if ((encoded_instruction >> 8) & 0b1111) == 0b1001 => {
                // 1001xxx, Compare and Branch on Nonzero, CBNZ, CBZ on page A8-354, v6T2
            }
            
            encoded_instruction if ((encoded_instruction >> 6) & 0b111111) == 0b101000 => {
                // 101000x, Byte-Reverse Word, REV on page A8-563, v6
            }
            
            encoded_instruction if ((encoded_instruction >> 6) & 0b111111) == 0b101001 => {
                // 101001x, Byte-Reverse Packed Halfword, REV16 on page A8-565, v6
            }

            encoded_instruction if ((encoded_instruction >> 6) & 0b111111) == 0b101011 => {
                // 101011x, Byte-Reverse Signed Halfword, REVSH on page A8-567, v6
            }

            encoded_instruction if ((encoded_instruction >> 8) & 0b1111) == 0b1011 => {
                // 1011xxx, Compare and Branch on Nonzero, CBNZ, CBZ on page A8-354, v6T2
            }
            
            encoded_instruction if ((encoded_instruction >> 9) & 0b111) == 0b110 => {
                // 110xxxx, Pop Multiple Registers, POP (Thumb) on page A8-535, v4T
            }

            encoded_instruction if ((encoded_instruction >> 8) & 0b1111) == 0b1110 => {
                // 1110xxx, Breakpoint, BKPT on page A8-344, v5
            }

            encoded_instruction if ((encoded_instruction >> 8) & 0b1111) == 0b1111 => {    
                // 1111xxx, If-Then, and hints, If-Then, and hints on page A6-227
            }

            _ => todo!()
        }

    }

    pub fn add_register_add(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {
        
        // rm
        let rm_as_u16: u16 = (encoded_instruction >> 6) & 0b111;
        let rm = Register::from_u16(rm_as_u16).expect("Cannot decode RM register!");
        println!("{:?}", rm);

        // rn
        let rn_as_u16: u16 = (encoded_instruction >> 3) & 0b111;
        let rn = Register::from_u16(rn_as_u16).expect("Cannot decode RN register!");
        println!("{:?}", rn);

        // rd
        let rd_as_u16: u16 = (encoded_instruction >> 0) & 0b111;
        let rd = Register::from_u16(rd_as_u16).expect("Cannot decode RD register!");
        println!("{:?}", rd);

        asm_line.instruction = Instruction::ADD;
        asm_line.reg1 = rd;
        asm_line.reg2 = rn;
        asm_line.reg3 = rm;
    }

    pub fn add_immediate_add_encoding_t1(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {

        // imm3
        let imm3_as_u16: u16 = (encoded_instruction >> 6) & 0b111;
        let imm3 = Register::from_u16(imm3_as_u16).expect("Cannot decode imm3 register!");
        println!("{:?}", imm3);

        // rn
        let rn_as_u16: u16 = (encoded_instruction >> 3) & 0b111;
        let rn = Register::from_u16(rn_as_u16).expect("Cannot decode RN register!");
        println!("{:?}", rn);

        // rd
        let rd_as_u16: u16 = (encoded_instruction >> 0) & 0b111;
        let rd = Register::from_u16(rd_as_u16).expect("Cannot decode RD register!");
        println!("{:?}", rd);

        asm_line.instruction = Instruction::ADD;
        asm_line.reg1 = rd;
        asm_line.reg2 = rn;
        asm_line.reg3 = imm3;
    }

    pub fn add_immediate_add_encoding_t2(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {

        // imm8
        let imm8_as_u16: u16 = (encoded_instruction >> 0) & 0xFF;
        let imm8:i32 = imm8_as_u16.into();
        println!("{:?}", imm8);

        // rdn
        let rdn_as_u16: u16 = (encoded_instruction >> 8) & 0b111;
        let rdn = Register::from_u16(rdn_as_u16).expect("Cannot decode RD register!");
        println!("{:?}", rdn);

        asm_line.instruction = Instruction::ADD;
        asm_line.update_flags = true;
        asm_line.reg1 = rdn; // destination and source register is the same
        asm_line.immediate = imm8;
    }

    pub fn move_mov(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {
        
        // rd
        let rd_as_u16: u16 = (encoded_instruction >> 8) & 0b111;
        let rd = Register::from_u16(rd_as_u16).expect("Cannot decode RD register!");
        println!("{:?}", rd);

        // imm
        let immediate: i32 = ((encoded_instruction >> 0) & 0xFF).into();

        asm_line.instruction = Instruction::MOV;
        asm_line.reg1 = rd;
        asm_line.immediate = immediate;
    }

    pub fn compare_cmp(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {
        
        // rn
        let rn_as_u16: u16 = (encoded_instruction >> 8) & 0b111;
        let rn = Register::from_u16(rn_as_u16).expect("Cannot decode RN register!");
        println!("{:?}", rn);

        // imm
        let immediate: i32 = ((encoded_instruction >> 0) & 0xFF).into();

        asm_line.instruction = Instruction::CMP;
        asm_line.reg1 = rn;
        asm_line.immediate = immediate;
    }

    pub fn arithmetic_right_shift(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {
        
        // rm
        let rm_as_u16: u16 = (encoded_instruction >> 3) & 0b111;
        let rm = Register::from_u16(rm_as_u16).expect("Cannot decode RM register!");
        println!("{:?}", rm);

        // rd
        let rd_as_u16: u16 = (encoded_instruction >> 0) & 0b111;
        let rd = Register::from_u16(rd_as_u16).expect("Cannot decode RD register!");
        println!("{:?}", rd);

        // imm5
        let immediate: i32 = ((encoded_instruction >> 6) & 0x1F).into();

        // imm8
        // I do not understand, why the immediate is shifted right by seven
        // https://publish.obsidian.md/cynixia/ARM+Instruction+Encoding lists exactly the reason
        // This page has a different encoding that the PDF!
        // The page just reads the first 7 bit as immediate!
        //let immediate: i32 = ((encoded_instruction >> 7) & 0xFF).into();

        asm_line.instruction = Instruction::ASR;
        asm_line.reg1 = rm;
        asm_line.reg2 = rd;
        asm_line.immediate = immediate;
    }

    pub fn conditional_branch_supervisor_call(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {

        // page 227
        match encoded_instruction {

            encoded_instruction if ((encoded_instruction >> 8) & 0b1111) == 0b1110 => { println!("Permanently UNDEFINED UDF on page A8-759a"); },

            encoded_instruction if ((encoded_instruction >> 8) & 0b1111) == 0b1111 => { 
                println!("Supervisor Call SVC (previously SWI) on page A8-721");
                self.supervisor_call(encoded_instruction, asm_line);
            },

            _ => { 
                println!("Conditional branch B on page A8-332"); 
                self.conditional_branch(encoded_instruction, asm_line);
            }
        }
    }

    pub fn unconditional_branch(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {
        // jump target: 4 + bits * 2
        //let mut counter:i32 = (encoded_instruction & 0xFF) as i32;
        //counter = 4 + counter * 2;

        let mut pc_rel: u16 = encoded_instruction & 0b111_1111_1111;
        pc_rel = pc_rel * 2;

        let sign_extend: i32 = sign_extend(pc_rel as i32, 12);

        //println!("sign_extend: {}", sign_extend);
        //println!("sign_extend: {}", sign_extend);

        //let pc_rel_2: i16 = pc_rel as i16;

        //println!("pc-rel offset: {:02x?}", pc_rel_2);

        //asm_line.jump_offset = pc_rel_2 as i32;

        // branch always (unconditional)
        asm_line.instruction = Instruction::BAL;
        asm_line.jump_offset = sign_extend;
    }

    // now: svc (Supervisor Call), earlier: swi (Software Interrupt)
    pub fn supervisor_call(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {

        asm_line.instruction = Instruction::SVC;

        let immediate: i32 = (encoded_instruction & 0xFF).into();
        asm_line.immediate = immediate;
    }

    // page 332
    pub fn conditional_branch(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {

        match encoded_instruction {

            encoded_instruction if ((encoded_instruction >> 12) & 0b1111) == 0b1101 => { 
                println!("B-instruction, doc/DDI0406C_d_armv7ar_arm.pdf, page 332");

                let condition: u16 = (encoded_instruction >> 8) & 0b1111;

                // the possible conditions are listed in the table A8.3 Conditional execution, page A8-286, doc/DDI0406C_d_armv7ar_arm.pdf
                match condition {

                    0b0000 => {
                        // // sign extension?????
                        // asm_line.immediate = ((encoded_instruction >> 0) & 0xFF).into();
                        // asm_line.immediate = sign_extend(asm_line.immediate, 32);

                        // // jump target: 4 + bits * 2
                        // let mut counter:i32 = (encoded_instruction & 0xFF) as i32;
                        // counter = 4 + counter * 2;

                        // println!("counter: {:02x?}", counter);

                        let mut pc_rel: u16 = encoded_instruction & 0b111_1111_1111;
                        pc_rel = pc_rel * 2;

                        let sign_extend: i32 = sign_extend(pc_rel as i32, 12);

                        asm_line.instruction = Instruction::BEQ;
                        asm_line.jump_offset = sign_extend;
                    },

                    0b1100 => {
                        // asm_line.instruction = Instruction::BGT;

                        // // // sign extension?????
                        // // asm_line.immediate = ((encoded_instruction >> 0) & 0xFF).into();
                        // // asm_line.immediate = sign_extend(asm_line.immediate, 32);

                        // // jump target: 4 + bits * 2
                        // let mut counter:i32 = (encoded_instruction & 0xFF) as i32;
                        // counter = 4 + counter * 2;

                        // println!("counter: {:02x?}", counter);

                        // asm_line.jump_offset = counter;

                        let mut pc_rel: u16 = encoded_instruction & 0b111_1111_1111;
                        pc_rel = pc_rel * 2;

                        let sign_extend: i32 = sign_extend(pc_rel as i32, 12);

                        asm_line.instruction = Instruction::BGT;
                        asm_line.jump_offset = sign_extend;
                    },

                    _ => todo!()
                }
                
            },

            _ => todo!()

        }

    }

    pub fn mov_move_low_registers(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {

        // rm
        let rm_as_u16: u16 = (encoded_instruction >> 3) & 0b111;
        let rm = Register::from_u16(rm_as_u16).expect("Cannot decode RM register!");
        println!("{:?}", rm);

        // rd
        let rd_as_u16: u16 = (((encoded_instruction >> 7) & 0b1) << 3) | ((encoded_instruction >> 0) & 0b111);
        let rd = Register::from_u16(rd_as_u16).expect("Cannot decode RD register!");
        println!("{:?}", rd);

        asm_line.instruction = Instruction::MOV;
        asm_line.reg1 = rd;
        asm_line.reg2 = rm;
    }

    pub fn branch_and_exchange_bx(&mut self, encoded_instruction: u16, asm_line: &mut ASMLine) {

        asm_line.instruction = Instruction::BX;

        // rm
        let rm_as_u16: u16 = (encoded_instruction >> 3) & 0x0F;
        let rm = Register::from_u16(rm_as_u16).expect("Cannot decode RM register!");
        println!("{:?}", rm);

        asm_line.reg1 = rm;
    }

}

#[test]
fn test_decode_opcode_mov_r0_r1() {

    let encoded_instruction: u16 = 0x4608;
    let next_encoded_instruction: u16 = 0x0000;  

    let mut asm_line: ASMLine = ASMLine::new();

    let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();
    thumb_decoder.decode(encoded_instruction, next_encoded_instruction, &mut asm_line);

    //println!("{}", asm_line);

    assert_eq!(" mov r0, r1", asm_line.to_string());
}

#[test]
fn test_decode_opcode_swi_5() {

    let encoded_instruction: u16 = 0xDF05;
    let next_encoded_instruction: u16 = 0x0000;  

    let mut asm_line: ASMLine = ASMLine::new();

    let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();
    thumb_decoder.decode(encoded_instruction, next_encoded_instruction, &mut asm_line);

    //println!("{}", asm_line);

    assert_eq!(" svc #5", asm_line.to_string());
}

#[test]
fn test_decode_opcode_bx_lr() {

    let encoded_instruction: u16 = 0x4770;
    let next_encoded_instruction: u16 = 0x0000;  

    let mut asm_line: ASMLine = ASMLine::new();

    let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();
    thumb_decoder.decode(encoded_instruction, next_encoded_instruction, &mut asm_line);

    //println!("{}", asm_line);

    assert_eq!(" bx lr", asm_line.to_string());
}

#[test]
fn test_decode_opcode_add_r0_r0_r1() {

    let encoded_instruction: u16 = 0x1840;
    let next_encoded_instruction: u16 = 0x0000;  

    let mut asm_line: ASMLine = ASMLine::new();

    let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();
    thumb_decoder.decode(encoded_instruction, next_encoded_instruction, &mut asm_line);

    //println!("{}", asm_line);

    assert_eq!(" add r0, r0, r1", asm_line.to_string());
}

#[test]
fn test_decode_opcode_adds() {

    let encoded_instruction: u16 = 0x3001;
    let next_encoded_instruction: u16 = 0x0000;  

    let mut asm_line: ASMLine = ASMLine::new();

    let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();
    thumb_decoder.decode(encoded_instruction, next_encoded_instruction, &mut asm_line);

    //println!("{}", asm_line);

    assert_eq!(" adds r0, #1", asm_line.to_string());
}

#[test]
fn test_decode_opcode_cmp_r0_10() {

    //let encoded_instruction: u16 = 0x0a28;
    let encoded_instruction: u16 = 0x280a;
    let next_encoded_instruction: u16 = 0x0000;  

    let mut asm_line: ASMLine = ASMLine::new();

    let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();
    thumb_decoder.decode(encoded_instruction, next_encoded_instruction, &mut asm_line);

    //println!("{}", asm_line);

    assert_eq!(" cmp r0, #10", asm_line.to_string());
}

#[test]
fn test_decode_opcode_cmp_r0_r3() {

    let encoded_instruction: u16 = 0x4299;
    let next_encoded_instruction: u16 = 0x0000;  

    let mut asm_line: ASMLine = ASMLine::new();

    let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();
    thumb_decoder.decode(encoded_instruction, next_encoded_instruction, &mut asm_line);

    //println!("{}", asm_line);

    assert_eq!(" cmp r0, r3", asm_line.to_string());
}

#[test]
fn test_beq_stop() {

    //let encoded_instruction: u16 = 0x06D0;
    let encoded_instruction: u16 = 0xD006;
    let next_encoded_instruction: u16 = 0x0000;  

    let mut asm_line: ASMLine = ASMLine::new();

    let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();
    thumb_decoder.decode(encoded_instruction, next_encoded_instruction, &mut asm_line);

    //println!("{}", asm_line);

    assert_eq!(" beq 6/", asm_line.to_string());
}

#[test]
fn test_wfi() {

    let encoded_instruction: u16 = 0xBF30;
    let next_encoded_instruction: u16 = 0x0000;  

    let mut asm_line: ASMLine = ASMLine::new();

    let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();
    thumb_decoder.decode(encoded_instruction, next_encoded_instruction, &mut asm_line);

    //println!("{}", asm_line);

    assert_eq!(" wfi", asm_line.to_string());
}

#[test]
fn test_ldr() {

    //let encoded_instruction: u16 = 0x0248;
    let encoded_instruction: u16 = 0x4802;
    let next_encoded_instruction: u16 = 0x0000;  

    let mut asm_line: ASMLine = ASMLine::new();

    let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();
    thumb_decoder.decode(encoded_instruction, next_encoded_instruction, &mut asm_line);

    //println!("{}", asm_line);

    assert_eq!(" ldr #2", asm_line.to_string());
}

#[test]
fn test_ldr_2() {

    // ldr r2, [pc, #0x34]

    //let encoded_instruction: u16 = 0x0d4a;
    let encoded_instruction: u16 = 0x4a0d;
    let next_encoded_instruction: u16 = 0x0000;  

    let mut asm_line: ASMLine = ASMLine::new();

    let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();
    thumb_decoder.decode(encoded_instruction, next_encoded_instruction, &mut asm_line);

    //println!("{}", asm_line);

    let result = asm_line.to_string();

    assert_eq!(" ldr #52", result);
}


#[test]
fn test_asrs() {

    //let encoded_instruction: u16 = 0x0010;
    let encoded_instruction: u16 = 0x1000;
    //let encoded_instruction: u16 = 0x1500;

    let next_encoded_instruction: u16 = 0x0000;   

    let mut asm_line: ASMLine = ASMLine::new();

    let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();
    thumb_decoder.decode(encoded_instruction, next_encoded_instruction, &mut asm_line);

    //println!("{}", asm_line);

    assert_eq!(" asr r0, r0, #0", asm_line.to_string());
    //assert_eq!(" asr r0, r0, #0x20", asm_line.to_string());
}

#[test]
fn test_bl() {

    let encoded_instruction: u16 = 0x7DF8;
    let next_encoded_instruction: u16 = 0x00F0;

    let mut asm_line: ASMLine = ASMLine::new();

    let mut thumb_decoder: ThumbDecoder = ThumbDecoder::new();
    thumb_decoder.decode(encoded_instruction, next_encoded_instruction, &mut asm_line);

    //println!("{}", asm_line);

    assert_eq!(" asr r0, r0, #0", asm_line.to_string());
    //assert_eq!(" asr r0, r0, #0x20", asm_line.to_string());
}
