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
    pub stack_size: usize,
    pub string_buffer: String,
    pub print_to_console: bool,
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
            stack_size: 0,
            string_buffer: String::from(""),
            print_to_console: false,
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
                if self.print_to_console {
                    print!("{}", imm_val);
                }
                self.string_buffer.push_str(format!("{}", imm_val).as_str());
            }

            AsmAstOperandType::Reg(asm_ast_reg_val) => {
                // registers defined in asm_ast.rs
                if self.print_to_console {
                    print!("{}", asm_ast_reg_val);
                }
                self.string_buffer.push_str(&asm_ast_reg_val.to_string());
            }

            AsmAstOperandType::Pseudo(pseudo_val) => {
                if self.print_to_console {
                    print!("{}", pseudo_val);
                }
                self.string_buffer.push_str(format!("{}", pseudo_val).as_str());
            }

            AsmAstOperandType::Memory(register, stack_val) => {
                //print!("{}(ebp)", stack_val);

                match data_type_size {
                    DataTypeSize::Byte => {
                        if *stack_val == 0 {
                            if self.print_to_console {
                                print!("byte ptr [rbp+{}]", stack_val);
                            }
                            self.string_buffer.push_str(format!("byte ptr [{}+{}]", register.to_string(), stack_val).as_str());
                        } else {
                            if self.print_to_console {
                                print!("byte ptr [rbp{}]", stack_val);
                            }
                            self.string_buffer.push_str(format!("byte ptr [{}{}]", register.to_string(), stack_val).as_str());
                        }
                    }
                    DataTypeSize::Word => {
                        if *stack_val == 0 {
                            if self.print_to_console {
                                print!("word ptr [rbp+{}]", stack_val);
                            }
                            self.string_buffer.push_str(format!("word ptr [{}+{}]", register.to_string(), stack_val).as_str());
                        } else {
                            if self.print_to_console {
                                print!("word ptr [rbp{}]", stack_val);
                            }
                            self.string_buffer.push_str(format!("word ptr [{}{}]", register.to_string(), stack_val).as_str());
                        }
                    }
                    DataTypeSize::DWord => {
                        if *stack_val == 0 {
                            if self.print_to_console {
                                print!("dword ptr [rbp+{}]", stack_val);
                            }
                            self.string_buffer.push_str(format!("dword ptr [{}+{}]", register.to_string(), stack_val).as_str());
                        } else {
                            if self.print_to_console {
                                print!("dword ptr [rbp{}]", stack_val);
                            }
                            self.string_buffer.push_str(format!("dword ptr [{}{}]", register.to_string(), stack_val).as_str());
                        }
                    }
                    DataTypeSize::QWord => {
                        if *stack_val == 0 {
                            if self.print_to_console {
                                print!("qword ptr [rbp+{}]", stack_val);
                            }
                            self.string_buffer.push_str(format!("qword ptr [{}+{}]", register.to_string(), stack_val).as_str());
                        } else {
                            if self.print_to_console {
                                print!("qword ptr [rbp{}]", stack_val);
                            }
                            self.string_buffer.push_str(format!("qword ptr [{}{}]", register.to_string(), stack_val).as_str());
                        }
                    }
                }

            }

            AsmAstOperandType::ComparisonType(comparison_type_val) => {
                if self.print_to_console {
                    print!("{}", comparison_type_val.to_lowercase());
                }
                self.string_buffer.push_str(format!("{}", comparison_type_val.to_lowercase()).as_str());
            }

            AsmAstOperandType::Label(label_val) => {
                if self.print_to_console {
                    print!("{}", label_val);
                }
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

    pub fn visit_asm_ast_program(&mut self, asm_ast_program: &mut AsmAstProgram) {
        // println!("[AsmAstMasmEmitterVisitor::visit_asm_ast_program()]");

        // page 43
        //println!(".section .note.GNU-stack,\"\",@progbits");

        // 32_BIT 32-BIT
        // 32-bit assembly
        let use_32_bit = false;
        if use_32_bit {

            // 32_BIT
            println!("    .386");
            self.string_buffer.push_str("    .386\n");
            println!("    .model flat, stdcall");
            self.string_buffer.push_str("    .model flat, stdcall\n");
            println!("    .stack 4096");
            self.string_buffer.push_str("    .stack 4096\n");
        }

        //
        // extern functions to call
        //

        // insert empty line
        println!("");
        self.string_buffer.push_str("\n");

        // 32_BIT
        // println!("ExitProcess PROTO, dwExitCode:DWORD");
        // self.string_buffer.push_str("\n");
        // self.string_buffer.push_str("ExitProcess PROTO, dwExitCode:DWORD\n");

        //
        // code segment 64_BIT 64-BIT
        //

        // println!("    .code");
        self.string_buffer.push_str("\n");
        self.string_buffer.push_str("    .code\n");

        // iterate over all functions
        for i in 0..asm_ast_program.functions.len() {

            // insert empty line to separate functions
            // println!("");
            self.string_buffer.push_str("\n");

            self.visit_asm_ast_function(&mut asm_ast_program.functions[i]);
        }

        // //
        // // terminate the process after main()
        // //

        // println!("    INVOKE ExitProcess, eax");
        // self.string_buffer.push_str("    INVOKE ExitProcess, eax\n");

        // println!("{} ENDP", "main");
        // self.string_buffer.push_str(format!("{} ENDP\n", "main").as_str());

        //
        // declare END (of application) along with declare start symbol (??? is that true?)
        //

        // 32_BIT
        // println!("END main ; specify the program's entry point");
        // self.string_buffer.push_str("\n");
        // self.string_buffer.push_str("END main ; specify the program's entry point\n");

        // 64_BIT 64-BIT (END of application without specifying any main entry point)
        // println!("END");
        self.string_buffer.push_str("\n");
        self.string_buffer.push_str("END\n");
    }

    pub fn visit_asm_ast_function(&mut self, asm_ast_function: &mut AsmAstFunction) {
        // println!("[AsmAstMasmEmitterVisitor::visit_asm_ast_function()] name={}", asm_ast_function.name);

        self.stack_size = self.stack_size + 1;

        // // page 43
        // println!(".globl {}", asm_ast_function.name);
        // println!("{}:", asm_ast_function.name);

        // // page 43
        // println!("pushq %rbp"); // see page 30, old base pointer is pushed to stack so that it can be restored by a ret instruction
        // println!("movq %rsp, %rbp"); // see page 30, let the stack pointer register point to the new top of the stack

        // Irvine, page 323
        // println!("{} PROC", asm_ast_function.name);
        self.string_buffer.push_str(format!("{} PROC\n", asm_ast_function.name).as_str());

        //
        // Prelude
        //

        // println!("\n    ; prelude - create stack frame and shadow area");
        self.string_buffer.push_str("\n    ; prelude - create stack frame and shadow area\n");

        // 32_BIT
        // println!("    push ebp ; save base of current stack frame to restore it later"); // save base of current stack frame to restore it later
        // self.string_buffer.push_str("    push ebp ; save base of current stack frame to restore it later\n"); // save base of current stack frame to restore it later

        // 64_BIT
        // println!("    push rbp ; save base of current stack frame to restore it later"); // save base of current stack frame to restore it later
        self.string_buffer.push_str("    push rbp ; save base of current stack frame to restore it later\n"); // save base of current stack frame to restore it later
        // println!("    mov rbp, rsp ; set new base of new stack frame (to current stack pointer)"); // set new base of new stack frame (to current stack pointer)
        self.string_buffer.push_str("    mov rbp, rsp ; set new base of new stack frame (to current stack pointer)\n"); // set new base of new stack frame (to current stack pointer)

        // 64_BIT shadow register area so that Win32, Win64 functions can be called
        // println!("    sub rsp, 8 * (4 + 2) ; allocate shadow register area + 2 QWORDs for stack alignment");
        self.string_buffer.push_str("    sub rsp, 8 * (4 + 2) ; allocate shadow register area + 2 QWORDs for stack alignment");
        // println!("    sub rsp, 8 * 7 ; allocate shadow register area + 2 QWORDs for stack alignment");
        // self.string_buffer.push_str("    sub rsp, 8 * 7 ; allocate shadow register area + 2 QWORDs for stack alignment");

        for i in 0..asm_ast_function.body.len() {
            self.visit_asm_ast_instruction(asm_ast_function.body[i].as_ref(), asm_ast_function.stack_frame_size);
        }

        // look at the last instruction, if it is not RET, add a RET
        let last_instruction = asm_ast_function.body[asm_ast_function.body.len()-1].as_ref();
        if last_instruction.instruction_type != AsmAstInstructionType::Ret {

            // // Irvine, page 323
            // println!("    pop ebp");
            // println!("    add esp, {:?}", asm_ast_function.stack_frame_size);

            // // ; epilog - restore stack pointer
            // println!("\n    ; epilog - restore stack pointer");
            // self.string_buffer.push_str("\n    ; epilog - restore stack pointer\n");
            // println!("    mov rsp, rbp");
            // self.string_buffer.push_str("    mov rsp, rbp\n");
            // println!("    pop rbp");
            // self.string_buffer.push_str("    pop rbp\n");

            //
            // Add automatic ret in case the user forgot to add a return statement
            //

            // println!("    ret");
            // self.string_buffer.push_str("    ret\n");
        }

        // println!("{} ENDP", asm_ast_function.name);
        self.string_buffer.push_str(format!("{} ENDP\n", asm_ast_function.name).as_str());

        self.stack_size = self.stack_size - 1;
    }

    #[allow(unreachable_code)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    #[allow(unreachable_patterns)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
    pub fn visit_asm_ast_instruction(&mut self, asm_ast_instruction: &AsmAstInstruction, stack_frame_size: i32) {
        // println!("[AsmAstMasmEmitterVisitor::visit_asm_ast_instruction()] instruction={:?}", asm_ast_instruction);

        match asm_ast_instruction.instruction_type {

            AsmAstInstructionType::Mov => {
                // println!("movl {:?} {:?}", asm_ast_instruction.src, asm_ast_instruction.dst);

                // print!("    mov ");
                self.string_buffer.push_str("    mov ");
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::QWord);
                // print!(", ");
                self.string_buffer.push_str(", ");
                self.emit_asm_ast_operand(&asm_ast_instruction.src, DataTypeSize::QWord);

                // comment
                // print!("{}", asm_ast_instruction.comment);
                self.string_buffer.push_str(format!("{}", asm_ast_instruction.comment).as_str());

                // newline
                // println!("");
                self.string_buffer.push_str("\n");
            }

            AsmAstInstructionType::Push => {
                // print!("    push ");
                self.string_buffer.push_str("    push ");
                self.emit_asm_ast_operand(&asm_ast_instruction.src, DataTypeSize::DWord);

                // comment
                // print!("{}", asm_ast_instruction.comment);
                self.string_buffer.push_str(format!("{}", asm_ast_instruction.comment).as_str());

                // newline
                // println!("");
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

                // print!("    {} ", mnemonic);
                self.string_buffer.push_str(format!("    {} ", mnemonic).as_str());

                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::DWord);

                // println!("");
                self.string_buffer.push_str("\n");
            }

            // https://www.felixcloutier.com/x86/mul
            //
            // This binary was generated inside: asm_ast_conversion_visitor
            AsmAstInstructionType::Binary => {
                // print!("    {} ", asm_ast_instruction.binary_operator.to_string());
                self.string_buffer.push_str(format!("    {} ", asm_ast_instruction.binary_operator.to_string()).as_str());
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::DWord);
                // print!(", ");
                self.string_buffer.push_str(", ");
                self.emit_asm_ast_operand(&asm_ast_instruction.src_2, DataTypeSize::DWord);
                // println!("");
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

                // // Irvine, page 323
                // println!("    add esp, {:?} ; restore stack pointer to old stack frame top", stack_frame_size);
                // self.string_buffer.push_str(format!("    add esp, {:?} ; restore stack pointer to old stack frame top\n", stack_frame_size).as_str());

                // // 32_bit
                // println!("    pop ebp ; restore old base pointer of old stack frame");
                // self.string_buffer.push_str("    pop ebp ; restore old base pointer of old stack frame\n");

                // // 64_bit
                // println!("    pop rbp ; restore old base pointer of old stack frame");
                // self.string_buffer.push_str("    pop rbp ; restore old base pointer of old stack frame\n");

                // ; epilog - restore stack pointer
                // println!("\n    ; epilog - restore stack pointer\n");
                self.string_buffer.push_str("\n    ; epilog - restore stack pointer\n");
                // println!("    mov rsp, rbp\n");
                self.string_buffer.push_str("    mov rsp, rbp\n");
                // println!("    pop rbp\n");
                self.string_buffer.push_str("    pop rbp\n");

                // from the main function (where self.stack_size is 0), do not execute a ret instruction
                // so that the generated line "INVOKE ExitProcess, eax" will execute for save process termination
                if self.stack_size != 0 {
                    // println!("\n    ; pops the return address from the top of the stack into the instruction pointer (EIP/RIP)\n");
                    // println!("    ret");

                    self.string_buffer.push_str("\n    ; pops the return address from the top of the stack into the instruction pointer (EIP/RIP)\n");
                    self.string_buffer.push_str("    ret\n");
                }
            }

            AsmAstInstructionType::Cdq => {
                // println!("    cdq");
                self.string_buffer.push_str("    cdq\n");
            }

            AsmAstInstructionType::Idiv | AsmAstInstructionType::Mod => {
                // print!("    idiv ");
                self.string_buffer.push_str("    idiv ");
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::DWord);
                self.string_buffer.push_str("\n");
                // println!("");
            }

            AsmAstInstructionType::Mul => {
                // print!("    mul ");
                self.string_buffer.push_str("    mul ");
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::DWord);
                self.string_buffer.push_str("\n");
                // println!("");
            }

            AsmAstInstructionType::Cmp => {
                // print!("    cmp ");
                self.string_buffer.push_str("    cmp ");
                self.emit_asm_ast_operand(&asm_ast_instruction.src_2, DataTypeSize::DWord);

                // print!(", ");
                self.string_buffer.push_str(", ");
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::DWord);

                // println!("");
                self.string_buffer.push_str("\n");
            }

            AsmAstInstructionType::Jmp => {
                // print!("    jmp");
                self.string_buffer.push_str("    jmp");
                // print!(" ");
                self.string_buffer.push_str(" ");
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::DWord);
                // println!("");
                self.string_buffer.push_str("\n");
            }

            AsmAstInstructionType::JmpCC => {
                // print!("    j");
                self.string_buffer.push_str("    j");
                self.emit_asm_ast_operand(&asm_ast_instruction.src, DataTypeSize::DWord);
                // print!(" ");
                self.string_buffer.push_str(" ");
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::DWord);
                // println!("");
                self.string_buffer.push_str("\n");
            }

            AsmAstInstructionType::Label => {
                // print!("{:?}", asm_ast_instruction.src);

                self.emit_asm_ast_operand(&asm_ast_instruction.src, DataTypeSize::DWord);
                // println!(":");
                self.string_buffer.push_str(":");
                self.string_buffer.push_str("\n");
            }

            AsmAstInstructionType::SetCC => {
                // print!("    set");
                self.string_buffer.push_str("    set");
                self.emit_asm_ast_operand(&asm_ast_instruction.src, DataTypeSize::Byte);
                // print!(" ");
                self.string_buffer.push_str(" ");
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::Byte);
                // println!("");
                self.string_buffer.push_str("\n");
            }

            AsmAstInstructionType::AllocateStack => {
                match asm_ast_instruction.src.operand_type {
                    AsmAstOperandType::Imm(imm_value) => {

                        // comment
                        // println!("{}", asm_ast_instruction.comment);
                        self.string_buffer.push_str(format!("{}", asm_ast_instruction.comment).as_str());

                        // instruction
                        // println!("    sub rsp, {:?}", imm_value); // save space on stack for all local variables
                        self.string_buffer.push_str(format!("    sub rsp, {:?}\n", imm_value).as_str()); // save space on stack for all local variables

                        // println!("\n    ; prelude - create stack frame and shadow area");
                        // self.string_buffer.push_str("\n    ; prelude - create stack frame and shadow area\n");
                        // // 64_BIT
                        // println!("    push rbp ; save base of current stack frame to restore it later"); // save base of current stack frame to restore it later
                        // self.string_buffer.push_str("    push rbp ; save base of current stack frame to restore it later\n"); // save base of current stack frame to restore it later
                        // println!("    mov rbp, rsp ; set new base of new stack frame (to current stack pointer)"); // set new base of new stack frame (to current stack pointer)
                        // self.string_buffer.push_str("    mov rbp, rsp ; set new base of new stack frame (to current stack pointer)\n"); // set new base of new stack frame (to current stack pointer)

                        // // 64_BIT shadow register area so that Win32, Win64 functions can be called
                        // println!("    sub rsp, 8 * (4 + 2) ; allocate shadow register area + 2 QWORDs for stack alignment");
                        // self.string_buffer.push_str("    sub rsp, 8 * (4 + 2) ; allocate shadow register area + 2 QWORDs for stack alignment\n");
                    }
                    _ => {

                    }
                }
            }

            // Nora Sandler, page 202
            AsmAstInstructionType::DeallocateStack => {
                match asm_ast_instruction.src.operand_type {
                    AsmAstOperandType::Imm(imm_value) => {
                        // remove stack frame
                        // println!("    add rsp, {:?}   ; remove stack frame", imm_value);
                        self.string_buffer.push_str(format!("    add rsp, {:?}  ; remove stack frame\n", imm_value).as_str());

                        // // ; epilog - restore stack pointer
                        // println!("\n    ; epilog - restore stack pointer\n");
                        // self.string_buffer.push_str("\n    ; epilog - restore stack pointer\n");
                        // println!("    mov rsp, rbp\n");
                        // self.string_buffer.push_str("    mov rsp, rbp\n");
                        // println!("    pop rbp\n");
                        // self.string_buffer.push_str("    pop rbp\n");
                    }
                    _ => {
                        todo!();
                    }
                }
            }

            AsmAstInstructionType::FunctionCall => {
                // println!("    call {}", asm_ast_instruction.identifier);
                self.string_buffer.push_str(format!("    call {}\n", asm_ast_instruction.identifier).as_str());
            }

            AsmAstInstructionType::Lea => {
                // println!("{:?}", asm_ast_instruction);

                // print!("    lea ");
                self.string_buffer.push_str("    lea ");
                self.emit_asm_ast_operand(&asm_ast_instruction.dst, DataTypeSize::QWord);

                // print!(", ");
                self.string_buffer.push_str(", ");
                self.emit_asm_ast_operand(&asm_ast_instruction.src, DataTypeSize::QWord);

                // println!("");
                self.string_buffer.push_str("\n");
            }

            _ => {
                panic!("{}", format!("Unhandled AsmAstInstructionType {:?}!\n", asm_ast_instruction.instruction_type).as_str());
            }
        }
    }
}