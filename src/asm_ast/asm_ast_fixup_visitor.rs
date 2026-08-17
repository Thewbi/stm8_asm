use std::collections::HashMap;

use crate::tacky::tacky::ValueElement;

use crate::AsmAstProgram;

use crate::asm_ast::asm_ast::AsmAstFunction;
use crate::asm_ast::asm_ast::AsmAstInstruction;
use crate::asm_ast::asm_ast::AsmAstInstructionType;
use crate::asm_ast::asm_ast::AsmAstOperand;
use crate::asm_ast::asm_ast::AsmAstOperandType;
use crate::asm_ast::asm_ast::AsmAstBinaryOperator;
use crate::asm_ast::asm_ast::AsmAstReg;

//
// Fixes up the Asm AST by 
//
// - replacing pseudo variables/operands with stack addresses
// - replacing mov which move from memory to memory without using a temp register (not possible in x86)
//
// 1. c_ast/IdentifierResolutionVisitor - checks for duplicate or undeclared variable names
// 2. tacky/TackyVisitor - Generate TACKY (from AST)
// 3. asm_ast/AsmAstConversionVisitor - Converts the AST into a ASM AST for assembly with a precursory form of mnenomics
// 4. asm_ast/AsmAstFixupVisitor - replacing pseudo operands/variables with stack addresses
// 5. asm_ast/AsmAstMasmEmitterVisitor / asm_ast/AsmAstASEmitterVisitor / ...
//

pub struct AsmAstFixupVisitor {
    pub asm_ast_program: AsmAstProgram,
    pub replace_pseudo: bool,
    pub stack_offset: i32,
    pub stack_offset_map: HashMap::<String, i32>,
}

impl AsmAstFixupVisitor {

    pub fn new() -> AsmAstFixupVisitor {
        AsmAstFixupVisitor {
            asm_ast_program: AsmAstProgram::new(),
            replace_pseudo: false,
            stack_offset: 0,
            stack_offset_map: HashMap::<String, i32>::new(),
        }
    }

    pub fn visit_asm_ast_program(&mut self, asm_ast_program: &mut AsmAstProgram) {
        // println!("[FixupAsmAstVisitor::visit_asm_ast_program()]");
        // println!("  name = {}", asm_ast_program.name);

        self.visit_asm_ast_function(&mut asm_ast_program.function);
    }

    pub fn visit_asm_ast_function(&mut self, asm_ast_function: &mut AsmAstFunction) {
        // println!("[FixupAsmAstVisitor::visit_asm_ast_function()]");
        // println!("  name = {}", asm_ast_function.name);

        // reset stack offset to get rid of the stale value from the last function definition
        self.stack_offset = 0;

        let mut new_body = Vec::<Box<AsmAstInstruction>>::new();
        for i in 0..asm_ast_function.body.len() {
            self.visit_asm_ast_instruction(&mut new_body, asm_ast_function.body[i].as_ref().clone());
        }

        if self.replace_pseudo {
            // patch allocate stack because right now, the required stack size is readily available
            let asm_ast_instruction = &mut new_body[0];
            asm_ast_instruction.src = AsmAstOperand { operand_type: AsmAstOperandType::Imm(self.stack_offset * -1) };
            // asm_ast_instruction.src = AsmAstOperand { operand_type: AsmAstOperandType::Imm(self.stack_offset) };

            asm_ast_function.stack_frame_size = self.stack_offset * -1;
        }

        // DEBUG
        println!("-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-");
        for i in 0..new_body.len() {
            //println!("{:?}", new_body[i]);
            println!("{}", new_body[i]);
        }
        println!("-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-");

        asm_ast_function.body = new_body;
    }

    #[allow(unreachable_code)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    #[allow(unreachable_patterns)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    pub fn visit_asm_ast_instruction(&mut self, new_body: &mut Vec::<Box<AsmAstInstruction>>, mut asm_ast_instruction: AsmAstInstruction) {

        match asm_ast_instruction.instruction_type {

            AsmAstInstructionType::AllocateStack => {
                println!("AllocateStack({:?})", asm_ast_instruction.src);
                // println!("{:?}", asm_ast_instruction);
                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Ret => {
                println!("Ret");
                // println!("{:?}", asm_ast_instruction);
                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Mov => {
                println!("Mov {:?} {:?}", asm_ast_instruction.src, asm_ast_instruction.dst);

                // replace pseudo operand by relative address on stack
                asm_ast_instruction.src = self.replace_pseudo_operand(&mut asm_ast_instruction.src);
                asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                if self.replace_pseudo {

                    let mut fix: bool = false;

                    // x86 mov cannot move from memory (stack or other memory) to memory directly!
                    if matches!(asm_ast_instruction.src.operand_type, AsmAstOperandType::Stack(_)) {
                        if matches!(asm_ast_instruction.dst.operand_type, AsmAstOperandType::Stack(_)) {
                            fix = true;
                        }
                    }

                    if fix {

                        let mut mov_1 = asm_ast_instruction.clone();
                        mov_1.instruction_type = AsmAstInstructionType::Mov;
                        mov_1.dst = AsmAstOperand { operand_type: AsmAstOperandType::Reg(AsmAstReg::R10) };
                        new_body.push(Box::new(mov_1));

                        asm_ast_instruction.src = AsmAstOperand { operand_type: AsmAstOperandType::Reg(AsmAstReg::R10) };
                        new_body.push(Box::new(asm_ast_instruction));
                        
                    } else {

                        new_body.push(Box::new(asm_ast_instruction));

                    }
                } else {
                    new_body.push(Box::new(asm_ast_instruction));
                }
            }

            AsmAstInstructionType::Unary => {
                println!("Unary {:?} {:?}", asm_ast_instruction.unary_operator, asm_ast_instruction.dst);

                asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Binary => {
                println!("Binary {:?} {:?} {:?}", asm_ast_instruction.binary_operator, asm_ast_instruction.src_2, asm_ast_instruction.dst);

                asm_ast_instruction.src_2 = self.replace_pseudo_operand(&mut asm_ast_instruction.src_2);
                asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                // add dword ptr [ebp-8], dword ptr [ebp-4]
                // new_body.push(Box::new(asm_ast_instruction));

                match asm_ast_instruction.binary_operator {

                    AsmAstBinaryOperator::Add => {

                        if self.replace_pseudo {

                            let mut fix: bool = false;

                            // x86 mov cannot move from memory (stack or other memory) to memory directly!
                            if matches!(asm_ast_instruction.src_2.operand_type, AsmAstOperandType::Stack(_)) {
                                if matches!(asm_ast_instruction.dst.operand_type, AsmAstOperandType::Stack(_)) {
                                    fix = true;
                                }
                            }

                            if fix {

                                // mov dword ptr [ebp-8], R10
                                let mut mov_1 = asm_ast_instruction.clone();
                                mov_1.instruction_type = AsmAstInstructionType::Mov;
                                mov_1.src = asm_ast_instruction.src_2.clone();
                                mov_1.dst = AsmAstOperand { operand_type: AsmAstOperandType::Reg(AsmAstReg::R10) };
                                new_body.push(Box::new(mov_1));

                                // add
                                // https://www.felixcloutier.com/x86/add
                                // Adds the destination operand (first operand) and the source operand (second operand) 
                                // and then stores the result in the destination operand.
                                asm_ast_instruction.src_2 = AsmAstOperand { operand_type: AsmAstOperandType::Reg(AsmAstReg::R10) };

                                println!("{}", asm_ast_instruction.clone());

                                new_body.push(Box::new(asm_ast_instruction));

                            } else {

                                new_body.push(Box::new(asm_ast_instruction));
                            }
                        } else {
                            new_body.push(Box::new(asm_ast_instruction));
                        }
                    }

                    _ => {

                    }
                }

            }

            AsmAstInstructionType::Cdq => {
                println!("Cdq");

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Idiv => {
                println!("Idiv {:?}", asm_ast_instruction.dst);

                asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Mod => {
                println!("Mod {:?}", asm_ast_instruction.dst);

                asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Mul => {
                println!("Mul {:?}", asm_ast_instruction.dst);

                asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Cmp => {
                println!("Cmp {:?} {:?}", asm_ast_instruction.src, asm_ast_instruction.src_2);

                asm_ast_instruction.src_2 = self.replace_pseudo_operand(&mut asm_ast_instruction.src_2);

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Jmp => {
                println!("Jmp {:?} {:?} {:?}", asm_ast_instruction.src, asm_ast_instruction.src_2, asm_ast_instruction.dst);

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::JmpCC => {
                println!("JmpCC {:?} {:?}", asm_ast_instruction.src, asm_ast_instruction.src_2);

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::Label => {
                println!("Label {:?}", asm_ast_instruction.src);

                new_body.push(Box::new(asm_ast_instruction));
            }

            AsmAstInstructionType::SetCC => {
                println!("SetCC {:?} {:?}", asm_ast_instruction.src, asm_ast_instruction.dst);

                asm_ast_instruction.dst = self.replace_pseudo_operand(&mut asm_ast_instruction.dst);

                new_body.push(Box::new(asm_ast_instruction));
            }
        }
    }

    #[allow(unreachable_code)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    #[allow(unreachable_patterns)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    pub fn replace_pseudo_operand(&mut self, asm_ast_operand: &AsmAstOperand) -> AsmAstOperand {

        let mut stack_offset_value = 0i32;

        match &asm_ast_operand.operand_type {

            AsmAstOperandType::Stack(stack_offset) => {
                // println!("Stack, offset:{}", stack_offset);
                return asm_ast_operand.clone();
            }

            AsmAstOperandType::Pseudo(pseudo_name) => {
                // println!("Pseudo");

                if self.replace_pseudo {

                    if self.stack_offset_map.contains_key(pseudo_name) {

                        stack_offset_value = *self.stack_offset_map.get(pseudo_name).unwrap();

                    } else {

                        self.stack_offset = self.stack_offset - 4;
                        self.stack_offset_map.insert(pseudo_name.to_string(), self.stack_offset);
                        
                        stack_offset_value = self.stack_offset;
                    }
                }

                return AsmAstOperand { operand_type: AsmAstOperandType::Stack(stack_offset_value) };
            }

            AsmAstOperandType::Imm(immediate_value) => {
                // println!("Imm");
            }

            AsmAstOperandType::Reg(register_name) => {
                // println!("Reg, register_name:{:?}", register_name);
            }

            _ => {
                // panic!("Test");
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", asm_ast_operand.operand_type ).as_str());

                return asm_ast_operand.clone();
            }
        }

        asm_ast_operand.clone()
    }
}




// match asm_ast_instruction.binary_operator {

                //     AsmAstBinaryOperator::Add => {
                //         // write!(f, "{}", format!("Binary(ADD, {:?}, {:?})", self.src_2, self.dst).as_str()).expect("Write failed!");
                //         println!("add");
                //     }

                //     // AsmAstBinaryOperator::Subtract => {
                //     //     write!(f, "{}", format!("Binary(SUB, {:?}, {:?})", self.src_2, self.dst).as_str()).expect("Write failed!");
                //     // }

                //     // AsmAstBinaryOperator::Multiply => {
                //     //     write!(f, "{}", format!("Binary(MUL, {:?}, {:?})", self.src_2, self.dst).as_str()).expect("Write failed!");
                //     // }

                //     _ => {
                //         todo!();
                //     }
                // }