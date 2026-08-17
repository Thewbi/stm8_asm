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
use crate::asm_ast::asm_ast_masm_emitter_visitor::AsmAstOperandType::ComparisonType;

//
// Output MASM assembly
//
// 1. c_ast/IdentifierResolutionVisitor - checks for duplicate or undeclared variable names
// 2. tacky/TackyVisitor - Generate TACKY (from AST)
// 3. asm_ast/AsmAstConversionVisitor - Converts the AST into a ASM AST for assembly with a precursory form of mnenomics
// 4. asm_ast/AsmAstFixupVisitor - replacing pseudo operands/variables with stack addresses
// 5. asm_ast/AsmAstMasmEmitterVisitor / asm_ast/AsmAstASEmitterVisitor / ...
//
// Irvine page 70 registers
//

pub struct AsmAstMasmEmitterVisitor {
    // pub asm_ast_program: AsmAstProgram,
    pub stack_size: usize,
    pub string_buffer: String,
}

pub enum DataTypeSize {
    Byte, // 1 Byte
    Word, // 2 Byte
    DWord, // 4 Byte
    QWord, // 8 Byte
}

impl AsmAstMasmEmitterVisitor {

    pub fn new() -> AsmAstMasmEmitterVisitor {
        AsmAstMasmEmitterVisitor {
            // asm_ast_program: AsmAstProgram::new(),
            stack_size: 0,
            string_buffer: String::from(""),
        }
    }

    //
    // util
    //

    #[allow(unreachable_code)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    #[allow(unreachable_patterns)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    pub fn emit_asm_ast_operand(&mut self, asm_ast_operand: &AsmAstOperand, data_type_size: DataTypeSize) {

        match &asm_ast_operand.operand_type {

            AsmAstOperandType::Imm(imm_val) => {
                print!("{}", imm_val);
                self.string_buffer.push_str(format!("{}", imm_val).as_str());
            }

            AsmAstOperandType::Reg(asm_ast_reg_val) => {
                match asm_ast_reg_val {

                    AsmAstReg::AX => {
                        print!("eax");
                        self.string_buffer.push_str("eax");
                    }

                    AsmAstReg::BX => {
                        print!("ebx");
                        self.string_buffer.push_str("ebx");
                    }

                    AsmAstReg::DX => {
                        print!("edx");
                        self.string_buffer.push_str("edx");
                    }

                    AsmAstReg::R10 => {
                        //print!("r10d");
                        print!("edx");
                        self.string_buffer.push_str("edx");
                    }

                    _ => {
                        todo!();
                    }
                }
            }

            AsmAstOperandType::Pseudo(pseudo_val) => {
                //panic!("Should not be here!");
                print!("{}", pseudo_val);
                self.string_buffer.push_str(format!("{}", pseudo_val).as_str());
            }

            AsmAstOperandType::Stack(stack_val) => {
                //print!("{}(ebp)", stack_val);

                match data_type_size {
                    DataTypeSize::Byte => {
                        print!("byte ptr [ebp{}]", stack_val);
                        self.string_buffer.push_str(format!("byte ptr [ebp{}]", stack_val).as_str());
                    }
                    DataTypeSize::Word => {
                        print!("word ptr [ebp{}]", stack_val);
                        self.string_buffer.push_str(format!("word ptr [ebp{}]", stack_val).as_str());
                    }
                    DataTypeSize::DWord => {
                        print!("dword ptr [ebp{}]", stack_val);
                        self.string_buffer.push_str(format!("dword ptr [ebp{}]", stack_val).as_str());
                    }
                    DataTypeSize::QWord => {
                        print!("qword ptr [ebp{}]", stack_val);
                        self.string_buffer.push_str(format!("qword ptr [ebp{}]", stack_val).as_str());
                    }
                }
                
            }

            AsmAstOperandType::ComparisonType(comparison_type_val) => {
                print!("{}", comparison_type_val.to_lowercase());
                self.string_buffer.push_str(format!("{}", comparison_type_val.to_lowercase()).as_str());
            }

            AsmAstOperandType::Label(label_val) => {
                print!("{}", label_val);
                self.string_buffer.push_str(format!("{}", label_val).as_str());
            }

            _ => {
                panic!("{}", format!("Unhandled AsmAstOperandType {:?}!\n", asm_ast_operand.operand_type).as_str());
            }
        }
    }

    //
    // non-util
    //

    pub fn visit_asm_ast_program(&mut self, asm_ast_program: &AsmAstProgram) {
        // println!("[AsmAstMasmEmitterVisitor::visit_asm_ast_program()]");

        // page 43
        //println!(".section .note.GNU-stack,\"\",@progbits");

        println!("    .386");
        self.string_buffer.push_str("    .386\n");
        println!("    .model flat, stdcall");
        self.string_buffer.push_str("    .model flat, stdcall\n");
        println!("    .stack 4096");
        self.string_buffer.push_str("    .stack 4096\n");

        //
        // extern functions to call
        //

        println!("ExitProcess PROTO, dwExitCode:DWORD");
        self.string_buffer.push_str("\n");
        self.string_buffer.push_str("ExitProcess PROTO, dwExitCode:DWORD\n");

        //
        // code segment
        //

        println!("    .code");
        self.string_buffer.push_str("\n");
        self.string_buffer.push_str("    .code\n");

        self.visit_asm_ast_function(&asm_ast_program.function);

        //
        // terminate the process after main()
        //

        println!("    INVOKE ExitProcess, eax");
        self.string_buffer.push_str("    INVOKE ExitProcess, eax\n");
        println!("{} ENDP", "main");
        self.string_buffer.push_str(format!("{} ENDP\n", "main").as_str());

        //
        // declare start symbol
        //

        println!("END main ; specify the program's entry point");
        self.string_buffer.push_str("\n");
        self.string_buffer.push_str("END main ; specify the program's entry point\n");
    }

    pub fn visit_asm_ast_function(&mut self, asm_ast_function: &AsmAstFunction) {
        // println!("[AsmAstMasmEmitterVisitor::visit_asm_ast_function()] name={}", asm_ast_function.name);

        self.stack_size = self.stack_size + 1;

        // // page 43
        // println!(".globl {}", asm_ast_function.name);
        // println!("{}:", asm_ast_function.name);

        // // page 43
        // println!("pushq %rbp"); // see page 30, old base pointer is pushed to stack so that it can be restored by a ret instruction
        // println!("movq %rsp, %rbp"); // see page 30, let the stack pointer register point to the new top of the stack
        
        // Irvine, page 323
        println!("{} PROC", asm_ast_function.name);
        self.string_buffer.push_str(format!("{} PROC\n", asm_ast_function.name).as_str());
        println!("    push ebp ; save base of current stack frame to restore it later"); // save base of current stack frame to restore it later
        self.string_buffer.push_str("    push ebp ; save base of current stack frame to restore it later\n"); // save base of current stack frame to restore it later
        println!("    mov ebp, esp ; set new base of new stack frame (to current stack pointer)"); // set new base of new stack frame (to current stack pointer)
        self.string_buffer.push_str("    mov ebp, esp ; set new base of new stack frame (to current stack pointer)\n"); // set new base of new stack frame (to current stack pointer)
        
        for i in 0..asm_ast_function.body.len() {
            self.visit_asm_ast_instruction(asm_ast_function.body[i].as_ref(), asm_ast_function.stack_frame_size);
        }

        /*
        //
        // Add automatic ret in case the user forgot to add a return statement
        //

        // Irvine, page 323
        println!("    pop ebp");
        println!("    add esp, {:?}", asm_ast_function.stack_frame_size);

        println!("    ret");
        */

        self.stack_size = self.stack_size - 1;
    }

    #[allow(unreachable_code)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    #[allow(unreachable_patterns)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    pub fn visit_asm_ast_instruction(&mut self, asm_ast_instruction: &AsmAstInstruction, stack_frame_size: i32) {
        // println!("[AsmAstMasmEmitterVisitor::visit_asm_ast_instruction()] instruction={:?}", asm_ast_instruction);

        match asm_ast_instruction.instruction_type {

            AsmAstInstructionType::AllocateStack => {
                //self.visit_tacky_unary(&mut asm_ast_function, asm_ast_instruction);
                match asm_ast_instruction.src.operand_type {
                    AsmAstOperandType::Imm(imm_value) => {
                        println!("    sub esp, {:?} ; save space on stack for all local variables", imm_value); // save space on stack for all local variables
                        // self.string_buffer.push_str("    sub esp, {:?} ; save space on stack for all local variables", imm_value); // save space on stack for all local variables
                        self.string_buffer.push_str(format!("    sub esp, {:?} ; save space on stack for all local variables\n", imm_value).as_str()); // save space on stack for all local variables
                    }
                    _ => {

                    }
                }
            }

            AsmAstInstructionType::Mov => {
                //self.visit_tacky_unary(&mut asm_ast_function, asm_ast_instruction);

                // println!("movl {:?} {:?}", asm_ast_instruction.src, asm_ast_instruction.dst);

                print!("    mov ");
                self.string_buffer.push_str("    mov ");
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::DWord);
                print!(", ");
                self.string_buffer.push_str(", ");
                self.emit_asm_ast_operand(&asm_ast_instruction.src, DataTypeSize::DWord);

                // print!(" ; 100");
                // self.string_buffer.push_str(" ; 100");

                println!("");
                self.string_buffer.push_str("\n");
            }

            AsmAstInstructionType::Unary => {

                let mut mnemonic = String::new();

                match &asm_ast_instruction.unary_operator {

                    AsmAstUnaryOperator::Neg => {
                        panic!("");
                    }
                    AsmAstUnaryOperator::Not => {
                        panic!("");
                    }
                    AsmAstUnaryOperator::Increment => {
                        mnemonic = "inc".to_string();
                    }
                    _ => {
                        println!("{}", format!("Unhandled AsmAstInstructionType {:?}!\n", asm_ast_instruction.unary_operator).as_str());
                    }

                }

                print!("    {} ", mnemonic);
                self.string_buffer.push_str(format!("    {} ", mnemonic).as_str());
                
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::DWord);

                println!("");
                self.string_buffer.push_str("\n");
            }

            // https://www.felixcloutier.com/x86/mul
            AsmAstInstructionType::Binary => {
                print!("    {} ", asm_ast_instruction.binary_operator.to_string());
                self.string_buffer.push_str(format!("    {} ", asm_ast_instruction.binary_operator.to_string()).as_str());
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::DWord);
                print!(", ");
                self.string_buffer.push_str(", ");
                self.emit_asm_ast_operand(&asm_ast_instruction.src_2, DataTypeSize::DWord);
                println!("");
                self.string_buffer.push_str("\n");
            }

            AsmAstInstructionType::Ret => {
                //self.visit_tacky_return(&mut asm_ast_function, asm_ast_instruction);

                // // page 44
                // println!("movq %rbp, %rsp"); // see page 30, old base pointer is pop from stack to remove stack frame
                // println!("popq %rbp"); // see page 30, let the stack pointer register point to the old top of the stack
                // println!("ret");

                // When translating this with godbolt, there is a add, rsp at the end of the function call

                // int sub(int a, int b);
                // int add(int a, int b);
                //
                // int main() {
                //     int a = sub(10, 5);
                //     return 0;
                // }
                //
                // int sub(int a, int b) {
                //     return 0;
                // }

                //println!("    add, rsp, {:?}", stack_frame_size);

                // Irvine, page 323
                println!("    add esp, {:?} ; restore stack pointer to old stack frame top", stack_frame_size);
                self.string_buffer.push_str(format!("    add esp, {:?} ; restore stack pointer to old stack frame top\n", stack_frame_size).as_str());
                println!("    pop ebp ; restore old base pointer of old stack frame");
                self.string_buffer.push_str("    pop ebp ; restore old base pointer of old stack frame\n");

                // from the main function (where self.stack_size is 0), do not execute a ret instruction
                // so that the generated line "INVOKE ExitProcess, eax" will execute for save process termination 
                if self.stack_size != 0 {
                    println!("    ret ; pops the return address from the top of the stack into the instruction pointer (EIP/RIP)");
                    self.string_buffer.push_str("    ret ; pops the return address from the top of the stack into the instruction pointer (EIP/RIP)\n");
                }
            }

            AsmAstInstructionType::Cdq => {
                println!("    cdq");
                self.string_buffer.push_str("    cdq\n");
            }

            AsmAstInstructionType::Idiv | AsmAstInstructionType::Mod => {
                // println!("    idiv {:?}\n", asm_ast_instruction.dst);
                print!("    idiv ");
                self.string_buffer.push_str("    idiv ");
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::DWord);
                self.string_buffer.push_str("\n");
                println!("");
            }

            AsmAstInstructionType::Mul => {
                print!("    mul ");
                self.string_buffer.push_str("    mul ");
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::DWord);
                self.string_buffer.push_str("\n");
                println!("");
            }

            AsmAstInstructionType::Cmp => {
                print!("    cmp ");
                self.string_buffer.push_str("    cmp ");

                self.emit_asm_ast_operand(&asm_ast_instruction.src_2, DataTypeSize::DWord);

                print!(", ");
                self.string_buffer.push_str(", ");

                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::DWord);

                println!("");
                self.string_buffer.push_str("\n");
            }

            AsmAstInstructionType::Jmp => {
                print!("    jmp");
                self.string_buffer.push_str("    jmp");
                // self.emit_asm_ast_operand(&asm_ast_instruction.src, DataTypeSize::DWord);
                print!(" ");
                self.string_buffer.push_str(" ");
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::DWord);
                println!("");
                self.string_buffer.push_str("\n");
            }

            AsmAstInstructionType::JmpCC => {
                print!("    j");
                self.string_buffer.push_str("    j");
                self.emit_asm_ast_operand(&asm_ast_instruction.src, DataTypeSize::DWord);
                print!(" ");
                self.string_buffer.push_str(" ");
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::DWord);
                println!("");
                self.string_buffer.push_str("\n");
            }

            AsmAstInstructionType::Label => {
                // print!("{:?}", asm_ast_instruction.src);
                self.emit_asm_ast_operand(&asm_ast_instruction.src, DataTypeSize::DWord);
                println!(":");
                self.string_buffer.push_str(":");
                self.string_buffer.push_str("\n");
            }

            AsmAstInstructionType::SetCC => {
                print!("    set");
                self.string_buffer.push_str("    set");
                self.emit_asm_ast_operand(&asm_ast_instruction.src, DataTypeSize::Byte);
                print!(" ");
                self.string_buffer.push_str(" ");
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::Byte);
                println!("");
                self.string_buffer.push_str("\n");
            }

            _ => {
                panic!("{}", format!("Unhandled AsmAstInstructionType {:?}!\n", asm_ast_instruction.instruction_type).as_str());
            }
        }
    }
    
}