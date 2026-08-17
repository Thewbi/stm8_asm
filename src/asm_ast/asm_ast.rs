use std::fmt;
use std::fmt::Display;

use crate::tacky::tacky::ValueElement;
use crate::tacky::tacky::UnaryOperator;
use crate::tacky::tacky::BinaryOperator;

pub struct AsmAstProgram {
    pub name: String,
    pub function: AsmAstFunction,
}

impl AsmAstProgram {
    pub fn new() -> AsmAstProgram {
        AsmAstProgram {
            name: String::from(""),
            function: AsmAstFunction::new(),
        }
    }
}

pub struct AsmAstFunction {
    pub name: String,
    pub body: Vec::<Box<AsmAstInstruction>>,
    pub stack_frame_size: i32,
}

impl AsmAstFunction {
    pub fn new() -> AsmAstFunction {
        AsmAstFunction {
            name: String::from(""),
            body: Vec::<Box<AsmAstInstruction>>::new(),
            stack_frame_size: 0i32,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AsmAstInstruction {
    pub instruction_type: AsmAstInstructionType,
    pub unary_operator: AsmAstUnaryOperator,
    pub binary_operator: AsmAstBinaryOperator,
    pub src: AsmAstOperand,
    pub src_2: AsmAstOperand,
    pub dst: AsmAstOperand,
}

impl AsmAstInstruction {
    pub fn new() -> AsmAstInstruction {
        AsmAstInstruction {
            instruction_type: AsmAstInstructionType::Ret,
            unary_operator: AsmAstUnaryOperator::Not,
            binary_operator: AsmAstBinaryOperator::Add,
            src: AsmAstOperand::new(),
            src_2: AsmAstOperand::new(),
            dst: AsmAstOperand::new(),
        }
    }
}

#[allow(unreachable_code)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
#[allow(unreachable_patterns)] // still under development, so enums will be extended and the match should catch unhandled options so the catch-all case needs to stay even if it throws warnings
impl fmt::Display for AsmAstInstruction {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        // match &self {
        //     AsmAstUnaryOperator::Neg => {
        //         write!(f, "neg").expect("Write failed!");
        //     }
        //     AsmAstUnaryOperator::Not => {
        //         write!(f, "not").expect("Write failed!");
        //     }
        // }

        match &self.instruction_type {

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

            AsmAstInstructionType::Ret => {
                write!(f, "{}", format!("Ret {:?}", self).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::Mov => {
                // write!(f, "Mov").expect("Write failed!");
                write!(f, "{}", format!("Mov(src:{:?}, dst:{:?})", self.src, self.dst).as_str()).expect("Write failed!");
            }

            AsmAstInstructionType::Unary => {
                // write!(f, "Unary").expect("Write failed!");

                match self.unary_operator {
                    AsmAstUnaryOperator::Neg => {
                        // write!(f, "{}", format!("Neg({:?}, {:?}, {:?})", self.dst, self.src, self.src_2).as_str()).expect("Write failed!");
                        write!(f, "{}", format!("Neg(dst:{:?})", self.dst).as_str()).expect("Write failed!");
                    }
                    AsmAstUnaryOperator::Not => {
                        // write!(f, "{}", format!("Not({:?}, {:?}, {:?})", self.dst, self.src, self.src_2).as_str()).expect("Write failed!");
                        write!(f, "{}", format!("Not(dst:{:?})", self.dst).as_str()).expect("Write failed!");
                    }
                    AsmAstUnaryOperator::Increment => {
                        write!(f, "{}", format!("Not(dst:{:?})", self.dst).as_str()).expect("Write failed!");
                    }
                    // _ => {
                    //     todo!();
                    // }
                }
            }

            AsmAstInstructionType::Binary => {
                // write!(f, "Binary").expect("Write failed!");

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
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum AsmAstInstructionType {
    AllocateStack,
    Ret,
    Mov,
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
    // FunCall,
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
            operand_type: AsmAstOperandType::Imm(0),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AsmAstOperandType {
    Imm(i32),
    Reg(AsmAstReg),
    Pseudo(String),
    Stack(i32),
    ComparisonType(String), // (E)qual, (N)ot (E)qual, (L)essThan, (L)essThan or (E)qual, (G)reaterThan, (G)reaterThan or (E)qual
    Label(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum AsmAstReg {
    AX, // -> EAX
    BX, // -> EBX
    DX, // -> EDX
    R10, // -> R10D
}

#[derive(Clone, Debug)]
pub enum AsmAstUnaryOperator {
    Neg,
    Not,
    Increment,
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
        }

        Ok(())
    }
}



// match self.src.operand_type {
                //     AsmAstOperandType::Imm(imm_value) => {
                //         write!(f, "{}", format!("Mov({:?})", imm_value).as_str()).expect("Write failed!");
                //     }
                //     _ => {
                //         todo!();
                //     }
                // }

                // match self.dst.operand_type {
                //     AsmAstOperandType::Imm(imm_value) => {
                //         write!(f, "{}", format!("Mov({:?})", imm_value).as_str()).expect("Write failed!");
                //     }
                //     _ => {
                //         todo!();
                //     }
                // }