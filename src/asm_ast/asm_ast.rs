use std::fmt;
use std::fmt::Display;

use crate::common::data_type;
use crate::tacky::tacky::ValueElement;
use crate::tacky::tacky::UnaryOperator;
use crate::tacky::tacky::BinaryOperator;

use crate::common::data_type::DataType;

pub struct AsmAstProgram {
    pub name: String,
    pub functions: Vec::<AsmAstFunction>,
}

impl AsmAstProgram {
    pub fn new() -> AsmAstProgram {
        AsmAstProgram {
            name: String::from(""),
            functions: Vec::<AsmAstFunction>::new(),
        }
    }
}

#[derive(Debug)]
pub struct AsmAstFunction {
    pub name: String,
    pub body: Vec::<Box<AsmAstInstruction>>,
    pub stack_frame_size: i32,
    pub return_type: Option<DataType>,
}

impl AsmAstFunction {
    pub fn new() -> AsmAstFunction {
        AsmAstFunction {
            name: String::from(""),
            body: Vec::<Box<AsmAstInstruction>>::new(),
            stack_frame_size: 0i32,
            return_type: Option::None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstAstAssemblyType {
    Byte,       // byte,   8 bit
    Word,       // short, 16 bit
    Doubleword,   // dword, 32 bit
    Quadword,   // qword, 64 bit
    Unknown
}

impl AstAstAssemblyType {
    pub fn from_data_type(data_type: &DataType)
        -> AstAstAssemblyType
    {
        match data_type {
            DataType::DataTypeUnsignedInt => {
                AstAstAssemblyType::Doubleword // dword, 32 bit
            }
            DataType::DataTypeInt => {
                AstAstAssemblyType::Doubleword // dword, 32 bit
            }
            DataType::DataTypeLong => {
                AstAstAssemblyType::Quadword // qword, 64 bit
            }
            _ => {
                unimplemented!("No implementation for {}", data_type);
            }
        }
    }
}

// Type for a intermediate ASM instruction.
// Intermediate means that this is not final MASM, TASM, FASM, GAS, ...
// but it is an abstract, intermediate assembly language
#[derive(Clone, Debug)]
pub struct AsmAstInstruction {
    pub(crate) id: i32,

    pub instruction_type: AsmAstInstructionType,
    pub unary_operator: AsmAstUnaryOperator,
    pub binary_operator: AsmAstBinaryOperator,
    pub src: AsmAstOperand,
    pub src_2: AsmAstOperand,
    pub dst: AsmAstOperand,
    pub identifier: String,
    pub comment: String,
    pub assembly_type: AstAstAssemblyType,
    pub offset: AsmAstOperand,
}

impl AsmAstInstruction {
    pub fn new() -> AsmAstInstruction {
        AsmAstInstruction {
            id: 0,

            instruction_type: AsmAstInstructionType::Ret,
            unary_operator: AsmAstUnaryOperator::Not,
            binary_operator: AsmAstBinaryOperator::Add,
            src: AsmAstOperand::new(),
            src_2: AsmAstOperand::new(),
            dst: AsmAstOperand::new(),
            identifier: String::new(),
            comment: String::new(),
            assembly_type: AstAstAssemblyType::Unknown,
            offset: AsmAstOperand::new(),
        }
    }
}

#[allow(unreachable_code)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
#[allow(unreachable_patterns)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
impl fmt::Display for AsmAstInstruction {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match &self.instruction_type {

            AsmAstInstructionType::Ret => {
                write!(f, "{}", format!("Ret {:?}", self).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::Mov => {
                write!(f, "{}", format!("Mov(assembly_type:{:?}, src:{:?}, dst:{:?})", self.assembly_type, self.src, self.dst).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::Movsx => {
                write!(f, "{}", format!("Movsx(src:{:?}, dst:{:?})", self.src, self.dst).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::Unary => {
                write!(f, "{}", format!("Unary(unary_operator:{:?}, assembly_type:{:?}, dst:{:?})", self.unary_operator, self.assembly_type, self.dst).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::Binary => {
                match self.binary_operator {
                    AsmAstBinaryOperator::Add => {
                        write!(f, "{}", format!("Binary(ADD, src_2:{:?}, dst:{:?})", self.src_2, self.dst).as_str()).expect("Write failed!");
                    }
                    AsmAstBinaryOperator::Subtract => {
                        write!(f, "{}", format!("Binary(SUB, src_2:{:?}, dst:{:?})", self.src_2, self.dst).as_str()).expect("Write failed!");
                    }
                    AsmAstBinaryOperator::Multiply => {
                        write!(f, "{}", format!("Binary(MUL, src_2:{:?}, dst:{:?})", self.src_2, self.dst).as_str()).expect("Write failed!");
                    }
                    _ => {
                        todo!();
                    }
                }
            }

            AsmAstInstructionType::Cdq => {
                write!(f, "Cdq").expect("Write failed!");
            }

            AsmAstInstructionType::Idiv => {
                write!(f, "{}", format!("Idiv(dst:{:?})", self.dst).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::Mod => {
                write!(f, "{}", format!("Mod(dst:{:?})", self.dst).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::Mul => {
                write!(f, "{}", format!("Mul(dst:{:?})", self.dst).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::Cmp => {
                write!(f, "{}", format!("Cmp(src:{:?}, src_2:{:?})", self.src, self.src_2).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::Jmp => {
                write!(f, "{}", format!("Jmp(src:{:?}, src_2:{:?}, dst:{:?})", self.src, self.src_2, self.dst).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::JmpCC => {
                write!(f, "{}", format!("JmpCC(src:{:?}, src_2:{:?})", self.src, self.src_2).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::Label => {
                write!(f, "{}", format!("Label(src:{:?})", self.src).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::SetCC => {
                write!(f, "{}", format!("SetCC(src:{:?}, dst:{:?})", self.src, self.dst).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::Lea => {
                write!(f, "{}", format!("Lea(src:{:?}, dst:{:?})", self.src, self.dst).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::FunctionCall => {
                write!(f, "{}", format!("FunctionCall(identifier:{:?})", self.identifier).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::AllocateStack => {
                match self.src.operand_type {
                    AsmAstOperandType::Imm(imm_value) => {
                        write!(f, "{}", format!("AllocateStack({:?})", imm_value).as_str()).expect("Write failed!");
                    }
                    _ => {
                        todo!();
                    }
                }
            }

            AsmAstInstructionType::DeallocateStack => {
                match self.src.operand_type {
                    AsmAstOperandType::Imm(imm_value) => {
                        write!(f, "{}", format!("DeallocateStack({:?})", imm_value).as_str()).expect("Write failed!");
                    }
                    _ => {
                        todo!();
                    }
                }
            }

            AsmAstInstructionType::Push => {
                write!(f, "{}", format!("Push(operand:{:?})", self.dst).as_str()).expect("Write failed!");
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AsmAstInstructionType {
    Ret,
    Mov,
    Movsx,
    Unary,
    Binary,
    Cdq,
    Idiv,
    Mod,
    Mul,
    Cmp,
    Jmp,
    JmpCC,
    SetCC,
    Lea,
    // JumpIfZero,
    // JumpIfNotZero,
    // Copy,
    // Load,
    // Store,
    // GetAddress,
    // AddPtr,
    // CopyToOffset,
    // CopyFromOffset,
    Label,
    FunctionCall,
    AllocateStack,
    DeallocateStack,
    Push,
    // ZeroExtend,
    // SignExtend,
    // Truncate,
    // IntToDouble,
    // DoubleToInt,
    // UIntToDouble,
    // DoubleToUInt,
    // VariableDeclaration,
}

#[derive(Clone, Debug)]
pub struct AsmAstOperand {
    pub operand_type: AsmAstOperandType,
}

impl AsmAstOperand {
    pub fn new() -> AsmAstOperand {
        AsmAstOperand {
            operand_type: AsmAstOperandType::Unknown,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AsmAstOperandType {
    Imm(i32),
    Reg(AsmAstReg),
    Pseudo(String),
    PseudoMem(String, i32),
    // Stack(i32), // implicitly uses the RBP register as a base plus an i32 offset
    Memory(AsmAstReg, i32), // on page 375, the Stack Operand is replaced by a more
    // general Memory Operand that uses an explicit register instead of implicit RBP like Stack(i32) did.
    // The format is now BaseRegister (AsmAstReg) + offset (i32)
    ComparisonType(String), // (E)qual, (N)ot (E)qual, (L)essThan, (L)essThan or (E)qual, (G)reaterThan, (G)reaterThan or (E)qual
    Label(String),
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AsmAstReg {
    AX, // -> EAX
    EAX,
    RAX,
    BX, // -> EBX
    DX, // -> EDX
    CX,
    DI,
    SI,
    R8,
    R9,
    R10,

    RBP, // stack frame base pointer register
}

impl AsmAstReg {
    pub(crate) fn output_register_matching_datatype(&self,
        data_type: &AstAstAssemblyType
    ) -> String
    {

        let mut string_buffer = String::from("");

        match self {
            AsmAstReg::AX => {
                // panic!("Unhandled: {}", self);

                // append suffix for data type
                match data_type {
                    AstAstAssemblyType::Doubleword => {
                        string_buffer.push_str("rax");
                    }
                    _ => {
                        panic!("");
                    }
                }
            }
            AsmAstReg::EAX => {
                // panic!("Unhandled: {}", self);

                // append suffix for data type
                match data_type {
                    AstAstAssemblyType::Doubleword => {
                        string_buffer.push_str("eax");
                    }
                    _ => {
                        panic!("");
                    }
                }
            }
            AsmAstReg::RAX => {
                // // panic!("Unhandled: {}", self);

                // // append suffix for data type
                // match data_type_size {
                //     AstAstAssemblyType::Doubleword => {
                //         string_buffer.push_str("rax");
                //     }
                //     _ => {
                //         panic!("");
                //     }
                // }
                string_buffer.push_str("rax");
            }
            AsmAstReg::R10 => {
                string_buffer.push_str("r10");

                // append suffix for data type
                match data_type {
                    // AstAstAssemblyType::Doubleword => {
                    //     string_buffer.push_str("d");
                    // }
                    AstAstAssemblyType::Doubleword => {
                        string_buffer.push_str("d");
                    }
                    AstAstAssemblyType::Unknown => {
                        panic!("Has no type! Found: {:?}", data_type);
                    }
                    _ => {
                        panic!("Unknown Type: {:?}", data_type);
                    }
                }
            }
            _ => {
                panic!("Unhandled: {}", self);
            }
        }

        string_buffer
    }
}

impl fmt::Display for AsmAstReg {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // AsmAstReg::AX => write!(f, "eax"),
            // AsmAstReg::BX => write!(f, "ebx"),
            // AsmAstReg::DX => write!(f, "edx"),
            // AsmAstReg::CX => write!(f, "ecx"),
            // AsmAstReg::DI => write!(f, "edi"),
            // AsmAstReg::SI => write!(f, "esi"),

            AsmAstReg::AX => write!(f, "ax"),
            AsmAstReg::EAX => write!(f, "eax"),
            AsmAstReg::RAX => write!(f, "rax"),

            AsmAstReg::BX => write!(f, "rbx"),
            AsmAstReg::DX => write!(f, "rdx"),
            AsmAstReg::CX => write!(f, "rcx"),
            AsmAstReg::DI => write!(f, "rdi"),
            AsmAstReg::SI => write!(f, "rsi"),

            AsmAstReg::R8 => write!(f, "r8"),
            // AsmAstReg::R8 => write!(f, "r8d"),

            AsmAstReg::R9 => write!(f, "r9"),
            // AsmAstReg::R9 => write!(f, "r9d"),

            AsmAstReg::R10 => write!(f, "r10"),
            // AsmAstReg::R10 => write!(f, "r10d"),

            AsmAstReg::RBP => write!(f, "rbp"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum AsmAstUnaryOperator {
    Neg,
    Not,
    Increment,
    AddAssignment,
}

#[allow(unreachable_code)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
#[allow(unreachable_patterns)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
impl fmt::Display for AsmAstUnaryOperator {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match &self {
            AsmAstUnaryOperator::Neg => {
                write!(f, "neg").expect("Write failed!");
            }
            AsmAstUnaryOperator::Not => {
                write!(f, "not").expect("Write failed!");
            }
            AsmAstUnaryOperator::Increment => {
                write!(f, "increment").expect("Write failed!");
            }
            AsmAstUnaryOperator::AddAssignment => {
                write!(f, "add_assignment").expect("Write failed!");
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum AsmAstBinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    And,
    Or,
    Xor,
    LeftShift,
    RightShift,

    LogicalAnd,
    LogicalOr,

    AddAssignment,
}

#[allow(unreachable_code)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
#[allow(unreachable_patterns)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
impl fmt::Display for AsmAstBinaryOperator {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match &self {

            AsmAstBinaryOperator::Add => {
                write!(f, "add").expect("Write failed!");
            }

            AsmAstBinaryOperator::Subtract => {
                write!(f, "sub").expect("Write failed!");
            }

            AsmAstBinaryOperator::Multiply => {
                write!(f, "mul").expect("Write failed!");
            }

            AsmAstBinaryOperator::Divide => {
                write!(f, "div").expect("Write failed!");
            }

            AsmAstBinaryOperator::Remainder => {
                write!(f, "mod").expect("Write failed!");
            }

            AsmAstBinaryOperator::Equal => {
                write!(f, "Equal").expect("Write failed!");
            }

            AsmAstBinaryOperator::NotEqual => {
                write!(f, "NotEqual").expect("Write failed!");
            }

            AsmAstBinaryOperator::LessThan => {
                write!(f, "LessThan").expect("Write failed!");
            }

            AsmAstBinaryOperator::LessThanOrEqual => {
                write!(f, "LessThanOrEqual").expect("Write failed!");
            }

            AsmAstBinaryOperator::GreaterThan => {
                write!(f, "GreaterThan").expect("Write failed!");
            }

            AsmAstBinaryOperator::GreaterThanOrEqual => {
                write!(f, "GreaterThanOrEqual").expect("Write failed!");
            }

            AsmAstBinaryOperator::And => {
                write!(f, "And").expect("Write failed!");
            }

            AsmAstBinaryOperator::Or => {
                write!(f, "Or").expect("Write failed!");
            }

            AsmAstBinaryOperator::Xor => {
                write!(f, "Xor").expect("Write failed!");
            }

            AsmAstBinaryOperator::LeftShift => {
                write!(f, "LeftShift").expect("Write failed!");
            }

            AsmAstBinaryOperator::RightShift => {
                write!(f, "LeftShift").expect("Write failed!");
            }

            AsmAstBinaryOperator::LogicalAnd => {
                write!(f, "LogicalAnd").expect("Write failed!");
            }

            AsmAstBinaryOperator::LogicalOr => {
                write!(f, "LogicalOr").expect("Write failed!");
            }

            AsmAstBinaryOperator::AddAssignment => {
                write!(f, "AddAssignment").expect("Write failed!");
            }
        }

        Ok(())
    }
}

//
// This function is called to print the intermediate ASM to a file!
// This is debug functionality to debug the intermediate AST generation!
pub fn print_asm_ast_program(program: &AsmAstProgram, string_buffer: &mut String, indent: usize) {

    // indent
    let indent_string = std::iter::repeat(" ").take(indent * 2).collect::<String>();
    string_buffer.push_str(&indent_string);

    // add program
    string_buffer.push_str(format!("Program(\"{}\")\n", program.name).as_str());

    // print functions in program
    for i in 0..program.functions.len() {
        print_asm_ast_function(&program.functions[i], string_buffer, indent + 1);
    }
}

pub fn print_asm_ast_function(function: &AsmAstFunction, string_buffer: &mut String, indent: usize) {

    // indent
    let indent_string = std::iter::repeat(" ").take(indent * 2).collect::<String>();
    string_buffer.push_str(&indent_string);

    // add function
    string_buffer.push_str(format!("Function(\"{}\")\n", function.name).as_str());

    // print instructions in function
    for i in 0..function.body.len() {
        print_asm_ast_instruction(&function.body[i], string_buffer, indent + 1);
    }
}

pub fn print_asm_ast_instruction(asm_ast_instruction: &AsmAstInstruction, string_buffer: &mut String, indent: usize) {

    let double_indent_string = std::iter::repeat(" ").take(indent * 4).collect::<String>();
    let tripple_indent_string = std::iter::repeat(" ").take(indent * 6).collect::<String>();

    if asm_ast_instruction.comment.len() > 0usize {
        string_buffer.push_str(format!("\n{}\n", &asm_ast_instruction.comment.clone()).as_str());
    }

    // indent
    let indent_string = std::iter::repeat(" ").take(indent * 2).collect::<String>();
    string_buffer.push_str(&indent_string);

    match &asm_ast_instruction.instruction_type {

        AsmAstInstructionType::Ret => {
            string_buffer.push_str(format!("Ret\n").as_str());
        }

        AsmAstInstructionType::Mov => {
            string_buffer.push_str(format!("Mov({:?}, src:{:?}, dst:{:?})\n",
                asm_ast_instruction.assembly_type,
                asm_ast_instruction.src,
                asm_ast_instruction.dst).as_str());
        }

        AsmAstInstructionType::Movsx => {
            string_buffer.push_str(format!("Movsx(src:{:?}, dst:{:?})\n",
                asm_ast_instruction.src,
                asm_ast_instruction.dst).as_str());
        }

        AsmAstInstructionType::Unary => {
            match asm_ast_instruction.src.operand_type {
                AsmAstOperandType::Unknown => {
                    string_buffer.push_str(format!("Unary(unary_operator:{:?}, assembly_type:{:?}, dst:{:?})",
                        asm_ast_instruction.unary_operator,
                        asm_ast_instruction.assembly_type,
                        asm_ast_instruction.dst
                    ).as_str());
                }
                _ => {
                    string_buffer.push_str(format!("Unary(unary_operator:{:?}, assembly_type:{:?}, dst:{:?}, src:{:?})",
                        asm_ast_instruction.unary_operator,
                        asm_ast_instruction.assembly_type,
                        asm_ast_instruction.dst,
                        asm_ast_instruction.src
                    ).as_str());
                }
            }
        }

        AsmAstInstructionType::Binary => {
            string_buffer.push_str(format!("Binary(binary_operator:{:?}, assembly_type:{:?}, src_2:{:?}, dst:{:?})",
                asm_ast_instruction.binary_operator,
                asm_ast_instruction.assembly_type,
                asm_ast_instruction.src_2,
                asm_ast_instruction.dst).as_str());
        }

        AsmAstInstructionType::Lea => {
            string_buffer.push_str(format!("Lea(src:{:?}, dst:{:?})\n",
                asm_ast_instruction.src,
                asm_ast_instruction.dst).as_str());
        }


        AsmAstInstructionType::AllocateStack => {
            match asm_ast_instruction.src.operand_type {
                AsmAstOperandType::Imm(imm_value) => {
                    string_buffer.push_str(format!("AllocateStack({:?})\n", imm_value).as_str());
                }
                _ => {
                    panic!("{}", format!("Unhandled ASM AST operand_type: '{:?}'!\n", asm_ast_instruction.src.operand_type).as_str());
                }
            }
        }

        AsmAstInstructionType::DeallocateStack => {
            match asm_ast_instruction.src.operand_type {
                AsmAstOperandType::Imm(imm_value) => {
                    string_buffer.push_str(format!("DeallocateStack({:?})\n", imm_value).as_str());
                }
                _ => {
                    panic!("{}", format!("Unhandled ASM AST operand_type: '{:?}'!\n", asm_ast_instruction.src.operand_type).as_str());
                }
            }
        }

        AsmAstInstructionType::Label => {
            string_buffer.push_str(format!("Label(value:{:?})\n",
                asm_ast_instruction.src).as_str());
                // asm_ast_instruction.dst).as_str());
        }

        AsmAstInstructionType::Cmp => {
            string_buffer.push_str(format!("Cmp(unary_operator:{:?},\n{}assembly_type:{:?},\n{}dst:{:?},\n{}src_2:{:?})\n",
                asm_ast_instruction.unary_operator,
                tripple_indent_string,
                asm_ast_instruction.assembly_type,
                tripple_indent_string,
                asm_ast_instruction.dst,
                tripple_indent_string,
                asm_ast_instruction.src_2
            ).as_str());
        }

        AsmAstInstructionType::SetCC => {
            string_buffer.push_str(format!("SetCC(dst:{:?}, src:{:?})\n",
                asm_ast_instruction.dst,
                asm_ast_instruction.src
            ).as_str());
        }

        AsmAstInstructionType::JmpCC => {
            string_buffer.push_str(format!("JmpCC(dst:{:?}, src:{:?})\n",
                asm_ast_instruction.dst,
                asm_ast_instruction.src
            ).as_str());
        }

        AsmAstInstructionType::Jmp => {
            string_buffer.push_str(format!("Jmp(dst:{:?}, src:{:?})\n",
                asm_ast_instruction.dst,
                asm_ast_instruction.src
            ).as_str());
        }

        _ => {
            panic!("{}", format!("Unhandled ASM AST Type: '{:?}'!\n", asm_ast_instruction.instruction_type).as_str());
        }
    }
}