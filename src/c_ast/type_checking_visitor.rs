use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use std::str::FromStr;
use std::sync::atomic::Ordering;

use crate::AstNode;
use crate::AstNodeType;

use crate::SymbolTable;
use crate::c_ast::ast_node::AstNodeOperatorType;
use crate::c_ast::ast_node_id_counter::AST_NODE_ID_COUNTER;
use crate::common::data_type::DataType;
use crate::common::data_type::DataType::DataTypeInt;
use crate::common::data_type::DataType::DataTypeUnknown;
use crate::common::symbol_table::SymbolTableEntry;
use crate::common::symbol_table::SymbolTableEntryType;

//
// Nora Sandler, page 178
//
// Every variable identifier has a type (int, long, double, float, char, byte, ...)
// A variable can also be an array or a struct.
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

    pub fn new(
        symbol_table_param: Rc<RefCell<SymbolTable>>) -> TypeCheckingVisitor {
        TypeCheckingVisitor {
            symbol_table: symbol_table_param,
            debug: true,
        }
    }

    pub fn print_symbol_table(&self) {
        self.symbol_table.borrow_mut().print_symbol_table();
    }

    pub fn retrieve_type(&self,
        ast_node: &AstNode,
        node_map: &Box<HashMap<usize, AstNode>>)
        -> DataType
    {
        match ast_node.node_type {
            AstNodeType::ConstInt => {
                return DataType::DataTypeInt;
            }
            AstNodeType::ConstUInt => {
                return DataType::DataTypeUnsignedInt;
            }
            AstNodeType::ConstLong => {
                return DataType::DataTypeLong;
            }
            AstNodeType::ConstULong => {
                return DataType::DataTypeUnsignedLong;
            }
            AstNodeType::Identifier => {
                if self.symbol_table.borrow_mut().contains(&ast_node.string_val) {
                    let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&ast_node.string_val);
                    // DEBUG
                    if self.debug {
                        println!("{:?}", symbol_table_entry);
                    }
                    return symbol_table_entry.data_type;
                } else {
                    todo!("NodeType: {:?}", ast_node.node_type);
                }
            }
            AstNodeType::Unary => {

                //
                // Assumption: translate file pointers_5.c
                // In pointers_5.c, the unary node contains a AddrOf() subnode
                //

                // RHS variable name
                let mut pointer_data_type: DataType = DataTypeUnknown;
                if let Some(right_node_id) = ast_node.rhs {
                    let right_node = node_map.get(&right_node_id).unwrap();

                    println!("Looking up '{}' in symbol table", right_node.string_val);

                    if !self.symbol_table.borrow_mut().contains(&right_node.string_val) {
                        panic!("Variable '{}' not contained!", &right_node.string_val);
                    }

                    let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&right_node.string_val);


                    pointer_data_type = symbol_table_entry.data_type;
                }

                // LHS - this is the AddrOf node which shows that this is a pointer
                if let Some(left_node_id) = ast_node.lhs {
                    let left_node = node_map.get(&left_node_id).unwrap();
                    // let lhs_type = self.retrieve_type(left_node, node_map);
                    match left_node.operator_type {
                        AstNodeOperatorType::AddrOf => {
                            return DataType::DataTypePointer(Box::new(pointer_data_type));
                        }
                        AstNodeOperatorType::Dereference => {
                            return pointer_data_type;
                        }
                        _ => {
                            todo!("Unhandled: {}", left_node.operator_type);
                        }
                    }
                }
                todo!("Unary: {:?}", ast_node.node_type);
            }
            // AstNodeType::Deref => {
            // }
            _ => {
                todo!("NodeType: {:?}", ast_node.node_type);
            }
        }
    }

    pub fn get_common_type(&self, type1: &DataType, type2: &DataType) -> DataType {
        // DESCRIPTION:
        //
        // Nora Sandler, page 254
        if type1 == type2 {
            return type1.clone();
        }
        return DataType::DataTypeLong;
    }

    pub fn visit(&mut self, ast_node_id: usize, node_map: &mut Box<HashMap<usize, AstNode>>) {

        let mut ast_node = AstNode::new(0);

        {
            ast_node = node_map.get(&ast_node_id).unwrap().clone();
        }

        // DEBUG
        if self.debug {
            println!("[visit_ex()] {:?}", ast_node.node_type);
        }

        match ast_node.node_type {

            AstNodeType::Program => {
                // visit all items in the body of the program
                for i in 0..ast_node.block_items.len() {
                    let index = ast_node.block_items.len()-1-i;
                    let block_item_id = ast_node.block_items[index];
                    self.visit(block_item_id, node_map);
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
                panic!("test");
            }

            AstNodeType::Expression => {
                // DEBUG
                if self.debug {
                    println!("{:?}", ast_node);
                }

                assert!(ast_node.lhs.is_some());

                let mut lhs_type = DataType::DataTypeUnknown;
                let mut rhs_type = DataType::DataTypeUnknown;

                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map);

                    let left_node = node_map.get(&left_node_id).unwrap();
                    lhs_type = self.retrieve_type(left_node, node_map);
                }
                // RHS
                let mut right_node = AstNode::new(0);
                if let Some(right_node_id) = ast_node.rhs {
                    self.visit(right_node_id, node_map);

                    right_node = node_map.get(&right_node_id).unwrap().clone();
                    rhs_type = self.retrieve_type(&right_node, node_map);
                }

                // not all expressions have a RHS. If there is a RHS, check types and insert cast if needed
                if ast_node.rhs.is_some() {

                    assert_ne!(lhs_type, DataTypeUnknown);
                    assert_ne!(rhs_type, DataTypeUnknown);

                    if lhs_type != rhs_type {
                        // DEBUG
                        if self.debug {
                            println!("lhs_type: {:?}, rhs_type: {:?}", lhs_type, rhs_type);
                        }

                        // insert a cast node into the AST!
                        let cast_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
                        let mut cast_ast_node = AstNode::new(cast_ast_node_id);
                        cast_ast_node.node_type = AstNodeType::Cast;
                        cast_ast_node.lhs = Some(ast_node.rhs.unwrap()); // LHS is the variable or literal (value element) which needs casting
                        cast_ast_node.analyzed_data_type = lhs_type; // analyzed_data_type is the type to cast into
                        cast_ast_node.parent_id = Some(ast_node.id);
                        node_map.insert(cast_ast_node_id, cast_ast_node);

                        // the left node becomes child of the new middle node
                        right_node.parent_id = Some(cast_ast_node_id);
                        node_map.insert(right_node.id, right_node);

                        // // DEBUG
                        // if self.debug {
                        //     println!("{}", cast_ast_node_id);
                        //     println!("{:?}", ast_node); // parent
                        //     println!("{:?}", cast_ast_node); // new middle
                        //     println!("{:?}", left_node); // child
                        // }

                        // clone parent, insert new LHS, replace parent in hashmap
                        let mut ast_node_clone = ast_node.clone();
                        ast_node_clone.rhs = Some(cast_ast_node_id);
                        node_map.insert(ast_node_clone.id, ast_node_clone);
                    }
                }
            }

            AstNodeType::Identifier => {
                // DEBUG
                if self.debug {
                    println!("{:?}", ast_node);
                }

                if !self.symbol_table.borrow_mut().contains(&ast_node.string_val) {
                    panic!("Variable '{}' not contained!", &ast_node.string_val);
                }
                let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&ast_node.string_val);
                ast_node.analyzed_data_type = symbol_table_entry.data_type;

                assert_ne!(ast_node.analyzed_data_type, DataType::DataTypeUnknown);
            }

            AstNodeType::Return => {
                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map);

                    let left_node = node_map.get(&left_node_id).unwrap();

                    if !self.symbol_table.borrow_mut().contains(&left_node.string_val) {
                        panic!("Variable '{}' not contained!", &left_node.string_val);
                    }
                    let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&left_node.string_val);
                    ast_node.analyzed_data_type = symbol_table_entry.data_type;

                    // the cloned node has been changed. Replace the original node in the node_map to preserve the change
                    node_map.insert(ast_node.id, ast_node);
                }
            }

            AstNodeType::If => {
                // Expression
                if let Some(expression_node_id) = ast_node.expression {
                    self.visit(expression_node_id, node_map);
                }

                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map);
                }

                // RHS
                if let Some(right_node_id) = ast_node.rhs {
                    self.visit(right_node_id, node_map);
                }
            }

            AstNodeType::Unary => {
                // println!("Unary: {:?}", ast_node);

                // RHS
                if let Some(right_node_id) = ast_node.rhs {
                    self.visit(right_node_id, node_map);

                    let right_node = node_map.get(&right_node_id).unwrap();

                    // Nora Sandler, page 254. Not is always creating integer values 1 (= true), (0 = false)
                    match ast_node.operator_type {
                        AstNodeOperatorType::Not => {
                            ast_node.analyzed_data_type = DataType::from_str("int").unwrap();
                        }
                        _ => {
                            if !self.symbol_table.borrow_mut().contains(&right_node.string_val) {
                                panic!("Variable '{}' not contained!", &right_node.string_val);
                            }
                            let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&right_node.string_val);
                            ast_node.analyzed_data_type = symbol_table_entry.data_type;

                            // the cloned node has been changed. Replace the original node in the node_map to preserve the change
                            node_map.insert(ast_node.id, ast_node);
                        }
                    }

                    // println!("Unary: {:?}", ast_node);
                }
            }

            AstNodeType::Binary => {

                // DEBUG
                if self.debug {
                    println!("{:?}", ast_node);
                }

                let mut lhs_type = DataType::DataTypeUnknown;
                let mut rhs_type = DataType::DataTypeUnknown;

                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map);

                    let left_node = node_map.get(&left_node_id).unwrap();

                    lhs_type = self.retrieve_type(left_node, node_map);

                    // check if a function name is used where a variable name should be used (e.g. b = foo + a)
                    if self.symbol_table.borrow_mut().contains(&left_node.string_val) {

                        let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&left_node.string_val);

                        // DEBUG
                        println!("{:?}", symbol_table_entry);

                        match symbol_table_entry.symbol_table_entry_type {
                            SymbolTableEntryType::Function => {
                                panic!("Function name used as a variable in an expression!");
                            }
                            _ => {

                            }
                        }

                        // retrieve LHS data type from symbol table
                        lhs_type = symbol_table_entry.data_type;
                    }
                }

                // RHS
                if let Some(right_node_id) = ast_node.rhs {
                    self.visit(right_node_id, node_map);

                    let right_node = node_map.get(&right_node_id).unwrap();

                    rhs_type = self.retrieve_type(right_node, node_map);

                    // check if a function name is used where a variable name should be used (e.g. b = a + foo)
                    if self.symbol_table.borrow_mut().contains(&right_node.string_val) {

                        let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&right_node.string_val);

                        // DEBUG
                        if self.debug {
                            println!("{:?}", symbol_table_entry);
                        }

                        match symbol_table_entry.symbol_table_entry_type {
                            SymbolTableEntryType::Function => {
                                panic!("Function name used as a variable in an expression!");
                            }
                            _ => {

                            }
                        }
                    }
                }

                // DEBUG
                if self.debug {
                    println!("LHS-Type: {}", lhs_type);
                    println!("RHS-Type: {}", rhs_type);
                }

                match ast_node.operator_type {

                    // Nora Sandler, page 255. Binary Logical-AND/OR has type int (1 == true, 0 == false)
                    AstNodeOperatorType::LogicalAnd | AstNodeOperatorType::LogicalOr => {
                        ast_node.analyzed_data_type = DataType::DataTypeInt;
                    }

                    _ => {
                        let common_data_type = self.get_common_type(&lhs_type, &rhs_type);

                        if let Some(left_node_id) = ast_node.lhs {
                            let mut left_node = node_map.get(&left_node_id).unwrap().clone();
                            if left_node.analyzed_data_type != common_data_type {

                                // insert a cast node into the AST!
                                let cast_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
                                let mut cast_ast_node = AstNode::new(cast_ast_node_id);
                                cast_ast_node.node_type = AstNodeType::Cast;
                                cast_ast_node.lhs = Some(left_node.id); // LHS is the variable or literal (value element) which needs casting
                                cast_ast_node.analyzed_data_type = common_data_type.clone(); // analyzed_data_type is the type to cast into
                                cast_ast_node.parent_id = Some(ast_node.id);
                                node_map.insert(cast_ast_node_id, cast_ast_node);

                                // the left node becomes child of the new middle node
                                left_node.parent_id = Some(cast_ast_node_id);

                                // // DEBUG
                                // if self.debug {
                                //     println!("{}", cast_ast_node_id);
                                //     println!("{:?}", ast_node); // parent
                                //     println!("{:?}", cast_ast_node); // new middle
                                //     println!("{:?}", left_node); // child
                                // }

                                // clone parent, insert new LHS, replace parent in hashmap
                                let mut ast_node_clone = ast_node.clone();
                                ast_node_clone.lhs = Some(cast_ast_node_id);
                                node_map.insert(ast_node_clone.id, ast_node_clone);
                            }
                        }

                        if let Some(right_node_id) = ast_node.rhs {
                            let right_node = node_map.get(&right_node_id).unwrap();
                            if right_node.analyzed_data_type != common_data_type {
                                // panic!();
                            }
                        }
                    }
                }
            }

            AstNodeType::Operator => {
            }

            AstNodeType::PrefixOperator => {
            }

            AstNodeType::DataType => {
            }

            AstNodeType::Declaration => {
                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map);
                }
            }

            // Structure:
            // RHS - return type
            // string_val - function name
            // parameters - the parameters
            // LHS - is a block which contains all instructions in it's block_items field
            AstNodeType::FunctionDeclaration => {

                let mut temp_function_name = String::new();
                let mut temp_parameter_count = 0;

                let mut symbol_table_entry = SymbolTableEntry::new();
                symbol_table_entry.symbol_table_entry_type = SymbolTableEntryType::Function;

                {
                    // function name
                    if let Some(function_name_id) = ast_node.function_name_ast_node {
                        let function_name_node = node_map.get(&function_name_id).unwrap();
                        temp_function_name = function_name_node.string_val.clone();
                    }
                }

                {
                    // parameters
                    temp_parameter_count = ast_node.parameters.len();
                    for i in 0..ast_node.parameters.len() {
                        let parameter_node_id = ast_node.parameters[i];
                        self.visit(parameter_node_id, node_map);
                    }
                }

                {
                    // LHS - body of function == block with statements as block_items
                    if let Some(left_node_id) = ast_node.lhs {
                        self.visit(left_node_id, node_map);

                        symbol_table_entry.has_body = true;
                    }
                }

                // RHS - Return type
                let mut temp_data_type_as_string = String::new();
                if let Some(right_node_id) = ast_node.rhs {

                    let right_node = node_map.get(&right_node_id).unwrap();

                    // DEBUG
                    if self.debug {
                        println!("{:?}", right_node);
                    }

                    assert!(right_node.node_type == AstNodeType::DataType);
                    temp_data_type_as_string = right_node.string_val.clone();

                    self.visit(right_node_id, node_map);
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

                let mut symbol_table_entry = SymbolTableEntry::new();
                symbol_table_entry.symbol_table_entry_type = SymbolTableEntryType::Variable;

                // data type
                if let Some(left_node_id) = ast_node.lhs {
                    let left_node = node_map.get(&left_node_id).unwrap();
                    // DEBUG
                    if self.debug {
                        print!("{:?}", left_node);
                    }
                    let data_type = left_node.string_val.clone();
                        let data_type_as_enum = match DataType::from_str(&data_type) {
                        Ok(data_type_result) => data_type_result,
                        Err(e) => panic!("should be valid DataType: {e}"),
                    };
                    symbol_table_entry.data_type = data_type_as_enum;
                    symbol_table_entry.is_array = left_node.node_type == AstNodeType::Array;
                }

                // identifier (RHS)
                let mut varname = String::from("");
                if let Some(right_node_id) = ast_node.rhs {
                    let right_node = node_map.get(&right_node_id).unwrap();
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
                if let Some(expression_node_id) = ast_node.expression {
                    let expression_node = node_map.get(&expression_node_id).unwrap();
                    // DEBUG
                    if self.debug {
                        print!("{:?}", expression_node);
                    }
                    self.visit(expression_node_id, node_map);
                }
            }

            AstNodeType::StructureDeclaration => {
            }

            AstNodeType::ParameterDeclaration => {
            }

            AstNodeType::Statement => {
                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map);
                }
            }

            AstNodeType::Block => {
                for i in 0..ast_node.block_items.len() {
                    // DEBUG
                    if self.debug {
                        println!("BlockItem {}:", i+1);
                    }
                    let index = ast_node.block_items.len()-1-i;
                    let block_item_id = ast_node.block_items[index];
                    self.visit(block_item_id, node_map);
                }
            }

            AstNodeType::BlockItem => {
                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map);
                }
            }

            AstNodeType::Conditional => {
            }

            AstNodeType::Compound => {
                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map);
                }
            }

            AstNodeType::While => {
            }

            AstNodeType::DoWhile => {
            }

            AstNodeType::For => {
                // LHS - initialization, e.g.: a = 0
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map);
                }

                // Expression - expression_ast_node, condition, e.g. a < 10
                if let Some(expression_node_id) = ast_node.expression {
                    self.visit(expression_node_id, node_map);
                }

                // RHS - post, e.g.: a = a + 1
                if let Some(right_node_id) = ast_node.rhs {
                    self.visit(right_node_id, node_map);
                }

                // BLOCK_ITEMS - instructions and declarations
                for i in 0..ast_node.block_items.len() {
                    let idx = ast_node.block_items.len()-1-i;
                    let block_item_id = ast_node.block_items[idx];
                    self.visit(block_item_id, node_map);
                }
            }

            AstNodeType::FunctionCall => {
                // DEBUG
                if self.debug {
                    println!("{:?}", ast_node);
                }

                let function_name = ast_node.string_val.clone();

                // DEBUG
                if self.debug {
                    println!("FunctionCall to function using identifier: {:?}", function_name);
                }

                // make sure, the identifier is a function call and not a variable
                if self.symbol_table.borrow_mut().contains(&function_name) {
                    let existing_symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&function_name);
                    if existing_symbol_table_entry.symbol_table_entry_type == SymbolTableEntryType::Variable {
                        panic!("[ERR] Symbol {} is not a function but a variable! Cannot call variable!", &function_name);
                    } else {

                        // DEBUG
                        if self.debug {
                            // DEBUG
                            println!("[OK] Symbol {} is a declared function! Checking arguments next!", &function_name);
                            // PARAMETERS / ARGUMENTS
                            for i in 0..ast_node.parameters.len() {
                                let parameter_ast_node = &ast_node.parameters[ast_node.parameters.len()-1-i];
                                print!("ARGUMENT_{}: {:?}", i, parameter_ast_node);
                            }
                        }

                        // argument count must match
                        if ast_node.parameters.len() != existing_symbol_table_entry.parameter_count {
                            panic!("[ERR] Symbol {} is called with incorrect amount of arguments! Arguments expected: {}. Found: {}.", &function_name, existing_symbol_table_entry.parameter_count, ast_node.parameters.len());
                        } else {
                            // DEBUG
                            if self.debug {
                                println!("[OK] Arguments to {} match! Arguments expected: {}. Found: {}.", &function_name, existing_symbol_table_entry.parameter_count, ast_node.parameters.len());
                            }
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
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map);
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