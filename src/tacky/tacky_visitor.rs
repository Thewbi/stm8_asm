use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::str::FromStr;

use std::sync::atomic::AtomicUsize;
use crate::Ordering;

use crate::common::data_type;
use crate::common::data_type::DataType;

use crate::c_ast::ast_node::AstNode;
use crate::c_ast::ast_node::AstNodeType;
use crate::c_ast::ast_node::AstNodeOperatorType;

use crate::AstNodeType::Program as ASTProgram;
use crate::AstNodeType::FunctionDeclaration as ASTFunction;

use crate::Instruction;

use crate::common::symbol_table::SymbolTable;
use crate::common::symbol_table::SymbolTableEntry;
use crate::common::symbol_table::SymbolTableEntryType;
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
    variable_naming_source: Rc<RefCell<VariableNamingSource>>, // https://www.youtube.com/watch?v=8O0Nt9qY_vo
    symbol_table: Rc<RefCell<SymbolTable>>, // https://www.youtube.com/watch?v=8O0Nt9qY_vo
    pub program: Program,
    pub function_declaration: Option<Box<TopLevel>>,
    pub debug: bool,
}

impl TackyVisitor {

    pub fn new(
        variable_naming_source_param: Rc<RefCell<VariableNamingSource>>,
        symbol_table_param: Rc<RefCell<SymbolTable>>)
        -> TackyVisitor
    {
        TackyVisitor {
            variable_naming_source: variable_naming_source_param,
            symbol_table: symbol_table_param,
            program: Program::new(),
            function_declaration: None,
            debug: true,
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
        ast_node_id: usize,
        node_map: &mut Box<HashMap<usize, AstNode>>,
        dst_name: &String,
        branch_counter: &mut usize)
        -> ValueElement
    {

        let mut ast_node = AstNode::new(0);

        {
            ast_node = node_map.get(&ast_node_id).unwrap().clone();
        }

        match &ast_node.node_type {

            AstNodeType::Program => {
                // visit all items in the body of the program
                for i in 0..ast_node.block_items.len() {
                    let block_item_id = ast_node.block_items[ast_node.block_items.len()-1-i];
                    let mut br_cnt = 0;
                    self.visit(block_item_id, node_map, &String::from(""), &mut br_cnt);
                }
            }

            AstNodeType::FunctionDeclaration => {

                // DEBUG
                if self.debug {
                    println!("{:?}", ast_node);
                }

                let mut function_name = String::new();
                if let Some(function_name_ast_node_id) = ast_node.function_name_ast_node {
                    let function_name_ast_node = node_map.get(&function_name_ast_node_id).unwrap();
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
                if let Some(data_type_id) = ast_node.rhs {

                    let data_type = node_map.get(&data_type_id).unwrap();

                    // DEBUG
                    if self.debug {
                        println!("{:?}", data_type);
                    }

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

                    match data_type.node_type {
                        AstNodeType::DataType => {
                            top_level_function.return_type = Some(DataType::from_str(&data_type.string_val).expect("REASON"));
                        },
                        _ => {

                        }
                    }
                }

                // arguments
                for param_id in &ast_node.parameters {
                    // DEBUG
                    if self.debug {
                        println!("{:?}", param_id);
                    }
                    let param = node_map.get(&param_id).unwrap();
                    let mut argument = Argument::new();
                    // name
                    if let Some(left_node_id) = param.lhs {
                        // DEBUG
                        if self.debug {
                            print!("{:?}", left_node_id);
                        }
                        let left_node = node_map.get(&left_node_id).unwrap();
                        argument.name = left_node.string_val.clone();
                    }
                    // data type
                    if let Some(right_node_id) = param.rhs {
                        let right_node = node_map.get(&right_node_id).unwrap();
                        // DEBUG
                        if self.debug {
                            print!("{:?}", right_node);
                        }
                        argument.data_type = DataType::from_str(&right_node.string_val).expect(format!("Type is not known: {}\n", &right_node.string_val).as_str());
                    }
                    top_level_function.arguments.push(Box::new(argument));
                }

                // insert function into program
                self.program.top_level.push(Box::new(top_level_function));

                // add instructions and declarations into body/block
                if let Some(block_id) = ast_node.lhs {

                    let block = node_map.get(&block_id).unwrap().clone();
                    for i in 0..block.block_items.len() {

                        let destination_var_name = String::from("");
                        let mut br_cnt = 0;
                        let last_block_item_id = block.block_items[block.block_items.len()-1-i];

                        self.visit(last_block_item_id, node_map, &destination_var_name, &mut br_cnt);
                    }

                    let last_block_item_id = &block.block_items[0];
                    let last_block_item = node_map.get(&last_block_item_id).unwrap();

                    if self.debug {
                        println!("block.block_items.len(): {}, last_block_item: {:?}", block.block_items.len(), last_block_item);
                    }

                    if let Some(block_item_id) = last_block_item.lhs {

                        let block_item = node_map.get(&block_item_id).unwrap();
                        if let Some(statement_id) = block_item.lhs {

                            let statement = node_map.get(&statement_id).unwrap();

                            // DEBUG
                            if self.debug {
                                println!("test: {:?}", statement.node_type);
                            }

                            if statement.node_type != AstNodeType::Return {
                                // panic!("Cannot compile function without ret!");

                                let return_value: ValueElement = ValueElement::Constant(String::from("0"));

                                let mut return_instruction: Instruction = Instruction::new();
                                return_instruction.instruction_type = InstructionType::Return;
                                return_instruction.src = return_value;

                                // block.block_items.push(Box::new(return_instruction));

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
                    self.visit(ast_node.block_items[ast_node.block_items.len()-1-i], node_map, &String::from(""), &mut br_cnt);
                }
            }

            AstNodeType::BlockItem => {
                // println!("id: {}", ast_node.node_id);
                if let Some(compound_id) = ast_node.lhs {
                    let compound = node_map.get(&compound_id).unwrap();
                    if let Some(left_node_id) = compound.lhs {
                        let mut br_cnt = 0;
                        self.visit(left_node_id, node_map, &String::from(""), &mut br_cnt);
                    }
                }
            }

            AstNodeType::Statement => {
                if let Some(sub_id) = ast_node.lhs {
                    self.visit(sub_id, node_map, &dst_name, branch_counter);
                }
            }

            AstNodeType::Compound => {
                if let Some(sub_id) = ast_node.lhs {
                    self.visit(sub_id, node_map, &dst_name, branch_counter);
                }
            }

            AstNodeType::Expression => {

                match ast_node.operator_type {

                    AstNodeOperatorType::Assignment => {
                        // DEBUG
                        if self.debug {
                            println!("");
                            println!("Assignment +++++++++++++++++++++++++++");
                            println!("Assignment: {:?}", ast_node);
                            println!("Assignment ---------------------------");
                            println!("");
                        }

                        if let Some(lhs_subnode_id) = ast_node.lhs {
                            let lhs_subnode = node_map.get(&lhs_subnode_id).unwrap();

                            // DEBUG
                            if self.debug {
                                println!("LHS: {:?}", lhs_subnode);
                            }

                            match lhs_subnode.operator_type {

                                // Pointers: LHS is a dereference pointer.
                                // This means the pointer is not lvalue converted to a value but it is
                                // converted to an object and the object gets a value assigned.
                                // This is implemented using TACKY Store()
                                AstNodeOperatorType::Dereference => {

                                    // TACKY Store
                                    let mut store_instruction: Instruction = Instruction::new();
                                    store_instruction.instruction_type = InstructionType::Store;

                                    // dst
                                    store_instruction.dst = ValueElement::Variable(String::from(lhs_subnode.string_val.clone()));

                                    // src
                                    if let Some(rhs_sub) = ast_node.rhs {
                                        // DEBUG
                                        if self.debug {
                                            println!("RHS: {:?}", rhs_sub);
                                        }
                                        let mut br_cnt = 0;
                                        store_instruction.src = self.visit(rhs_sub, node_map, &dst_name, &mut br_cnt);
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
                                        if self.debug {
                                            println!("LHS: {:?}", lhs_sub);
                                        }
                                        dst_name = lhs_sub.string_val.clone();
                                        copy_instruction.dst = ValueElement::Variable(dst_name.clone());
                                    }

                                    if let Some(rhs_sub_id) = ast_node.rhs {

                                        let mut br_cnt = 0;
                                        copy_instruction.src = self.visit(rhs_sub_id, node_map, &dst_name, &mut br_cnt);

                                        let rhs_sub = node_map.get(&rhs_sub_id).unwrap();
                                        if self.debug {
                                            println!("RHS: {:?}", rhs_sub);
                                        }

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

                        // DEBUG
                        if self.debug {
                            println!("{:?}", ast_node);
                        }

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
                                if self.debug {
                                    println!("ARGUMENT_{}: {:?}", i, parameter_ast_node);
                                }

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
                        if self.debug {
                            println!("NotApplicable");
                        }
                        if let Some(sub_id) = ast_node.lhs {
                            let mut br_cnt = 0;
                            return self.visit(sub_id, node_map, &dst_name, &mut br_cnt);
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

                assert_ne!(ast_node.analyzed_data_type, DataType::DataTypeUnknown);

                if let Some(expression_id) = ast_node.lhs {

                    //
                    // Case: return with value
                    //

                    let dst_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    // DEBUG
                    if self.debug {
                        println!("{}", dst_name);
                    }

                    let mut br_cnt = 0;
                    let return_value: ValueElement = self.visit(expression_id, node_map, &dst_name, &mut br_cnt);

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
                    return_instruction.data_type = ast_node.analyzed_data_type;
                    return_instruction.src = return_value;

                    // append instruction to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(return_instruction));

                } else {

                    //
                    // Case: return without value
                    //

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

                            // DEBUG
                            if self.debug {
                                println!("{:?}", ast_node);
                                println!("{:?}", ast_node.lhs);
                                println!("{:?}", ast_node.rhs);
                            }

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

                                let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                                let mut br_cnt = 0;
                                let rhs_value_element = self.visit(rhs_id, node_map, &temp_var_name, &mut br_cnt);

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
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    let mut br_cnt = 0;
                    let lhs_value_element = self.visit(lhs_id, node_map, &temp_var_name, &mut br_cnt);
                    binary_instruction.src_2 = lhs_value_element;
                }

                // RHS
                if let Some(rhs_id) = ast_node.rhs {
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    let mut br_cnt = 0;
                    let rhs_value_element = self.visit(rhs_id, node_map, &temp_var_name, &mut br_cnt);
                    binary_instruction.src = rhs_value_element;
                }

                // operator
                if let Some(operator_id) = ast_node.operator {
                    let operator = node_map.get(&operator_id).unwrap();
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

                    exp_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    let lhs_value_element = self.visit(lhs_id, node_map, &exp_result_var_name, branch_counter);

                    // DEBUG
                    if self.debug {
                        println!("{:?}", lhs_value_element);
                    }

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
                if let Some(lhs_id) = ast_node.lhs {

                    stmt_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    let lhs_value_element = self.visit(lhs_id, node_map, &stmt_result_var_name, branch_counter);

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

                    *branch_counter = *branch_counter + 1;

                    // RHS
                    stmt_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    // let mut br_cnt = 0;
                    let rhs_value_element = self.visit(rhs_id, node_map, &stmt_result_var_name, branch_counter);
                }

                // Label
                let mut label_instruction: Instruction = Instruction::new();
                label_instruction.instruction_type = InstructionType::Label;
                label_instruction.label = self.final_end_label(&ast_node, "if_");

                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(label_instruction));

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

                    // let temp_var_name = self.new_temp_var();
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let lhs_value_element = self.visit(lhs_id, node_map, &temp_var_name, &mut br_cnt);

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
                    // let temp_var_name = self.new_temp_var();
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    let mut br_cnt = 0;
                    let rhs_value_element = self.visit(rhs_id, node_map, &temp_var_name, &mut br_cnt);
                }

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
                    // DEBUG
                    if self.debug {
                        println!("{:?}", lhs_id);
                    }
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    let mut br_cnt = 0;
                    let lhs_value_element = self.visit(lhs_id, node_map, &temp_var_name, &mut br_cnt);
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
                    exp_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    let lhs_value_element = self.visit(lhs_id, node_map, &exp_result_var_name, branch_counter);
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
                    self.visit(block_item_id, node_map,  &String::from(""), &mut br_cnt);
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

                    // println!("{:?}", rhs);

                    // let temp_var_name = self.new_temp_var();
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                    let mut br_cnt = 0;
                    let rhs_value_element = self.visit(rhs_id, node_map, &temp_var_name, &mut br_cnt);

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
                // // DEBUG
                // println!("{:?}", variable_identifier);
                let variable_identifier = ast_node.string_val.clone();
                return ValueElement::Variable(variable_identifier);
            }

            AstNodeType::VariableDeclaration => {
                let node_as_string = ast_node.serialize(node_map);

                // add a comment
                let mut comment_declaration: Instruction = Instruction::new();
                comment_declaration.instruction_type = InstructionType::Comment;
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

                // RHS - identifier
                let mut variable_identifier = String::from("ERROR");
                if let Some(right_node_id) = ast_node.rhs {
                    let right_node = node_map.get(&right_node_id).unwrap();
                    // DEBUG
                    if self.debug {
                        print!("{:?}", right_node);
                    }
                    variable_identifier = right_node.string_val.clone();
                    // DEBUG
                    if self.debug {
                        println!("{:?}", variable_identifier);
                    }
                }

                let symbol_table_entry = self.symbol_table.borrow_mut().get(&variable_identifier);

                // LHS - data type
                let mut data_type_as_string = String::from("ERROR");
                let mut left_node = &AstNode::new(0);
                if let Some(left_node_id) = ast_node.lhs {
                    left_node = node_map.get(&left_node_id).unwrap();
                    // DEBUG
                    if self.debug {
                        print!("{:?}", left_node);
                    }
                    data_type_as_string = left_node.string_val.clone();
                }

                // create temporary variable
                let mut var_declaration: Instruction = Instruction::new();
                var_declaration.instruction_type = InstructionType::VariableDeclaration;
                var_declaration.data_type = DataType::from_str(&data_type_as_string).expect("Need analyzed data type!");
                var_declaration.label = variable_identifier.clone();

                // create symbol table entry for temporary variable
                let mut symbol_table_entry = SymbolTableEntry::new();
                symbol_table_entry.symbol_table_entry_type = SymbolTableEntryType::Variable;
                symbol_table_entry.data_type = DataType::from_str(&data_type_as_string).expect("Need type");
                symbol_table_entry.is_array = left_node.node_type == AstNodeType::Array;
                self.symbol_table.borrow_mut().insert(variable_identifier.clone(), symbol_table_entry);

                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(var_declaration));

                //
                // Initializer
                //

                // expression
                if let Some(expression_node_id) = ast_node.expression {

                    let expression_node = node_map.get(&expression_node_id).unwrap();

                    // DEBUG
                    if self.debug {
                        print!("{:?}", expression_node);
                    }

                    match expression_node.node_type {
                        AstNodeType::SingleInit => {
                            // DEBUG
                            if self.debug {
                                println!("SingleInit");
                            }
                            if let Some(left_node_id) = expression_node.lhs {
                                let left_node = node_map.get(&left_node_id).unwrap();
                                // DEBUG
                                if self.debug {
                                    print!("{:?}", left_node);
                                }
                                // Copy
                                let mut copy_instruction: Instruction = Instruction::new();
                                copy_instruction.instruction_type = InstructionType::Copy;
                                match &left_node.node_type {
                                    AstNodeType::Expression => {
                                        let mut br_cnt = 0;
                                        copy_instruction.src = self.visit(left_node_id, node_map, &variable_identifier, &mut br_cnt);
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
                            // panic!("test");

                            // Format:
                            // Rule 77: declaration -> declaration_specifiers init_declarator_list SEMICOLON
                            //
                            // LHS contains the data type
                            // RHS contains the initializer list. It is an ASTNode that stores the values in the block_items properties

                            // the data type of the objects in the initializer list is
                            let data_type = DataType::from_str(&data_type_as_string).expect("Need type");
                            println!("DataType: {}", data_type);

                            // LHS - data type
                            if let Some(left_node_id) = ast_node.lhs {
                                let left_node = node_map.get(&left_node_id).unwrap();
                                // DEBUG
                                if self.debug {
                                    println!("DataType-Initializer: {:?}", left_node.string_val);
                                }
                            }

                            // RHS - identifier
                            let mut identifier = String::new();
                            if let Some(right_node_id) = ast_node.rhs {
                                let right_node = node_map.get(&right_node_id).unwrap();
                                // DEBUG
                                if self.debug {
                                    println!("{:?}", right_node.string_val);
                                }
                                identifier = right_node.string_val.clone();
                            }

                            // RHS - initializer list
                            if let Some(expression_node_id) = ast_node.expression {
                                let expression_node = node_map.get(&expression_node_id).unwrap();
                                // // DEBUG
                                // if self.debug {
                                //     println!("{:?}", expression_node);
                                // }
                                let mut current_offset = 0;
                                for i in 0..expression_node.block_items.len() {
                                    let block_item_id = expression_node.block_items[expression_node.block_items.len()-1-i];
                                    let block_item = node_map.get(&block_item_id).unwrap();
                                    // // DEBUG
                                    // if self.debug {
                                    //     println!("{:?}", block_item);
                                    // }
                                    if let Some(left_node_id) = block_item.lhs {
                                        // println!("LHS: {:?}", left_node_id);
                                        let left_node = node_map.get(&left_node_id).unwrap();
                                        println!("Value: {:?}", left_node.string_val);

                                        let mut copy_to_offset: Instruction = Instruction::new();
                                        copy_to_offset.instruction_type = InstructionType::CopyToOffset;
                                        copy_to_offset.data_type = data_type.clone();
                                        copy_to_offset.src = ValueElement::Constant(left_node.string_val.clone());
                                        copy_to_offset.dst = ValueElement::Constant(identifier.clone());
                                        copy_to_offset.offset = current_offset;

                                        // append instruction to latest top-level element of the program
                                        let last = self.program.top_level.len() - 1;
                                        self.program.top_level[last].body.push(Box::new(copy_to_offset));


                                        current_offset = current_offset + data_type.get_size() as i32;
                                    }
                                }
                            }
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
                // println!("End Cast");

                let left_node_id = ast_node.lhs.unwrap();
                let left_node = node_map.get(&left_node_id).unwrap();

                // // DEBUG
                // println!("{:?}", left_node);
                // match left_node.node_type {
                //     AstNodeType::Identifier => {
                //         println!("{:?}", left_node.string_val);
                //     }
                //     _ => {
                //         unimplemented!();
                //     }
                // }

                //
                // Nora Sandler, page 260
                //
                // 1. The target type (cast to) is determined.
                // 2. The inner object (Variable or LiteralValue) is retrieved.
                // 3. The type of the inner object is determined.
                // 4. If the inner object's type matches the target type (cast to) already, then early out.
                //

                let mut br_cnt = 0;
                let src_value_element: ValueElement = self.visit(left_node_id, node_map, &String::from(""), &mut br_cnt);
                match src_value_element {

                    ValueElement::Variable(ref variable_name) => {
                        // determine the type of the variable which is cast
                        let symbol_table_entry: SymbolTableEntry = self.symbol_table.borrow_mut().get(variable_name);

                        // destination time
                        let target_type = ast_node.analyzed_data_type.clone();

                        // DEBUG
                        if self.debug {
                            println!("Casting ValueElement:{:?} of SourceType:{:?} into DataType:{:?}", &src_value_element, symbol_table_entry.data_type, target_type);
                        }

                        // create new temporary variable name for the destination variable
                        let dest_var_name = self.variable_naming_source.borrow_mut().new_temp_var();

                        // create temporary variable to cast into
                        let mut dest_var_declaration: Instruction = Instruction::new();
                        dest_var_declaration.instruction_type = InstructionType::VariableDeclaration;
                        dest_var_declaration.data_type = target_type.clone();
                        dest_var_declaration.label = dest_var_name.clone();

                        // append instruction to latest top-level element of the program
                        let last = self.program.top_level.len() - 1;
                        self.program.top_level[last].body.push(Box::new(dest_var_declaration));

                        // create symbol table entry for temporary variable
                        let mut symbol_table_entry = SymbolTableEntry::new();
                        symbol_table_entry.symbol_table_entry_type = SymbolTableEntryType::Variable;

                        let temp = ast_node.analyzed_data_type.clone();
                        symbol_table_entry.data_type = DataType::from_str(&target_type.clone().to_string()).expect("msg");
                        self.symbol_table.borrow_mut().insert(dest_var_name.clone(), symbol_table_entry);

                        match target_type {
                            DataType::DataTypeInt => {
                                let dst_value_element: ValueElement = ValueElement::Variable(dest_var_name.clone());

                                let mut truncate: Instruction = Instruction::new();
                                truncate.instruction_type = InstructionType::Truncate;
                                truncate.data_type = target_type;
                                truncate.src = src_value_element;
                                truncate.dst = dst_value_element;

                                // append instruction to latest top-level element of the program
                                let last = self.program.top_level.len() - 1;
                                self.program.top_level[last].body.push(Box::new(truncate));
                            }
                            DataType::DataTypeLong => {

                                let dst_value_element: ValueElement = ValueElement::Variable(dest_var_name.clone());

                                let mut sign_extend: Instruction = Instruction::new();
                                sign_extend.instruction_type = InstructionType::SignExtend;
                                sign_extend.data_type = target_type;
                                sign_extend.src = src_value_element;
                                sign_extend.dst = dst_value_element;

                                // append instruction to latest top-level element of the program
                                let last = self.program.top_level.len() - 1;
                                self.program.top_level[last].body.push(Box::new(sign_extend));
                            }
                            _ => {
                                unimplemented!();
                            }
                        }

                        return ValueElement::Variable(dest_var_name.clone());
                    }

                    ValueElement::Constant(ref constant_value) => {
                        // println!("constant_value: {}", constant_value);
                        return ValueElement::Constant(constant_value.clone());
                    }

                    _ => {
                        unimplemented!("{:?}", src_value_element);
                    }
                }
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