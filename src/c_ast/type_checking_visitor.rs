use std::rc::Rc;

use std::cell::RefCell;

use std::collections::HashMap;

use std::str::FromStr;

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use crate::AstNode;
use crate::AstNodeType;

use crate::SymbolTable;
use crate::c_ast::ast_node::AstNodeOperatorType;
use crate::common::data_type::DataType;
use crate::common::data_type::DataType::DataTypeInt;
use crate::common::symbol_table::SymbolTableEntry;
use crate::common::symbol_table::SymbolTableEntryType;

// pub static AST_NODE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
use crate::c_ast::ast_node_id_counter::AST_NODE_ID_COUNTER;

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
    // node_map: HashMap::<usize, Rc<RefCell<AstNode>>>,
}

impl TypeCheckingVisitor {

    pub fn new(symbol_table_param: Rc<RefCell<SymbolTable>>) -> TypeCheckingVisitor {
        TypeCheckingVisitor {
            symbol_table: symbol_table_param,
            debug: false,
            // node_map: HashMap::<usize, Rc<RefCell<AstNode>>>::new(),
        }
    }

    pub fn print_symbol_table(&self) {
        self.symbol_table.borrow_mut().print_symbol_table();
    }

    pub fn get_node_type(&self, node_id: usize, node_map: &mut HashMap::<usize, AstNode>) -> AstNodeType {
        // self.get_node(node_id).node_type
        //AstNodeType::Program

        let node = node_map.get(&node_id).unwrap();
        return node.node_type.clone();
    }

    // pub fn get_node(&self, node_id: usize, node_map: &mut HashMap::<usize, AstNode>) -> AstNode {
    //     let node = node_map.get(&node_id).unwrap();
    //     return node;
    //     // return node.borrow().clone();
    //     // AstNode::new(0usize)
    // }

    pub fn insert_new_parent(&mut self, node_id: usize, node_map: &HashMap::<usize, AstNode>) -> HashMap::<usize, AstNode> {
        println!("test");

        node_map.clone()

        // //
        // // HOW DO I FIGHT THE BORROW-SCHMECKLER CORRECTLY?
        // //
        // // i have no idea! How about a round of supra mayro krat?
        // //

        // let mut temp_map = HashMap::<usize, Rc<RefCell<AstNode>>>::new();

        // let mut used_map = std::mem::replace(&mut node_map, temp_map);

        // // let data_type_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // let data_type_ast_node: AstNode = AstNode::new(300usize);
        // let rc = Rc::new(RefCell::new(data_type_ast_node));
        // used_map.insert(300usize, rc.clone());

        // let node = used_map.get(&node_id).unwrap();
        // node.borrow_mut().parent_id = Some(300usize);
        // // node.borrow_mut().insert_new_parent(&mut used_map);

        // let mut temp_map = std::mem::replace(&mut node_map, used_map);
    }

    pub fn retrieve_type(&mut self, ast_node: &AstNode) -> DataType {
        // check if the operand is a literal or a symbol
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
                    // // DEBUG
                    // println!("{:?}", symbol_table_entry);
                    return symbol_table_entry.data_type;
                } else {
                    todo!("NodeType: {:?}", ast_node.node_type);
                }
            }
            _ => {
                todo!("NodeType: {:?}", ast_node.node_type);
            }
        }
    }

    // Nora Sandler, page 254
    pub fn get_common_type(&self, type1: &DataType, type2: &DataType) -> DataType {
        if type1 == type2 {
            return *type1;
        }
        return DataType::DataTypeLong;
    }

    //pub fn visit(&mut self, ast_node: &mut AstNode: &mut HashMap::<usize, Rc<RefCell<AstNode>>>) {
    //pub fn visit(&mut self, ast_node_ref: &Rc<RefCell<AstNode>>: &mut HashMap::<usize, Rc<RefCell<AstNode>>>) {
    pub fn visit(&mut self, ast_node_id: usize, node_map: &HashMap::<usize, AstNode>) -> HashMap::<usize, AstNode> {

        let ast_node = node_map.get(&ast_node_id).unwrap();
        let mut node_map = node_map.clone();

        // // DEBUG
        // if self.debug {
        //     println!("[visit_ex()] {:?}", ast_node.node_type);
        // }

        // let mut ast_node = ast_node_ref.borrow_mut();

        // let ast_node = node_map.get(&ast_node_id).unwrap().borrow();
        //let mut ast_node = node_map.get_mut(&ast_node_id).unwrap().borrow_mut();
        // let mut ast_node = ast_node.clone();

        // let ast_node_type = self.get_node_type(ast_node_id, node_map);
        // let mut ast_node = self.get_node(ast_node_id);

        match ast_node.node_type {

            AstNodeType::Program => {
                // visit all items in the body of the program
                for i in 0..ast_node.block_items.len() {
                    let index = ast_node.block_items.len()-1-i;
                    let block_item_id = ast_node.block_items[index];
                    //let mut block_item_node = node_map.get(&block_item_id).unwrap().borrow();
                    //self.visit(&mut block_item_node);
                    // let block_item_node = node_map.get_mut(&block_item_id).unwrap();
                    node_map = self.visit(block_item_id, &node_map);
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
                if let Some(left_node_id) = ast_node.lhs {
                    // let mut left_node = node_map.get(&left_node_id).unwrap();
                    node_map = self.visit(left_node_id, &node_map);
                }
                // RHS
                if let Some(right_node_id) = ast_node.rhs {
                    // let mut right_node = node_map.get(&right_node_id).unwrap();
                    node_map = self.visit(right_node_id, &node_map);
                }
            }

            AstNodeType::Identifier => {
                // DEBUG
                if self.debug {
                    println!("{:?}", ast_node);
                }
            }

            AstNodeType::Return => {
                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    // let mut left_node = node_map.get(&left_node_id).unwrap();
                    node_map = self.visit(left_node_id, &node_map);
                }
            }

            AstNodeType::If => {
                // Expression
                if let Some(expression_node_id) = ast_node.expression {
                    // let mut expression_node = node_map.get(&expression_node_id).unwrap();
                    node_map = self.visit(expression_node_id, &node_map);
                }

                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    // let mut left_node = node_map.get(&left_node_id).unwrap();
                    node_map = self.visit(left_node_id, &node_map);
                }

                // RHS
                if let Some(right_node_id) = ast_node.rhs {
                    // let mut right_node = node_map.get(&right_node_id).unwrap();
                    node_map = self.visit(right_node_id, &node_map);
                }
            }

            AstNodeType::Unary => {
                // println!("Unary: {:?}", ast_node);

                // RHS
                if let Some(right_node_id) = ast_node.rhs {
                    // let right_node = node_map.get(&right_node_id).unwrap();
                    node_map = self.visit(right_node_id, &node_map);

                    // Nora Sandler, page 254. Not is always creating integer values 1 (= true), (0 = false)
                    match ast_node.operator_type {
                        AstNodeOperatorType::Not => {
                            // ast_node.analyzed_data_type = DataType::from_str("int").unwrap();

                            let mut ast_node_cloned = ast_node.clone();
                            ast_node_cloned.set_analyzed_data_type(DataType::DataTypeInt);

                            node_map.insert(ast_node_cloned.id, ast_node_cloned);
                        }
                        _ => {
                            //ast_node.analyzed_data_type = right_node.analyzed_data_type.clone();
                            // let right_node = self.get_node(right_node_id);
                            let right_node = node_map.get(&right_node_id).unwrap();
                            if !self.symbol_table.borrow_mut().contains(&right_node.string_val) {
                                panic!("Variable '{}' not contained!", &right_node.string_val);
                            }
                            let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&right_node.string_val);

                            // ast_node.analyzed_data_type = symbol_table_entry.data_type;
                            //ast_node.set_analyzed_data_type(symbol_table_entry.data_type);

                            panic!();
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

                let mut lhs_type = DataType::DataTypeUnknown;
                let mut rhs_type = DataType::DataTypeUnknown;

                // LHS
                if let Some(left_node_id) = ast_node.lhs {


                    // let mut left_node = node_map.get(&left_node_id).unwrap();
                    // let left_node = self.get_node(left_node_id);
                    let left_node = node_map.get(&left_node_id).unwrap().clone();
                    let node_map = self.visit(left_node_id, &node_map);

                    lhs_type = self.retrieve_type(&left_node);

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

                    // let mut right_node = node_map.get(&right_node_id).unwrap();
                    // let right_node = self.get_node(right_node_id);
                    let right_node = node_map.get(&right_node_id).unwrap().clone();
                    node_map = self.visit(right_node_id, &node_map);

                    rhs_type = self.retrieve_type(&right_node);

                    // check if a function name is used where a variable name should be used (e.g. b = a + foo)
                    if self.symbol_table.borrow_mut().contains(&right_node.string_val) {

                        let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&right_node.string_val);

                        // DEBUG
                        println!("{:?}", symbol_table_entry);

                        match symbol_table_entry.symbol_table_entry_type {
                            SymbolTableEntryType::Function => {
                                panic!("Function name used as a variable in an expression!");
                            }
                            _ => {

                            }
                        }
                    }
                }

                println!("LHS-Type: {}", lhs_type);
                println!("RHS-Type: {}", rhs_type);

                match ast_node.operator_type {

                    // Nora Sandler, page 255. Binary Logical-AND/OR has type int (1 == true, 0 == false)
                    AstNodeOperatorType::LogicalAnd | AstNodeOperatorType::LogicalOr => {

                        // ast_node.analyzed_data_type = DataType::DataTypeInt;
                        //ast_node.set_analyzed_data_type(DataType::DataTypeInt);
                        todo!();
                    }

                    _ => {
                        let common_data_type = self.get_common_type(&lhs_type, &rhs_type);

                        if let Some(left_node_id) = ast_node.lhs {

                            // let left_node = node_map.get(&left_node_id).unwrap().borrow();
                            // let left_node = self.get_node(left_node_id);

                            let mut left_node = node_map.get(&left_node_id).unwrap().clone();
                            lhs_type = self.retrieve_type(&left_node);
                            if lhs_type != common_data_type {

                                //
                                // insert a cast node into the AST!
                                //

                                let cast_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
                                println!("{}", cast_ast_node_id);
                                let mut cast_ast_node = AstNode::new(cast_ast_node_id);
                                cast_ast_node.node_type = AstNodeType::Cast;
                                cast_ast_node.lhs = Some(left_node.id);
                                cast_ast_node.string_val = "new_node test".to_string();
                                cast_ast_node.parent_id = Some(ast_node.id);
                                node_map.insert(cast_ast_node_id, cast_ast_node);

                                // the left node becomes child of the new middle node
                                left_node.parent_id = Some(cast_ast_node_id);
                                node_map.insert(left_node.id, left_node);

                                // println!("{:?}", ast_node); // parent
                                // println!("{:?}", cast_ast_node); // new middle
                                // println!("{:?}", left_node); // child

                                // clone parent, insert new LHS, replace parent in hashmap
                                let mut ast_node_clone = ast_node.clone();
                                ast_node_clone.lhs = Some(cast_ast_node_id);
                                node_map.insert(ast_node_clone.id, ast_node_clone);

                                // let t = node_map.get(&ast_node.id).unwrap();
                                // println!("{:?}", t);
                                // assert_eq!(t.lhs.unwrap(), 43usize);


                                // TODO
                                // let new_ast_node = left_node.insert_new_parent(&mut node_map);
                                // node_map.insert(new_ast_node.id, Rc::new(RefCell::new(new_ast_node)));

                                // return self.insert_new_parent(left_node.id, node_map);



                                // let new_ast_node = AstNode::new(300usize);
                                // node_map.insert(300usize, new_ast_node);

                                // println!("test {}", node_map.contains_key(&300usize));

                                // panic!();
                            }
                        }

                        if let Some(right_node_id) = ast_node.rhs {
                            let right_node = node_map.get(&right_node_id).unwrap();
                            rhs_type = self.retrieve_type(&right_node);
                            if rhs_type != common_data_type {
                                // insert a cast node into the AST!
                                // right_node.insert_new_parent(&mut node_map);
                                panic!();
                            }
                        }

                        // let converted_lhs_type = self.convert_to(lhs_type, common_data_type);
                        // let converted_rhs_type = self.convert_to(rhs_type, common_data_type);

                        // ast_node.analyzed_data_type =
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
                    // let mut left_node = node_map.get(&left_node_id).unwrap();
                    node_map = self.visit(left_node_id, &node_map);
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
                if let Some(function_name_id) = ast_node.function_name_ast_node {
                    let function_name_node = node_map.get(&function_name_id).unwrap();
                    temp_function_name = function_name_node.string_val.clone();
                }

                // parameters
                let temp_parameter_count = ast_node.parameters.len();
                for i in 0..ast_node.parameters.len() {
                    // let mut param_node = node_map.get(&ast_node.parameters[i]).unwrap();
                    node_map = self.visit(ast_node.parameters[i], &node_map);
                }

                // LHS - body of function == block with statements as block_items
                if let Some(left_node_id) = ast_node.lhs {
                    // let mut left_node = node_map.get(&left_node_id).unwrap();
                    node_map = self.visit(left_node_id, &node_map);

                    symbol_table_entry.has_body = true;
                }

                // RHS - Return type
                let mut temp_data_type_as_string = String::new();
                if let Some(right_node_id) = ast_node.rhs {
                    // println!("{:?}", right_node);

                    let right_node = node_map.get(&right_node_id).unwrap();

                    assert!(right_node.node_type == AstNodeType::DataType);

                    temp_data_type_as_string = right_node.string_val.clone();

                    node_map = self.visit(right_node_id, &node_map);
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
                if let Some(left_node_id) = ast_node.lhs {
                    let right_node = node_map.get(&left_node_id).unwrap();

                    // DEBUG
                    if self.debug {
                        print!("{:?}", right_node);
                    }

                    data_type = right_node.string_val.clone();
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
                    node_map = self.visit(expression_node_id, &node_map);
                }
            }

            AstNodeType::StructureDeclaration => {
            }

            AstNodeType::ParameterDeclaration => {
            }

            AstNodeType::Statement => {
                if let Some(left_node_id) = ast_node.lhs {
                    let left_node = node_map.get(&left_node_id).unwrap();
                    node_map = self.visit(left_node_id, &node_map);
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
                    let block_item_node = node_map.get(&block_item_id).unwrap();

                    node_map = self.visit(block_item_id, &node_map);
                }
            }

            AstNodeType::BlockItem => {
                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    let left_node = node_map.get(&left_node_id).unwrap();

                    node_map = self.visit(left_node_id, &node_map);
                }
            }

            AstNodeType::Conditional => {
            }

            AstNodeType::Compound => {
                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    // let mut left_node = node_map.get(&left_node_id).unwrap();

                    node_map = self.visit(left_node_id, &node_map);
                }
            }

            AstNodeType::While => {
            }

            AstNodeType::DoWhile => {
            }

            AstNodeType::For => {
                // LHS - initialization, e.g.: a = 0
                let lhs_ast_node_id = 0;
                if let Some(left_node_id) = ast_node.lhs {
                    // let mut left_node = node_map.get(&left_node_id).unwrap();
                    node_map = self.visit(left_node_id, &node_map);
                }

                // Expression - expression_ast_node, condition, e.g. a < 10
                if let Some(expression_node_id) = ast_node.expression {
                    // let mut expression_node = node_map.get(&expression_node_id).unwrap();
                    node_map = self.visit(expression_node_id, &node_map);
                }

                // RHS - post, e.g.: a = a + 1
                if let Some(right_node_id) = ast_node.rhs {
                    // let mut right_node = node_map.get(&right_node_id).unwrap();
                    node_map = self.visit(right_node_id, &node_map);
                }

                // BLOCK_ITEMS - instructions and declarations
                for i in 0..ast_node.block_items.len() {
                    let idx = ast_node.block_items.len()-1-i;
                    let block_item_id = ast_node.block_items[idx];
                    // let mut block_item_node = node_map.get(&block_item_id).unwrap();
                    node_map = self.visit(block_item_id, &node_map);
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
                if let Some(left_node_id) = ast_node.lhs {
                    // let mut left_node = node_map.get(&left_node_id).unwrap();
                    node_map = self.visit(left_node_id, &node_map);
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

            _ => {
                panic!();
            }
        }

        // let t = node_map.get(&27usize).unwrap();
        // println!("{:?}", t);

        return node_map;
    }
}