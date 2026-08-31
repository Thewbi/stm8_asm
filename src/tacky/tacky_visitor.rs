use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::str::FromStr;

use std::sync::atomic::AtomicUsize;
use crate::Ordering;

use crate::common::data_type::DataType;

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
use crate::tacky::tacky::Argument;

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

    pub fn visit(&mut self,
        ast_node: &AstNode,
        dst_name: &String,
        branch_counter: &mut usize,
        node_map: &HashMap::<usize, AstNode>) -> ValueElement {

        match &ast_node.node_type {

            AstNodeType::Program => {
                // visit all items in the body of the program
                for i in 0..ast_node.block_items.len() {
                    let mut br_cnt = 0;
                    let block_item_id = &ast_node.block_items[ast_node.block_items.len()-1-i];
                    let ast_node = node_map.get(block_item_id).unwrap();
                    self.visit(&ast_node, &String::from(""), &mut br_cnt, node_map);
                }
            }

            AstNodeType::FunctionDeclaration => {

                // DEBUG
                // println!("{:?}", ast_node);

                let mut function_name = String::new();
                if let Some(function_name_ast_node_id) = ast_node.function_name_ast_node.as_ref() {
                    let function_name_ast_node = node_map.get(function_name_ast_node_id).unwrap();
                    function_name = function_name_ast_node.string_val.clone();
                }

                // there should be a non-empty name for the function to call
                assert!(function_name.len() > 0);

                // TACKY top level unit (StaticVariable, StaticConstant or Function)
                let mut top_level_function: TopLevel = TopLevel::new();
                top_level_function.name = String::from(function_name.clone());
                top_level_function.top_level_type = TopLevelType::Function;
                top_level_function.global = true;

                // return type
                if let Some(data_type_ast_node_id) = ast_node.rhs.as_ref() {

                    // DEBUG
                    // println!("{:?}", data_type);

                    // DataType {
                    // DataTypeByte,
                    // DataTypeChar,
                    // DataTypeInt,
                    // DataTypeShort,
                    // DataTypeLong,
                    // DataTypeFloat,
                    // DataTypeDouble,

                    // DataTypeVoid, // is it beneficial to treat void as a data type? (void-pointer?)

                    // DataTypeUnknown,

                    let data_type_ast_node = node_map.get(data_type_ast_node_id).unwrap();
                    match data_type_ast_node.node_type {

                        AstNodeType::DataType => {

                            top_level_function.return_type = Some(DataType::from_str(&data_type_ast_node.string_val).expect("REASON"));

                            // match data_type.string_val.as_str() {
                            //     "int" => {
                            //         top_level_function.return_type = Some(DataType::DataTypeInt);
                            //     }
                            //     _ => {
                            //         todo!();
                            //     }
                            // }
                            //top_level_function.return_type = Some(data_type.as_ref().clone());
                            //top_level_function.return_type = data_type.clone();
                        },
                        _ => {

                        }
                    }

                }
                // match ast_node.rhs {
                //     _ => {
                //         println!("{:?}", ast_node.rhs);
                //         top_level_function.return_type = ast_node.rhs;
                //         panic!("test");
                //     }
                // }
                // top_level_function.return_type = Some(ast_node.rhs);

                // arguments
                for param_ast_node_id in &ast_node.parameters {

                    let param_ast_node = node_map.get(param_ast_node_id).unwrap();

                    // // DEBUG
                    // println!("{:?}", param);

                    let mut argument = Argument::new();

                    // name
                    if let Some(left_ast_node_id) = param_ast_node.lhs.as_ref() {

                        let left_ast_node = node_map.get(left_ast_node_id).unwrap();

                        // DEBUG
                        // print!("{:?}", left_node);

                        argument.name = left_ast_node.string_val.clone();
                    }
                    // data type
                    if let Some(right_ast_node_id) = param_ast_node.rhs.as_ref() {

                        let right_ast_node = node_map.get(right_ast_node_id).unwrap();

                        // DEBUG
                        // print!("{:?}", right_node);

                        argument.data_type = DataType::from_str(&right_ast_node.string_val).expect(format!("Type is not known: {}\n", &right_ast_node.string_val).as_str());
                    }

                    top_level_function.arguments.push(Box::new(argument));
                }

                // insert function into program
                self.program.top_level.push(Box::new(top_level_function));

                // add instructions and declarations into body/block
                if let Some(block_ast_node_id) = ast_node.lhs.as_ref() {

                    let block_ast_node = node_map.get(block_ast_node_id).unwrap();

                    for i in 0..block_ast_node.block_items.len() {

                        let destination_var_name = String::from("");
                        let mut br_cnt = 0;
                        let block_item_ast_node_id = &block_ast_node.block_items[block_ast_node.block_items.len()-1-i];
                        let block_item_ast_node = node_map.get(block_item_ast_node_id).unwrap();

                        self.visit(&block_item_ast_node, &destination_var_name, &mut br_cnt, node_map);
                    }

                    // add return

                    let last_block_item_ast_node_id = &block_ast_node.block_items[0];
                    let last_block_item_ast_node = node_map.get(last_block_item_ast_node_id).unwrap();

                    //println!("block.block_items.len(): {}, last_block_item: {:?}", block.block_items.len(), last_block_item);

                    if let Some(left_block_item_id) = last_block_item_ast_node.lhs.as_ref() {

                        let left_block_item_ast_node = node_map.get(left_block_item_id).unwrap();

                        if let Some(statement_ast_node_id) = left_block_item_ast_node.lhs {

                            let statement_ast_node = node_map.get(&statement_ast_node_id).unwrap();

                            // DEBUG
                            // println!("test: {:?}", statement.node_type);

                            if statement_ast_node.node_type != AstNodeType::Return {
                                // panic!("Cannot compile function without ret!");

                                let return_value: ValueElement = ValueElement::Constant(String::from("0"));

                                let mut return_instruction: Instruction = Instruction::new();
                                return_instruction.instruction_type = InstructionType::Return;
                                return_instruction.src = return_value;

                                // append instruction to latest top-level element of the program
                                let last = self.program.top_level.len() - 1;
                                self.program.top_level[last].body.push(Box::new(return_instruction));
                            }
                        }
                    }
                }
            }

            AstNodeType::Block => {
                // println!("id: {}", ast_node.node_id);
                for i in 0..ast_node.block_items.len() {
                    let mut br_cnt = 0;

                    let id = ast_node.block_items.len()-1-i;
                    let ast_node_id = ast_node.block_items[id];

                    let ast_node = node_map.get(&ast_node_id).unwrap();

                    self.visit(&ast_node, &String::from(""), &mut br_cnt, node_map);
                }
            }

            AstNodeType::BlockItem => {
                // println!("id: {}", ast_node.node_id);
                if let Some(compound_id) = ast_node.lhs {
                    let compound_ast_node = node_map.get(&compound_id).unwrap();
                    if let Some(left_node_id) = compound_ast_node.lhs {
                        let left_ast_node = node_map.get(&left_node_id).unwrap();
                        let mut br_cnt = 0;
                        self.visit(&left_ast_node, &String::from(""), &mut br_cnt, node_map);
                    }
                }
            }

            AstNodeType::Statement => {
                if let Some(sub_id) = ast_node.lhs {
                    let ast_node = node_map.get(&sub_id).unwrap();
                    self.visit(&ast_node, &dst_name, branch_counter, node_map);
                }
            }

            AstNodeType::Compound => {
                if let Some(sub_id) = ast_node.lhs.as_ref() {
                    let ast_node = node_map.get(&sub_id).unwrap();
                    self.visit(&ast_node, &dst_name, branch_counter, node_map);
                }
            }

            AstNodeType::Expression => {

                match ast_node.operator_type {

                    AstNodeOperatorType::Assignment => {
                        // println!("");
                        // println!("Assignment +++++++++++++++++++++++++++");
                        // println!("Assignment: {:?}", ast_node);
                        // println!("Assignment ---------------------------");
                        // println!("");

                        if let Some(lhs_subnode_id) = ast_node.lhs {
                            // println!("LHS: {:?}", lhs_subnode);
                            let lhs_subnode = node_map.get(&lhs_subnode_id).unwrap();

                            match lhs_subnode.operator_type {

                                // Pointers: LHS is a dereference pointer.
                                // This means the pointer is not lvalue converted to a value but it is
                                // converted to an object and the object gets a value assigned.
                                // This is implemented using TACKY Store()
                                AstNodeOperatorType::Dereference => {
                                    // panic!("Dereference");

                                    // TACKY Store
                                    let mut store_instruction: Instruction = Instruction::new();
                                    store_instruction.instruction_type = InstructionType::Store;

                                    // dst
                                    //let mut br_cnt = 0;
                                    //store_instruction.dst = self.visit(&lhs_subnode, &dst_name, &mut br_cnt);
                                    store_instruction.dst = ValueElement::Variable(String::from(lhs_subnode.string_val.clone()));

                                    // src
                                    if let Some(rhs_sub_id) = ast_node.rhs {
                                        let rhs_sub = node_map.get(&rhs_sub_id).unwrap();
                                        // println!("RHS: {:?}", rhs_sub);
                                        let mut br_cnt = 0;
                                        store_instruction.src = self.visit(&rhs_sub, &dst_name, &mut br_cnt, node_map);
                                    }

                                    // append instruction to latest top-level element of the program
                                    let last = self.program.top_level.len() - 1;
                                    self.program.top_level[last].body.push(Box::new(store_instruction));
                                }

                                _ => {

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

                                    if let Some(lhs_sub_id) = ast_node.lhs {
                                        let lhs_sub = node_map.get(&lhs_sub_id).unwrap();
                                        // println!("LHS: {:?}", lhs_sub);

                                        dst_name = lhs_sub.string_val.clone();
                                        copy_instruction.dst = ValueElement::Variable(dst_name.clone());
                                    }

                                    if let Some(rhs_sub_id) = ast_node.rhs {
                                        // println!("RHS: {:?}", rhs_sub);

                                        let rhs_sub = node_map.get(&rhs_sub_id).unwrap();

                                        let mut br_cnt = 0;
                                        copy_instruction.src = self.visit(&rhs_sub, &dst_name, &mut br_cnt, node_map);

                                        // determine if the copy instruction is output or not
                                        match rhs_sub.node_type {

                                            AstNodeType::Binary => {
                                                // DEBUG
                                                // println!("binary");
                                                output_copy_instruction = false;
                                            }

                                            AstNodeType::Expression => {
                                                // DEBUG
                                                // println!("binary");
                                                output_copy_instruction = false;
                                            }

                                            _ => {
                                                println!("NodeType: {:?}", rhs_sub.node_type);
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
                            }
                        }
                    }

                    AstNodeOperatorType::FunctionCall => {

                        // println!("{:?}", ast_node);

                        // FunCall
                        let mut function_call_instruction: Instruction = Instruction::new();
                        function_call_instruction.instruction_type = InstructionType::FunCall;

                        if let Some(lhs_ast_node_id) = ast_node.lhs {

                            let lhs_ast_node = node_map.get(&lhs_ast_node_id).unwrap();

                            function_call_instruction.function_name = lhs_ast_node.string_val.clone();

                            // PARAMETERS / ARGUMENTS
                            for i in 0..lhs_ast_node.parameters.len() {

                                let parameter_ast_node_id = &lhs_ast_node.parameters[lhs_ast_node.parameters.len()-1-i];
                                let parameter_ast_node = node_map.get(&parameter_ast_node_id).unwrap();

                                // DEBUG
                                // println!("ARGUMENT_{}: {:?}", i, parameter_ast_node);

                                match parameter_ast_node.node_type {

                                    AstNodeType::ConstInt | AstNodeType::ConstLong => {
                                        function_call_instruction.parameters.push(Box::new(ValueElement::Constant(String::from(parameter_ast_node.string_val.clone()))));
                                    }

                                    _ => {
                                        panic!("Please add node_type!");
                                    }

                                }
                            }
                        }

                        function_call_instruction.dst = ValueElement::Variable(dst_name.clone());

                        // append instruction to latest top-level element of the program
                        let last = self.program.top_level.len() - 1;
                        self.program.top_level[last].body.push(Box::new(function_call_instruction));

                        let result = ValueElement::Variable(dst_name.clone());

                        return result;
                    }

                    AstNodeOperatorType::NotApplicable => {
                        // DEBUG
                        // println!("NotApplicable");
                        if let Some(sub_id) = ast_node.lhs {
                            let sub_ast_node = node_map.get(&sub_id).unwrap();
                            let mut br_cnt = 0;
                            return self.visit(&sub_ast_node, &dst_name, &mut br_cnt, node_map);
                        }
                    }

                    AstNodeOperatorType::Cast => {
                        // DEBUG
                        println!("{:?}", ast_node);
                    }

                    _ => {
                        panic!("{}", format!("Unhandled AstNodeOperatorType {:?}!\n", ast_node.operator_type).as_str());
                    }
                }
            }

            AstNodeType::Return => {

                if let Some(expression_id) = ast_node.lhs {

                    let expression = node_map.get(&expression_id).unwrap();

                    let dst_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    // DEBUG
                    // println!("{}", dst_name);

                    let mut br_cnt = 0;
                    let return_value: ValueElement = self.visit(&expression, &dst_name, &mut br_cnt, node_map);

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

                if let Some(operator_id) = ast_node.lhs {

                    let operator = node_map.get(&operator_id).unwrap();

                    match operator.operator_type {

                        AstNodeOperatorType::AddrOf => {

                            // println!("{:?}", ast_node);

                            // // DEBUG
                            // println!("{:?}", ast_node.lhs);
                            // println!("{:?}", ast_node.rhs);

                            let mut get_address_instruction: Instruction = Instruction::new();
                            get_address_instruction.instruction_type = InstructionType::GetAddress;
                            if let Some(rhs_node_id) = ast_node.rhs {
                                let rhs_node = node_map.get(&rhs_node_id).unwrap();
                                get_address_instruction.src = ValueElement::Variable(rhs_node.string_val.clone());
                            }
                            get_address_instruction.dst = ValueElement::Variable(dst_name.to_string());

                            // append instruction to latest top-level element of the program
                            let last = self.program.top_level.len() - 1;
                            self.program.top_level[last].body.push(Box::new(get_address_instruction));

                            return ValueElement::Variable(String::from(dst_name.to_string()));
                        }

                        AstNodeOperatorType::Dereference => {

                            // Nora Sandler, page 371, Listing 14-19

                            println!("{:?}", ast_node);

                            // DEBUG
                            println!("{:?}", ast_node.lhs);
                            println!("{:?}", ast_node.rhs);

                            let mut load_instruction: Instruction = Instruction::new();
                            load_instruction.instruction_type = InstructionType::Load;

                            if let Some(rhs_node_id) = ast_node.rhs {
                                let rhs_node = node_map.get(&rhs_node_id).unwrap();
                                load_instruction.src = ValueElement::Variable(rhs_node.string_val.clone());
                            }
                            load_instruction.dst = ValueElement::Variable(dst_name.to_string());

                            // append instruction to latest top-level element of the program
                            let last = self.program.top_level.len() - 1;
                            self.program.top_level[last].body.push(Box::new(load_instruction));

                            return ValueElement::Variable(String::from(dst_name.to_string()));
                        }

                        _ => {

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
                            if let Some(rhs_id) = ast_node.rhs {

                                let rhs = node_map.get(&rhs_id).unwrap();

                                let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                                let mut br_cnt = 0;
                                let rhs_value_element = self.visit(&rhs, &temp_var_name, &mut br_cnt, node_map);

                                // source and destination are the same thing
                                unary_instruction.src = rhs_value_element.clone();
                                unary_instruction.dst = rhs_value_element.clone();
                            }

                            //
                            // operator
                            //

                            if let Some(operator_id) = ast_node.lhs {

                                let operator = node_map.get(&operator_id).unwrap();

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

                                    AstNodeOperatorType::Dereference => {
                                        println!("Dereference {:?}", ast_node);
                                        unary_instruction.unary_operator = UnaryOperator::Dereference;
                                        unary_instruction.dst = ValueElement::Variable(dst_name.to_string());
                                    }

                                    AstNodeOperatorType::AddrOf => {
                                        // println!("AddrOf {:?}", ast_node);
                                        // unary_instruction.unary_operator = UnaryOperator::AddrOf;
                                        panic!("The addrof operator is turned into TACKY: GetAddress()");
                                    }

                                    _ => {
                                        panic!("{}", format!("Unhandled OperatorType '{:?}'!\n", operator.operator_type).as_str());
                                    }
                                }
                            }

                            // append instruction to latest top-level element of the program
                            let last = self.program.top_level.len() - 1;
                            self.program.top_level[last].body.push(Box::new(unary_instruction));

                            return ValueElement::Variable(String::from(dst_name.to_string()));
                        }
                    }
                }
            }

            AstNodeType::Binary => {

                let mut binary_instruction: Instruction = Instruction::new();
                binary_instruction.instruction_type = InstructionType::Binary;
                binary_instruction.dst = ValueElement::Variable(dst_name.to_string());

                // LHS
                if let Some(lhs_id) = ast_node.lhs {
                    let lhs = node_map.get(&lhs_id).unwrap();

                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let lhs_value_element = self.visit(&lhs, &temp_var_name, &mut br_cnt, node_map);
                    binary_instruction.src_2 = lhs_value_element;
                }

                // RHS
                if let Some(rhs_id) = ast_node.rhs {
                    let rhs = node_map.get(&rhs_id).unwrap();

                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let rhs_value_element = self.visit(&rhs, &temp_var_name, &mut br_cnt, node_map);
                    binary_instruction.src = rhs_value_element;
                }

                // operator
                if let Some(operator_id) = ast_node.operator {

                    let operator = node_map.get(&operator_id).unwrap();

                    // node_type.get(operator);

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

                        AstNodeOperatorType::And => {
                            binary_instruction.binary_operator = BinaryOperator::And;
                        }

                        AstNodeOperatorType::Or => {
                            binary_instruction.binary_operator = BinaryOperator::Or;
                        }

                        AstNodeOperatorType::Xor => {
                            binary_instruction.binary_operator = BinaryOperator::Xor;
                        }

                        AstNodeOperatorType::LeftShift => {
                            binary_instruction.binary_operator = BinaryOperator::LeftShift;
                        }

                        AstNodeOperatorType::RightShift => {
                            binary_instruction.binary_operator = BinaryOperator::RightShift;
                        }

                        AstNodeOperatorType::LogicalAnd => {
                            binary_instruction.binary_operator = BinaryOperator::LogicalAnd;
                        }

                        AstNodeOperatorType::LogicalOr => {
                            binary_instruction.binary_operator = BinaryOperator::LogicalOr;
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

            AstNodeType::ConstInt |
            AstNodeType::ConstLong |
            AstNodeType::ConstUInt |
            AstNodeType::ConstULong |
            AstNodeType::ConstDouble => {
                return ValueElement::Constant(ast_node.string_val.clone());
            }

            AstNodeType::If => {

                // expression
                let mut exp_result_var_name = String::from("");
                if let Some(lhs_id) = ast_node.expression {

                    let lhs = node_map.get(&lhs_id).unwrap();

                    exp_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    let lhs_value_element = self.visit(&lhs, &exp_result_var_name, branch_counter, node_map);

                    // println!("{:?}", lhs_value_element);

                    match lhs_value_element {
                        ValueElement::Variable(variable_identifier) => {
                            exp_result_var_name = variable_identifier;
                        }

                        ValueElement::Constant(constant_value) => {
                            // do nothing
                        }

                        ValueElement::Cast(src_type, dst_type) => {
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
                if let Some(lhs_id) = ast_node.lhs {

                    let lhs = node_map.get(&lhs_id).unwrap();

                    stmt_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    let lhs_value_element = self.visit(&lhs, &stmt_result_var_name, branch_counter, node_map);

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
                if let Some(rhs_id) = ast_node.rhs {

                    let rhs = node_map.get(&rhs_id).unwrap();

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
                    let rhs_value_element = self.visit(&rhs, &stmt_result_var_name, branch_counter, node_map);
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
                if let Some(lhs_id) = ast_node.lhs {

                    let lhs = node_map.get(&lhs_id).unwrap();

                    // let temp_var_name = self.new_temp_var();
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let lhs_value_element = self.visit(&lhs, &temp_var_name, &mut br_cnt, node_map);

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
                if let Some(rhs_id) = ast_node.rhs {

                    let rhs = node_map.get(&rhs_id).unwrap();

                    // let temp_var_name = self.new_temp_var();
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let rhs_value_element = self.visit(&rhs, &temp_var_name, &mut br_cnt, node_map);
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

                if let Some(lhs_id) = ast_node.lhs {

                    let lhs = node_map.get(&lhs_id).unwrap();

                    // println!("{:?}", lhs);

                    // let temp_var_name = self.new_temp_var();
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let lhs_value_element = self.visit(&lhs, &temp_var_name, &mut br_cnt, node_map);
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
                if let Some(lhs_id) = ast_node.expression {
                    let lhs = node_map.get(&lhs_id).unwrap();
                    exp_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    let lhs_value_element = self.visit(&lhs, &exp_result_var_name, branch_counter, node_map);
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

                    let block_item_id = ast_node.block_items[ast_node.block_items.len()-1-i];
                    let block_item = node_map.get(&block_item_id).unwrap();

                    let mut br_cnt = 0;
                    self.visit(block_item, &String::from(""), &mut br_cnt, node_map);
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

                if let Some(rhs_id) = ast_node.rhs {

                    let rhs = node_map.get(&rhs_id).unwrap();

                    // println!("{:?}", rhs);

                    // let temp_var_name = self.new_temp_var();
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let rhs_value_element = self.visit(&rhs, &temp_var_name, &mut br_cnt, node_map);

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

                // let variable_identifier = self.variable_naming_source.borrow_mut().get_replaced_variable_name(&ast_node.string_val);
                // return ValueElement::Variable(variable_identifier.expect("REASON"));

                let variable_identifier = ast_node.string_val.clone();
                return ValueElement::Variable(variable_identifier);

                // // DEBUG
                // println!("{:?}", variable_identifier);
            }

            AstNodeType::VariableDeclaration => {

                let node_as_string = ast_node.serialize(node_map);

                let mut comment_declaration: Instruction = Instruction::new();
                comment_declaration.instruction_type = InstructionType::Comment;
                // comment_declaration.label = "This is a comment".to_string();
                comment_declaration.label = node_as_string;

                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(comment_declaration));

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
                if let Some(left_node_id) = ast_node.lhs {
                    let left_node = node_map.get(&left_node_id).unwrap();
                    // print!("{:?}", left_node);
                    var_declaration.data_type = left_node.string_val.clone();
                }

                // RHS - identifier
                let mut variable_identifier = String::from("ERROR");
                if let Some(right_node_id) = ast_node.rhs {

                    let right_node = node_map.get(&right_node_id).unwrap();
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
                if let Some(expression_node_id) = ast_node.expression {

                    let expression_node = node_map.get(&expression_node_id).unwrap();

                    // // DEBUG
                    // print!("{:?}", expression_node);

                    match expression_node.node_type {

                        AstNodeType::SingleInit => {

                            // // DEBUG
                            // println!("SingleInit");

                            if let Some(left_node_id) = expression_node.lhs {
                                let left_node = node_map.get(&left_node_id).unwrap();

                                // print!("{:?}", left_node);
                                // var_declaration.data_type = left_node.string_val.clone();

                                // Copy
                                let mut copy_instruction: Instruction = Instruction::new();
                                copy_instruction.instruction_type = InstructionType::Copy;
                                match &left_node.node_type {
                                    AstNodeType::Expression => {
                                        let mut br_cnt = 0;
                                        copy_instruction.src = self.visit(&left_node, &variable_identifier, &mut br_cnt, node_map);
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

            AstNodeType::Cast => {
                // println!("{:?}", ast_node);
                // //return ValueElement::Cast(ast_node.string_val.clone());
                // binary_instruction.src_2 = lhs_value_element;

                // TACKY Cast
                let mut cast_instruction: Instruction = Instruction::new();
                cast_instruction.instruction_type = InstructionType::Cast;

                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(cast_instruction));

                // return cast_instruction;
                // String::from(dst_name.to_string())
                return ValueElement::Cast(String::from("type_dst"), String::from("type_src"));
            }

            AstNodeType::EmptyStatement => {
                // println!("{:?}", ast_node);
            }

            _ => {
                panic!("{}", format!("Unhandled NodeType '{:?}'!\n", ast_node.node_type).as_str());
            }
        }

        ValueElement::None
    }
}