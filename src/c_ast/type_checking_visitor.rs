use std::cell::RefCell;
use std::rc::Rc;

use std::str::FromStr;

use crate::AstNode;
use crate::AstNodeType;

use crate::SymbolTable;
use crate::c_ast::ast_node::AstNodeOperatorType;
use crate::common::data_type::DataType;
use crate::common::data_type::DataType::DataTypeInt;
use crate::common::symbol_table::SymbolTableEntry;
use crate::common::symbol_table::SymbolTableEntryType;

//
// Nora Sandler, page 178
//
// Every variable identifier has a type (int, long, double, float, char, byte, ...)
//
// Every function identifier has a return type and a fixed number of arguments (exception: variadic functions)
// A function cannot be declared with a function body more than once!
//
// It is not possible to call a variable like a function (function pointers???)
//
// The purpose of Type Checking is that all declarations of identifiers and all usages have compatible types!
//

pub struct TypeCheckingVisitor {
    symbol_table: Rc<RefCell<SymbolTable>>, // https://www.youtube.com/watch?v=8O0Nt9qY_vo
    debug: bool,
}

impl TypeCheckingVisitor {

    pub fn new(symbol_table_param: Rc<RefCell<SymbolTable>>) -> TypeCheckingVisitor {
        TypeCheckingVisitor {
            symbol_table: symbol_table_param,
            debug: false,
        }
    }

    pub fn print_symbol_table(&self) {
        self.symbol_table.borrow_mut().print_symbol_table();
    }

    pub fn visit(&mut self, ast_node: &mut AstNode) {

        // DEBUG
        if self.debug {
            println!("[visit_ex()] {:?}", ast_node.node_type);
        }

        match ast_node.node_type {

            AstNodeType::Program => {
                // visit all items in the body of the program
                for i in 0..ast_node.block_items.len() {
                    let index = ast_node.block_items.len()-1-i;
                    self.visit(&mut ast_node.block_items[index]);
                }
            }

            AstNodeType::ConstInt => {
            }

            AstNodeType::ConstLong => {
            }

            AstNodeType::ConstUInt => {
            }

            AstNodeType::ConstULong => {
            }

            AstNodeType::ConstDouble => {
            }

            AstNodeType::Structure => {
            }

            AstNodeType::Array => {
            }

            AstNodeType::Expression => {

                // // DEBUG
                // println!("{:?}", ast_node);

                // LHS
                if let Some(left_node) = ast_node.lhs.as_mut() {
                    self.visit(left_node);
                }
                // RHS
                if let Some(right_node) = ast_node.rhs.as_mut() {
                    self.visit(right_node);
                }
            }

            AstNodeType::Identifier => {
                // // DEBUG
                if self.debug {
                    println!("{:?}", ast_node);
                }
            }

            AstNodeType::Return => {
                // LHS
                if let Some(left_node) = ast_node.lhs.as_mut() {
                    self.visit(left_node);
                }
            }

            AstNodeType::If => {
                // Expression
                if let Some(expression_node) = ast_node.expression.as_mut() {
                    self.visit(expression_node);
                }

                // LHS
                if let Some(left_node) = ast_node.lhs.as_mut() {
                    self.visit(left_node);
                }

                // RHS
                if let Some(right_node) = ast_node.rhs.as_mut() {
                    self.visit(right_node);
                }
            }

            AstNodeType::Unary => {
                // println!("Unary: {:?}", ast_node);

                // RHS
                if let Some(right_node) = ast_node.rhs.as_mut() {
                    self.visit(right_node);

                    // Nora Sandler, page 254. Not is always creating integer values 1 (= true), (0 = false)
                    match ast_node.operator_type {
                        AstNodeOperatorType::Not => {
                            ast_node.analyzed_data_type = DataType::from_str("int").unwrap();
                        }
                        _ => {
                            //ast_node.analyzed_data_type = right_node.analyzed_data_type.clone();
                            if !self.symbol_table.borrow_mut().contains(&right_node.string_val) {
                                panic!("Variable '{}' not contained!", &right_node.string_val);
                            }
                            let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&right_node.string_val);
                            ast_node.analyzed_data_type = symbol_table_entry.data_type;
                        }
                    }

                    println!("Unary: {:?}", ast_node);
                }
            }

            AstNodeType::Binary => {

                // DEBUG
                if self.debug {
                    println!("{:?}", ast_node);
                }

                // LHS
                if let Some(left_node) = ast_node.lhs.as_mut() {
                    self.visit(left_node);

                    // check if a variable name is used where a variable name should be used (e.g. b = a + foo)
                    if self.symbol_table.borrow_mut().contains(&left_node.string_val) {
                        let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&left_node.string_val);
                        match symbol_table_entry.symbol_table_entry_type {
                            SymbolTableEntryType::Function => {
                                panic!("Function name used as a variable in an expression!");
                            }
                            _ => {

                            }
                        }
                    }
                }
                // RHS
                if let Some(right_node) = ast_node.rhs.as_mut() {
                    self.visit(right_node);
                }
            }

            AstNodeType::Operator => {
            }

            AstNodeType::PrefixOperator => {
            }

            AstNodeType::DataType => {
            }

            AstNodeType::Declaration => {
                if let Some(left_node) = ast_node.lhs.as_mut() {
                    self.visit(left_node);
                }
            }

            // Structure:
            // RHS - return type
            // string_val - function name
            // parameters - the parameters
            // LHS - is a block which contains all instructions in it's block_items field
            AstNodeType::FunctionDeclaration => {

                let mut symbol_table_entry = SymbolTableEntry::new();
                symbol_table_entry.symbol_table_entry_type = SymbolTableEntryType::Function;

                // function name
                let mut temp_function_name = String::new();
                if let Some(function_name) = ast_node.function_name_ast_node.as_ref() {
                    temp_function_name = function_name.string_val.clone();
                }

                // parameters
                let temp_parameter_count = ast_node.parameters.len();
                for i in 0..ast_node.parameters.len() {
                    self.visit(&mut ast_node.parameters[i]);
                }

                // LHS - body of function == block with statements as block_items
                if let Some(left_node) = ast_node.lhs.as_mut() {
                    self.visit(left_node);

                    symbol_table_entry.has_body = true;
                }

                // RHS - Return type
                let mut temp_data_type_as_string = String::new();
                if let Some(right_node) = ast_node.rhs.as_mut() {
                    // println!("{:?}", right_node);

                    assert!(right_node.node_type == AstNodeType::DataType);
                    temp_data_type_as_string = right_node.string_val.clone();

                    self.visit(right_node);
                }

                let data_type_as_enum = match DataType::from_str(&temp_data_type_as_string) {
                    Ok(data_type_result) => data_type_result,
                    Err(e) => panic!("should be valid DataType: {e}"),
                };

                symbol_table_entry.data_type = data_type_as_enum;
                symbol_table_entry.parameter_count = temp_parameter_count;

                // if the function has already been inserted into the symbol table, check if
                // the next declaration matches (in return data type and parameter amount)

                if self.symbol_table.borrow_mut().contains(&temp_function_name) {
                    let existing_symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&temp_function_name);
                    if symbol_table_entry != existing_symbol_table_entry {
                        panic!("[ERR] Invalid function declaration for '{}'! Does not match earlier declaration (return type and/or parameter count)!", temp_function_name);
                    }
                    if symbol_table_entry.has_body && existing_symbol_table_entry.has_body {
                        panic!("[ERR] Two different function declarations for '{}' both having a body found! Only one body per declaration is allowed!", temp_function_name)
                    }
                } else {
                    // add identifier into symbol table
                    self.symbol_table.borrow_mut().insert(temp_function_name.clone(), symbol_table_entry);
                }
            }

            AstNodeType::VariableDeclaration => {

                // data type
                let mut data_type = String::from("");
                if let Some(left_node) = ast_node.lhs.as_mut() {

                    // DEBUG
                    if self.debug {
                        print!("{:?}", left_node);
                    }

                    data_type = left_node.string_val.clone();
                }

                let data_type_as_enum = match DataType::from_str(&data_type) {
                    Ok(data_type_result) => data_type_result,
                    Err(e) => panic!("should be valid DataType: {e}"),
                };

                let mut symbol_table_entry = SymbolTableEntry::new();
                symbol_table_entry.symbol_table_entry_type = SymbolTableEntryType::Variable;
                symbol_table_entry.data_type = data_type_as_enum;

                // identifier (RHS)
                let mut varname = String::from("");
                if let Some(right_node) = ast_node.rhs.as_mut() {

                    // DEBUG
                    if self.debug {
                        print!("{:?}", right_node);
                    }

                    varname = right_node.string_val.clone();
                }

                // add identifier into symbol table
                self.symbol_table.borrow_mut().insert(varname, symbol_table_entry);

                // initialization expression
                //
                // perform semantic analysis of the initialization expression
                // replace variable names with the unique variable names from
                // the map stored inside the variable_naming_source
                if let Some(expression_node) = ast_node.expression.as_mut() {
                    // DEBUG
                    if self.debug {
                        print!("{:?}", expression_node);
                    }
                    self.visit(expression_node);
                }
            }

            AstNodeType::StructureDeclaration => {
            }

            AstNodeType::ParameterDeclaration => {
            }

            AstNodeType::Statement => {
                if let Some(left_node) = ast_node.lhs.as_mut() {
                    self.visit(ast_node.lhs.as_mut().unwrap());
                }
            }

            AstNodeType::Block => {
                for i in 0..ast_node.block_items.len() {

                    // DEBUG
                    if self.debug {
                        println!("BlockItem {}:", i+1);
                    }

                    let index = ast_node.block_items.len()-1-i;
                    self.visit(&mut ast_node.block_items[index]);
                }
            }

            AstNodeType::BlockItem => {
                // LHS
                if let Some(left_node) = ast_node.lhs.as_mut() {
                    self.visit(left_node);
                }
            }

            AstNodeType::Conditional => {
            }

            AstNodeType::Compound => {
                // LHS
                if let Some(left_node) = ast_node.lhs.as_mut() {
                    self.visit(left_node);
                }
            }

            AstNodeType::While => {
            }

            AstNodeType::DoWhile => {
            }

            AstNodeType::For => {
                // LHS - initialization, e.g.: a = 0
                let lhs_ast_node_id = 0;
                if let Some(left_node) = ast_node.lhs.as_mut() {
                    self.visit(left_node);
                }

                // Expression - expression_ast_node, condition, e.g. a < 10
                if let Some(expression_node) = ast_node.expression.as_mut() {
                    self.visit(expression_node);
                }

                // RHS - post, e.g.: a = a + 1
                if let Some(right_node) = ast_node.rhs.as_mut() {
                    self.visit(right_node);
                }

                // BLOCK_ITEMS - instructions and declarations
                for i in 0..ast_node.block_items.len() {
                    let idx = ast_node.block_items.len()-1-i;
                    let block_item = ast_node.block_items[idx].as_mut();
                    self.visit(block_item);
                }
            }

            AstNodeType::FunctionCall => {
                // println!("{:?}", ast_node);
                let function_name = ast_node.string_val.clone();

                // DEBUG
                println!("FunctionCall to function using identifier: {:?}", function_name);

                // make sure, the identifier is a function call and not a variable
                if self.symbol_table.borrow_mut().contains(&function_name) {
                    let existing_symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&function_name);
                    if existing_symbol_table_entry.symbol_table_entry_type == SymbolTableEntryType::Variable {
                        panic!("[ERR] Symbol {} is not a function but a variable! Cannot call variable!", &function_name);
                    } else {
                        // DEBUG
                        println!("[OK] Symbol {} is a declared function! Checking arguments next!", &function_name);
                        // // PARAMETERS / ARGUMENTS
                        // for i in 0..ast_node.parameters.len() {
                        //     let parameter_ast_node = &ast_node.parameters[ast_node.parameters.len()-1-i];
                        //     print!("ARGUMENT_{}: {:?}", i, parameter_ast_node);
                        // }
                        // argument count must match
                        if ast_node.parameters.len() != existing_symbol_table_entry.parameter_count {
                            panic!("[ERR] Symbol {} is called with incorrect amount of arguments! Arguments expected: {}. Found: {}.", &function_name, existing_symbol_table_entry.parameter_count, ast_node.parameters.len());
                        } else {
                            // DEBUG
                            println!("[OK] Arguments to {} match! Arguments expected: {}. Found: {}.", &function_name, existing_symbol_table_entry.parameter_count, ast_node.parameters.len());
                            // TODO: type check each individual argument.
                            // the type of the actual argument has to match or be convertible to the formal parameter's data type!
                        }
                    }
                } else {
                    panic!("[ERR] Symbol {} is not contained in the symbol table! Cannot call undefined symbol!", &function_name);
                }
            }

            AstNodeType::StorageClassSpecifier => {
            }

            AstNodeType::Pointer => {
            }

            AstNodeType::Switch => {
            }

            AstNodeType::Case => {
            }

            AstNodeType::Default => {
            }

            AstNodeType::Break => {
            }

            AstNodeType::Continue => {
            }

            AstNodeType::EmptyStatement => {
            }

            AstNodeType::SingleInit => {
                // LHS
                if let Some(left_node) = ast_node.lhs.as_mut() {
                    self.visit(left_node);
                }
            }

            AstNodeType::CompoundInit => {
            }

            AstNodeType::Subscript => {
            }

            AstNodeType::MemberDeclaration => {
            }

            AstNodeType::Dot => {
            }

            AstNodeType::Arrow => {
            }

            AstNodeType::AssignmentOperator => {
            }

            AstNodeType::Cast => {
            }

            AstNodeType::Unknown => {
            }
        }
    }
}