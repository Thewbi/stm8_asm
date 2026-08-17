use std::rc::Rc;
use std::cell::RefCell;

use std::sync::atomic::AtomicUsize;
use crate::Ordering;

use crate::c_ast::ast_node::AstNode;
use crate::c_ast::ast_node::AstNodeType;
use crate::c_ast::ast_node::AstNodeOperatorType;

use crate::AstNodeType::Program as ASTProgram;
use crate::AstNodeType::FunctionDeclaration as ASTFunction;

use crate::Instruction;

use crate::tacky::tacky::Program;
use crate::tacky::tacky::TopLevel;
use crate::tacky::tacky::TopLevelType;
use crate::tacky::tacky::ValueElement;
use crate::tacky::tacky::InstructionType;
use crate::tacky::tacky::UnaryOperator;
use crate::tacky::tacky::BinaryOperator;

use crate::VariableNamingSource;

// https://stackoverflow.com/questions/32935808/generate-sequential-ids-for-each-instance-of-a-struct
// static TEMP_VAR_COUNTER: AtomicUsize = AtomicUsize::new(0);

//
// Generates TACKY from an AST
//
// 1. c_ast/IdentifierResolutionVisitor - checks for duplicate or undeclared variable names
// 2. tacky/TackyVisitor - Generate TACKY (from AST)
// 3. asm_ast/AsmAstConversionVisitor - Converts the AST into a ASM AST for assembly with a precursory form of mnenomics
// 4. asm_ast/AsmAstFixupVisitor - replacing pseudo operands/variables with stack addresses
// 5. asm_ast/AsmAstMasmEmitterVisitor / asm_ast/AsmAstASEmitterVisitor / ...
//

pub struct TackyVisitor {
    variable_naming_source: Rc<RefCell<VariableNamingSource>>,
    pub program: Program,
    pub function_declaration: Option<Box<TopLevel>>,
}

impl TackyVisitor {

    pub fn new(variable_naming_source_param: Rc<RefCell<VariableNamingSource>>) -> TackyVisitor {
        TackyVisitor {
            variable_naming_source: variable_naming_source_param,
            program: Program::new(),
            function_declaration: None,
        }
    }

    pub fn new_end_label(&mut self, ast_node: &AstNode, prefix: &str, index: &mut usize) -> String {

        let mut t = String::from(prefix);
        t.push_str(ast_node.id.to_string().as_str());
        t.push_str("_end_label_");
        t.push_str(index.to_string().as_str());

        t
    }

    pub fn final_end_label(&mut self, ast_node: &AstNode, prefix: &str) -> String {

        let mut t = String::from(prefix);
        t.push_str(ast_node.id.to_string().as_str());
        t.push_str("_end_label");

        t
    }

    pub fn continue_label(&mut self, ast_node: &AstNode, prefix: &str) -> String {

        let mut t = String::from(prefix);
        t.push_str(ast_node.id.to_string().as_str());
        t.push_str("_continue_label");

        t
    }
    
    pub fn start_label(&mut self, ast_node: &AstNode, prefix: &str) -> String {

        let mut t = String::from(prefix);
        t.push_str(ast_node.id.to_string().as_str());
        t.push_str("_start_label");

        t
    }

    pub fn visit(&mut self, ast_node: &AstNode, dst_name: &String, branch_counter: &mut usize) -> ValueElement {

        match &ast_node.node_type {

            AstNodeType::Program => {
                // visit all items in the body of the program
                for i in 0..ast_node.block_items.len() {
                    let mut br_cnt = 0;

                    let block_item = &ast_node.block_items[ast_node.block_items.len()-1-i];
                    self.visit(&block_item, &String::from(""), &mut br_cnt);
                }
            }

            AstNodeType::FunctionDeclaration => {

                // TACKY
                let mut top_level_function: TopLevel = TopLevel::new();
                top_level_function.name = String::from(ast_node.string_val.clone());
                top_level_function.top_level_type = TopLevelType::Function;
                top_level_function.global = true;

                // insert function into program
                self.program.top_level.push(Box::new(top_level_function));

                // add instructions and declarations into body/block
                if let Some(block) = ast_node.lhs.as_ref() {
                    for i in 0..block.block_items.len() {
                        let mut br_cnt = 0;
                        self.visit(&block.block_items[block.block_items.len()-1-i], &String::from(""), &mut br_cnt);
                    }
                }
            }

            AstNodeType::Block => {
                // println!("id: {}", ast_node.node_id);
                for i in 0..ast_node.block_items.len() {
                    let mut br_cnt = 0;
                    self.visit(&ast_node.block_items[ast_node.block_items.len()-1-i], &String::from(""), &mut br_cnt);
                }
            }

            AstNodeType::BlockItem => {
                // println!("id: {}", ast_node.node_id);
                if let Some(compound) = ast_node.lhs.as_ref() {
                    if let Some(left_node) = compound.lhs.as_ref() {
                        let mut br_cnt = 0;
                        self.visit(&left_node, &String::from(""), &mut br_cnt);
                    }
                }
            }

            AstNodeType::Statement => {
                if let Some(sub) = ast_node.lhs.as_ref() {
                    // let mut br_cnt = 0;
                    self.visit(&sub, &dst_name, /*&mut*/ branch_counter);
                }
            }

            AstNodeType::Compound => {
                if let Some(sub) = ast_node.lhs.as_ref() {
                    // let mut br_cnt = 0;
                    self.visit(&sub, &dst_name, /*&mut*/ branch_counter);
                }
            }

            AstNodeType::Expression => {

                match ast_node.operator_type {

                    AstNodeOperatorType::LessThan => {
                        // println!("LessThan");
                    }

                    AstNodeOperatorType::Assignment => {
                        // println!("Assignment");

                        // if a binary instruction is executed, a value is correctly assigned
                        // to the target variable by the binary instruction itself.
                        //
                        // During assignments, binary executions are wrapped in assignment AstNodes.
                        // As the TACKY visitor will generate a copy for the assignment AstNode, 
                        // this assignment has no added value when there is a binary instruction prior
                        // (which already assigns, as stated above).
                        //
                        // Therefore if an assignment wraps a binary instruction, the copy instruction
                        // is not emitted. If a assignment does not wrap a binary instruction, the 
                        // copy is emitted
                        let mut output_copy_instruction = true;

                        // Copy - this is new for the init part of for loops!
                        let mut copy_instruction: Instruction = Instruction::new();
                        copy_instruction.instruction_type = InstructionType::Copy;

                        let mut dst_name = String::from("");

                        if let Some(lhs_sub) = ast_node.lhs.as_ref() {
                            // println!("LHS: {:?}", lhs_sub);
                            dst_name = lhs_sub.string_val.clone();
                            copy_instruction.dst = ValueElement::Variable(dst_name.clone());
                        }

                        if let Some(rhs_sub) = ast_node.rhs.as_ref() {
                            // println!("RHS: {:?}", rhs_sub);

                            // let dst_name = self.new_temp_var();
                            let mut br_cnt = 0;
                            copy_instruction.src = self.visit(&rhs_sub, &dst_name, &mut br_cnt);

                            match rhs_sub.node_type {

                                AstNodeType::Binary => {
                                    println!("binary");
                                    output_copy_instruction = false;
                                }

                                _ => {
                                    // todo!();
                                }
                            }
                        }

                        // to understand this check, read large comment above
                        if output_copy_instruction {
                            // append instruction to latest top-level element of the program
                            let last = self.program.top_level.len() - 1;
                            self.program.top_level[last].body.push(Box::new(copy_instruction));
                        }
                    }

                    AstNodeOperatorType::NotApplicable => {
                        // DEBUG
                        // println!("NotApplicable");
                        if let Some(sub) = ast_node.lhs.as_ref() {
                            let mut br_cnt = 0;
                            return self.visit(&sub, &dst_name, &mut br_cnt);
                        }
                    }

                    _ => {
                        panic!("{}", format!("Unhandled AstNodeOperatorType {:?}!\n", ast_node.operator_type).as_str());
                    }
                }
            }

            AstNodeType::Return => {

                if let Some(expression) = ast_node.lhs.as_ref() {

                    let dst_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let return_value: ValueElement = self.visit(&expression, &dst_name, &mut br_cnt);

                    // OPTIMIZATION: potential for optimization
                    // Currently, return will always return a local temporary variable e.g. tmp.0
                    // which lives in memory on the stack. As such it moves that stack address to EAX
                    // before emitting the ret mnemonic. 
                    //
                    // In some cases, the result is already available in EAX and not in memory.
                    // one such example is return 10 * 3 where the mul mneomic moves the result into
                    // the EAX register anyways. There is not need to move the EAX value to the stack
                    // into the temporary result variable so that return can then move the value back
                    // into EAX from the temporary stack variable!
                    let mut return_instruction: Instruction = Instruction::new();
                    return_instruction.instruction_type = InstructionType::Return;
                    
                    return_instruction.src = return_value;

                    // append instruction to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(return_instruction));

                } else {

                    let mut return_instruction: Instruction = Instruction::new();
                    return_instruction.instruction_type = InstructionType::Return;

                    // append instruction to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(return_instruction));

                }
            }

            AstNodeType::Unary => {

                let mut unary_instruction: Instruction = Instruction::new();
                unary_instruction.instruction_type = InstructionType::Unary;

                // // DEBUG
                // println!("{:?}", ast_node.lhs);
                // println!("{:?}", ast_node.rhs);

                //
                // src
                //

                //
                // dst
                //

                // RHS contains the source
                if let Some(rhs) = ast_node.rhs.as_ref() {

                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let rhs_value_element = self.visit(&rhs, &temp_var_name, &mut br_cnt);
                    
                    // source and destination are the same thing
                    unary_instruction.src = rhs_value_element.clone();
                    unary_instruction.dst = rhs_value_element.clone();
                }

                //
                // operator
                //

                if let Some(operator) = ast_node.lhs.as_ref() {

                    match operator.operator_type {

                        AstNodeOperatorType::Complement => {
                            unary_instruction.unary_operator = UnaryOperator::Complement;
                        }

                        AstNodeOperatorType::Negate => {
                            unary_instruction.unary_operator = UnaryOperator::Negate;
                        }

                        AstNodeOperatorType::Not => {
                            unary_instruction.unary_operator = UnaryOperator::Not;
                        }

                        AstNodeOperatorType::Increment => {
                            unary_instruction.unary_operator = UnaryOperator::Increment;
                        }

                        _ => {
                            panic!("{}", format!("Unhandled OperatorType {:?}!\n", operator.operator_type).as_str());
                        }
                    }
                }

                

                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(unary_instruction));

                return ValueElement::Variable(String::from(dst_name.to_string()))
            }

            AstNodeType::Binary => {
                
                let mut binary_instruction: Instruction = Instruction::new();
                binary_instruction.instruction_type = InstructionType::Binary;
                binary_instruction.dst = ValueElement::Variable(dst_name.to_string());

                // LHS
                if let Some(lhs) = ast_node.lhs.as_ref() {

                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let lhs_value_element = self.visit(&lhs, &temp_var_name, &mut br_cnt);
                    binary_instruction.src_2 = lhs_value_element;
                }

                // RHS
                if let Some(rhs) = ast_node.rhs.as_ref() {

                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let rhs_value_element = self.visit(&rhs, &temp_var_name, &mut br_cnt);
                    binary_instruction.src = rhs_value_element;
                }

                // operator
                if let Some(operator) = ast_node.operator.as_ref() {

                    match operator.operator_type {

                        AstNodeOperatorType::Addition => {
                            binary_instruction.binary_operator = BinaryOperator::Add;
                        }

                        AstNodeOperatorType::Subtraction => {
                            binary_instruction.binary_operator = BinaryOperator::Subtract;
                        }

                        AstNodeOperatorType::Multiplication => {
                            binary_instruction.binary_operator = BinaryOperator::Multiply;
                        }

                        AstNodeOperatorType::Division => {
                            binary_instruction.binary_operator = BinaryOperator::Division;
                        }

                        AstNodeOperatorType::Remainder => {
                            binary_instruction.binary_operator = BinaryOperator::Remainder;
                        }

                        AstNodeOperatorType::LessThan => {
                            binary_instruction.binary_operator = BinaryOperator::LessThan;
                        }

                        AstNodeOperatorType::GreaterThan => {
                            binary_instruction.binary_operator = BinaryOperator::GreaterThan;
                        }

                        AstNodeOperatorType::Equal => {
                            binary_instruction.binary_operator = BinaryOperator::Equal;
                        }

                        AstNodeOperatorType::NotEqual => {
                            binary_instruction.binary_operator = BinaryOperator::NotEqual;
                        }

                        AstNodeOperatorType::LessThanOrEqual => {
                            binary_instruction.binary_operator = BinaryOperator::LessThanOrEqual;
                        }

                        AstNodeOperatorType::GreaterThanOrEqual => {
                            binary_instruction.binary_operator = BinaryOperator::GreaterThanOrEqual;
                        }

                        _ => {
                            panic!("{}", format!("Unhandled OperatorType {:?}!\n", operator.operator_type).as_str());
                        }
                    }
                }

                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(binary_instruction));

                return ValueElement::Variable(String::from(dst_name.to_string()))
            }

            AstNodeType::ConstInt => {
                return ValueElement::Constant(ast_node.string_val.clone());
            }

            AstNodeType::If => {

                // expression
                let mut exp_result_var_name = String::from("");
                if let Some(lhs) = ast_node.expression.as_ref() {

                    exp_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    let lhs_value_element = self.visit(&lhs, &exp_result_var_name, branch_counter);

                    // println!("{:?}", lhs_value_element);

                    match lhs_value_element {
                        ValueElement::Variable(variable_identifier) => {
                            exp_result_var_name = variable_identifier;
                        }

                        ValueElement::Constant(constant_value) => {
                            // do nothing
                        }

                        ValueElement::None => {
                            // do nothing
                        }
                    }
                }

                // JumpIfZero
                let mut jump_if_zero_instruction: Instruction = Instruction::new();
                jump_if_zero_instruction.instruction_type = InstructionType::JumpIfZero;
                jump_if_zero_instruction.src = ValueElement::Variable(exp_result_var_name);

                if ast_node.rhs.is_none() {
                    jump_if_zero_instruction.label = self.final_end_label(&ast_node, "if_");
                } else {
                    jump_if_zero_instruction.label = self.new_end_label(&ast_node, "if_", branch_counter);
                }

                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(jump_if_zero_instruction));

                // LHS - if-branch statement / body
                let mut stmt_result_var_name = String::from("");
                if let Some(lhs) = ast_node.lhs.as_ref() {

                    stmt_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    let lhs_value_element = self.visit(&lhs, &stmt_result_var_name, branch_counter);

                    if !ast_node.rhs.is_none() {

                        // if the branch was taken, unconditionally jump to end_label -- Jump(end)
                        // unless there is no else branch in which case, no jump is necessary
                        let mut jump_instruction: Instruction = Instruction::new();
                        jump_instruction.instruction_type = InstructionType::Jump;
                        // jump_instruction.label = String::from("end_label");
                        jump_instruction.label = self.final_end_label(&ast_node, "if_");
                        // jump_instruction.label = self.new_end_label(/*&mut*/ branch_counter);
                        // append instruction to latest top-level element of the program
                        let last = self.program.top_level.len() - 1;
                        self.program.top_level[last].body.push(Box::new(jump_instruction));

                        // Label
                        let mut label_instruction: Instruction = Instruction::new();
                        label_instruction.instruction_type = InstructionType::Label;
                        // label_instruction.label = String::from("end_label");
                        label_instruction.label = self.new_end_label(/*&mut*/ &ast_node, "if_", branch_counter);

                        // append instruction to latest top-level element of the program
                        let last = self.program.top_level.len() - 1;
                        self.program.top_level[last].body.push(Box::new(label_instruction));
                    }
                }

                // RHS - else-branch statement / body
                if let Some(rhs) = ast_node.rhs.as_ref() {

                    // // Jump(end)
                    // let mut jump_instruction: Instruction = Instruction::new();
                    // jump_instruction.instruction_type = InstructionType::Jump;
                    // // jump_instruction.label = String::from("end_label");
                    // jump_instruction.label = self.new_end_label(/*&mut*/ branch_counter);
                    // // append instruction to latest top-level element of the program
                    // let last = self.program.top_level.len() - 1;
                    // self.program.top_level[last].body.push(Box::new(jump_instruction));

                    *branch_counter = *branch_counter + 1;

                    // RHS
                    // stmt_result_var_name = self.new_temp_var();
                    stmt_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    // let mut br_cnt = 0;
                    let rhs_value_element = self.visit(&rhs, &stmt_result_var_name, /*&mut*/ branch_counter);
                    // binary_instruction.src_2 = rhs_value_element;

                    // // if the branch was taken, unconditionally jump to end_label -- Jump(end)
                    // let mut jump_instruction: Instruction = Instruction::new();
                    // jump_instruction.instruction_type = InstructionType::Jump;
                    // // jump_instruction.label = String::from("end_label");
                    // jump_instruction.label = self.final_end_label(&ast_node);
                    // // jump_instruction.label = self.new_end_label(/*&mut*/ branch_counter);
                    // // append instruction to latest top-level element of the program
                    // let last = self.program.top_level.len() - 1;
                    // self.program.top_level[last].body.push(Box::new(jump_instruction));
                }

                // only output the end label once
                // if output_end_label {
                    // Label
                    let mut label_instruction: Instruction = Instruction::new();
                    label_instruction.instruction_type = InstructionType::Label;
                    // label_instruction.label = String::from("end_label");
                    label_instruction.label = self.final_end_label(&ast_node, "if_");
                    // label_instruction.label = self.new_end_label(/*&mut*/ branch_counter);

                    // append instruction to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(label_instruction));
                // }

                return ValueElement::Constant(ast_node.string_val.clone());
            }
            
            AstNodeType::While => {

                // Label<continue_label>
                //
                //     -- condition
                //     <instructions for condition>
                //     v = <result of condition>       // There is no TACKY definition for an assignment! How is this assignment defined?
                //     JumpIfNotZero(v, break_label)
                //
                //     -- body
                //     <instructions for body>
                //     Jump(continue_label)
                //
                // Label<break_label>

                let continue_label_name = self.continue_label(&ast_node, "while_");

                // continue label
                let mut label_instruction: Instruction = Instruction::new();
                label_instruction.instruction_type = InstructionType::Label;
                label_instruction.label = continue_label_name.clone();
                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(label_instruction));

                let mut binary_instruction: Instruction = Instruction::new();
                binary_instruction.instruction_type = InstructionType::Binary;
                binary_instruction.dst = ValueElement::Variable(dst_name.to_string());

                // LHS
                if let Some(lhs) = ast_node.lhs.as_ref() {

                    // let temp_var_name = self.new_temp_var();
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let lhs_value_element = self.visit(&lhs, &temp_var_name, &mut br_cnt);

                    // JumpIfZero
                    let mut jump_if_zero_instruction: Instruction = Instruction::new();
                    jump_if_zero_instruction.instruction_type = InstructionType::JumpIfZero;
                    jump_if_zero_instruction.src = lhs_value_element;

                    jump_if_zero_instruction.label = self.final_end_label(&ast_node, "while_");

                    // append instruction to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(jump_if_zero_instruction));
                }

                // RHS - body/statement
                if let Some(rhs) = ast_node.rhs.as_ref() {

                    // let temp_var_name = self.new_temp_var();
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let rhs_value_element = self.visit(&rhs, &temp_var_name, &mut br_cnt);
                    // binary_instruction.src = rhs_value_element;

                    // // append instruction to latest top-level element of the program
                    // let last = self.program.top_level.len() - 1;
                    // self.program.top_level[last].body.push(Box::new(binary_instruction));
                }

                // // expression
                // let mut exp_result_var_name = String::from("");
                // if let Some(lhs) = ast_node.expression.as_ref() {
                //     exp_result_var_name = self.new_temp_var();

                //     let lhs_value_element = self.visit(&lhs, &exp_result_var_name, branch_counter);
                //     binary_instruction.src_2 = lhs_value_element;
                // }

                // Jump to continue label
                let mut jump_instruction: Instruction = Instruction::new();
                jump_instruction.instruction_type = InstructionType::Jump;
                jump_instruction.label = continue_label_name.clone();
                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(jump_instruction));                

                // end label
                let mut label_instruction: Instruction = Instruction::new();
                label_instruction.instruction_type = InstructionType::Label;
                label_instruction.label = self.final_end_label(&ast_node, "while_");
                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(label_instruction));
            }
            
            AstNodeType::For => {

                //     -- init
                //     <instructions for init>
                //
                // Label(start)
                //
                //     -- condition
                //     <instructions for condition>
                //     v = <result of condition>       // There is no TACKY definition for an assignment! How is this assignment defined?
                //     JumpIfNotZero(v, break_label)
                //
                //     -- body
                //     <instructions for body>
                //
                // Label<continue_label>
                //
                //     -- post
                //     <instructions for post>
                //
                //     Jump(start)
                //
                // Label<break_label>

                let start_label_name = self.start_label(&ast_node, "for_");
                let final_end_label_name = self.final_end_label(&ast_node, "for_");

                //
                // LHS - initialization, e.g.: a = 0
                //

                if let Some(lhs) = ast_node.lhs.as_ref() {

                    // println!("{:?}", lhs);

                    // let temp_var_name = self.new_temp_var();
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let lhs_value_element = self.visit(&lhs, &temp_var_name, &mut br_cnt);
                    // // append instruction to latest top-level element of the program
                    // let last = self.program.top_level.len() - 1;
                    // self.program.top_level[last].body.push(Box::new(binary_instruction));
                }

                // start label
                let mut label_instruction: Instruction = Instruction::new();
                label_instruction.instruction_type = InstructionType::Label;
                label_instruction.label = start_label_name.clone();

                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(label_instruction));

                //
                // expression - condition, e.g. a < 10
                //

                // expression
                let mut exp_result_var_name = String::from("");
                if let Some(lhs) = ast_node.expression.as_ref() {
                    exp_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    let lhs_value_element = self.visit(&lhs, &exp_result_var_name, branch_counter);
                }

                // JumpIfZero
                let mut jump_if_zero_instruction: Instruction = Instruction::new();
                jump_if_zero_instruction.instruction_type = InstructionType::JumpIfZero;
                jump_if_zero_instruction.src = ValueElement::Variable(exp_result_var_name); // Variable
                jump_if_zero_instruction.label = final_end_label_name.clone();

                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(jump_if_zero_instruction));

                //
                // Body Statement - block_items
                //

                for i in 0..ast_node.block_items.len() {
                    let mut br_cnt = 0;
                    self.visit(&ast_node.block_items[ast_node.block_items.len()-1-i], &String::from(""), &mut br_cnt);
                }

                // continue label
                let mut label_instruction: Instruction = Instruction::new();
                label_instruction.instruction_type = InstructionType::Label;
                label_instruction.label = self.continue_label(&ast_node, "for_");
                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(label_instruction));

                //
                // RHS - post: e.g.: a = a + 1
                //

                if let Some(rhs) = ast_node.rhs.as_ref() {

                    // println!("{:?}", rhs);

                    // let temp_var_name = self.new_temp_var();
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let rhs_value_element = self.visit(&rhs, &temp_var_name, &mut br_cnt);

                    // // append instruction to latest top-level element of the program
                    // let last = self.program.top_level.len() - 1;
                    // self.program.top_level[last].body.push(Box::new(binary_instruction));
                }

                // Jump
                let mut jump_instruction: Instruction = Instruction::new();
                jump_instruction.instruction_type = InstructionType::Jump;
                jump_instruction.label = start_label_name.clone();
                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(jump_instruction));

                // end/break label
                let mut label_instruction: Instruction = Instruction::new();
                label_instruction.instruction_type = InstructionType::Label;
                label_instruction.label = final_end_label_name.clone();
                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(label_instruction));

            }

            AstNodeType::Identifier => {
                // println!("{:?}", ast_node);

                // original
                // return ValueElement::Variable(ast_node.string_val.clone());

                //let variable_identifier = self.variable_naming_source.borrow_mut().get_replaced_variable_name(&ast_node.string_val);
                let variable_identifier = ast_node.string_val.clone();

                // // DEBUG
                // println!("{:?}", variable_identifier);
                
                return ValueElement::Variable(variable_identifier);
            }
            
            AstNodeType::VariableDeclaration => {

                // In TACKY, variables are not declared. It is allowed to use variables without declaring them.
                // Only if a variable is initialized, this is output into TACKY as a variable assignment.
                //
                // This implementation will output a variable declaration although it is not strictly required!
                //
                // Nora Sandler, page 110: "[..] we can discard variable declarations at this stage;
                // in TACKY, you don't need to declare variables before using them. 
                // But we do need to emit TACKY to initialize varibles.
                // If a declaration includes an initializer, we'll handle it like a normal variable assignment. 
                // If a declaration doesn't have an initializer, we won't amit any TACKY at all."

                // println!("{:?}", ast_node);

                //
                // Create variable declaration
                //

                let mut var_declaration: Instruction = Instruction::new();
                var_declaration.instruction_type = InstructionType::VariableDeclaration;

                // LHS - data type
                if let Some(left_node) = ast_node.lhs.as_ref() {
                    // print!("{:?}", left_node);
                    var_declaration.data_type = left_node.string_val.clone();
                }

                // RHS - identifier
                let mut variable_identifier = String::from("ERROR");
                if let Some(right_node) = ast_node.rhs.as_ref() {
                    // print!("{:?}", right_node);

                    // variable_identifier = self.variable_naming_source.borrow_mut().get_replaced_variable_name(&right_node.string_val);
                    variable_identifier = right_node.string_val.clone();

                    // // DEBUG
                    // println!("{:?}", variable_identifier);

                    var_declaration.label = variable_identifier.clone();                    
                }

                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(var_declaration));

                //
                // Initializer
                //

                // expression
                if let Some(expression_node) = ast_node.expression.as_ref() {

                    // // DEBUG
                    // print!("{:?}", expression_node);

                    match expression_node.node_type {

                        AstNodeType::SingleInit => {

                            // // DEBUG
                            // println!("SingleInit");

                            if let Some(left_node) = expression_node.lhs.as_ref() {
                                // print!("{:?}", left_node);
                                // var_declaration.data_type = left_node.string_val.clone();

                                // Copy
                                let mut copy_instruction: Instruction = Instruction::new();
                                copy_instruction.instruction_type = InstructionType::Copy;
                                match &left_node.node_type {

                                    AstNodeType::Expression => {
                                        let mut br_cnt = 0;
                                        copy_instruction.src = self.visit(&left_node, &variable_identifier, &mut br_cnt);
                                    }

                                    _ => {
                                        panic!("Unhandled type: {:?}", left_node.node_type);
                                    }
                                }

                                copy_instruction.dst = ValueElement::Variable(variable_identifier);

                                // append instruction to latest top-level element of the program
                                let last = self.program.top_level.len() - 1;
                                self.program.top_level[last].body.push(Box::new(copy_instruction));
                            }

                        }

                        AstNodeType::CompoundInit => {
                            panic!("test");
                        }

                        _ => {
                            panic!("test");
                        }
                        
                    }

                    // Copy
                }
            }

            AstNodeType::DataType => {
                // println!("{:?}", ast_node);
            }

            AstNodeType::StructureDeclaration => {
                // println!("{:?}", ast_node);
            }

            AstNodeType::EmptyStatement => {
                // println!("{:?}", ast_node);
            }

            _ => {
                panic!("{}", format!("Unhandled NodeType {:?}!\n", ast_node.node_type).as_str());
            }
        }

        //ValueElement::Constant(String::from("unhandled"))
        ValueElement::None
    }
}



// #[derive(Debug, PartialEq)]
// pub enum AstNodeType {
//     Program,
//     ConstInt,
//     ConstLong,
//     ConstUInt,
//     ConstULong,
//     ConstDouble,
//     Structure,
//     Array,
//     Expression,
//     Identifier,
//     Return,
//     If,
//     Unary,
//     Binary,
//     Operator,
//     PrefixOperator,
//     DataType,
//     Declaration, // variable declaration or function declaration
//     FunctionDeclaration,
//     VariableDeclaration,
//     StructureDeclaration,
//     ParameterDeclaration,
//     Statement,
//     Block,
//     BlockItem,
//     Conditional, // elvis operator
//     Compound,
//     While,
//     DoWhile,
//     For,
//     FunctionCall,
//     StorageClassSpecifier,
//     Pointer,
//     Switch,
//     Case,
//     Default,
//     Break,
//     Continue,
//     EmptyStatement,
//     SingleInit,
//     CompoundInit,
//     Subscript,
//     MemberDeclaration,
//     Dot,
//     Arrow,
//     AssignmentOperator,
//     Unknown,
// }


// println!("id: {}", ast_node.node_id);
                // if let Some(compound) = ast_node.lhs.as_ref() {
                //     if let Some(left_node) = compound.lhs.as_ref() {
                //         self.visit(&left_node);
                //     }
                // }


                
                    // match return_value {

                    //     Constant(string_value) => {

                    //     }

                    //     Variable(string_value) => {
                    //         return_instruction.src = ValueElement::Variable(dst_name);
                    //     }

                    //     None => {

                    //     }
                    // }




                    

                // if let Some(sub) = ast_node.lhs.as_ref() {

                //     // // match sub.expression_type {
                //     // match sub.operator_type {

                //     //     OperatorType::LessThan => {
                //     //         println!("LessThan");
                //     //     }

                //     //     OperatorType::Assignment => {
                //     //         println!("Assignment");
                //     //     }

                //     //     OperatorType::NotApplicable => {
                //     //         println!("NotApplicable");
                //     //     }

                //     //     _ => {
                //     //         panic!("Test");
                //     //     }
                //     // }

                //     let mut br_cnt = 0;
                //     self.visit(&sub, &dst_name, &mut br_cnt);
                // }


                // let mut branch_counter:usize = 0;
                // self.if_index = self.if_index + 1;

                // let mut output_end_label:bool = false;
                // if *branch_counter == 0 {
                //     output_end_label = true;
                // }

                // let mut binary_instruction: Instruction = Instruction::new();
                // binary_instruction.instruction_type = InstructionType::Binary;
                // binary_instruction.dst = ValueElement::Variable(dst_name.to_string());


                

    // pub fn new_temp_var(&mut self) -> String {

    //     let temp = TEMP_VAR_COUNTER.fetch_add(1, Ordering::SeqCst);

    //     let mut t = String::from("tmp.");
    //     t.push_str(temp.to_string().as_str());

    //     t
    // }



    // println!("{:?}", lhs);

                    // match lhs.node_type {

                    //     AstNodeType::Identifier => {
                    //         // simple variable
                    //         exp_result_var_name = self.variable_naming_source.borrow_mut().get_replaced_variable_name(&lhs.string_val);
                    //         let lhs_value_element = self.visit(&lhs, &exp_result_var_name, branch_counter);
                    //     }

                    //     _ => {
                    //         exp_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    //         let lhs_value_element = self.visit(&lhs, &exp_result_var_name, branch_counter);
                    //     }
                    // }

                    // let mut temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    // let mut br_cnt = 0;
                    // let rhs_value_element = self.visit(&rhs, &temp_var_name, &mut br_cnt);

                    // println!("{:?}", rhs_value_element);

                    // match rhs_value_element {
                    //     ValueElement::Variable(variable_name) => {
                    //         // resolve unique name for variable name
                    //         temp_var_name = self.variable_naming_source.borrow_mut().get_replaced_variable_name(&variable_name);
                    //     }
                    //     _ => {
                    //         todo!();
                    //     }
                    // }

                    // //unary_instruction.src = rhs_value_element;
                    // unary_instruction.src = ValueElement::Variable(temp_var_name);