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

use crate::c_ast::ast_node::AstNodeOperatorType;

//
// Converts or lowers a TACKY application into an ASM AST (of precursor/pseudo ASM instructions).
//
// 1. c_ast/IdentifierResolutionVisitor - checks for duplicate or undeclared variable names
// 2. tacky/TackyVisitor - Generate TACKY (from AST)
// 3. asm_ast/AsmAstConversionVisitor - Converts the AST into a ASM AST for assembly with a precursory form of mnenomics
// 4. asm_ast/AsmAstFixupVisitor - replacing pseudo operands/variables with stack addresses
// 5. asm_ast/AsmAstMasmEmitterVisitor / asm_ast/AsmAstASEmitterVisitor / ...
//
// This visitor generates a precursory form of real assembly instructions.
// For each indidivual line of TACKY instruction, it prepares one or more precursor assembly instructions 
// that are required to execute functionality that the TACKY instruction hints at. 
//
// e.g. div, remainder/module, mul, ... need to arrange parameters and execute data type 
// conversion (cdq, ...) before finally executing the arithmetic mnemonic. This visitor
// will prepare all the pseudo mnemonics required to execute each TACKY function/line.
//

pub struct AsmAstConversionVisitor {
    pub asm_ast_program: AsmAstProgram,
}

impl AsmAstConversionVisitor {

    pub fn new() -> AsmAstConversionVisitor {
        AsmAstConversionVisitor {
            asm_ast_program: AsmAstProgram::new(),
        }
    }

    // util
    pub fn move_src_to_ax(&mut self, tacky_instruction: &Instruction) -> AsmAstInstruction {

        let mut mov: AsmAstInstruction = AsmAstInstruction::new();
        mov.instruction_type = AsmAstInstructionType::Mov;

        match &tacky_instruction.src {

            ValueElement::Constant(constant_value) => {
                mov.src = AsmAstOperand { operand_type: AsmAstOperandType::Imm(i32::from_str_radix(&constant_value, 10).expect("REASON")) };
            }

            ValueElement::Variable(variable_name) => {
                mov.src = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }

            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_instruction.src).as_str());
            }
        }

        mov.dst = AsmAstOperand{ operand_type: AsmAstOperandType::Reg(AsmAstReg::AX) };

        mov
    }

    pub fn visit_tacky_program(&mut self, tacky_node_program: &Program) {
        println!("[AsmAstConversionVisitor::visit_tacky_program()]");

        self.asm_ast_program.name = tacky_node_program.name.clone();
        
        for i in 0..tacky_node_program.top_level.len() {

            let temp_top_level_item:&Box<TopLevel> = &tacky_node_program.top_level[i];

            match &temp_top_level_item.as_ref().top_level_type {

                TopLevelType::Function => {
                    self.visit_tacky_function(&temp_top_level_item.as_ref());
                }

                TopLevelType::StaticVariable => {
                    todo!("TopLevelType::StaticVariable");
                }

                TopLevelType::StaticConstant => {
                    todo!("TopLevelType::StaticConstant");
                }
            }
        }
    }

    pub fn visit_tacky_function(&mut self, tacky_node_top_level_function: &TopLevel) {
        println!("[AsmAstConversionVisitor::visit_tacky_function()]");

        let mut asm_ast_function: AsmAstFunction = AsmAstFunction::new();
        asm_ast_function.name = tacky_node_top_level_function.name.clone();

        // insert allocate stack instruction - page 42
        let mut asm_ast_allocate_stack: AsmAstInstruction = AsmAstInstruction::new();
        asm_ast_allocate_stack.instruction_type = AsmAstInstructionType::AllocateStack;
        asm_ast_function.body.push(Box::new(asm_ast_allocate_stack));

        for i in 0..tacky_node_top_level_function.body.len() {

            let tacky_instruction:&Instruction = &tacky_node_top_level_function.body[i].as_ref();
            match tacky_instruction.instruction_type {

                InstructionType::Unary => {
                    self.visit_tacky_unary(&mut asm_ast_function, tacky_instruction);
                }

                InstructionType::Binary => {
                    self.visit_tacky_binary(&mut asm_ast_function, tacky_instruction);
                }

                InstructionType::Return => {
                    self.visit_tacky_return(&mut asm_ast_function, tacky_instruction);
                }

                InstructionType::JumpIfZero => {
                    self.visit_tacky_jump_if_zero(&mut asm_ast_function, tacky_instruction);
                }

                InstructionType::Label => {
                    self.visit_tacky_label(&mut asm_ast_function, tacky_instruction);
                }

                InstructionType::VariableDeclaration => {
                    self.visit_tacky_variable_declaration(&mut asm_ast_function);
                }

                InstructionType::Copy => {
                    self.visit_tacky_copy(&mut asm_ast_function, tacky_instruction);
                }

                InstructionType::Jump => {
                    self.visit_tacky_jump(&mut asm_ast_function, tacky_instruction);
                }

                _ => {
                    panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_instruction.instruction_type).as_str());
                }
            }
        }

        self.asm_ast_program.function = asm_ast_function;
    }

    pub fn visit_tacky_jump(&mut self, asm_ast_function: &mut AsmAstFunction, tacky_node_jump: &Instruction) {
        println!("TODO: visit_tacky_jump()");

        println!("{:?}", tacky_node_jump);

        let mut jump: AsmAstInstruction = AsmAstInstruction::new();
        jump.instruction_type = AsmAstInstructionType::Jmp;
        jump.dst = AsmAstOperand { operand_type: AsmAstOperandType::Label(tacky_node_jump.label.clone()) };

        asm_ast_function.body.push(Box::new(jump));
    }

    pub fn visit_tacky_copy(&mut self, asm_ast_function: &mut AsmAstFunction, tacky_node_copy: &Instruction) {
        //println!("TODO: visit_tacky_copy()");

        // DEBUG
        println!("{:?}", tacky_node_copy);

        // sometimes copies are generated, that copy src to dst where src and dst are the same object
        // ignore those cases
        if tacky_node_copy.src == tacky_node_copy.dst {
            return;
        }

        // Instruction { 
        //      instruction_type: Copy, 
        //      src: Constant(123), 
        //      src_2: None, 
        //      dst: Variable(userdef_var.0), 
        //      unary_operator: Not, 
        //      binary_operator: Add, 
        //      label: "", 
        //      data_type: "", 
        //      function_name: "", 
        //      parameters: [], 
        //      offset: 0, 
        //      index: None, 
        //      scale: 0 
        // }

        //
        // MOV
        //
        // TACKY Unary ==> Mov + Unary
        //

        let mut mov: AsmAstInstruction = AsmAstInstruction::new();
        mov.instruction_type = AsmAstInstructionType::Mov;

        match &tacky_node_copy.src {
            ValueElement::Constant(constant_value) => {
                mov.src = AsmAstOperand { operand_type: AsmAstOperandType::Imm(i32::from_str_radix(&constant_value, 10).expect("REASON")) };
            }
            ValueElement::Variable(variable_name) => {
                mov.src = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_copy.src).as_str());
            }
        }

        match &tacky_node_copy.dst {
            ValueElement::Variable(variable_name) => {
                mov.dst = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_copy.dst).as_str());
            }
        }

        asm_ast_function.body.push(Box::new(mov));
    }

    pub fn visit_tacky_variable_declaration(&mut self, asm_ast_function: &mut AsmAstFunction) {
        println!("TODO: visit_tacky_variable_declaration()");
    }

    pub fn visit_tacky_label(&mut self, asm_ast_function: &mut AsmAstFunction, tacky_node_label: &Instruction) {
        
        let mut label: AsmAstInstruction = AsmAstInstruction::new();
        label.instruction_type = AsmAstInstructionType::Label;
        label.src = AsmAstOperand { operand_type: AsmAstOperandType::Label(tacky_node_label.label.clone()) };

        asm_ast_function.body.push(Box::new(label));
    }

    // Converts or lowers a TACKY application into an ASM AST (of precursor/pseudo ASM instructions).
    //
    // Binary(LessThan, src_1:Constant(1), src_2:Constant(2), dst:Variable(tmp.0))
    // JumpIfZero(Constant(tmp.0), if_14.end_label)
    //
    // Nora Sandler, page 78 ff
    //
    // Cmp(Imm(0), condition)
    // JmpCC(E, target)
    // 
    // Nora Sandler, page 86
    //
    // JumpIfZero(val, target)
    // --->
    // Cmp(Imm(0), val)
    // JmpCC(E, target)
    pub fn visit_tacky_jump_if_zero(&mut self, asm_ast_function: &mut AsmAstFunction, tacky_node_jump_if_zero: &Instruction) {
        
        println!("[AsmAstConversionVisitor::visit_tacky_jump_if_zero() {:?}]", tacky_node_jump_if_zero);

        //
        // Cmp(Imm(0), condition)
        //

        let mut cmp: AsmAstInstruction = AsmAstInstruction::new();
        cmp.instruction_type = AsmAstInstructionType::Cmp;
        cmp.dst = AsmAstOperand { 
            operand_type: AsmAstOperandType::Imm(0) 
        };

        match &tacky_node_jump_if_zero.src {

            ValueElement::Constant(constant_value) => {
                let parsed_value = i32::from_str_radix(&constant_value, 10).expect("REASON");
                cmp.src_2 = AsmAstOperand { 
                    operand_type: AsmAstOperandType::Imm(parsed_value) 
                };
            }

            ValueElement::Variable(variable_name) => {
                println!("{}", variable_name);
                cmp.src_2 = AsmAstOperand { 
                    operand_type: AsmAstOperandType::Pseudo(variable_name.clone())
                };
            }

            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_jump_if_zero.src).as_str());
            }
        }

        asm_ast_function.body.push(Box::new(cmp));

        //
        // JmpCC(E, target)
        //

        let mut jump_cc: AsmAstInstruction = AsmAstInstruction::new();
        jump_cc.instruction_type = AsmAstInstructionType::JmpCC;
        jump_cc.src = AsmAstOperand { operand_type: AsmAstOperandType::ComparisonType(String::from("E")) }; // E as in Equal
        jump_cc.dst = AsmAstOperand { operand_type: AsmAstOperandType::Label(tacky_node_jump_if_zero.label.clone()) };

        asm_ast_function.body.push(Box::new(jump_cc));
    }

    pub fn visit_tacky_return(&mut self, asm_ast_function: &mut AsmAstFunction, tacky_node_return: &Instruction) {
        println!("[AsmAstConversionVisitor::visit_tacky_return() {:?}]", tacky_node_return);

        // mov eax, src
        let mut mov: AsmAstInstruction = AsmAstInstruction::new();
        mov.instruction_type = AsmAstInstructionType::Mov;
        match &tacky_node_return.src {
            ValueElement::Constant(constant_value) => {
                let parsed_value = i32::from_str_radix(&constant_value, 10).expect("REASON");
                mov.src = AsmAstOperand { operand_type: AsmAstOperandType::Imm(parsed_value) };
            }
            ValueElement::Variable(variable_name) => {
                mov.src = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            ValueElement::None => {
                // do nothing
            }
            // _ => {
            //     panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_return.src).as_str());
            // }
        }
        mov.dst = AsmAstOperand{ operand_type: AsmAstOperandType::Reg(AsmAstReg::AX) };
        asm_ast_function.body.push(Box::new(mov));

        // ret
        let mut ret: AsmAstInstruction = AsmAstInstruction::new();
        ret.instruction_type = AsmAstInstructionType::Ret;

        asm_ast_function.body.push(Box::new(ret));
    }

    // Nora Sandler, page 41
    //
    // Unary is implemented in TACKY as a UNARY node with a 
    // - src which is the variable or constant to increment
    // - dst which is the place to put the result into
    // - unary_operator which is the unary operation to apply (inc, dec, not, neg, complement, ...)
    //
    // This will be translated into a Mov from src to dest followed by a unary oparation using dst.
    pub fn visit_tacky_unary(&mut self, asm_ast_function: &mut AsmAstFunction, tacky_node_unary: &Instruction) {
        println!("[AsmAstConversionVisitor::visit_tacky_unary()]");

        //
        // MOV
        //
        // TACKY Unary ==> Mov + Unary
        //

        let mut mov: AsmAstInstruction = AsmAstInstruction::new();
        mov.instruction_type = AsmAstInstructionType::Mov;

        match &tacky_node_unary.src {
            ValueElement::Constant(constant_value) => {
                mov.src = AsmAstOperand { operand_type: AsmAstOperandType::Imm(i32::from_str_radix(&constant_value, 10).expect("REASON")) };
            }
            ValueElement::Variable(variable_name) => {
                mov.src = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_unary.src).as_str());
            }
        }

        match &tacky_node_unary.dst {
            ValueElement::Variable(variable_name) => {
                mov.dst = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_unary.dst).as_str());
            }
        }

        asm_ast_function.body.push(Box::new(mov));

        //
        // UNARY
        //
        // TACKY Unary ==> Mov + Unary
        //
        
        let mut unary: AsmAstInstruction = AsmAstInstruction::new();
        unary.instruction_type = AsmAstInstructionType::Unary;
        match &tacky_node_unary.unary_operator {
            UnaryOperator::Complement => {
                unary.unary_operator = AsmAstUnaryOperator::Not;
            }
            UnaryOperator::Negate => {
                unary.unary_operator = AsmAstUnaryOperator::Neg;
            }
            UnaryOperator::Increment => {
                unary.unary_operator = AsmAstUnaryOperator::Increment;
            }
            _ => {
                panic!("{}", format!("Unhandled unary_operator {:?}!\n", tacky_node_unary.unary_operator).as_str());
            }
        }

        match &tacky_node_unary.dst {
            ValueElement::Variable(variable_name) => {
                unary.dst = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_unary.dst).as_str());
            }
        }

        asm_ast_function.body.push(Box::new(unary));
    }

    // Nora Sandler, page 63
    pub fn visit_tacky_binary(&mut self, asm_ast_function: &mut AsmAstFunction, tacky_instruction: &Instruction) {
        println!("[AsmAstConversionVisitor::visit_tacky_binary()]");

        match &tacky_instruction.binary_operator {

            BinaryOperator::Division => {
                self.visit_tacky_binary_division(asm_ast_function, tacky_instruction, &AsmAstInstructionType::Idiv);
            }

            BinaryOperator::Remainder => {
                self.visit_tacky_binary_division(asm_ast_function, tacky_instruction, &AsmAstInstructionType::Mod);
            }

            BinaryOperator::Multiply => {
                self.visit_tacky_binary_multiplication(asm_ast_function, tacky_instruction);
            }

            BinaryOperator::Equal => {
                self.visit_tacky_binary_relational(asm_ast_function, tacky_instruction, &AstNodeOperatorType::Equal);
            }
            BinaryOperator::NotEqual => {
                self.visit_tacky_binary_relational(asm_ast_function, tacky_instruction, &AstNodeOperatorType::NotEqual);
            }
            BinaryOperator::LessThan => {
                self.visit_tacky_binary_relational(asm_ast_function, tacky_instruction, &AstNodeOperatorType::LessThan);
            }
            BinaryOperator::LessThanOrEqual => {
                self.visit_tacky_binary_relational(asm_ast_function, tacky_instruction, &AstNodeOperatorType::LessThanOrEqual);
            }
            BinaryOperator::GreaterThan => {
                self.visit_tacky_binary_relational(asm_ast_function, tacky_instruction, &AstNodeOperatorType::GreaterThan);
            }
            BinaryOperator::GreaterThanOrEqual => {
                self.visit_tacky_binary_relational(asm_ast_function, tacky_instruction, &AstNodeOperatorType::GreaterThanOrEqual);
            }

            _ => {
                self.visit_tacky_binary_standard(asm_ast_function, tacky_instruction);
            }
        }
    }

    pub fn visit_tacky_binary_relational(&mut self, asm_ast_function: &mut AsmAstFunction, 
        tacky_node_binary: &Instruction, operator_type_param: &AstNodeOperatorType) {

        println!("[AsmAstConversionVisitor::visit_tacky_binary_relational()] {:?}", tacky_node_binary);

        // nora sandler, page 87
        //
        // Cmp(src2, src1) <----------- Cannot compare two immedites, needs at least one memory address or temp register. 
        //                 <----------- See visit_tacky_binary_division() or visit_tacky_binary_multiplication()
        // Mov(Imm(0), dst)
        // SetCC(relational_operator, dst)

        //
        // Cmp(src2, src1)
        //

        let mut cmp: AsmAstInstruction = AsmAstInstruction::new();
        cmp.instruction_type = AsmAstInstructionType::Cmp;

        match &tacky_node_binary.src {
            ValueElement::Constant(constant_value) => {

                // mul needs a register or memory operand to function. It cannot work with immediate values
                let mut mov: AsmAstInstruction = AsmAstInstruction::new();
                mov.instruction_type = AsmAstInstructionType::Mov;
                mov.src = AsmAstOperand { operand_type: AsmAstOperandType::Imm(i32::from_str_radix(&constant_value, 10).expect("REASON")) };
                mov.dst = AsmAstOperand{ operand_type: AsmAstOperandType::Reg(AsmAstReg::BX) };
                asm_ast_function.body.push(Box::new(mov));

                cmp.src_2 = AsmAstOperand{ operand_type: AsmAstOperandType::Reg(AsmAstReg::BX) };
            }
            ValueElement::Variable(variable_name) => {
                cmp.src_2 = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_binary.src).as_str());
            }
        }

        match &tacky_node_binary.src_2 {
            ValueElement::Constant(constant_value) => {
                cmp.dst = AsmAstOperand { operand_type: AsmAstOperandType::Imm(i32::from_str_radix(&constant_value, 10).expect("REASON")) };
            }
            ValueElement::Variable(variable_name) => {
                cmp.dst = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_binary.src_2).as_str());
            }
        }

        println!("{}", cmp);

        asm_ast_function.body.push(Box::new(cmp));

        //
        // MOV, Nora Sandler, page 63
        //
        // TACKY Binary ==> Mov(src1, dst) + Binary(binary_operator, src2, dst)
        //

        let mut mov: AsmAstInstruction = AsmAstInstruction::new();
        mov.instruction_type = AsmAstInstructionType::Mov;
        mov.src = AsmAstOperand { operand_type: AsmAstOperandType::Imm(0) };

        match &tacky_node_binary.dst {
            ValueElement::Constant(constant_value) => {
                mov.dst = AsmAstOperand { operand_type: AsmAstOperandType::Imm(i32::from_str_radix(&constant_value, 10).expect("REASON")) };
            }
            ValueElement::Variable(variable_name) => {
                mov.dst = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_binary.dst).as_str());
            }
        }

        asm_ast_function.body.push(Box::new(mov));

        //
        // SetCC(relational_operator, dst)
        //

        let mut set_cc: AsmAstInstruction = AsmAstInstruction::new();
        set_cc.instruction_type = AsmAstInstructionType::SetCC;
        // set_cc.src = AsmAstOperand { operand_type: AsmAstOperandType::ComparisonType(String::from("L")) }; // L as in (L)essThan
        set_cc.src = AsmAstOperand { operand_type: AsmAstOperandType::ComparisonType(operator_type_param.to_string()) };

        match &tacky_node_binary.dst {
            ValueElement::Constant(constant_value) => {
                set_cc.dst = AsmAstOperand { operand_type: AsmAstOperandType::Imm(i32::from_str_radix(&constant_value, 10).expect("REASON")) };
            }
            ValueElement::Variable(variable_name) => {
                set_cc.dst = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_binary.dst).as_str());
            }
        }

        asm_ast_function.body.push(Box::new(set_cc));

    }

    pub fn visit_tacky_binary_division(&mut self, asm_ast_function: &mut AsmAstFunction, 
        tacky_node_binary: &Instruction, instruction_type_param: &AsmAstInstructionType) {

        println!("[AsmAstConversionVisitor::visit_tacky_binary_division()]");

        //
        // MOV, Nora Sandler, page 63
        //
        // TACKY Binary ==> Mov(src1, dst) + Binary(binary_operator, src2, dst)
        //

        asm_ast_function.body.push(Box::new(self.move_src_to_ax(tacky_node_binary)));

        //
        // Cdq, page 63
        //

        let mut cdq: AsmAstInstruction = AsmAstInstruction::new();
        cdq.instruction_type = AsmAstInstructionType::Cdq;

        asm_ast_function.body.push(Box::new(cdq));

        //
        // Idiv, page 63
        //

        let mut idiv: AsmAstInstruction = AsmAstInstruction::new();
        idiv.instruction_type = instruction_type_param.clone();

        match &tacky_node_binary.src_2 {

            ValueElement::Constant(constant_value) => {

                // idiv needs a register or memory operand to function. It cannot work with immediate values
                let mut mov: AsmAstInstruction = AsmAstInstruction::new();
                mov.instruction_type = AsmAstInstructionType::Mov;
                mov.src = AsmAstOperand { operand_type: AsmAstOperandType::Imm(i32::from_str_radix(&constant_value, 10).expect("REASON")) };
                mov.dst = AsmAstOperand{ operand_type: AsmAstOperandType::Reg(AsmAstReg::BX) };
                asm_ast_function.body.push(Box::new(mov));

                idiv.dst = AsmAstOperand{ operand_type: AsmAstOperandType::Reg(AsmAstReg::BX) };
            }
            ValueElement::Variable(variable_name) => {
                idiv.dst = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_binary.dst).as_str());
            }
        }

        asm_ast_function.body.push(Box::new(idiv));

        //
        // MOV, Nora Sandler, page 63
        //
        // TACKY Binary ==> Mov(src1, dst) + Binary(binary_operator, src2, dst)
        //

        let mut mov: AsmAstInstruction = AsmAstInstruction::new();
        mov.instruction_type = AsmAstInstructionType::Mov;

        // idiv puts the division result into eax
        // idiv puts the remainder result into edx

        match instruction_type_param {
            AsmAstInstructionType::Idiv => {
                mov.src = AsmAstOperand{ operand_type: AsmAstOperandType::Reg(AsmAstReg::AX) };
            }
            AsmAstInstructionType::Mod => {
                mov.src = AsmAstOperand{ operand_type: AsmAstOperandType::Reg(AsmAstReg::DX) };
            }
            _ => {
                todo!();
            }
        }

        match &tacky_node_binary.dst {
            ValueElement::Constant(constant_value) => {
                mov.dst = AsmAstOperand { operand_type: AsmAstOperandType::Imm(i32::from_str_radix(&constant_value, 10).expect("REASON")) };
            }
            ValueElement::Variable(variable_name) => {
                mov.dst = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_binary.dst).as_str());
            }
        }

        // println!("{}", mov);

        asm_ast_function.body.push(Box::new(mov));
    }

    pub fn visit_tacky_binary_remainder(&mut self, asm_ast_function: &mut AsmAstFunction, tacky_node_binary: &Instruction) {
        println!("[AsmAstConversionVisitor::visit_tacky_binary_remainder()]");

        todo!();
    }

    pub fn visit_tacky_binary_multiplication(&mut self, asm_ast_function: &mut AsmAstFunction, tacky_instruction: &Instruction) {

        // https://www.felixcloutier.com/x86/mul
        //
        // The destination operand is an implied operand located in register AL, AX or EAX (depending on the size of the operand); 
        // the source operand is located in a general-purpose register or a memory location.

        // Move first operand into EAX (which also contains the result afterwards)
        asm_ast_function.body.push(Box::new(self.move_src_to_ax(tacky_instruction)));

        // emit a mul pseudo mnemonic

        let mut mul: AsmAstInstruction = AsmAstInstruction::new();
        mul.instruction_type = AsmAstInstructionType::Mul;

        match &tacky_instruction.src_2 {

            ValueElement::Constant(constant_value) => {

                // mul needs a register or memory operand to function. It cannot work with immediate values
                let mut mov: AsmAstInstruction = AsmAstInstruction::new();
                mov.instruction_type = AsmAstInstructionType::Mov;
                mov.src = AsmAstOperand { operand_type: AsmAstOperandType::Imm(i32::from_str_radix(&constant_value, 10).expect("REASON")) };
                mov.dst = AsmAstOperand{ operand_type: AsmAstOperandType::Reg(AsmAstReg::BX) };
                asm_ast_function.body.push(Box::new(mov));

                mul.dst = AsmAstOperand{ operand_type: AsmAstOperandType::Reg(AsmAstReg::BX) };
            }

            ValueElement::Variable(variable_name) => {
                mul.dst = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }

            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_instruction.src).as_str());
            }
        }

        asm_ast_function.body.push(Box::new(mul));

        // move result available in EAX into the temp variable on the stack so that the final result
        // can retrieve it from the temporary variable on the stack

        //
        // MOV, Nora Sandler, page 63
        //
        // TACKY Binary ==> Mov(src1, dst) + Binary(binary_operator, src2, dst)
        //

        let mut mov: AsmAstInstruction = AsmAstInstruction::new();
        mov.instruction_type = AsmAstInstructionType::Mov;

        // idiv puts the result into eax
        mov.src = AsmAstOperand{ operand_type: AsmAstOperandType::Reg(AsmAstReg::AX) };

        match &tacky_instruction.dst {
            ValueElement::Constant(constant_value) => {
                mov.dst = AsmAstOperand { operand_type: AsmAstOperandType::Imm(i32::from_str_radix(&constant_value, 10).expect("REASON")) };
            }
            ValueElement::Variable(variable_name) => {
                mov.dst = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_instruction.src).as_str());
            }
        }

        println!("{}", mov);

        asm_ast_function.body.push(Box::new(mov));
    }

    pub fn visit_tacky_binary_standard(&mut self, asm_ast_function: &mut AsmAstFunction, tacky_node_binary: &Instruction) {
        println!("[AsmAstConversionVisitor::visit_tacky_binary_standard()]");

        //
        // MOV, Nora Sandler, page 63
        //
        // TACKY Binary ==> Mov(src1, dst) + Binary(binary_operator, src2, dst)
        //

        let mut mov: AsmAstInstruction = AsmAstInstruction::new();
        mov.instruction_type = AsmAstInstructionType::Mov;

        match &tacky_node_binary.src {
            ValueElement::Constant(constant_value) => {
                mov.src = AsmAstOperand { operand_type: AsmAstOperandType::Imm(i32::from_str_radix(&constant_value, 10).expect("REASON")) };
            }
            ValueElement::Variable(variable_name) => {
                mov.src = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_binary.src).as_str());
            }
        }

        match &tacky_node_binary.dst {
            ValueElement::Variable(variable_name) => {
                mov.dst = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_binary.dst).as_str());
            }
        }

        asm_ast_function.body.push(Box::new(mov));

        //
        // BINARY, Nora Sandler, page 63
        //
        // TACKY Binary ==> Mov(src1, dst) + Binary(binary_operator, src2, dst)
        //

        let mut binary: AsmAstInstruction = AsmAstInstruction::new();
        binary.instruction_type = AsmAstInstructionType::Binary;
        match &tacky_node_binary.binary_operator {

            BinaryOperator::Add => {
                binary.binary_operator = AsmAstBinaryOperator::Add;
            }

            BinaryOperator::Subtract => {
                binary.binary_operator = AsmAstBinaryOperator::Subtract;
            }

            BinaryOperator::Multiply => {
                binary.binary_operator = AsmAstBinaryOperator::Multiply;
            }

            BinaryOperator::Equal => {
                binary.binary_operator = AsmAstBinaryOperator::Equal;
            }

            BinaryOperator::NotEqual => {
                binary.binary_operator = AsmAstBinaryOperator::NotEqual;
            }

            BinaryOperator::LessThan => {
                binary.binary_operator = AsmAstBinaryOperator::LessThan;
            }

            BinaryOperator::LessThanOrEqual => {
                binary.binary_operator = AsmAstBinaryOperator::LessThanOrEqual;
            }

            BinaryOperator::GreaterThan => {
                binary.binary_operator = AsmAstBinaryOperator::GreaterThan;
            }

            BinaryOperator::GreaterThanOrEqual => {
                binary.binary_operator = AsmAstBinaryOperator::GreaterThanOrEqual;
            }

            _ => {
                panic!("{}", format!("Unhandled BinaryOperator {:?}!\n", tacky_node_binary.binary_operator).as_str());
            }
        }

        match &tacky_node_binary.src_2 {
            ValueElement::Constant(constant_value) => {
                println!("{}", constant_value);
                binary.src_2 = AsmAstOperand { operand_type: AsmAstOperandType::Imm(i32::from_str_radix(&constant_value, 10).expect("REASON")) };
            }
            ValueElement::Variable(variable_name) => {
                binary.src_2 = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_binary.src_2).as_str());
            }
        }

        match &tacky_node_binary.dst {
            ValueElement::Variable(variable_name) => {
                binary.dst = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
            }
            _ => {
                panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_binary.dst).as_str());
            }
        }

        asm_ast_function.body.push(Box::new(binary));
    }
}

// // unary.src = tacky_node_unary.src.clone();
// match &tacky_node_unary.src {
//     ValueElement::Constant(constant_value) => {
//         unary.src = AsmAstOperand { operand_type: AsmAstOperandType::Imm(i32::from_str_radix(&constant_value, 10).expect("REASON")) };
//     }
//     ValueElement::Variable(variable_name) => {
//         unary.src = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
//     }
//     _ => {
//         panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_unary.src).as_str());
//     }
// }

// match &tacky_node_unary.src {
//     ValueElement::Variable(variable_name) => {
//         unary.src = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
//     }
//     _ => {
//         panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_unary.src).as_str());
//     }
// }

// match &tacky_node_unary.src {
//     ValueElement::Variable(variable_name) => {
//         unary.src = AsmAstOperand { operand_type: AsmAstOperandType::Pseudo(variable_name.clone()) };
//     }
//     _ => {
//         panic!("{}", format!("Unhandled InstructionType {:?}!\n", tacky_node_unary.src).as_str());
//     }
// }

// mov.src = AsmAstOperand{ operand_type: AsmAstOperandType::Reg(AsmAstReg::BX) };

// mov.src = AsmAstOperand{ operand_type: AsmAstOperandType::Reg(AsmAstReg::BX) };