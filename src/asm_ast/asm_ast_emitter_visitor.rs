use crate::tacky::tacky::Program;
use crate::tacky::tacky::TopLevel;
use crate::tacky::tacky::TopLevelType;
use crate::tacky::tacky::ValueElement;
use crate::tacky::tacky::Instruction;
use crate::tacky::tacky::InstructionType;
use crate::tacky::tacky::UnaryOperator;
use crate::tacky::tacky::BinaryOperator;

use crate::AsmAstProgram;
use crate::asm_ast::asm_ast::AsmAstFunction;
use crate::asm_ast::asm_ast::AsmAstInstruction;
use crate::asm_ast::asm_ast::AsmAstInstructionType;
use crate::asm_ast::asm_ast::AsmAstReg;
use crate::asm_ast::asm_ast::AsmAstOperand;
use crate::asm_ast::asm_ast::AsmAstOperandType;
use crate::asm_ast::asm_ast::AsmAstUnaryOperator;
use crate::asm_ast::asm_ast::AsmAstBinaryOperator;

// Emits mnemonics for the linux AS assembler
//
// 1. c_ast/IdentifierResolutionVisitor - checks for duplicate or undeclared variable names
// 2. tacky/TackyVisitor - Generate TACKY (from AST)
// 3. asm_ast/AsmAstConversionVisitor - Converts the AST into a ASM AST for assembly with a precursory form of mnenomics
// 4. asm_ast/AsmAstFixupVisitor - replacing pseudo operands/variables with stack addresses
// 5. asm_ast/AsmAstMasmEmitterVisitor / asm_ast/AsmAstASEmitterVisitor / ...

pub struct AsmAstASEmitterVisitor {
}

impl AsmAstASEmitterVisitor {

    pub fn new() -> AsmAstASEmitterVisitor {
        AsmAstASEmitterVisitor {
        }
    }

    pub fn visit_asm_ast_program(&mut self, asm_ast_program: &mut AsmAstProgram) {
        // println!("[AsmAstASEmitterVisitor::visit_asm_ast_program()]");

        // page 43
        println!(".section .note.GNU-stack,\"\",@progbits");

        // self.visit_asm_ast_function(&asm_ast_program.function);
        for i in 0..asm_ast_program.functions.len() {
            self.visit_asm_ast_function(&mut asm_ast_program.functions[i]);
        }
    }

    pub fn visit_asm_ast_function(&mut self, asm_ast_function: &mut AsmAstFunction) {
        // println!("[AsmAstASEmitterVisitor::visit_asm_ast_function()] name={}", asm_ast_function.name);

        // page 43
        println!(".globl {}", asm_ast_function.name);
        println!("{}:", asm_ast_function.name);

        // page 43
        println!("pushq %rbp"); // see page 30, old base pointer is pushed to stack so that it can be restored by a ret instruction
        println!("movq %rsp, %rbp"); // see page 30, let the stack pointer register point to the new top of the stack

        for i in 0..asm_ast_function.body.len() {
            self.visit_asm_ast_instruction(asm_ast_function.body[i].as_ref(), asm_ast_function.stack_frame_size);
        }
    }

    #[allow(unreachable_code)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    #[allow(unreachable_patterns)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    pub fn visit_asm_ast_instruction(&mut self, asm_ast_instruction: &AsmAstInstruction, stack_frame_size: i32) {
        // println!("[AsmAstASEmitterVisitor::visit_asm_ast_instruction()] instruction={:?}", asm_ast_instruction);

        match asm_ast_instruction.instruction_type {

            AsmAstInstructionType::AllocateStack => {
                //self.visit_tacky_unary(&mut asm_ast_function, asm_ast_instruction);
                match asm_ast_instruction.src.operand_type {
                    AsmAstOperandType::Imm(imm_value) => {
                        println!("    subq ${:?}, %rsp", imm_value);
                    }
                    _ => {

                    }
                }
            }

            AsmAstInstructionType::Mov => {
                //self.visit_tacky_unary(&mut asm_ast_function, asm_ast_instruction);

                // println!("movl {:?} {:?}", asm_ast_instruction.src, asm_ast_instruction.dst);

                print!("movl ");
                self.emit_asm_ast_operand(&asm_ast_instruction.src);
                print!(", ");
                self.emit_asm_ast_operand(&asm_ast_instruction.dst);
                println!("");
            }

            AsmAstInstructionType::Unary => {
                //self.visit_tacky_unary(&mut asm_ast_function, asm_ast_instruction);

                print!("{:?} ", asm_ast_instruction.unary_operator);
                self.emit_asm_ast_operand(&asm_ast_instruction.dst);
                println!("");
            }

            AsmAstInstructionType::Ret => {
                //self.visit_tacky_return(&mut asm_ast_function, asm_ast_instruction);

                // page 44
                println!("movq %rbp, %rsp"); // see page 30, old base pointer is pop from stack to remove stack frame
                println!("popq %rbp"); // see page 30, let the stack pointer register point to the old top of the stack
                println!("ret");
            }

            _ => {
                panic!("{}", format!("Unhandled AsmAstInstructionType {:?}!\n", asm_ast_instruction.instruction_type).as_str());
            }
        }
    }

    #[allow(unreachable_code)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    #[allow(unreachable_patterns)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    pub fn emit_asm_ast_operand(&mut self, asm_ast_operand: &AsmAstOperand) {

        match &asm_ast_operand.operand_type {

            AsmAstOperandType::Imm(imm_val) => {
                print!("${}", imm_val);
            }

            AsmAstOperandType::Reg(asm_ast_reg_val) => {
                match asm_ast_reg_val {
                    AsmAstReg::AX => {
                        print!("%eax");
                    }
                    AsmAstReg::R10 => {
                        print!("%r10d");
                    }
                    _ => {
                        todo!();
                    }
                }
            }

            AsmAstOperandType::Pseudo(pseudo_val) => {
                panic!("Should not be here!");
            }

            AsmAstOperandType::Memory(register, stack_val) => {
                print!("{}(%{})", stack_val, register.to_string());
            }

            _ => {
                todo!();
            }
        }
    }
}






        // for i in 0..asm_ast_program.top_level.len() {

        //     let temp_top_level_item:&Box<TopLevel> = &asm_ast_program.top_level[i];
        //     match &temp_top_level_item.as_ref().top_level_type {

        //         TopLevelType::Function => {
        //             self.visit_asm_ast_function(&temp_top_level_item.as_ref());
        //         }

        //         TopLevelType::StaticVariable => {
        //             todo!("TopLevelType::StaticVariable");
        //         }

        //         TopLevelType::StaticConstant => {
        //             todo!("TopLevelType::StaticConstant");
        //         }
        //     }
        // }