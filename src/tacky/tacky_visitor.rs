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

use crate::common::struct_decl::StructDeclMember;
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

//
// Generates TACKY from an AST
//
// 1. src\c_ast\IdentifierResolutionVisitor - checks for duplicate or undeclared variable names
// 2. src\tacky\tacky_visitor.rs (This file) - Generate TACKY (from AST)
// 3. asm_ast/AsmAstConversionVisitor - Converts the AST into a ASM AST for assembly with a precursory form of mnenomics
// 4. asm_ast/AsmAstFixupVisitor - replacing pseudo operands/variables with stack addresses
// 5. asm_ast/AsmAstMasmEmitterVisitor / asm_ast/AsmAstASEmitterVisitor / ...
//

pub struct TackyVisitor {
    variable_naming_source: Rc<RefCell<VariableNamingSource>>, // https://www.youtube.com/watch?v=8O0Nt9qY_vo
    symbol_table: Rc<RefCell<SymbolTable>>, // https://www.youtube.com/watch?v=8O0Nt9qY_vo
    pub program: Program,
    pub function_declaration: Option<Box<TopLevel>>,
    pub type_system: Rc<RefCell<HashMap::<String, DataType>>>,

    // current_data_type: DataType,
    pub debug: bool,

}

impl TackyVisitor {

    pub fn new(
        variable_naming_source_param: Rc<RefCell<VariableNamingSource>>,
        symbol_table_param: Rc<RefCell<SymbolTable>>,
        type_system_param: Rc<RefCell<HashMap::<String, DataType>>>)
        -> TackyVisitor
    {
        TackyVisitor {
            variable_naming_source: variable_naming_source_param,
            symbol_table: symbol_table_param,
            program: Program::new(),
            function_declaration: None,
            type_system: type_system_param,
            // current_data_type: DataType::DataTypeUnknown,
            debug: false,
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

    fn declare_temporary_variable(&mut self,
        temp_var_name: &String,
        data_type: DataType)
    {
        // // DEBUG
        // if temp_var_name == "tmp.4" {
        //     println!("[TackyVisitor::declare_temporary_variable()]test");
        // }

        // create symbol table entry for temporary variable
        let mut symbol_table_entry = SymbolTableEntry::new();
        symbol_table_entry.name = temp_var_name.clone();
        symbol_table_entry.initial_name = temp_var_name.clone();
        symbol_table_entry.symbol_table_entry_type = SymbolTableEntryType::Variable;
        symbol_table_entry.data_type = data_type.clone();
        symbol_table_entry.is_array = false;
        symbol_table_entry.array_element_count = 0;

        self.symbol_table.borrow_mut().insert(temp_var_name.clone(), symbol_table_entry);

        // create temporary variable to cast into
        let mut dest_var_declaration: Instruction = Instruction::new();
        dest_var_declaration.instruction_type = InstructionType::VariableDeclaration;
        dest_var_declaration.data_type = data_type.clone();
        dest_var_declaration.label = temp_var_name.clone();

        // append instruction to latest top-level element of the program
        let last = self.program.top_level.len() - 1;
        self.program.top_level[last].body.push(Box::new(dest_var_declaration));
    }

    fn do_while_handler(&mut self, ast_node: &AstNode, node_map: &mut Box<HashMap<usize, AstNode>>) {
        // int main() {
        //     int count = 5;
        //     do {
        //         count = count - 1;
        //     } while (count <= 0);
        //     return 0;
        // }

        // Label<continue_label>
        //
        //     -- body
        //     <instructions for body>
        //
        //     -- condition
        //     <instructions for condition>
        //     v = <result of condition>
        //     JumpIfZero(v, end_label) -- if the condition is false, jump to the end_label
        //
        //     Jump(continue_label) -- next iteration
        //
        // Label<end_label>

        let continue_label_name = self.continue_label(&ast_node, "while_");

        // continue label
        let mut label_instruction: Instruction = Instruction::new();
        label_instruction.instruction_type = InstructionType::Label;
        label_instruction.label = continue_label_name.clone();
        // append instruction to latest top-level element of the program
        let last = self.program.top_level.len() - 1;
        self.program.top_level[last].body.push(Box::new(label_instruction));

        //
        // Body Statement - block_items
        //

        for i in 0..ast_node.block_items.len() {
            let block_item_id = ast_node.block_items[ast_node.block_items.len()-1-i];
            let block_item = node_map.get(&block_item_id).unwrap();
            let mut br_cnt = 0;
            self.visit(block_item_id, node_map,  &String::from(""), &mut br_cnt);
        }

        // LHS - expression
        if let Some(lhs_id) = ast_node.expression {

            let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
            self.declare_temporary_variable(&temp_var_name, DataType::DataTypeInt);

            let mut br_cnt = 0;
            let ( lhs_value_element, data_type ) = self.visit(lhs_id, node_map, &temp_var_name, &mut br_cnt);

            // println!("lhs_value_element: {:?}", lhs_value_element);

            // JumpIfZero (jump to end label if the condition is false)
            let mut jump_if_zero_instruction: Instruction = Instruction::new();
            jump_if_zero_instruction.instruction_type = InstructionType::JumpIfZero;
            jump_if_zero_instruction.src = lhs_value_element;

            jump_if_zero_instruction.label = self.final_end_label(&ast_node, "while_");

            // append instruction to latest top-level element of the program
            let last = self.program.top_level.len() - 1;
            self.program.top_level[last].body.push(Box::new(jump_if_zero_instruction));
        }

        // jump to continue label
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

    fn cast_handler(&mut self, ast_node: &AstNode, node_map: &mut Box<HashMap<usize, AstNode>>) -> ValueElement {
        // println!("{:?}", ast_node);
        // println!("End Cast");

        // DEBUG
        if self.debug {
            println!("[tacky_visitor][AstNodeType::Cast] id: {}, {:?}", ast_node.id, ast_node);
        }

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
        let ( src_value_element, data_type ) = self.visit(left_node_id, node_map, &String::from(""), &mut br_cnt);
        match src_value_element {

            ValueElement::Variable(ref variable_name) => {

                // println!("variable_name: '{}'", variable_name);

                // determine the type of the variable which is cast
                let symbol_table_entry: SymbolTableEntry =
                    self.symbol_table.borrow_mut().get(variable_name).unwrap();

                // destination type
                let target_type = ast_node.get_analyzed_data_type();

                // DEBUG
                if self.debug {
                    println!("Casting ValueElement:{:?} of SourceType:{:?} into DataType:{:?}",
                        &src_value_element, symbol_table_entry.data_type, target_type);
                }

                // create new temporary variable name for the destination variable
                let dest_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                self.declare_temporary_variable(&dest_var_name, DataType::DataTypeInt);

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
                symbol_table_entry.name = dest_var_name.clone();
                symbol_table_entry.initial_name = dest_var_name.clone();
                symbol_table_entry.symbol_table_entry_type = SymbolTableEntryType::Variable;

                let temp = ast_node.get_analyzed_data_type();
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

    fn expression_to_varname(&mut self,
        ast_node: &AstNode,
        node_map: &mut Box<HashMap<usize, AstNode>>,
        branch_counter: &mut usize) -> String
    {
        let mut exp_result_var_name = String::new();
        if let Some(lhs_id) = ast_node.expression {

            exp_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
            self.declare_temporary_variable(&exp_result_var_name, DataType::DataTypeInt);

            let ( lhs_value_element, data_type )
                = self.visit(lhs_id, node_map, &exp_result_var_name, branch_counter);

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

        return exp_result_var_name;
    }

    fn retrieve_field_from_struct(&mut self, struct_data_type: DataType, field_name: String) -> StructDeclMember {
        // struct might have no members in it if ???
        let struct_tag = match struct_data_type {
            DataType::DataTypeStruct(struct_decl) => {
                struct_decl.tag
            }
            _ => {
                panic!();
            }
        };

        // retrieve the full definition of the structure including fields
        let ts = self.type_system.borrow_mut();
        let struct_data_type = ts.get(&struct_tag).unwrap().clone();

        // println!("abc: {:?}", struct_data_type);

        let field = match struct_data_type {
            DataType::DataTypeStruct(struct_decl) => {
                // // DEBUG
                // println!("{:?}", struct_decl);

                // find struct member that matches the RHS of the DOT
                let field_index_option = struct_decl.members.iter()
                    .position(|m| m.member_name == field_name);
                let field_index = field_index_option.expect("Field not found in struct!");
                let field_option = struct_decl.members.get(field_index);
                let field = field_option.unwrap().clone();
                field
            }
            _ => {
                panic!();
            }
        };

        // DEBUG
        // println!("field: {:?}", field);

        field
    }

    fn switch_handler(&mut self,
        ast_node: &AstNode,
        node_map: &mut Box<HashMap<usize, AstNode>>,
        branch_counter: &mut usize) {

        //
        // The goal is to convert the AST nodes of a switch into
        // TACKY instructions
        //

        // create the final end-label
        let mut final_end_label_instruction: Instruction = Instruction::new();
        final_end_label_instruction.instruction_type = InstructionType::Label;
        final_end_label_instruction.label = self.final_end_label(&ast_node, "switch_final_");

        //
        // expression
        //

        let global_selector_var_name
            = self.expression_to_varname(&ast_node, node_map, branch_counter);

        //
        // Body Statement - block_items (cases and a default statement)
        //

        let mut default_branch_option: Option::<usize> = None;
        let mut counter: usize = 0;

        for i in 0..ast_node.block_items.len() {

            let block_item_id = ast_node.block_items[ast_node.block_items.len()-1-i];
            let block_item = node_map.get(&block_item_id).unwrap();

            //println!("{:?}", block_item);

            // look for the default item
            match block_item.node_type {

                AstNodeType::Case => {

                    //
                    // For each case, first evaluate an equals expression
                    // between the switch-statements selector value and
                    // the case's local selector value. If they match,
                    // execute the body statements and then jump to the
                    // overall end label of the switch statement.
                    // If they are not equal, jump to the local end label
                    // which is local to that case statement
                    //

                    // DEBUG
                    // println!("[CASE]");

                    // add a comment
                    let mut comment: Instruction = Instruction::new();
                    comment.instruction_type = InstructionType::Comment;
                    comment.label = "".to_string();
                    // append comment to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(comment));
                    // add a comment
                    let mut comment: Instruction = Instruction::new();
                    comment.instruction_type = InstructionType::Comment;
                    comment.label = format!("CASE #{}", counter).to_string();
                    // append comment to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(comment));
                    // add a comment
                    let mut comment: Instruction = Instruction::new();
                    comment.instruction_type = InstructionType::Comment;
                    comment.label = "".to_string();
                    // append comment to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(comment));

                    // create the local end-label for this case
                    let mut local_end_label_instruction: Instruction = Instruction::new();
                    local_end_label_instruction.instruction_type = InstructionType::Label;
                    local_end_label_instruction.label = self.new_end_label(&block_item, "switch_case_", &mut counter);
                    counter = counter + 1;

                    // retrieve the target value (= local selector value) from EXPRESSION
                    let local_selector_ast_node = node_map.get(&block_item.expression.unwrap()).unwrap();

                    // DEBUG
                    // println!("[CASE-EXPRESSION] {:?}", &local_selector_ast_node.string_val);

                    // temp variable to capture the result of the comparison between
                    // the global and the local selector
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    let temp_var = self.declare_temporary_variable(&temp_var_name.clone(), DataType::DataTypeInt);

                    // add a comment
                    let mut comment: Instruction = Instruction::new();
                    comment.instruction_type = InstructionType::Comment;
                    comment.label = "perform equals comparison between local and global selector".to_string();
                    // append comment to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(comment));

                    // compare the switch'S input variable (global selector) to the
                    // case expression local selector
                    let mut cmp_equals_instruction: Instruction = Instruction::new();
                    cmp_equals_instruction.instruction_type = InstructionType::Binary;
                    cmp_equals_instruction.binary_operator = BinaryOperator::Equal;
                    cmp_equals_instruction.data_type = DataType::DataTypeInt;
                    cmp_equals_instruction.src = ValueElement::Variable(global_selector_var_name.clone());
                    cmp_equals_instruction.src_2 = ValueElement::Constant(local_selector_ast_node.string_val.clone());
                    cmp_equals_instruction.dst = ValueElement::Variable(temp_var_name.clone());
                    // append instruction to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(cmp_equals_instruction));

                    // add a comment
                    let mut comment: Instruction = Instruction::new();
                    comment.instruction_type = InstructionType::Comment;
                    comment.label = "jump over the body to the local end label if the expression evaluates to false".to_string();
                    // append comment to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(comment));
                    // jump over the body to the local end label if the expression evaluates to false
                    let mut jump_if_zero_instruction: Instruction = Instruction::new();
                    jump_if_zero_instruction.instruction_type = InstructionType::JumpIfZero;
                    jump_if_zero_instruction.src = ValueElement::Variable(temp_var_name.clone());
                    jump_if_zero_instruction.label = local_end_label_instruction.label.clone();
                    // append instruction to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(jump_if_zero_instruction));

                    // add a comment
                    let mut comment: Instruction = Instruction::new();
                    comment.instruction_type = InstructionType::Comment;
                    comment.label = "output statements for the body".to_string();
                    // append comment to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(comment));
                    // output statements for the body
                    let mut br_cnt = 0;
                    self.visit(block_item_id, node_map, &String::from(""), &mut br_cnt);

                    // add a comment
                    let mut comment: Instruction = Instruction::new();
                    comment.instruction_type = InstructionType::Comment;
                    comment.label = "jump to final end label because only one branch is executed ever".to_string();
                    // append comment to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(comment));
                    // jump to final end label because only one branch is executed ever
                    let mut jump_instruction: Instruction = Instruction::new();
                    jump_instruction.instruction_type = InstructionType::Jump;
                    jump_instruction.label = final_end_label_instruction.label.clone();

                    // append instruction to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(jump_instruction));

                    // insert local end label
                    // append instruction to latest top-level element of the program
                    let last = self.program.top_level.len() - 1;
                    self.program.top_level[last].body.push(Box::new(local_end_label_instruction));
                }
                AstNodeType::Default => {

                    // DEBUG
                    // println!("[DEFAULT]");

                    default_branch_option = Some(block_item_id);
                }
                _ => {
                    panic!();
                }
            }
        }

        // process the buffered default branch
        // output statements for the body
        if let Some(default_branch_id) = default_branch_option {

            // add a comment
            let mut comment: Instruction = Instruction::new();
            comment.instruction_type = InstructionType::Comment;
            comment.label = "".to_string();
            // append comment to latest top-level element of the program
            let last = self.program.top_level.len() - 1;
            self.program.top_level[last].body.push(Box::new(comment));
            // add a comment
            let mut comment: Instruction = Instruction::new();
            comment.instruction_type = InstructionType::Comment;
            comment.label = format!("DEFAULT").to_string();
            // append comment to latest top-level element of the program
            let last = self.program.top_level.len() - 1;
            self.program.top_level[last].body.push(Box::new(comment));
            // add a comment
            let mut comment: Instruction = Instruction::new();
            comment.instruction_type = InstructionType::Comment;
            comment.label = "".to_string();
            // append comment to latest top-level element of the program
            let last = self.program.top_level.len() - 1;
            self.program.top_level[last].body.push(Box::new(comment));

            let mut br_cnt = 0;
            self.visit(default_branch_id, node_map, &String::from(""), &mut br_cnt);
        }

        // append instruction to latest top-level element of the program
        let last = self.program.top_level.len() - 1;
        self.program.top_level[last].body.push(Box::new(final_end_label_instruction));
    }

    fn arrow_handler(&mut self,
        ast_node_id: usize,
        node_map: &mut Box<HashMap<usize, AstNode>>,
        dst_name: String) -> ValueElement
    {
        // let ast_node = node_map.get(&ast_node_id).unwrap();

        // // DEBUG
        // if self.debug {
        //     println!("ast_node_id: {} file: {}, line: {}", ast_node_id, ast_node.file, ast_node.line);
        // }

        // retrieve the value of this expression, write it into
        // a temporary variable and return a ValueElement

        // self.find_base_variable(ast_node_id, node_map);

        let ast_node = node_map.get(&ast_node_id).unwrap().clone();

        let rhs_ast_node = node_map.get(&ast_node.rhs.unwrap()).unwrap().clone();

        let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
        self.declare_temporary_variable(&temp_var_name, DataType::DataTypeUnsignedLong);

        // DEBUG
        // println!("temp_var_name: {}", temp_var_name);

        let lhs_node_id = ast_node.lhs.unwrap();

        // // DEBUG
        // if lhs_node_id == 50 {
        //     println!("ast_node: {:?}", ast_node);
        //     println!("test");
        // }

        let mut br_cnt = 0;
        let ( lhs_value_element, lhs_data_type )
            = self.visit(lhs_node_id, node_map, &temp_var_name, &mut br_cnt);

        // // ASSERT: need data_type
        // assert_ne!(lhs_data_type, DataType::DataTypeUnknown,
        //     "{}", format!("AST-Node {:?} has no data type after evaluation inside the TACKY-visitor!", lhs_node_id).as_str());

        // println!("lhs_value_element: {:?}", lhs_value_element);
        // println!("lhs_data_type: {}", lhs_data_type);

        // find offset

        let rhs_variable_name = rhs_ast_node.string_val.clone();
        // println!("rhs_variable_name: {}", rhs_variable_name);

        let field: StructDeclMember = match lhs_data_type {
            DataType::DataTypePointer(base_data_type) => {
                self.retrieve_field_from_struct(*base_data_type, rhs_variable_name)
            }
            _ => {
                match lhs_value_element.clone() {
                    ValueElement::Variable(variable_name) => {

                        // resolve variable name to struct and within the struct, resolve RHS field
                        let base_symbol_table_entry: SymbolTableEntry = self.symbol_table.borrow_mut().retrieve(&variable_name);
                        let data_type_variable = base_symbol_table_entry.data_type.clone();
                        let field: StructDeclMember = match data_type_variable {
                            DataType::DataTypePointer(base_data_type) => {
                                self.retrieve_field_from_struct(*base_data_type, rhs_variable_name)
                            }
                            _ => {
                                panic!("Unhandled case: {}", lhs_data_type);
                            }
                        };

                        // println!("field: {:?}", field);

                        field
                    },
                    ValueElement::Constant(_) => todo!(),
                    ValueElement::None => todo!(),
                    // _ => {
                    //     panic!("Unhandled case: {}", lhs_data_type);
                    // }
                }
            }
        };

        let mut copy_instruction: Instruction = Instruction::new();
        copy_instruction.instruction_type = InstructionType::CopyFromOffsetPointerDeref;
        copy_instruction.offset = field.offset as i32;
        copy_instruction.data_type = field.data_type.clone();
        copy_instruction.src = lhs_value_element;
        copy_instruction.dst = ValueElement::Variable(dst_name.clone());

        // // DEBUG
        // if self.debug {
        //     println!("{:?}", copy_instruction);
        // }

        // append instruction to latest top-level element of the program
        let last = self.program.top_level.len() - 1;
        self.program.top_level[last].body.push(Box::new(copy_instruction));

        ValueElement::Variable(dst_name.clone())
    }

    fn retrieve_offset(&mut self,
        base_variable_name: &String,
        field_ast_node_id: usize,
        node_map: &mut Box<HashMap<usize, AstNode>>) -> i32
    {

        let mut field_name = String::new();
        let field_ast_node = node_map.get(&field_ast_node_id).unwrap();
        match field_ast_node.node_type {
            AstNodeType::Identifier => {
                field_name = field_ast_node.string_val.clone();
            }
            _ => {

                todo!();
            }
        }

        // println!("field_name: {}", field_name);

        let base_symbol_table_entry: SymbolTableEntry = self.symbol_table.borrow_mut().retrieve(&base_variable_name);

        // println!("base_symbol_table_entry: {:?}", &base_symbol_table_entry);
        // println!("base_symbol_table_entry.data_type: {:?}", &base_symbol_table_entry.data_type);

        let mut struct_decl_member = StructDeclMember::new();
        match base_symbol_table_entry.data_type {
            DataType::DataTypeStruct(struct_decl_val) => {
                struct_decl_member = struct_decl_val.members.iter().find(|f| f.member_name == field_name).unwrap().clone();
            }
            _ => {
                panic!("Unhandled type: {}", base_symbol_table_entry.data_type);
            }
        }

        // println!("struct_decl_member: {:?}", struct_decl_member);

        match struct_decl_member.data_type {
            DataType::DataTypeChar |
            DataType::DataTypeUnsignedChar |
            DataType::DataTypeInt |
            DataType::DataTypeSignedInt |
            DataType::DataTypeUnsignedInt |
            DataType::DataTypeShort |
            DataType::DataTypeSignedShort |
            DataType::DataTypeUnsignedShort |
            DataType::DataTypeLong |
            DataType::DataTypeUnsignedLong |
            DataType::DataTypeFloat |
            DataType::DataTypeDouble => {
                return struct_decl_member.offset as i32;
            }
            DataType::DataTypeStruct(struct_decl_val_2) => {
                // recurse here and add offset to field offset
                //return struct_decl_member.offset + self.retrieve_offset(field_name, field_ast_node.rhs)
                todo!();
            }
            _ => {
                panic!();
            }
        }

        // let ts = self.type_system.borrow_mut();
        // let base_data_type = ts.get(&symbol_table_entry.data_type);

        //println!("base_data_type: {:?}", base_data_type);

        // //
        // let mut struct_decl: StructDecl = StructDecl::new();
        // match lhs_data_type {
        //     DataType::DataTypePointer(dest_data_type) => {
        //         // println!("dest_data_type: {}", &dest_data_type);
        //         match *dest_data_type {
        //             DataType::DataTypeStruct(struct_decl_val) => {
        //                 // println!("struct_decl: {:?}", struct_decl_val);
        //                 struct_decl = struct_decl_val;
        //                 let struct_decl_member = struct_decl.members.iter().find(|f| f.member_name == field_name);
        //                 return struct_decl_member.unwrap().data_type.clone();
        //             }
        //             _ => {
        //                 panic!("Unhandled ptr destination data type: {:?}", dest_data_type);
        //             }
        //         }
        //     }

        // return 4;
    }

    /// Will enrich the instruction passed into this function via the copy_instruction parameter.
    /// Will insert values into the offset and data_type members.
    /// The reason is that now, the TACKY instruction can use the correct offset along
    /// with the base address to access the nested field while copy data from memory into the
    /// destination.
    ///
    /// e.g.
    /// b = struct_ptr->a;
    /// b = struct_0.a;
    /// b = struct_0.next->a;
    fn ast_node_to_copy_from_offset_instruction(&mut self,
        ast_node: &AstNode,
        node_map: &mut Box<HashMap<usize, AstNode>>,
        copy_instruction: &mut Instruction) -> ValueElement
    {
        //println!("[TackyVisitor] {:?}: ", ast_node);

        let lhs_sub_ast_node_id = ast_node.lhs.unwrap();

        // let destination_var_name = String::from("");
        // let mut branch_counter: usize = 0;
        // let ( value_element, data_type )
        //     = self.visit(lhs_sub_ast_node_id, node_map, &destination_var_name, &mut branch_counter);

        // //println!("[TackyVisitor] value_element: {:?}", value_element);
        // //println!("[TackyVisitor] data_type: {:?}", data_type);

        let lhs_sub_ast_node = node_map.get(&lhs_sub_ast_node_id).unwrap();

        // if lhs_sub_ast_node.node_type == AstNodeType::Dot {
        //     println!("do found!");
        // }

        // DEBUG
        // println!("{}, {}", lhs_lhs_sub.id, lhs_lhs_sub.string_val);

        let lhs_src_name = lhs_sub_ast_node.string_val.clone();

        // DEBUG
        // println!("[TACKY_VISITOR] src_name: '{}'", lhs_src_name);

        // TODO: learn why borrow_mut() works but borrow() does not!
        let symbol_table_entry = self.symbol_table.borrow_mut().get(&lhs_src_name).unwrap();

        // DEBUG
        if self.debug {
            println!("symbol_table_entry: {:?}", symbol_table_entry);
        }

        let symbol_data_type = symbol_table_entry.data_type;

        //
        // to identify the offset, extract the name from the RHS.
        // SideNote: The name has not been replaced.
        // Retrieve datatype and the offset from the struct datatype
        //

        let lhs_rhs_sub_id = ast_node.rhs.unwrap();
        let lhs_rhs_sub = node_map.get(&lhs_rhs_sub_id).unwrap();

        // DEBUG
        if self.debug {
            println!("Node_id: {}, node.string_val: '{}'", lhs_rhs_sub.id, lhs_rhs_sub.string_val);
        }

        match symbol_data_type {

            DataType::DataTypeStruct(struct_decl) => {

                // // DEBUG
                // let ts = self.type_system.borrow_mut();
                // let struct_data_type = ts.get(&struct_decl.tag).unwrap();
                // println!("{:?}", struct_data_type);

                // // DEBUG
                // println!("{:?}", struct_decl);

                // find struct member that matches the RHS of the DOT
                let field_index_option = struct_decl.members.iter()
                    .position(|m| m.member_name == lhs_rhs_sub.string_val);
                let field_index = field_index_option.expect("Field not found in struct!");
                let field_option = struct_decl.members.get(field_index);
                let field = field_option.unwrap();

                // copy offset into the TACKY instruction
                copy_instruction.offset = field.offset as i32;
                copy_instruction.data_type = field.data_type.clone();
            }

            DataType::DataTypePointer(ptr_target_data_type) => {

                // DEBUG
                if self.debug {
                    println!("{:?}", ptr_target_data_type);
                }

                match *ptr_target_data_type {

                    DataType::DataTypeStruct(struct_decl) => {

                        // find struct member that matches the RHS of the DOT
                        let field_index_option = struct_decl.members.iter()
                            .position(|m| m.member_name == lhs_rhs_sub.string_val);
                        let field_index = field_index_option.expect("Field not found in struct!");
                        let field_option = struct_decl.members.get(field_index);
                        let field = field_option.unwrap();

                        // copy offset into the TACKY instruction
                        copy_instruction.offset = field.offset as i32;
                        copy_instruction.data_type = field.data_type.clone();
                    }

                    _ => {
                        todo!();
                    }
                }
            }

            _ => {
                todo!("Unhandled data type: {:?}", symbol_data_type);
            }
        }

        return ValueElement::Variable(lhs_src_name.clone());
    }

    pub fn find_base_variable(&mut self,
        ast_node_id: usize,
        node_map: &mut Box<HashMap<usize, AstNode>>)
        -> ( DataType, usize )
    {
        let ast_node = node_map.get(&ast_node_id).unwrap();

        // DEBUG
        // if self.debug {
            println!("ast_node_id: {} file: {}, line: {}", ast_node_id, ast_node.file, ast_node.line);
        // }

        match &ast_node.node_type {

            AstNodeType::Dot => {
                let rhs_node = node_map.get(&ast_node.rhs.unwrap()).unwrap().clone();
                let ( data_type, offset ) = self.find_base_variable(ast_node.lhs.unwrap(), node_map);

                println!("data_type: {:?}", data_type);

                match data_type {

                    DataType::DataTypeStruct(struct_decl) => {

                        // // DEBUG
                        // let ts = self.type_system.borrow_mut();
                        // let struct_data_type = ts.get(&struct_decl.tag).unwrap();
                        // println!("{:?}", struct_data_type);

                        // // DEBUG
                        // println!("{:?}", struct_decl);

                        // find struct member that matches the RHS of the DOT
                        let field_index_option = struct_decl.members.iter()
                            .position(|m| m.member_name == rhs_node.string_val);
                        let field_index = field_index_option.expect("Field not found in struct!");
                        let field_option = struct_decl.members.get(field_index);
                        let field = field_option.unwrap();

                        // // copy offset into the TACKY instruction
                        // copy_instruction.offset = field.offset as i32;
                        // copy_instruction.data_type = field.data_type.clone();

                        // field.data_type
                        // let symbol_table_entry = self.symbol_table.borrow_mut().get(&var_name).unwrap().clone();

                        return ( field.data_type.clone(), offset + field.offset );
                    }

                    _ => {
                        panic!();
                    }
                }
            }

            AstNodeType::Arrow => {
                let rhs_node = node_map.get(&ast_node.rhs.unwrap()).unwrap().clone();
                // let ( symbol_table_entry, offset ) = self.find_base_variable(ast_node.lhs.unwrap(), node_map);
                let ( data_type, offset ) = self.find_base_variable(ast_node.lhs.unwrap(), node_map);

                println!("data_type: {:?}", data_type);

                match data_type {

                    DataType::DataTypePointer(dest_data_type) => {
                        println!("dest_data_type: {:?}", dest_data_type);
                        match *dest_data_type {
                            DataType::DataTypeStruct(struct_decl) => {
                                let field_index_option = struct_decl.members.iter()
                                    .position(|m| m.member_name == rhs_node.string_val);
                                let field_index = field_index_option.expect("Field not found in struct!");
                                let field_option = struct_decl.members.get(field_index);
                                let field = field_option.unwrap();

                                return ( field.data_type.clone(), field.offset );
                            }
                            _ => {
                                todo!();
                            }
                        }
                    }

                    DataType::DataTypeStruct(struct_decl) => {

                        // // DEBUG
                        // let ts = self.type_system.borrow_mut();
                        // let struct_data_type = ts.get(&struct_decl.tag).unwrap();
                        // println!("{:?}", struct_data_type);

                        // // DEBUG
                        // println!("{:?}", struct_decl);

                        // find struct member that matches the RHS of the DOT
                        let field_index_option = struct_decl.members.iter()
                            .position(|m| m.member_name == rhs_node.string_val);
                        let field_index = field_index_option.expect("Field not found in struct!");
                        let field_option = struct_decl.members.get(field_index);
                        let field = field_option.unwrap();

                        // // copy offset into the TACKY instruction
                        // copy_instruction.offset = field.offset as i32;
                        // copy_instruction.data_type = field.data_type.clone();

                        // field.data_type
                        // let symbol_table_entry = self.symbol_table.borrow_mut().get(&var_name).unwrap().clone();

                        return ( field.data_type.clone(), offset + field.offset );
                    }

                    _ => {
                        panic!();
                    }
                }
            }

            AstNodeType::Identifier => {
                let var_name = ast_node.string_val.clone();
                let symbol_table_entry = self.symbol_table.borrow_mut().get(&var_name).unwrap().clone();
                // let symbol_data_type = symbol_table_entry.data_type;

                // symbol_table_entry.data_type.;

                // let ts = self.type_system.borrow_mut();
                // let data_type = ts.get(&.);

                return ( symbol_table_entry.data_type, 0 );
            }

            _ => {
                panic!();
            }

        }
    }

    fn function_declaration_handler(&mut self,
        ast_node: &AstNode,
        node_map: &mut Box<HashMap<usize, AstNode>>) -> ( ValueElement, DataType )
    {
        //
        // LHS - body of function delaration
        //
        // The LHS node is optional. If it is missing, this FunctionDeclaration
        // is a prototype and the TACKY visitor should not export any TACKY instruction
        if let Some(lhs_ast_node_id) = ast_node.lhs {
            // keep processing
        } else {
            let return_value: ValueElement = ValueElement::Constant(String::from("0"));
            return ( return_value, DataType::DataTypeUnknown );
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

            match data_type.node_type.clone() {
                AstNodeType::DataType => {
                    top_level_function.return_type = Some(DataType::from_str(&data_type.string_val).expect("REASON"));
                },
                AstNodeType::TypeSpecifier(type_name) => {
                    let ts = self.type_system.borrow_mut();
                    let data_type = ts.get(&type_name).unwrap();

                    top_level_function.return_type = Some(data_type.clone());

                    // function_definition_ast_node.set_analyzed_data_type(data_type.clone());

                    // // Assumption: return-value data-type of function
                    // // return-value data-type of function
                    // function_definition_ast_node.rhs = Some(sub_ast_node.id);
                }
                _ => {
                    panic!();
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
                let ts = self.type_system.borrow_mut();
                let ts_data_type = ts.get(&right_node.string_val).unwrap().clone();
                //argument.data_type = DataType::from_str(&right_node.string_val).expect(format!("Type is not known: {}\n", &right_node.string_val).as_str());
                argument.data_type = ts_data_type;
            }
            top_level_function.arguments.push(Box::new(argument));
        }

        // insert function into program
        self.program.top_level.push(Box::new(top_level_function));

        // add instructions and declarations into body/block
        if let Some(block_id) = ast_node.lhs {

            // resolve node_id to ASTNode
            let block = node_map.get(&block_id).unwrap().clone();

            // only if the body contains elements. Only if it is not an empty body
            if block.block_items.len() > 0 {

                // visit each statement in the body
                for i in 0..block.block_items.len() {

                    // prepare parameters to visit()
                    let destination_var_name = String::from("");
                    let mut br_cnt = 0;
                    let last_block_item_id = block.block_items[block.block_items.len()-1-i];

                    // call visit()
                    self.visit(last_block_item_id, node_map, &destination_var_name, &mut br_cnt);
                }

                let last_block_item_id = &block.block_items[0];
                let last_block_item = node_map.get(&last_block_item_id).unwrap();

                // DEBUG
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

        ( ValueElement::None, DataType::DataTypeUnknown )
    }

    pub fn visit(&mut self,
        ast_node_id: usize,
        node_map: &mut Box<HashMap<usize, AstNode>>,
        dst_name: &String,
        branch_counter: &mut usize) -> ( ValueElement, DataType )
    {
        let ast_node = node_map.get(&ast_node_id).unwrap().clone();

        // here, for each ASTNode type, create a TACKY instruction.
        //
        // The TACKY instruction needs to be of type struct Instruction defined in tacky.rs.
        // Then when all necessary information is copied into the Instruction struct, insert
        // that struct into the current body, using this line:
        // ```
        // // append instruction to latest top-level element of the program
        // let last = self.program.top_level.len() - 1;
        // self.program.top_level[last].body.push(Box::new(copy_instruction));
        // ```
        match &ast_node.node_type {

            AstNodeType::Program => {

                // // insert global scope into program
                // let mut top_level_global: TopLevel = TopLevel::new();
                // top_level_global.name = String::from("global");
                // top_level_global.top_level_type = TopLevelType::Scope;
                // top_level_global.global = true;
                // self.program.top_level.push(Box::new(top_level_global));

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

                self.function_declaration_handler(&ast_node, node_map);
            }

            AstNodeType::Block => {
                // println!("id: {}", ast_node.node_id);
                //let mut value_element: ValueElement = ValueElement::None;
                let mut touple = ( ValueElement::None, DataType::DataTypeUnknown );
                for i in 0..ast_node.block_items.len() {
                    let mut br_cnt = 0;
                    let bli = ast_node.block_items[ast_node.block_items.len()-1-i];
                    touple = self.visit(bli, node_map, &String::from(""), &mut br_cnt);
                }
                return touple;
            }

            AstNodeType::BlockItem => {
                // println!("id: {}", ast_node.node_id);
                if let Some(compound_id) = ast_node.lhs {
                    let compound = node_map.get(&compound_id).unwrap();
                    if let Some(left_node_id) = compound.lhs {
                        let mut br_cnt = 0;
                        return self.visit(left_node_id, node_map, &String::from(""), &mut br_cnt);
                    }
                }
            }

            AstNodeType::Statement => {
                if let Some(sub_id) = ast_node.lhs {
                    return self.visit(sub_id, node_map, &dst_name, branch_counter);
                }
            }

            AstNodeType::Compound => {
                if let Some(sub_id) = ast_node.lhs {
                    return self.visit(sub_id, node_map, &dst_name, branch_counter);
                }
            }

            AstNodeType::Expression => {

                // DEBUG
                if self.debug {
                    println!("[Expression] AstNodeId: {}, AstNodeType: {:?}", ast_node.id, ast_node.node_type);
                }

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

                        // LHS can be
                        // - a variable
                        // - a pointer variable
                        // - an array subscript
                        if let Some(lhs_subnode_id) = ast_node.lhs {

                            let lhs_sub = node_map.get(&lhs_subnode_id).unwrap().clone();
                            match lhs_sub.node_type {

                                AstNodeType::Subscript => {
                                    let instruction = self.subscript_as_destination(node_map, &ast_node);

                                    // append instruction to latest top-level element of the program
                                    let last = self.program.top_level.len() - 1;
                                    self.program.top_level[last].body.push(Box::new(instruction));
                                }

                                _ => {
                                    let mut dst_name = String::new();

                                    let mut copy_instruction: Instruction = Instruction::new();

                                    match lhs_sub.node_type {

                                        // HHHHHH
                                        AstNodeType::Dot | AstNodeType::Arrow => {
                                            copy_instruction.instruction_type = InstructionType::CopyToOffset;
                                            let value_element
                                                = self.ast_node_to_copy_from_offset_instruction(&lhs_sub,
                                                    node_map,
                                                    &mut copy_instruction);
                                            copy_instruction.dst = value_element;
                                        }

                                        _ => {
                                            dst_name = lhs_sub.string_val.clone();

                                            // need a name
                                            assert!(dst_name.len() != 0,
                                                "[ERR] [tacky_visitor::Assignment]
                                                No name discovered in Node: {} for destination!", lhs_sub.id);

                                            // Copy - this is new for the init part of for loops!
                                            copy_instruction.instruction_type = InstructionType::Copy;
                                            copy_instruction.dst = ValueElement::Variable(dst_name.clone());
                                        }
                                    }

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
                                                let ( lhs_value_element, data_type ) = self.visit(rhs_sub, node_map, &dst_name, &mut br_cnt);
                                                store_instruction.src = lhs_value_element;
                                            }

                                            // append instruction to latest top-level element of the program
                                            let last = self.program.top_level.len() - 1;
                                            self.program.top_level[last].body.push(Box::new(store_instruction));
                                        }

                                        _ => {

                                            // if a binary instruction is executed, a value is correctly assigned
                                            // to the target variable by the binary instruction itself.
                                            // Same goes for array subscript instructions.
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
                                            // let mut copy_instruction: Instruction = Instruction::new();
                                            // copy_instruction.instruction_type = InstructionType::Copy;

                                            if let Some(rhs_sub_id) = ast_node.rhs {

                                                // need a name
                                                if dst_name.len() == 0 {
                                                    dst_name = self.variable_naming_source.borrow_mut().new_temp_var();

                                                    // Honolulu
                                                    //self.declare_temporary_variable(&dst_name, DataType::DataTypeInt);
                                                    self.declare_temporary_variable(&dst_name, DataType::DataTypeUnsignedLong);
                                                }

                                                // need a name
                                                assert!(dst_name.len() != 0,
                                                    "[ERR] [tacky_visitor::Assignment]
                                                    No name discovered in Node: {} for destination!", lhs_sub.id);

                                                let mut br_cnt = 0;

                                                let ( lhs_value_element, data_type ) = self.visit(rhs_sub_id, node_map, &dst_name, &mut br_cnt);
                                                copy_instruction.src = lhs_value_element;

                                                let rhs_sub = node_map.get(&rhs_sub_id).unwrap();

                                                // DEBUG
                                                if self.debug {
                                                    println!("RHS: {:?}", rhs_sub);
                                                }

                                                // determine if the copy instruction needs to be output or not
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

                                                    AstNodeType::Subscript => {
                                                        // DEBUG
                                                        // println!("Subscript");
                                                        output_copy_instruction = false;
                                                    }

                                                    _ => {
                                                        // println!("NodeType: {:?}", rhs_sub.node_type);
                                                    }
                                                }
                                            }

                                            if copy_instruction.src == copy_instruction.dst {
                                                output_copy_instruction = false;
                                            }

                                            // to understand this check, read large comment above
                                            if output_copy_instruction {

                                                // add a comment
                                                let mut comment: Instruction = Instruction::new();
                                                comment.instruction_type = InstructionType::Comment;
                                                comment.label = "Copy for expression/assignment".to_string();

                                                // append comment to latest top-level element of the program
                                                let last = self.program.top_level.len() - 1;
                                                self.program.top_level[last].body.push(Box::new(comment));

                                                // append instruction to latest top-level element of the program
                                                let last = self.program.top_level.len() - 1;
                                                self.program.top_level[last].body.push(Box::new(copy_instruction));
                                            }
                                        }
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
                                    AstNodeType::ConstInt |
                                    AstNodeType::ConstLong |
                                    AstNodeType::ConstUInt => {
                                        function_call_instruction.parameters.push(
                                            Box::new(ValueElement::Constant(
                                                String::from(parameter_ast_node.string_val.clone()))));
                                    }
                                    AstNodeType::Identifier => {
                                        function_call_instruction.parameters.push(
                                            Box::new(ValueElement::Variable(
                                                String::from(parameter_ast_node.string_val.clone()))));
                                    }
                                    _ => {
                                        panic!("Please add node_type for: '{:?}'! ast_node: {:?}",
                                            parameter_ast_node.node_type,
                                            parameter_ast_node);
                                    }
                                }
                            }
                        }

                        function_call_instruction.dst = ValueElement::Variable(dst_name.clone());

                        // append instruction to latest top-level element of the program
                        let last = self.program.top_level.len() - 1;
                        self.program.top_level[last].body.push(Box::new(function_call_instruction));

                        let result = ValueElement::Variable(dst_name.clone());

                        return ( result, DataType::DataTypeUnknown );
                    }

                    AstNodeOperatorType::Cast => {
                        // DEBUG
                        if self.debug {
                            println!("{:?}", ast_node);
                        }
                    }

                    AstNodeOperatorType::AddrOf => {

                        // A assignment of an address to a pointer in C such as
                        //
                        // int *my_pointer = my_array;
                        // int *my_pointer = &my_int_variable;
                        //
                        // creates an AST node of AstNodeOperatorType AddrOf which is
                        // converted to TACKY in this match-case.
                        //
                        // In TACKY, a assignment of an address to a pointer
                        // is represented as a GetAddress() TACKY instruction:
                        //
                        // GetAddress(src:Variable(userdef_var.0), dst:Variable(userdef_var.1))
                        //
                        // The source variable is contained in the ASTNode.LHS
                        // the destination variable name is passed in as a parameter to this function.

                        // DEBUG
                        if self.debug {
                            println!("{:?}", ast_node);
                        }

                        let mut get_address_instruction: Instruction = Instruction::new();
                        get_address_instruction.instruction_type = InstructionType::GetAddress;

                        // source variable name is retrieved from LHS
                        if let Some(lhs_node_id) = ast_node.lhs {
                            let lhs_node = node_map.get(&lhs_node_id).unwrap();
                            get_address_instruction.src = ValueElement::Variable(lhs_node.string_val.clone());
                        }

                        // the destination variable name is passed in as a parameter to this functino
                        let dest_variable_element = ValueElement::Variable(dst_name.to_string());
                        get_address_instruction.dst = dest_variable_element.clone();

                        // self.symbol_table.borrow_mut().insert(variable_identifier.clone(), symbol_table_entry);

                        // make the destinatin variable a pointer by setting the is_pointer flag to true
                        // in the symbol table
                        let mut symbol_table_entry = self.symbol_table.borrow_mut().get(&dst_name).unwrap();
                        symbol_table_entry.is_pointer = true;
                        self.symbol_table.borrow_mut().insert(dst_name.clone(), symbol_table_entry);

                        // append instruction to latest top-level element of the program
                        let last = self.program.top_level.len() - 1;
                        self.program.top_level[last].body.push(Box::new(get_address_instruction));

                        return ( dest_variable_element, DataType::DataTypeUnknown );
                    }

                    AstNodeOperatorType::MulAssignment |
                    AstNodeOperatorType::AddAssignment |
                    AstNodeOperatorType::SubAssignment |
                    AstNodeOperatorType::RightShiftAssignment => {

                        let mut add_assignment_instruction: Instruction = Instruction::new();
                        add_assignment_instruction.instruction_type = InstructionType::from_operator_type(&ast_node.operator_type);

                        let mut dst_name = String::from("");

                        // LHS - used as destination (the variable that is assigned to)
                        if let Some(lhs_sub_id) = ast_node.lhs {
                            let lhs_sub = node_map.get(&lhs_sub_id).unwrap();
                            // DEBUG
                            if self.debug {
                                println!("LHS: {:?}", lhs_sub);
                            }
                            dst_name = lhs_sub.string_val.clone();
                            add_assignment_instruction.dst = ValueElement::Variable(dst_name.clone());
                        }

                        // RHS - used as src (The value that is assigned)
                        if let Some(rhs_sub_id) = ast_node.rhs {
                            let mut br_cnt = 0;
                            let ( value_element, data_type ) = self.visit(rhs_sub_id, node_map, &dst_name, &mut br_cnt);
                            add_assignment_instruction.src = value_element;
                        }

                        // append instruction to latest top-level element of the program
                        let last = self.program.top_level.len() - 1;
                        self.program.top_level[last].body.push(Box::new(add_assignment_instruction));
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

                    _ => {
                        panic!("{}", format!("Unhandled AstNodeOperatorType {:?}!\n", ast_node.operator_type).as_str());
                    }
                }
            }

            AstNodeType::Return => {

                // DEBUG
                if self.debug {
                    println!("[TV][AstNodeType::Return]------------------------------------------------------------");
                    println!("[TV][AstNodeType::Return] id: {} ast_node: {:?}", ast_node.id, ast_node);
                    println!("[TV][AstNodeType::Return] ast_node.get_analyzed_data_type(): {}", ast_node.get_analyzed_data_type());
                }

                // a data type that is NOT unknown is required!
                assert_ne!(ast_node.get_analyzed_data_type(), DataType::DataTypeUnknown,
                    "[TV][ERR] A type is required!");

                //
                // add a comment
                //

                let mut comment: Instruction = Instruction::new();
                comment.instruction_type = InstructionType::Comment;
                comment.label = "Return instruction".to_string();

                // append comment to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(comment));

                //
                // add a statement
                //

                if let Some(expression_id) = ast_node.lhs {

                    //
                    // Case: return with value
                    //

                    // // This is later needed in array subscripts as direct return values
                    // let create_temporary_variable = true;
                    // if create_temporary_variable {

                        let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                        self.declare_temporary_variable(&temp_var_name, ast_node.get_analyzed_data_type());

                        // // create symbol table entry for temporary variable
                        // let mut symbol_table_entry = SymbolTableEntry::new();
                        // symbol_table_entry.name = temp_var_name.clone();
                        // symbol_table_entry.initial_name = temp_var_name.clone();
                        // symbol_table_entry.symbol_table_entry_type = SymbolTableEntryType::Variable;
                        // symbol_table_entry.data_type = ast_node.get_analyzed_data_type();
                        // symbol_table_entry.is_array = false;
                        // symbol_table_entry.array_element_count = 0;
                        // self.symbol_table.borrow_mut().insert(temp_var_name.clone(), symbol_table_entry);
                    // }

                    // // create temporary variable to cast into
                    // let mut dest_var_declaration: Instruction = Instruction::new();
                    // dest_var_declaration.instruction_type = InstructionType::VariableDeclaration;
                    // dest_var_declaration.data_type = DataType::DataTypeInt;
                    // dest_var_declaration.label = temp_var_name.clone();

                    // // append instruction to latest top-level element of the program
                    // let last = self.program.top_level.len() - 1;
                    // self.program.top_level[last].body.push(Box::new(dest_var_declaration));

                    // DEBUG
                    if self.debug {
                        println!("[TV][AstNodeType::Return] dst_name: '{}'", dst_name);
                    }

                    // // DEBUG
                    // if expression_id == 130 {
                    //     println!("test");
                    // }

                    let mut br_cnt = 0;
                    let ( value_element, data_type ) = self.visit(expression_id, node_map, &temp_var_name, &mut br_cnt);

                    let return_value: ValueElement = value_element;

                    // DEBUG
                    if self.debug {
                        println!("[TV][AstNodeType::Return] return_value: {:?}", return_value);
                    }

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
                    return_instruction.data_type = ast_node.get_analyzed_data_type();
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

                // LHS - the unary operator
                if let Some(operator_id) = ast_node.lhs {

                    let operator_ast_node = node_map.get(&operator_id).unwrap();
                    match operator_ast_node.operator_type {

                        AstNodeOperatorType::AddrOf => {
                            // DEBUG
                            if self.debug {
                                println!("{:?}", ast_node);
                                println!("{:?}", ast_node.lhs);
                                println!("{:?}", ast_node.rhs);
                            }

                            let mut get_address_instruction: Instruction = Instruction::new();
                            get_address_instruction.instruction_type = InstructionType::GetAddress;

                            // examples:
                            // int *b = &a;
                            // int *b = &struct_var.a;
                            // int *b = &struct_var.a.b.c.d.e.f;

                            // RHS - variable to get the address of
                            if let Some(rhs_node_id) = ast_node.rhs {

                                let rhs_ast_node = node_map.get(&rhs_node_id).unwrap();
                                match rhs_ast_node.node_type {
                                    AstNodeType::Dot | AstNodeType::Arrow => {

                                        let mut base_variable_name = String::new();
                                        if let Some(name_ast_node_id) = rhs_ast_node.lhs {
                                            let name_ast_node = node_map.get(&name_ast_node_id).unwrap();
                                            let name = name_ast_node.string_val.clone();
                                            // println!("name: '{}'", name);

                                            // write the name of the base variable into the src variable
                                            get_address_instruction.src = ValueElement::Variable(name.clone());

                                            base_variable_name = name.clone();
                                        }

                                        if let Some(field_ast_node_id) = rhs_ast_node.rhs {
                                            //let name_ast_node = node_map.get(&name_ast_node_id).unwrap();

                                            get_address_instruction.offset
                                                = self.retrieve_offset(&base_variable_name, field_ast_node_id, node_map);
                                        }

                                        //let offset_value = self.retrieve_offset(name.clone(), );

                                        // add an offset which will become part of the LEA instruction as an offset
                                        //get_address_instruction.offset = 123;
                                        // todo!();
                                    }
                                    _ => {
                                        get_address_instruction.src = ValueElement::Variable(rhs_ast_node.string_val.clone());
                                    }
                                }

                            }
                            get_address_instruction.dst = ValueElement::Variable(dst_name.to_string());

                            // append instruction to latest top-level element of the program
                            let last = self.program.top_level.len() - 1;
                            self.program.top_level[last].body.push(Box::new(get_address_instruction));

                            let value_element = ValueElement::Variable(String::from(dst_name.to_string()));
                            return ( value_element, DataType::DataTypeUnknown );
                        }

                        AstNodeOperatorType::Dereference => {

                            // Nora Sandler, page 371, Listing 14-19
                            //
                            // The C dereference operation is implemented as a TACKY LOAD instruction

                            // println!("{:?}", ast_node);

                            // // DEBUG
                            // println!("{:?}", ast_node.lhs);
                            // println!("{:?}", ast_node.rhs);

                            // add a comment
                            let mut comment: Instruction = Instruction::new();
                            comment.instruction_type = InstructionType::Comment;
                            comment.label = "dereference".to_string();

                            // append comment to latest top-level element of the program
                            let last = self.program.top_level.len() - 1;
                            self.program.top_level[last].body.push(Box::new(comment));

                            // the dereference operation generates a LOAD instruction
                            // LOAD referres to memory-load. The address stored in the src
                            // variable is used to access memory and load a value from that
                            // address into the destination variable
                            let mut load_instruction: Instruction = Instruction::new();
                            load_instruction.instruction_type = InstructionType::Load;
                            if let Some(rhs_node_id) = ast_node.rhs {
                                let rhs_node = node_map.get(&rhs_node_id).unwrap();
                                load_instruction.src = ValueElement::Variable(rhs_node.string_val.clone());
                            }
                            load_instruction.dst = ValueElement::Variable(dst_name.to_string());

                            // insert dst into the symbol table
                            let mut symbol_table_entry: SymbolTableEntry = SymbolTableEntry::new();
                            symbol_table_entry.name = dst_name.to_string().clone();
                            symbol_table_entry.initial_name = dst_name.to_string().clone();
                            symbol_table_entry.symbol_table_entry_type = SymbolTableEntryType::Variable;
                            // pointers are 64-bit (long) on 64-bit systems always
                            symbol_table_entry.data_type = DataType::DataTypePointer(Box::new(DataType::DataTypeLong));
                            symbol_table_entry.parameter_count = 0usize;
                            symbol_table_entry.has_body = false;
                            symbol_table_entry.is_array = false;
                            symbol_table_entry.is_pointer = false;

                            // DEBUG
                            if self.debug {
                                println!("[TackyVisitor] {:?}", symbol_table_entry);
                            }

                            self.symbol_table.borrow_mut().insert(dst_name.clone(), symbol_table_entry);

                            // append instruction to latest top-level element of the program
                            let last = self.program.top_level.len() - 1;
                            self.program.top_level[last].body.push(Box::new(load_instruction));

                            return ( ValueElement::Variable(String::from(dst_name.to_string())), DataType::DataTypeUnknown );
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

                            // RHS - contains the source
                            if let Some(rhs_id) = ast_node.rhs {

                                let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                                self.declare_temporary_variable(&temp_var_name, DataType::DataTypeInt);

                                let mut br_cnt = 0;
                                let ( rhs_value_element, data_type ) = self.visit(rhs_id, node_map, &temp_var_name, &mut br_cnt);

                                // source and destination are the same thing
                                unary_instruction.src = rhs_value_element.clone();
                                unary_instruction.dst = rhs_value_element.clone();
                            }

                            //
                            // LHS - contains the operator operator
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

                                    AstNodeOperatorType::Decrement => {
                                        unary_instruction.unary_operator = UnaryOperator::Decrement;
                                    }

                                    AstNodeOperatorType::PrefixIncrement => {
                                        unary_instruction.unary_operator = UnaryOperator::Increment;
                                    }

                                    AstNodeOperatorType::PrefixDecrement => {
                                        unary_instruction.unary_operator = UnaryOperator::Decrement;
                                    }

                                    AstNodeOperatorType::Dereference => {
                                        // DEBUG
                                        if self.debug {
                                            println!("Dereference {:?}", ast_node);
                                        }
                                        unary_instruction.unary_operator = UnaryOperator::Dereference;
                                        unary_instruction.dst = ValueElement::Variable(dst_name.to_string());
                                    }

                                    AstNodeOperatorType::AddrOf => {
                                        // DEBUG
                                        if self.debug {
                                            println!("AddrOf {:?}", ast_node);
                                        }
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

                            return ( ValueElement::Variable(String::from(dst_name.to_string())), DataType::DataTypeUnknown );
                        }
                    }
                }
            }

            AstNodeType::Binary => {
                let mut binary_instruction: Instruction = Instruction::new();
                binary_instruction.instruction_type = InstructionType::Binary;

                // destination
                binary_instruction.dst = ValueElement::Variable(dst_name.to_string());

                // LHS - src 2
                if let Some(lhs_id) = ast_node.lhs {
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    self.declare_temporary_variable(&temp_var_name, DataType::DataTypeInt);

                    let mut br_cnt = 0;
                    let ( lhs_value_element, data_type) = self.visit(lhs_id, node_map, &temp_var_name, &mut br_cnt);
                    binary_instruction.src_2 = lhs_value_element;
                }

                // RHS - src
                if let Some(rhs_id) = ast_node.rhs {
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    self.declare_temporary_variable(&temp_var_name, DataType::DataTypeInt);

                    let mut br_cnt = 0;
                    let ( rhs_value_element, data_type) = self.visit(rhs_id, node_map, &temp_var_name, &mut br_cnt);
                    binary_instruction.src = rhs_value_element;
                }

                // operator
                if let Some(operator_id) = ast_node.operator {
                    let operator_ast_node = node_map.get(&operator_id).unwrap();
                    match operator_ast_node.operator_type {

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
                            panic!("{}", format!("Unhandled OperatorType {:?}!\n", operator_ast_node.operator_type).as_str());
                        }
                    }
                }

                // copy data type from type-checked ast node into the TACKY instruction
                binary_instruction.data_type = ast_node.get_analyzed_data_type().clone();

                // println!("{:?}", ast_node);

                assert_ne!(binary_instruction.data_type, DataType::DataTypeUnknown,
                    "{}", format!("AST-Node {:?} has no data type!", ast_node.id).as_str());

                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(binary_instruction));

                return ( ValueElement::Variable(String::from(dst_name.to_string())), DataType::DataTypeUnknown );
            }

            AstNodeType::ConstInt |
            AstNodeType::ConstLong |
            AstNodeType::ConstUInt |
            AstNodeType::ConstULong |
            AstNodeType::ConstDouble |
            AstNodeType::ConstChar => {
                return ( ValueElement::Constant(ast_node.string_val.clone()), DataType::DataTypeUnknown );
            }

            AstNodeType::Conditional | AstNodeType::If => {

                //
                // Case 1 - if without else or else-if
                //

                // -- condition
                // <instructions for condition>
                // v = <result of condition>
                // JumpIfZero(v, if-end-label) -- if the condition is false, jump to the if-end-label
                //      -- body
                //     <instructions for body>
                //     Jump(if-end-label) -- branch taken, jump to the final end label
                // Label<if-end-label> -- create the if-end-label

                //
                // Case 2 - if with else (no else-if)
                //

                // -- condition
                // <instructions for condition>
                // v = <result of condition>
                // JumpIfZero(v, if-0-label) -- if the condition is false, jump to the if-0-label
                //     -- body
                //     <instructions for body>
                //     Jump(if-end-label) -- branch taken, jump to the final end label
                // Label<if-0-label>
                //     -- body
                //     <instructions for body>
                //     Jump(if-end-label) -- branch taken, jump to the final end label
                // Label<if-end-label> -- create the if-end-label

                //
                // Case 3 - if (several branches) with else (no else-if)
                //

                // -- condition 0
                // <instructions for condition 0>
                // v = <result of condition 0>
                // JumpIfZero(v, if-0-label) -- if the condition is false, jump to the if-0-label
                //     -- body
                //     <instructions for body 0>
                //     Jump(if-end-label) -- branch taken, jump to the final end label
                //
                // Label<if-0-label>
                // -- condition 1
                // <instructions for condition 1>
                // v = <result of condition 1>
                // JumpIfZero(v, if-1-label) -- if the condition is false, jump to the if-1-label
                //     -- body
                //     <instructions for body 1>
                //     Jump(if-end-label) -- branch taken, jump to the final end label
                // ...
                //
                // Label<if-(n-1)-label>
                // -- condition (n-1)
                // <instructions for condition (n-1)>
                // v = <result of condition (n-1)>
                // JumpIfZero(v, if-n-label) -- if the condition is false, jump to the if-n-label
                //     -- body
                //     <instructions for body (n-1)>
                //     Jump(if-end-label) -- branch taken, jump to the final end label
                //
                // Label<if-n-label>
                //     -- body
                //     <instructions for body>
                //     Jump(if-end-label) -- branch taken, jump to the final end label
                // Label<if-end-label> -- create the if-end-label

                //
                // expression
                //

                let exp_result_var_name
                    = self.expression_to_varname(&ast_node, node_map, branch_counter);

                // JumpIfZero
                let mut jump_if_zero_instruction: Instruction = Instruction::new();
                jump_if_zero_instruction.instruction_type = InstructionType::JumpIfZero;
                jump_if_zero_instruction.src = ValueElement::Variable(exp_result_var_name);

                // handle jump next label from within the if-branch
                if ast_node.rhs.is_none() {
                    // jump to the final end label, if there are no else or if-else branches
                    jump_if_zero_instruction.label = self.final_end_label(&ast_node, "if_");
                } else {
                    // if there is a else or if-else branch, jump to that label
                    jump_if_zero_instruction.label = self.new_end_label(&ast_node, "if_", branch_counter);
                }

                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(jump_if_zero_instruction));

                // LHS - if-branch statement / body
                let mut stmt_result_var_name = String::from("");
                if let Some(lhs_id) = ast_node.lhs {

                    stmt_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    self.declare_temporary_variable(&stmt_result_var_name, DataType::DataTypeInt);

                    // build TACKY for the statements in the IF-Branch (LHS)
                    match ast_node.node_type {
                        AstNodeType::Conditional => {
                            //panic!();

                            // move/copy value into the destination

                            // dest node of the conditional / elvis opereator
                            let destination_ast_node_id = ast_node.expression.unwrap();
                            let destination_ast_node = node_map.get(&destination_ast_node_id).unwrap();

                            let mut copy_instruction: Instruction = Instruction::new();
                            copy_instruction.instruction_type = InstructionType::Copy;
                            copy_instruction.dst = ValueElement::Variable(dst_name.clone());

                            // copy_instruction.src = ValueElement::Constant(String::from("111"));

                            let lhs_value_element_data_type
                                = self.visit(lhs_id, node_map, &stmt_result_var_name, branch_counter);
                            copy_instruction.src = lhs_value_element_data_type.0;

                            // append instruction to latest top-level element of the program
                            let last = self.program.top_level.len() - 1;
                            self.program.top_level[last].body.push(Box::new(copy_instruction));
                        }
                        AstNodeType::If => {
                            let _lhs_value_element
                                = self.visit(lhs_id, node_map, &stmt_result_var_name, branch_counter);
                        }
                        _ => {
                            panic!();
                        }
                    }

                    if !ast_node.rhs.is_none() {

                        // if the branch was taken, unconditionally jump to end_label -- Jump(end)
                        // unless there is no else branch in which case, no jump is necessary
                        let mut jump_instruction: Instruction = Instruction::new();
                        jump_instruction.instruction_type = InstructionType::Jump;
                        jump_instruction.label = self.final_end_label(&ast_node, "if_");
                        // append instruction to latest top-level element of the program
                        let last = self.program.top_level.len() - 1;
                        self.program.top_level[last].body.push(Box::new(jump_instruction));

                        // Label - for this current branch
                        let mut label_instruction: Instruction = Instruction::new();
                        label_instruction.instruction_type = InstructionType::Label;
                        label_instruction.label = self.new_end_label(&ast_node, "if_", branch_counter);

                        // append instruction to latest top-level element of the program
                        let last = self.program.top_level.len() - 1;
                        self.program.top_level[last].body.push(Box::new(label_instruction));
                    }
                }

                // RHS - else-branch or else-if statement / body
                // recurse into the rest of the else / else-if branches
                if let Some(rhs_id) = ast_node.rhs {

                    *branch_counter = *branch_counter + 1;

                    // RHS
                    stmt_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    self.declare_temporary_variable(&stmt_result_var_name, DataType::DataTypeInt);

                    // build TACKY for the statements in the ELSE-Branch (RHS)
                    match ast_node.node_type {
                        AstNodeType::Conditional => {
                            // move/copy value into the destination

                            // dest node of the conditional / elvis opereator
                            let destination_ast_node_id = ast_node.expression.unwrap();
                            let destination_ast_node = node_map.get(&destination_ast_node_id).unwrap();

                            let mut copy_instruction: Instruction = Instruction::new();
                            copy_instruction.instruction_type = InstructionType::Copy;
                            copy_instruction.dst = ValueElement::Variable(dst_name.clone());

                            // copy_instruction.src = ValueElement::Constant(String::from("999"));

                            let rhs_value_element_data_type
                                = self.visit(rhs_id, node_map, &stmt_result_var_name, branch_counter);
                            copy_instruction.src = rhs_value_element_data_type.0;

                            // append instruction to latest top-level element of the program
                            let last = self.program.top_level.len() - 1;
                            self.program.top_level[last].body.push(Box::new(copy_instruction));
                        }
                        AstNodeType::If => {
                            let _rhs_value_element
                                = self.visit(rhs_id, node_map, &stmt_result_var_name, branch_counter);
                        }
                        _ => {
                            panic!();
                        }
                    }
                }

                // Label - final end label
                let mut label_instruction: Instruction = Instruction::new();
                label_instruction.instruction_type = InstructionType::Label;
                label_instruction.label = self.final_end_label(&ast_node, "if_");

                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(label_instruction));

                //return ( ValueElement::Constant(ast_node.string_val.clone()), DataType::DataTypeUnknown );
                return ( ValueElement::Variable(dst_name.clone()), DataType::DataTypeUnknown );
            }

            // AstNodeType::Conditional => {

            //     //
            //     // Case - if with else (no else-if)
            //     //

            //     // -- condition
            //     // <instructions for condition>
            //     // v = <result of condition>
            //     // JumpIfZero(v, if-0-label) -- if the condition is false, jump to the if-0-label
            //     //     -- body
            //     //     <instructions for body>
            //     //     Jump(if-end-label) -- branch taken, jump to the final end label
            //     // Label<if-0-label>
            //     //     -- body
            //     //     <instructions for body>
            //     //     Jump(if-end-label) -- branch taken, jump to the final end label
            //     // Label<if-end-label> -- create the if-end-label

            // }

            AstNodeType::Switch => {
                self.switch_handler(&ast_node, node_map, branch_counter);
            }

            AstNodeType::Case => {
                for i in 0..ast_node.block_items.len() {
                    let block_item_id = ast_node.block_items[ast_node.block_items.len()-1-i];
                    let block_item = node_map.get(&block_item_id).unwrap();
                    let mut br_cnt = 0;
                    self.visit(block_item_id, node_map,  &String::from(""), &mut br_cnt);
                }
            }

            AstNodeType::Default => {
                for i in 0..ast_node.block_items.len() {
                    let block_item_id = ast_node.block_items[ast_node.block_items.len()-1-i];
                    let block_item = node_map.get(&block_item_id).unwrap();
                    let mut br_cnt = 0;
                    self.visit(block_item_id, node_map,  &String::from(""), &mut br_cnt);
                }
            }

            AstNodeType::Break => {

            }

            AstNodeType::While => {

                // Label<continue_label>
                //
                //     -- condition
                //     <instructions for condition>
                //     v = <result of condition> -- There is no TACKY definition for an assignment! How is this assignment defined?
                //     JumpIfZero(v, break_label) -- if the condition is false, jump to the break_label
                //
                //     -- body
                //     <instructions for body>
                //     Jump(continue_label) -- next iteration
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

                // LHS - expression
                if let Some(lhs_id) = ast_node.lhs {

                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    self.declare_temporary_variable(&temp_var_name, DataType::DataTypeInt);

                    let mut br_cnt = 0;
                    let ( lhs_value_element, data_type ) = self.visit(lhs_id, node_map, &temp_var_name, &mut br_cnt);

                    // JumpIfZero (jump to end label if the condition is false)
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
                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    self.declare_temporary_variable(&temp_var_name, DataType::DataTypeInt);

                    let mut br_cnt = 0;
                    let rhs_value_element = self.visit(rhs_id, node_map, &temp_var_name, &mut br_cnt);
                }

                // jump to continue label
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

            AstNodeType::DoWhile => {
                self.do_while_handler(&ast_node, node_map);
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
                    self.declare_temporary_variable(&temp_var_name, DataType::DataTypeInt);

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

                    // new temporary variable used as destination to store the comparison result
                    exp_result_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    self.declare_temporary_variable(&exp_result_var_name, DataType::DataTypeInt);

                    // add new temporary variable into the symbol table as otherwise the
                    // AsmAstFixupVisitor fails later when it retrieves the variable from the symbol table

                    // create symbol table entry for temporary variable
                    let mut symbol_table_entry = SymbolTableEntry::new();
                    symbol_table_entry.name = exp_result_var_name.clone();
                    symbol_table_entry.initial_name = exp_result_var_name.clone();
                    symbol_table_entry.symbol_table_entry_type = SymbolTableEntryType::Variable;
                    symbol_table_entry.data_type = DataType::DataTypeInt;
                    symbol_table_entry.is_array = false;
                    symbol_table_entry.array_element_count = 0;

                    // this will generate the TACKY instruction which implements the predicate / comparison
                    self.symbol_table.borrow_mut().insert(exp_result_var_name.clone(), symbol_table_entry);

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

                //
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

                    let temp_var_name = self.variable_naming_source.borrow_mut().new_temp_var();
                    self.declare_temporary_variable(&temp_var_name, DataType::DataTypeInt);

                    let mut br_cnt = 0;
                    let rhs_value_element = self.visit(rhs_id, node_map, &temp_var_name, &mut br_cnt);
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

                // DEBUG
                if self.debug {
                    println!("[TV][AstNodeType::Identifier] variable_identifier: '{:?}'", variable_identifier);
                }

                return ( ValueElement::Variable(variable_identifier), DataType::DataTypeUnknown );
            }

            AstNodeType::VariableDeclaration => {

                // DEBUG
                if self.debug {
                    println!("[TackyVisitor] ast-node-id: {}, {:?}", ast_node.id, ast_node);
                }

                let mut node_as_string = String::from("");
                node_as_string.push_str(ast_node.serialize(node_map).as_str());

                // add a comment
                let mut comment: Instruction = Instruction::new();
                comment.instruction_type = InstructionType::Comment;
                comment.label = node_as_string;

                // append comment to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(comment));

                // In TACKY, variables are not declared. It is allowed to use variables
                // without declaring them. Only if a variable is initialized, this is
                // output into TACKY as a variable assignment.
                //
                // This implementation will output a variable declaration although it is
                // not strictly required!
                //
                // Nora Sandler, page 110: "[..] we can discard variable declarations at this stage;
                // in TACKY, you don't need to declare variables before using them.
                // But we do need to emit TACKY to initialize varibles.
                // If a declaration includes an initializer, we'll handle it like a normal variable
                // assignment. If a declaration doesn't have an initializer, we won't amit any TACKY
                // at all."

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
                        print!("Id: {}, {:?}", right_node.id, right_node);
                    }

                    variable_identifier = right_node.string_val.clone();

                    // DEBUG
                    if self.debug {
                        println!("{:?}", variable_identifier);
                    }
                }

                let mut explicit_array_element_count: i32 = -1;

                // LHS - data type
                let mut data_type_as_string = String::from("ERROR");
                let lhs_data_type = DataType::DataTypeUnknown;
                let mut is_array: bool = false;

                let mut left_node = &AstNode::new(0);
                if let Some(left_node_id) = ast_node.lhs {

                    // retrieve left node by id
                    left_node = node_map.get(&left_node_id).unwrap();

                    // DEBUG
                    if self.debug {
                        println!("[TackyVisitor][LHS]: Id: {}, {:?}", left_node.id, left_node);
                    }

                    is_array = left_node.node_type == AstNodeType::Array;
                    //let is_bool = left_node.data_type.unwrap() == DataType::DataTypeBool;

                    data_type_as_string = left_node.string_val.clone();

                    // LHS of left node itself.
                    //
                    match left_node.node_type {
                        AstNodeType::Structure => {
                            if let Some(rhs_node_id) = left_node.rhs {
                                let lhs_rhs_node = node_map.get(&rhs_node_id).unwrap();
                                data_type_as_string = lhs_rhs_node.string_val.clone();
                            }
                        }
                        AstNodeType::Array => {
                            // This node contains the element count of arrays
                            // (if the user decided to specify that explicitly in code between angular brackets)
                            //
                            // This node is optional since it is allowed to not
                            // specify the array size when an initializer is used while
                            // declaring the array. e.g.: int arr[] = {64, 34, 25, 12, 22, 11, 90};
                            if let Some(left_node_lhs_id) = left_node.lhs {

                                // retrieve node by id
                                let left_node_lhs = node_map.get(&left_node_lhs_id).unwrap();

                                // DEBUG
                                if self.debug {
                                    println!("****Id: {}, {:?}", left_node_lhs.id, left_node_lhs);
                                    println!("{}", left_node_lhs.string_val);
                                }

                                explicit_array_element_count = i32::from_str_radix(&left_node_lhs.string_val, 10).expect("REASON")
                            }
                        }
                        _ => {
                            // lhs_data_type = match DataType::from_str(data_type_as_string.as_str()) {
                            //     Ok(data_type_result) => data_type_result,
                            //     Err(e) => panic!("should be valid DataType: {e}"),
                            // };
                            let ts = self.type_system.borrow_mut();
                            let lhs_data_type = ts.get(data_type_as_string.to_lowercase().as_str()).unwrap().clone();
                        }
                    }
                }

                // DEBUG
                if self.debug {
                    println!("[TackyVisitor] lhs_data_type: {:?}", lhs_data_type);
                }

                let mut implicit_array_element_count: i32 = -1;

                // when using a initializer during array declaration,
                // the compound statement that initializes
                // the array is given in the expression node
                if let Some(expression_id) = ast_node.expression {

                    // retrieve node by id
                    let expression_node = node_map.get(&expression_id).unwrap();

                    // DEBUG
                    if self.debug {
                        println!("Id: {}, {:?}", expression_node.id, expression_node);
                    }

                    // determine the length of the array by looking at the initializer
                    // the initializer of an array needs to provide a value for each
                    // element. If the user does not explicitly specify the length of
                    // the array with angular brackets, then the length of the initializer
                    // becomes the length of the array
                    implicit_array_element_count = expression_node.block_items.len() as i32;
                    if implicit_array_element_count == 0 {
                        implicit_array_element_count = -1;
                    }
                }
                // DEBUG
                if self.debug {
                    println!("explicit_array_element_count: {}", explicit_array_element_count);
                    println!("implicit_array_element_count: {}", implicit_array_element_count);
                }
                let mut array_element_count: i32 = 0;

                // check array dimension
                if is_array {

                    if explicit_array_element_count < 0 && implicit_array_element_count < 0 {
                        panic!("[ERR] No length found for array!");
                    } else if explicit_array_element_count < 0 && implicit_array_element_count > 0 {
                        array_element_count = implicit_array_element_count;
                    } else if explicit_array_element_count > 0 && implicit_array_element_count <= 0 {
                        array_element_count = explicit_array_element_count;
                    } else if explicit_array_element_count > 0 && implicit_array_element_count > 0 {
                        if explicit_array_element_count != implicit_array_element_count {
                            panic!("[ERR] Explicit array length and length of initializer do not match!");
                        } else {
                            array_element_count = explicit_array_element_count;
                        }
                    }

                    // an array of size zero is weird!
                    // https://stackoverflow.com/questions/9722632/what-happens-if-i-define-a-0-size-array-in-c-c
                    assert_ne!(array_element_count, 0, "[ERR] An array of size 0 is not allowed!");
                }
                // DEBUG
                if self.debug {
                    println!("explicit_array_element_count: {}", explicit_array_element_count);
                    println!("implicit_array_element_count: {}", implicit_array_element_count);
                    println!("=> array_element_count: {}", array_element_count);
                }
                // create TACKY variable declaration
                let mut var_declaration: Instruction = Instruction::new();
                var_declaration.instruction_type = InstructionType::VariableDeclaration;
                let data_type: DataType = self.type_system.borrow().get(&data_type_as_string.to_lowercase()).unwrap().clone();
                var_declaration.data_type = data_type.clone();
                var_declaration.label = variable_identifier.clone();

                if self.symbol_table.borrow_mut().contains_key(&variable_identifier) {
                    // this branch will perform an update and leave all other properties
                    // as is. This is is important because the type checking visitor which
                    // created the initial entry has added status about the variable being
                    // a pointer. This status should not be lost!
                    // This is a mess!!!!!
                    //
                    // TODO: Why is there a new entry inserted into the symbol table?
                    // I think that here, the status if it is an array and the amount of array
                    // elements are updated. If this is the case, why was this not done directly
                    // inside the type_checking_visitor?
                    //
                    // Maybe move all symbol_table_entry creation into the type checking visitor
                    // There are more creations inside this tacky visistor. It is unclear why
                    // there is a separation between the type checking- and the tacky visitor
                    // for symbol table updates!?!?!?!?
                    let mut symbol_table_entry = self.symbol_table.borrow_mut().get(&variable_identifier).unwrap();
                    symbol_table_entry.is_array = is_array;
                    symbol_table_entry.array_element_count = array_element_count;
                    // DEBUG
                    if self.debug {
                        println!("[TackyVisitor] key:'{}', value{:?}", variable_identifier.clone(), symbol_table_entry);
                    }
                    self.symbol_table.borrow_mut().insert(variable_identifier.clone(), symbol_table_entry);
                } else {
                    // create symbol table entry for temporary variable
                    let mut symbol_table_entry = SymbolTableEntry::new();
                    symbol_table_entry.name = variable_identifier.clone();
                    symbol_table_entry.initial_name = variable_identifier.clone();
                    symbol_table_entry.symbol_table_entry_type = SymbolTableEntryType::Variable;
                    symbol_table_entry.data_type = data_type.clone();
                    symbol_table_entry.is_array = is_array;
                    symbol_table_entry.array_element_count = array_element_count;
                    // DEBUG
                    if self.debug {
                        println!("[TackyVisitor] key:'{}', value{:?}", variable_identifier.clone(), symbol_table_entry);
                    }

                    // TODO: Why is there a new entry inserted into the symbol table?
                    // I think that here, the status if it is an array and the amount of array
                    // elements are updated. If this is the case, why was this not done directly
                    // inside the type_checking_visitor?
                    //
                    // Maybe move all symbol_table_entry creation into the type checking visitor
                    // There are more creations inside this tacky visistor. It is unclear why
                    // there is a separation between the type checking- and the tacky visitor
                    // for symbol table updates!?!?!?!?
                    self.symbol_table.borrow_mut().insert(variable_identifier.clone(), symbol_table_entry);
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

                            //
                            // LHS - is where the expression to assign to the destination lives
                            //

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
                                        let ( value_element, data_type )
                                            = self.visit(left_node_id, node_map, &variable_identifier, &mut br_cnt);

                                        // DEBUG
                                        if self.debug {
                                            println!("[TV] AstNodeType::VariableDeclaration - data_type: {}, value_element: {:?}", data_type, value_element);
                                        }

                                        copy_instruction.src = value_element;

                                        // if the target is of type _Bool, convert non-null values
                                        // to the true constant
                                        if lhs_data_type == DataType::DataTypeBool {
                                            match copy_instruction.src {
                                                ValueElement::Constant(literal_val) => {
                                                    if literal_val == "0" {
                                                        copy_instruction.src = ValueElement::Constant(String::from("0"));
                                                    } else {
                                                        copy_instruction.src = ValueElement::Constant(String::from("1"));
                                                    }
                                                }
                                                _ => {
                                                    panic!();
                                                }
                                            }
                                        }
                                    }

                                    _ => {
                                        panic!("Unhandled type: {:?}", left_node.node_type);
                                    }
                                }

                                // DEBUG
                                // println!("{:?}", copy_instruction.src);

                                // need a source
                                assert_ne!(copy_instruction.src, ValueElement::None);

                                copy_instruction.dst = ValueElement::Variable(variable_identifier);
                                if copy_instruction.src != copy_instruction.dst {
                                    // append instruction to latest top-level element of the program
                                    let last = self.program.top_level.len() - 1;
                                    self.program.top_level[last].body.push(Box::new(copy_instruction));
                                }
                            }
                        }

                        AstNodeType::CompoundInit => {
                            // panic!("test");

                            // Format:
                            // Rule 77: declaration -> declaration_specifiers init_declarator_list SEMICOLON
                            //
                            // LHS contains the data type
                            // RHS contains the initializer list. It is an ASTNode that stores the values in the block_items properties

                            // // determine the type of the variable which is cast
                            // let symbol_table_entry: SymbolTableEntry =
                            //     self.symbol_table.borrow_mut().get(variable_name).unwrap();

                            let data_type: DataType = self.type_system.borrow().get(&data_type_as_string).unwrap().clone();

                            // the data type of the objects in the initializer list is
                            // let data_type = DataType::from_str(&data_type_as_string).expect("Need type");

                            // DEBUG
                            if self.debug {
                                println!("DataType: {}", data_type);
                            }

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

                                // DEBUG
                                if self.debug {
                                    println!("{:?}", expression_node);
                                }

                                // add a comment
                                let mut comment_declaration: Instruction = Instruction::new();
                                comment_declaration.instruction_type = InstructionType::Comment;
                                comment_declaration.label = "initialize array elements".to_string();

                                // append comment to latest top-level element of the program
                                let last = self.program.top_level.len() - 1;
                                self.program.top_level[last].body.push(Box::new(comment_declaration));

                                let mut current_offset = 0;
                                for i in 0..expression_node.block_items.len() {

                                    let block_item_id = expression_node.block_items[expression_node.block_items.len()-1-i];
                                    let block_item = node_map.get(&block_item_id).unwrap();

                                    // DEBUG
                                    if self.debug {
                                        println!("{:?}", block_item);
                                    }

                                    if let Some(left_node_id) = block_item.lhs {

                                        // DEBUG
                                        if self.debug {
                                            println!("LHS: {:?}", left_node_id);
                                        }

                                        let left_node = node_map.get(&left_node_id).unwrap();

                                        // DEBUG
                                        if self.debug {
                                            println!("Value: {:?}", left_node.string_val);
                                        }

                                        let mut copy_to_offset: Instruction = Instruction::new();
                                        copy_to_offset.instruction_type = InstructionType::CopyToOffset;
                                        copy_to_offset.data_type = data_type.clone();
                                        copy_to_offset.src = ValueElement::Constant(left_node.string_val.clone());
                                        copy_to_offset.dst = ValueElement::Variable(identifier.clone());
                                        copy_to_offset.offset = current_offset;

                                        // append instruction to latest top-level element of the program
                                        let last = self.program.top_level.len() - 1;
                                        self.program.top_level[last].body.push(Box::new(copy_to_offset));

                                        current_offset = current_offset + data_type.get_size(&self.type_system.borrow()) as i32;
                                    }
                                }
                            }
                        }

                        _ => {
                            panic!("test");
                        }
                    }
                }
            }

            AstNodeType::DataType => {
                // println!("{:?}", ast_node);
            }

            AstNodeType::StructureDeclaration => {
                // println!("{:?}", ast_node);
            }

            AstNodeType::EnumDeclaration => {
                // println!("{:?}", ast_node);
            }

            AstNodeType::Cast => {
                return ( self.cast_handler(&ast_node, node_map), DataType::DataTypeUnknown );
            }

            AstNodeType::EmptyStatement => {
                // println!("{:?}", ast_node);
            }

            AstNodeType::AddAssignment => {
                // DEBUG
                if self.debug {
                    println!("{}, {:?}", ast_node.id, ast_node);
                }

                let mut add_assignment: Instruction = Instruction::new();
                add_assignment.instruction_type = InstructionType::AddAssignment;
            }

            AstNodeType::Subscript => {

                //
                // EXAMPLE:
                // arr[0] = 1; // subscript as target
                // int a = arr[0]; // subscript as source -> Subscript
                //

                // DEBUG
                if self.debug {
                    println!("{}, {:?}", ast_node.id, ast_node);
                    println!("[TackyVisitor::visit::subscript] dst_name: {}", dst_name);
                }

                //
                // add a comment
                //

                let mut comment: Instruction = Instruction::new();
                comment.instruction_type = InstructionType::Comment;
                comment.label = "Subscript for array access".to_string();

                // append comment to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(comment));

                //
                // subscript instruction (subscript used as source)
                //

                let subscript_instruction = self.subscript_as_source(node_map, dst_name, &ast_node);

                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(subscript_instruction));

                return ( ValueElement::Variable(dst_name.clone()), DataType::DataTypeUnknown );
            }

            AstNodeType::Typedef => {
                // TODO: check that the typedeffed type actually exists!
            }

            AstNodeType::Dot => {
                let ast_node = node_map.get(&ast_node_id).unwrap().clone();

                // DEBUG
                if self.debug {
                    println!("ast_node_id: {} file: {}, line: {}", ast_node_id, ast_node.file, ast_node.line);
                }

                // retrieve the value of this expression, write it into
                // a temporary variable and return a ValueElement

                let mut copy_instruction: Instruction = Instruction::new();
                copy_instruction.instruction_type = InstructionType::CopyFromOffset;
                let value_element
                    = self.ast_node_to_copy_from_offset_instruction(&ast_node,
                        node_map,
                        &mut copy_instruction);
                copy_instruction.src = value_element;
                copy_instruction.dst = ValueElement::Variable(dst_name.clone());

                // DEBUG
                if self.debug {
                    println!("{:?}", copy_instruction);
                }

                let current_data_type = copy_instruction.data_type.clone();

                // DEBUG
                if self.debug {
                    println!("{:?}", current_data_type);
                }

                // append instruction to latest top-level element of the program
                let last = self.program.top_level.len() - 1;
                self.program.top_level[last].body.push(Box::new(copy_instruction));

                return ( ValueElement::Variable(dst_name.clone()), current_data_type );
            }

            AstNodeType::Arrow => {
                return ( self.arrow_handler(ast_node_id, node_map, dst_name.clone()), DataType::DataTypeUnknown );
            }

            _ => {
                panic!("{}", format!("Unhandled NodeType '{:?}'!\n", ast_node.node_type).as_str());
            }
        }

        ( ValueElement::None, DataType::DataTypeUnknown )
    }

    // generates and returns a instruction that has the effect:
    // assigns an value retrieved by array subscript into a destination variable
    fn subscript_as_source(&mut self,
        node_map: &mut Box<HashMap<usize, AstNode>>,
        dst_name: &String,
        ast_node: &AstNode
    )
        -> Instruction
    {
        let mut subscript_instruction: Instruction = Instruction::new();
        subscript_instruction.instruction_type = InstructionType::SubscriptSrc;

        //
        // src
        //

        // determine the type and name of the variable which is assigned
        if let Some(left_node_id) = ast_node.lhs {

            let left_node = node_map.get(&left_node_id).unwrap();

            // data type of the array
            let symbol_table_entry: SymbolTableEntry = self.symbol_table.borrow_mut().get(&left_node.string_val).unwrap();
            subscript_instruction.data_type = symbol_table_entry.data_type.clone();

            // variable name of the object / array that is subscripted
            let src_value_element: ValueElement = ValueElement::Variable(left_node.string_val.clone());
            subscript_instruction.src = src_value_element;
        }

        // RHS - index into the array subscript / might also be a variable
        if let Some(right_node_id) = ast_node.rhs {
            let mut br_cnt = 0;
            let ( value_element, data_type ) = self.visit(right_node_id, node_map, &dst_name, &mut br_cnt);
            subscript_instruction.index = value_element;
        }

        //
        // dst
        //

        subscript_instruction.dst = ValueElement::Variable(dst_name.clone());

        // DEBUG
        if self.debug {
            println!("{:?}", subscript_instruction);
        }

        subscript_instruction
    }

    // generates and returns a instruction that has the effect:
    // assigns a source value into an array subscript which is the destination
    fn subscript_as_destination(&mut self,
        node_map: &mut Box<HashMap<usize, AstNode>>,
        ast_node: &AstNode
    )
        -> Instruction
    {

        // DEBUG
        if self.debug {
            println!("{:?}", ast_node);
        }

        let mut subscript_instruction: Instruction = Instruction::new();
        subscript_instruction.instruction_type = InstructionType::SubscriptDst;

        //
        // src - RHS is the node for the assigned value
        //

        let rhs_node_id = ast_node.rhs.unwrap();
        let dst_name = self.variable_naming_source.borrow_mut().new_temp_var();
        self.declare_temporary_variable(&dst_name, DataType::DataTypeInt);

        let mut branch_counter = 0usize;
        let ( lhs_value_element, data_type ) = self.visit(rhs_node_id, node_map, &dst_name, &mut branch_counter);

        subscript_instruction.src = lhs_value_element;

        //
        // dst - LHS is the node of the subscript
        //

        let lhs_node_id = ast_node.lhs.unwrap();
        let lhs_node = node_map.get(&lhs_node_id).unwrap();

        let lhs_lhs_node_id = lhs_node.lhs.unwrap();
        let lhs_lhs_node = node_map.get(&lhs_lhs_node_id).unwrap();

        let lhs_rhs_node_id = lhs_node.rhs.unwrap();
        let lhs_rhs_node = node_map.get(&lhs_rhs_node_id).unwrap();

        subscript_instruction.dst = ValueElement::Variable(lhs_lhs_node.string_val.clone());

        // instead of using the string value of the lhs node directly, call visit() recursively
        // because the index might be created from a more complex expression which needs visiting itself!

        match lhs_rhs_node.node_type {
            AstNodeType::ConstInt |
            AstNodeType::ConstLong |
            AstNodeType::ConstUInt |
            AstNodeType::ConstULong |
            AstNodeType::ConstDouble => {
                let string_val = lhs_rhs_node.string_val.clone();
                let temp_val = i32::from_str_radix(&string_val, 10).expect("REASON");
                subscript_instruction.offset = temp_val.clone();
                subscript_instruction.index = ValueElement::Constant(string_val.clone());
            }
            AstNodeType::Identifier => {
                subscript_instruction.index = ValueElement::Variable(lhs_rhs_node.string_val.clone());
            }
            _ => {
                panic!("");
            }
        }

        let symbol_table_entry: SymbolTableEntry = self.symbol_table.borrow_mut().get(&lhs_lhs_node.string_val).unwrap();
        subscript_instruction.data_type = symbol_table_entry.data_type.clone();

        // DEBUG
        if self.debug {
            println!("{:?}", subscript_instruction);
        }

        subscript_instruction
    }
}
