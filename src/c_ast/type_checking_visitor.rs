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
use crate::common::struct_decl::StructDecl;
use crate::common::symbol_table::SymbolTableEntry;
use crate::common::symbol_table::SymbolTableEntryType;

//
// Nora Sandler, page 178
//
// Every variable identifier has a type (int, long, double, float, char, byte, ...)
// A variable can also be an array or a struct, union.
//
// Every function identifier has a return type and a fixed number of arguments
// (exception: variadic functions)
// A function cannot be declared with a function body more than once!
//
// It is not possible to call a variable like a function (function pointers???)
//
// The purpose of type checking is that all declarations of identifiers and all
// usages have compatible types!
//

pub struct TypeCheckingVisitor {
    // https://www.youtube.com/watch?v=8O0Nt9qY_vo
    symbol_table: Rc<RefCell<SymbolTable>>,
    type_system: Rc<RefCell<HashMap::<String, DataType>>>,
    debug: bool,
}

impl TypeCheckingVisitor {

    pub fn new(
        symbol_table_param: Rc<RefCell<SymbolTable>>,
        type_system_param: Rc<RefCell<HashMap::<String, DataType>>>)
        -> TypeCheckingVisitor
    {
        TypeCheckingVisitor {
            symbol_table: symbol_table_param,
            type_system: type_system_param,
            debug: false,
        }
    }

    pub fn print_symbol_table(&self) {
        self.symbol_table.borrow().print_symbol_table();
    }

    pub fn retrieve_data_type_from_node_type(&self,
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
                    // if self.debug {
                        println!("[TypeCheckingVisitor] {:?}", symbol_table_entry);
                    // }
                    return symbol_table_entry.data_type.clone();
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
                let mut pointer_data_type: DataType = DataType::DataTypeUnknown;
                if let Some(right_node_id) = ast_node.rhs {
                    let right_node = node_map.get(&right_node_id).unwrap();

                    // DEBUG
                    if self.debug {
                        println!("Looking up '{}' in symbol table", right_node.string_val);
                    }

                    if !self.symbol_table.borrow_mut().contains(&right_node.string_val) {
                        panic!("Variable '{}' not contained in symbol table!", &right_node.string_val);
                    }

                    let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&right_node.string_val);

                    pointer_data_type = symbol_table_entry.data_type;
                }

                // LHS - this is the AddrOf node which shows that this is a pointer
                if let Some(left_node_id) = ast_node.lhs {
                    let left_node = node_map.get(&left_node_id).unwrap();
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

            AstNodeType::Subscript => {
                // as a type, return the data type of the subscripted array.
                // The subscripted array is contained in the LHS

                // LHS - contains the array
                if let Some(left_node_id) = ast_node.lhs {

                    // retrieve node by id
                    let left_node = node_map.get(&left_node_id).unwrap();

                    // println!("var_name: {}", left_node.string_val);

                    // DEBUG
                    if self.debug {
                        println!("Looking up '{}' in symbol table", left_node.string_val);
                    }

                    if !self.symbol_table.borrow_mut().contains(&left_node.string_val) {
                        panic!("Variable '{}' not contained in symbol table!", &left_node.string_val);
                    }

                    let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&left_node.string_val);

                    // needs to have a type
                    assert_ne!(symbol_table_entry.data_type, DataType::DataTypeUnknown);

                    return symbol_table_entry.data_type;
                }

                panic!("Cannot determine type for node:{}", &ast_node.lhs.unwrap());
            }

            AstNodeType::Binary => {
                // as a type, return the common type of both operands
                //
                // HINT: There might have been casts inserted by this code (type_checking_visitor)
                // during the visit-calls and if you check the .dot file you will not see the
                // inserted casts unless this code has run! This means during debugging, the
                // .dot file on your hard drive is stale and only contains data from the last
                // successfull run, but not from the current debugging session!

                if self.debug {
                    println!("ast_node.id: {:?}", ast_node.id);
                    println!("{:?}", ast_node);
                }

                // LHS
                let mut lhs_data_type = DataType::DataTypeUnknown;
                if let Some(left_node_id) = ast_node.lhs {

                    // println!("LHS:ID:{}", left_node_id);

                    // retrieve node by id
                    let left_node = node_map.get(&left_node_id).unwrap();
                    match left_node.node_type {

                        AstNodeType::ConstUInt => {
                            lhs_data_type = DataType::DataTypeUnsignedInt;
                        }

                        AstNodeType::Cast => {
                            lhs_data_type = left_node.get_analyzed_data_type();
                        }

                        AstNodeType::Identifier => {
                            let lhs_symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&left_node.string_val);
                            lhs_data_type = lhs_symbol_table_entry.data_type;
                        }

                        AstNodeType::Subscript => {
                            lhs_data_type = left_node.get_analyzed_data_type();
                        }

                        _ => {
                            panic!("Unhandled type: {:?}", left_node.node_type);
                        }
                    }

                    // println!("{:?}", left_node);
                    // println!("{}", &left_node.string_val);
                }
                // LHS needs to have a type
                assert_ne!(lhs_data_type, DataType::DataTypeUnknown);

                // RHS
                let mut rhs_data_type = DataType::DataTypeUnknown;
                if let Some(right_node_id) = ast_node.rhs {

                    // retrieve node by id
                    let right_node = node_map.get(&right_node_id).unwrap();

                    match right_node.node_type {

                        AstNodeType::Cast => {
                            rhs_data_type = right_node.get_analyzed_data_type();
                        }

                        AstNodeType::Identifier => {
                            let rhs_symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&right_node.string_val);
                            rhs_data_type = rhs_symbol_table_entry.data_type;
                        }

                        AstNodeType::ConstUInt => {
                            rhs_data_type = DataType::DataTypeUnsignedInt;
                        }

                        _ => {
                            panic!("Unhandled type: {:?}", right_node.node_type);
                        }
                    }
                }
                // RHS needs to have a type
                assert_ne!(rhs_data_type, DataType::DataTypeUnknown);

                let common_type = self.get_common_type(&lhs_data_type, &rhs_data_type);

                // ast_node.analyzed_data_type = common_type.clone();

                common_type
            }

            AstNodeType::Dot | AstNodeType::Arrow => {

                println!("-------------------------------------------------------------");
                println!("ast_node_id: {}", ast_node.id);

                // Assumption !!!!!
                // code is: struct.struct_field
                // retrieve the type of struct_field and return
                // it as data_type

                // let mut lhs_data_type = DataType::DataTypeUnknown;

                let mut struct_variable_name = String::from("");
                let mut struct_field_name = String::from("");

                // LHS - the struct variable name
                if let Some(left_node_id) = ast_node.lhs {
                    // println!("LHS:ID:{}", left_node_id);

                    // retrieve node by id
                    let left_node = node_map.get(&left_node_id).unwrap();

                    println!("{} {:?}", left_node.id, left_node);

                    struct_variable_name = left_node.string_val.clone();

                    // match left_node.node_type {
                    //     _ => {
                    //     }
                    // }
                }

                // LHS - the struct variable name
                if let Some(right_node_id) = ast_node.rhs {

                    // retrieve node by id
                    let left_node = node_map.get(&right_node_id).unwrap();

                    struct_field_name = left_node.string_val.clone();
                }

                println!("struct:'{}', field:'{}'", struct_variable_name, struct_field_name);

                // println!("print_symbol_table() ------------------------------------------------------------");
                // let mut index = 0;
                // for (key, value) in self.symbol_table.borrow_mut().into_iter() {
                //     println!("{}) {} / {:?}", index, key, value);
                //     // println!("{} / {:?}", key, value.data_type);
                //     index = index + 1;
                // }
                // println!("---------------------------------------------------------------------------------");

                // DEBUG
                self.symbol_table.borrow_mut().print_symbol_table();

                // // replace original variable name with new variable name
                // let new_string_val = match self.variable_naming_source.borrow_mut().get_replaced_variable_name(&user_choosen_variable_name) {
                //     Ok(var_name) => var_name,
                //     Err(e) => {
                //         user_choosen_variable_name.clone()
                //         //panic!("{}", e);
                //     }
                // };

                // resolve variable name via the symbol table
                let struct_symbol_option = self.symbol_table.borrow_mut().get(&struct_variable_name);
                let struct_symbol = struct_symbol_option.expect(format!("No symbol for '{}' found in symbol table!\n", struct_variable_name).as_str());
                let struct_data_type: DataType = struct_symbol.data_type;

                // self.type_system.borrow_mut().get(struct_data_type.);
                match struct_data_type {

                    DataType::DataTypeStruct(struct_decl) => {
                        // println!("struct_decl");

                        // find struct member that matches the RHS of the DOT
                        let field_index_option = struct_decl.members.iter()
                            .position(|m| m.member_name == struct_field_name);
                        let field_index = field_index_option.expect("Field not found in struct!");
                        let field = struct_decl.members.get(field_index);
                        let struct_decl_member = field.unwrap();

                        return struct_decl_member.data_type.clone();
                    }

                    DataType::DataTypePointer(data_type_box) => {
                        return *data_type_box.clone();
                    }

                    _ => {
                        panic!("Unhandled DataType: {}", struct_data_type);
                    }
                }
            }

            AstNodeType::Return => {
                // DEBUG
                if self.debug {
                    println!("------------------------------------------------");
                    println!("{} {:?}", ast_node.id, ast_node);
                }

                todo!();

                // let lhs_node_id = ast_node.lhs.unwrap();

                // // let lhs_node = node_map.get(ast_node.lhs.unwrap()).unwrap();

                // return self.visit(lhs_node_id, &mut node_map, &DataType::DataTypeUnknown);
                // // return lhs_node.analyzed_data_type;
            }

            _ => {
                println!("{} {:?}", ast_node.id, ast_node);
                todo!("NodeType: {:?}", ast_node.node_type);
            }
        }
    }

    pub fn get_common_type(&self, lhs_type: &DataType, rhs_type: &DataType) -> DataType {

        // DEBUG
        if self.debug {
            println!("LHS-Type: {}", lhs_type);
            println!("RHS-Type: {}", rhs_type);
        }

        // DESCRIPTION:
        //
        // Nora Sandler, page 254
        if lhs_type == rhs_type {
            return lhs_type.clone();
        }

        // WBI: signed + unsigned = signed
        if *lhs_type == DataType::DataTypeInt && *rhs_type == DataType::DataTypeUnsignedInt {
            return DataType::DataTypeInt;
        }
        if *lhs_type == DataType::DataTypeUnsignedInt && *rhs_type == DataType::DataTypeInt {
            return DataType::DataTypeInt;
        }

        return DataType::DataTypeLong;
    }

    pub fn visit(&mut self,
        ast_node_id: usize,
        node_map: &mut Box<HashMap<usize, AstNode>>,
        expected_return_data_type: &DataType) -> DataType
    {
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
                let mut data_type: DataType = DataType::DataTypeUnknown;
                // visit all items in the body of the program
                for i in 0..ast_node.block_items.len() {
                    let index = ast_node.block_items.len()-1-i;
                    let block_item_id = ast_node.block_items[index];
                    data_type = self.visit(block_item_id, node_map, expected_return_data_type);
                }
                return data_type;
            }

            AstNodeType::ConstInt => {
                return DataType::DataTypeInt;
            }

            AstNodeType::ConstLong => {
                return DataType::DataTypeLong;
            }

            AstNodeType::ConstUInt => {
                return DataType::DataTypeUnsignedInt;
            }

            AstNodeType::ConstULong => {
                return DataType::DataTypeUnsignedLong;
            }

            AstNodeType::ConstDouble => {
                return DataType::DataTypeDouble;
            }

            AstNodeType::Structure => {
                todo!();
            }

            AstNodeType::Array => {
                todo!();
            }

            AstNodeType::Expression => {
                // DEBUG
                if self.debug {
                    println!("------------------------------------------------");
                    println!("{} {:?}", ast_node.id, ast_node);
                }

                assert!(ast_node.lhs.is_some());

                // LHS - data type
                let mut lhs_data_type = DataType::DataTypeUnknown;
                let mut left_node = AstNode::new(0);
                if let Some(left_node_id) = ast_node.lhs {

                    lhs_data_type = self.visit(left_node_id, node_map, expected_return_data_type);
                    println!("lhs_data_type: {:?}", lhs_data_type);

                    // retrieve
                    left_node = node_map.get(&left_node_id).unwrap().clone();
                }

                // RHS - variable name or const/literal value
                let mut rhs_data_type = DataType::DataTypeUnknown;
                let mut rhs_variable_name = String::new();
                let mut right_node = AstNode::new(0);
                if let Some(right_node_id) = ast_node.rhs {

                    // visit the right side - This might change the AST by inserting cast() nodes
                    rhs_data_type = self.visit(right_node_id, node_map, expected_return_data_type);

                    match rhs_data_type {

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
                        DataType::DataTypeDouble |
                        DataType::DataTypeVoid => {
                            return rhs_data_type;
                        }

                        _ => {
                            // todo!();
                        }

                    }

                    right_node = node_map.get(&right_node_id).unwrap().clone();
                    // rhs_type = self.retrieve_data_type_from_node_type(&right_node, node_map);

                    // process variable name and resolve it's data type
                    rhs_variable_name = right_node.string_val.clone();
                }

                //
                // check types and insert cast if needed
                //

                match lhs_data_type {

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
                    DataType::DataTypeDouble |
                    DataType::DataTypeVoid => {
                        return rhs_data_type;
                    }

                    _ => {
                        //panic!("");

                        //assert_ne!(lhs_type, DataTypeUnknown);
                        //assert_ne!(rhs_type, DataTypeUnknown);

                        // DEBUG
                        //if self.debug {
                            println!("------------------------------------------------");
                            println!("{} {:?}", ast_node.id, ast_node);
                        //}

                        // DEBUG
                        // if self.debug {
                            println!("lhs_type: {:?}, rhs_type: {:?}", lhs_data_type, rhs_data_type);
                        // }

                        //
                        // Check for and insert CAST
                        //

                        if rhs_data_type != DataType::DataTypeUnknown && lhs_data_type != rhs_data_type {

                            // DEBUG
                            if self.debug {
                                println!("lhs_type: {:?}, rhs_type: {:?}", lhs_data_type, rhs_data_type);
                            }

                            // insert a cast node into the AST!
                            let cast_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

                            // DEBUG
                            if self.debug {
                                println!("{}", cast_ast_node_id);
                            }

                            let mut cast_ast_node = AstNode::new(cast_ast_node_id);
                            cast_ast_node.node_type = AstNodeType::Cast;
                            cast_ast_node.lhs = Some(ast_node.rhs.unwrap()); // LHS is the variable or literal (value element) which needs casting
                            cast_ast_node.set_analyzed_data_type(lhs_data_type.clone()); // analyzed_data_type is the type to cast into
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

                        let symbol_table_entry_lhs_option = self.symbol_table.borrow_mut().get(&rhs_variable_name);
                        let symbol_table_entry_lhs = symbol_table_entry_lhs_option.expect(format!("No symbol for '{}' found in symbol table!\n", rhs_variable_name).as_str());

                        // DEBUG
                        if self.debug {
                            println!("symbol_table_entry_lhs: {:?}", symbol_table_entry_lhs);
                        }

                        // Nora Sandler, page 399
                        //
                        // if the RHS of the assignment is an Array, then the type checker has to insert
                        // a new AddrOf node into the AST!
                        //
                        // The AddrOf node has to have the type "Pointer to Array-Element-Type"
                        // and the purpose of this node is to implicitly convert an array to a pointer to the first
                        // element of said array, when the array is used in a pointer assignment like so:
                        //
                        // array_4.c
                        // "int *my_pointer = my_array;"

                        //
                        // insert ADDR OF operator
                        //

                        if symbol_table_entry_lhs.is_array {

                            let insert_node: bool = false;
                            if insert_node {

                                // insert a AddrOf node into the AST!
                                let addrof_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
                                let mut addrof_ast_node = AstNode::new(addrof_ast_node_id);
                                addrof_ast_node.node_type = AstNodeType::Expression;
                                addrof_ast_node.operator_type = AstNodeOperatorType::AddrOf;
                                addrof_ast_node.lhs = Some(ast_node.lhs.unwrap()); // LHS is the variable or literal (value element) which needs casting
                                addrof_ast_node.set_analyzed_data_type(DataType::DataTypePointer(Box::new(symbol_table_entry_lhs.data_type))); // analyzed_data_type is the type to cast into
                                addrof_ast_node.parent_id = Some(ast_node.id);
                                node_map.insert(addrof_ast_node_id, addrof_ast_node);

                                // the left node becomes child of the new middle node
                                left_node.parent_id = Some(addrof_ast_node_id);
                                node_map.insert(left_node.id, left_node);

                                // clone parent, insert new LHS, replace parent in hashmap
                                let mut ast_node_clone = ast_node.clone();
                                ast_node_clone.lhs = Some(addrof_ast_node_id);

                                // update changed, cloned node in node_map
                                node_map.insert(ast_node_clone.id, ast_node_clone);

                            } else {

                                ast_node.operator_type = AstNodeOperatorType::AddrOf;
                                ast_node.set_analyzed_data_type(DataType::DataTypePointer(Box::new(symbol_table_entry_lhs.data_type)));

                                // update changed, cloned node in node_map
                                node_map.insert(ast_node.id, ast_node);
                            }
                        }
                    }
                }

                return lhs_data_type;
            }

            AstNodeType::Identifier => {
                // DEBUG
                if self.debug {
                    println!("{:?}", ast_node);
                }

                // first check if the symbol exists in the symbol table
                if !self.symbol_table.borrow_mut().contains(&ast_node.string_val) {
                    panic!("Variable '{}' not contained inside the symbol table!", &ast_node.string_val);
                }

                // retrieve the symbol entry for the symbol
                let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&ast_node.string_val);

                // a type is required
                assert_ne!(symbol_table_entry.data_type, DataType::DataTypeUnknown);

                // update the node
                ast_node.set_analyzed_data_type(symbol_table_entry.data_type.clone());

                // the cloned node has been changed. Replace the original node in the node_map to preserve the change
                node_map.insert(ast_node.id, ast_node);

                return symbol_table_entry.data_type.clone();
            }

            AstNodeType::Return => {

                // DEBUG
                // if self.debug {
                    println!("-------------------------------------------------------------");
                    println!("id: {} ast_node: {:?}", ast_node.id, ast_node);
                // }

                // find the function which is terminated when this return executes

                // let mut data_type: DataType = DataType::DataTypeUnknown;

                // LHS
                if let Some(left_node_id) = ast_node.lhs {

                    let data_type = self.visit(left_node_id, node_map, expected_return_data_type);

                    println!("{}", data_type);

                    ast_node.set_analyzed_data_type(data_type.clone());

                    // the cloned node has been changed.
                    // Replace the original node in the node_map to preserve the change
                    node_map.insert(ast_node.id, ast_node);

                    // let left_node = node_map.get(&left_node_id).unwrap();
                    // match left_node.node_type {

                    //     AstNodeType::ConstInt |
                    //     AstNodeType::ConstLong |
                    //     AstNodeType::ConstUInt |
                    //     AstNodeType::ConstULong |
                    //     AstNodeType::ConstDouble => {
                    //         // DEBUG
                    //         if self.debug {
                    //             print!("Constant");
                    //             print!(" {:?}", left_node.string_val);
                    //             println!(" [{:?}]", left_node.analyzed_data_type);
                    //         }

                    //         // DEBUG
                    //         if self.debug {
                    //             println!("id: {}", ast_node.id);
                    //         }

                    //         ast_node.analyzed_data_type = DataType::DataTypeInt;

                    //         if expected_return_data_type != &ast_node.analyzed_data_type {
                    //             panic!("Return type of function does not match data type of the return statement!");
                    //         }

                    //         data_type = self.retrieve_data_type_from_node_type(&ast_node, node_map);

                    //         // the cloned node has been changed. Replace the original node in the node_map to preserve the change
                    //         node_map.insert(ast_node.id, ast_node);
                    //     }

                    //     AstNodeType::Identifier => {
                    //         if !self.symbol_table.borrow_mut().contains(&left_node.string_val) {
                    //             panic!("Variable '{}' not contained!", &left_node.string_val);
                    //         }
                    //         let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&left_node.string_val);
                    //         ast_node.analyzed_data_type = symbol_table_entry.data_type;

                    //         if expected_return_data_type != &ast_node.analyzed_data_type {
                    //             panic!("Return type of function does not match data type of the return statement!");
                    //         }

                    //         // the cloned node has been changed. Replace the original node in the node_map to preserve the change
                    //         node_map.insert(ast_node.id, ast_node);
                    //     }

                    //     _ => {
                    //         panic!("Unexpected node type!");
                    //     }
                    // }

                    return data_type;
                }

                todo!();
            }

            AstNodeType::If => {
                // Expression
                if let Some(expression_node_id) = ast_node.expression {
                    self.visit(expression_node_id, node_map, expected_return_data_type);
                }

                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map, expected_return_data_type);
                }

                // RHS
                if let Some(right_node_id) = ast_node.rhs {
                    self.visit(right_node_id, node_map, expected_return_data_type);
                }

                return DataType::DataTypeUnknown;
            }

            AstNodeType::Unary => {
                // DEBUG
                // if self.debug {
                    println!("------------------------------------------------");
                    println!("{} {:?}", ast_node.id, ast_node);
                // }

                let mut data_type: DataType = DataType::DataTypeUnknown;

                let left_node_id_option = ast_node.lhs;

                // RHS
                if let Some(right_node_id) = ast_node.rhs {

                    self.visit(right_node_id, node_map, expected_return_data_type);

                    let right_node = node_map.get(&right_node_id).unwrap();

                    // Nora Sandler, page 254. Not is always creating integer values 1 (= true), (0 = false)
                    match ast_node.operator_type {

                        AstNodeOperatorType::Not => {
                            ast_node.set_analyzed_data_type(DataType::from_str("int").unwrap());
                        }

                        _ => {
                            if !self.symbol_table.borrow_mut().contains(&right_node.string_val) {
                                panic!("Variable '{}' not contained!", &right_node.string_val);
                            }
                            let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&right_node.string_val);
                            ast_node.set_analyzed_data_type(symbol_table_entry.data_type.clone());

                            data_type = symbol_table_entry.data_type.clone();

                            // the cloned node has been changed. Replace the original node in the node_map to preserve the change
                            node_map.insert(ast_node.id, ast_node);
                        }
                    }
                    // println!("Unary: {:?}", ast_node);
                }

                // LHS
                if let Some(left_node_id) = left_node_id_option {

                    let left_node = node_map.get(&left_node_id).unwrap();

                    match left_node.node_type {

                        AstNodeType::Operator => {

                            match left_node.operator_type {

                                AstNodeOperatorType::AddrOf => {
                                    let data_type_pointer = DataType::DataTypePointer(Box::new(data_type));
                                    return data_type_pointer;
                                }

                                _ => {
                                    todo!("Not implemented: '{:?}'", left_node.operator_type);
                                }
                            }
                        }

                        _ => {
                            todo!("Not implemented: '{:?}'", left_node.node_type);
                        }
                    }
                }

                return data_type;
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

                    // // DEBUG
                    // if left_node_id == 67 {
                    //     println!("test");
                    // }

                    self.visit(left_node_id, node_map, expected_return_data_type);

                    let left_node = node_map.get(&left_node_id).unwrap();

                    lhs_type = self.retrieve_data_type_from_node_type(left_node, node_map);

                    // check if a function name is used where a variable name should be used (e.g. b = foo + a)
                    if self.symbol_table.borrow_mut().contains(&left_node.string_val) {

                        let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&left_node.string_val);

                        // DEBUG
                        if self.debug {
                            println!("[TypeCheckingVisitor] {:?}", symbol_table_entry);
                        }

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

                    self.visit(right_node_id.clone(), node_map, expected_return_data_type);

                    let right_node = node_map.get(&right_node_id).unwrap();

                    rhs_type = self.retrieve_data_type_from_node_type(right_node, node_map);

                    // check if a function name is used where a variable name should be used (e.g. b = a + foo)
                    if self.symbol_table.borrow_mut().contains(&right_node.string_val) {

                        let symbol_table_entry = self.symbol_table.borrow_mut().retrieve(&right_node.string_val);

                        // DEBUG
                        if self.debug {
                            println!("[TypeCheckingVisitor] {:?}", symbol_table_entry);
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

                let common_data_type;

                match ast_node.operator_type {

                    // Nora Sandler, page 255. Binary Logical-AND/OR has type int (1 == true, 0 == false)
                    AstNodeOperatorType::LogicalAnd | AstNodeOperatorType::LogicalOr => {
                        ast_node.set_analyzed_data_type(DataType::DataTypeInt);
                        common_data_type = DataType::DataTypeInt;
                    }

                    _ => {
                        common_data_type = self.get_common_type(&lhs_type, &rhs_type);

                        ast_node.set_analyzed_data_type(common_data_type.clone());

                        let ast_node_parent_id = ast_node.id;

                        // if the types differ, insert a cast in order to align
                        // the type of the LHS with the common type
                        if let Some(left_node_id) = ast_node.lhs {

                            let mut left_node = node_map.get(&left_node_id).unwrap().clone();
                            // DEBUG
                            if self.debug {
                                println!("left_node.analyzed_data_type: {}", left_node.get_analyzed_data_type());
                            }
                            // the node needs a type!
                            assert_ne!(left_node.get_analyzed_data_type(), DataType::DataTypeUnknown, "Node-Id: {}, Node has no analyzed_data_type!", left_node.id);
                            if left_node.get_analyzed_data_type() != common_data_type {

                                //
                                // insert a cast node into the AST!
                                //

                                // retrieve id
                                let cast_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

                                // DEBUG
                                if self.debug {
                                    println!("{}", cast_ast_node_id);
                                }

                                // build ASTNode
                                let mut cast_ast_node = AstNode::new(cast_ast_node_id);
                                cast_ast_node.node_type = AstNodeType::Cast;
                                cast_ast_node.lhs = Some(left_node.id); // LHS is the variable or literal (value element) which needs casting
                                cast_ast_node.set_analyzed_data_type(common_data_type.clone()); // analyzed_data_type is the type to cast into
                                cast_ast_node.parent_id = Some(ast_node.id);
                                cast_ast_node.file = String::from("test.c");
                                cast_ast_node.line = 999;

                                // add to ASTNode map
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

                        // if the types differ, insert a cast in order to align
                        // the type of the RHS with the common type
                        if let Some(right_node_id) = ast_node.rhs {

                            let mut right_node = node_map.get(&right_node_id).unwrap().clone();
                            // the node needs a type!
                            assert_ne!(right_node.get_analyzed_data_type(), DataType::DataTypeUnknown, "Node-Id: {}, Node has no analyzed_data_type!", right_node.id);
                            if right_node.get_analyzed_data_type() != common_data_type {

                                // retrieve id
                                let cast_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

                                // build ASTNode
                                let mut cast_ast_node = AstNode::new(cast_ast_node_id);
                                cast_ast_node.node_type = AstNodeType::Cast;
                                cast_ast_node.lhs = Some(right_node.id); // LHS is the variable or literal (value element) which needs casting
                                cast_ast_node.set_analyzed_data_type(common_data_type.clone()); // analyzed_data_type is the type to cast into
                                cast_ast_node.parent_id = Some(ast_node_parent_id);
                                cast_ast_node.file = String::from("test.c");
                                cast_ast_node.line = 999;

                                // add to ASTNode map
                                node_map.insert(cast_ast_node_id, cast_ast_node);

                                // the left node becomes child of the new middle node
                                right_node.parent_id = Some(cast_ast_node_id);

                                // // DEBUG
                                // if self.debug {
                                //     println!("{}", cast_ast_node_id);
                                //     println!("{:?}", ast_node); // parent
                                //     println!("{:?}", cast_ast_node); // new middle
                                //     println!("{:?}", left_node); // child
                                // }

                                // clone parent, insert new LHS
                                let mut ast_node_clone = ast_node.clone();
                                ast_node_clone.lhs = Some(cast_ast_node_id);

                                // replace parent in hashmap
                                node_map.insert(ast_node_clone.id, ast_node_clone);
                            }
                        }

                        // the cloned node has been changed.
                        // Replace the original node in the node_map to preserve the change
                        node_map.insert(ast_node.id, ast_node);
                    }
                }

                let temp_node = node_map.get(&ast_node_id).unwrap().clone();
                let mut ast_node_clone = temp_node.clone();
                ast_node_clone.set_analyzed_data_type(common_data_type.clone());

                // update changed node
                node_map.insert(ast_node_clone.id, ast_node_clone);

                return common_data_type.clone();
            }

            AstNodeType::Operator => {
                todo!();
            }

            AstNodeType::PrefixOperator => {
                todo!();
            }

            AstNodeType::DataType => {
                return self.type_system.borrow_mut().get(&ast_node.string_val).unwrap().clone();
                // todo!();
            }

            AstNodeType::Declaration => {
                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    return self.visit(left_node_id, node_map, expected_return_data_type);
                }

                panic!();
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
                        self.visit(parameter_node_id, node_map, expected_return_data_type);
                    }
                }

                let mut return_type: DataType = DataType::DataTypeUnknown;

                // RHS - Return type
                let mut temp_data_type_as_string = String::new();
                let mut data_type_as_enum = DataType::DataTypeUnknown;
                if let Some(right_node_id) = ast_node.rhs {

                    let right_node = node_map.get(&right_node_id).unwrap();

                    // DEBUG
                    if self.debug {
                        println!("{:?}", right_node);
                    }

                    assert!(right_node.node_type == AstNodeType::DataType);
                    temp_data_type_as_string = right_node.string_val.clone();

                    // determine the data type that is assigned to this AST node
                    data_type_as_enum = match DataType::from_str(&temp_data_type_as_string) {
                        Ok(data_type_result) => data_type_result,
                        Err(e) => panic!("should be valid DataType: {e}"),
                    };

                    return_type = self.visit(right_node_id, node_map, &data_type_as_enum);
                }

                {
                    // LHS - body of function == block with statements as block_items
                    if let Some(left_node_id) = ast_node.lhs {
                        self.visit(left_node_id, node_map, &data_type_as_enum);

                        symbol_table_entry.has_body = true;
                    }
                }

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
                    // add identifier and respective entry into symbol table
                    self.symbol_table.borrow_mut().insert(temp_function_name.clone(), symbol_table_entry);
                }

                return return_type;
            }

            AstNodeType::VariableDeclaration => {

                // DEBUG
                // if self.debug {
                    println!("-----------------------------------------------------------");
                    println!("{} {:?} file:{}, line:{}", ast_node.id, ast_node, ast_node.file, ast_node.line);
                // }

                let mut data_type: DataType = DataType::DataTypeUnknown;

                //
                // The goal here is to create a symbol entry in the symbol table for
                // a local variable and also to type check and name replace the
                // initializer if there is an initializer
                //

                // DEBUG
                if self.debug {
                    print!("VARIABLE_DECLARATION: {}: {:?}", ast_node.id, ast_node);
                }

                let mut symbol_table_entry = SymbolTableEntry::new();
                symbol_table_entry.symbol_table_entry_type = SymbolTableEntryType::Variable;

                // if let Some(data_type_node_id) = ast_node.data_type {
                //     let data_type_node = node_map.get(&data_type_node_id).unwrap();
                //     // DEBUG
                //     if self.debug {
                //         print!("DATA_TYEP-NODE: {}: {:?}", data_type_node.id, data_type_node);
                //     }
                // }

                // data type
                if let Some(left_node_id) = ast_node.lhs {

                    let left_node = node_map.get(&left_node_id).unwrap();

                    // DEBUG
                    if self.debug {
                        println!("LEFT-NODE: {}: {:?} node_type:{:?}", left_node.id, left_node, left_node.node_type);
                    }

                    let mut data_type_as_string = String::from("");

                    // set the data type
                    match left_node.node_type {

                        AstNodeType::Structure => {

                            symbol_table_entry.is_struct = true;

                            // if the variable is a struct-type, the type is part of the right subnode
                            // it is not part of the node itself
                            let left_right_node = node_map.get(&left_node.rhs.unwrap()).unwrap();
                            data_type_as_string = left_right_node.string_val.clone();

                            let type_sys = self.type_system.borrow();
                            let struct_data_type_option = type_sys.get(&data_type_as_string);
                            if let Some(struct_data_type) = struct_data_type_option {
                                symbol_table_entry.data_type = struct_data_type.clone();
                                data_type = struct_data_type.clone();
                            } else {
                                panic!("The type system has no definition for '{:?}'", data_type_as_string);
                            }
                        }

                        _ => {
                            data_type_as_string = left_node.string_val.clone();

                            println!("data_type: '{}'", data_type_as_string);

                            let ts = self.type_system.borrow();
                            if let Some(data_type_ts) = ts.get(&data_type_as_string) {
                                symbol_table_entry.data_type = data_type_ts.clone();
                                data_type = data_type_ts.clone();
                            } else {
                                panic!("should be valid DataType: {}. ast_node.id:{}, File: {}, Line: {}", data_type_as_string, ast_node.id, ast_node.file, ast_node.line);
                            }

                            // let data_type_as_enum = match DataType::from_str(&data_type_as_string) {
                            //     Ok(data_type_result) => data_type_result,
                            //     Err(e) => panic!("should be valid DataType: {e}. ast_node.id:{}, File: {}, Line: {}", ast_node.id, ast_node.file, ast_node.line),
                            // };
                            // symbol_table_entry.data_type = data_type_as_enum.clone();
                            // data_type = data_type_as_enum.clone();
                        }
                    }

                    symbol_table_entry.is_array = left_node.node_type == AstNodeType::Array;
                }

                // identifier (RHS)
                let mut varname = String::from("");
                if let Some(right_node_id) = ast_node.rhs {

                    let right_node = node_map.get(&right_node_id).unwrap();
                    // DEBUG
                    if self.debug {
                        print!("{}, {:?}", right_node.id, right_node);
                    }
                    varname = right_node.string_val.clone();

                    // detect the fact that this variable is a pointer
                    match &right_node.node_type {

                        AstNodeType::Pointer => {
                            symbol_table_entry.is_pointer = true;

                            // TODO: asdf

                            // this was inserted for res\C\samples\c_samples\struct_pointer_0.c
                            // Change the type. Replace 'base type' by 'pointer to base type' because
                            // when the expression is evaluated as the type_checking_visitor (this
                            // struct) visits the expression, types are compared and if one is a
                            // 'base type' and the other is a pointer to base type, then a nonsensical
                            // cast is inserted. Make both types 'pointer to base type' to prevent
                            // the cast.
                            let base_data_type = symbol_table_entry.data_type;
                            symbol_table_entry.data_type = DataType::DataTypePointer(Box::new(base_data_type));
                        }

                        _ => {
                            // panic!("Unhandled NodeType: {:?}", &right_node.node_type);
                        }
                    }
                }
                symbol_table_entry.name = varname.clone();

                // DEBUG
                if self.debug {
                    println!("[TypeCheckingVisitor] symbol_table_entry:\n {:?}", symbol_table_entry);
                }

                // add identifier into symbol table
                self.symbol_table.borrow_mut().insert(varname, symbol_table_entry);

                // initialization expression (initializer, variable is initialized at declaration)
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
                    self.visit(expression_node_id, node_map, expected_return_data_type);
                }

                return data_type;
            }

            AstNodeType::StructureDeclaration => {
                // DEBUG
                // if self.debug {
                    println!("{} {:?}", ast_node.id, ast_node);
                // }
                return self.type_system.borrow_mut().get(&ast_node.string_val).unwrap().clone();
                // todo!();
            }

            AstNodeType::ParameterDeclaration => {
                todo!();
            }

            AstNodeType::VariadicParameterDeclaration => {
                todo!();
            }

            AstNodeType::Statement => {
                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    return self.visit(left_node_id, node_map, expected_return_data_type);
                }
                todo!();
            }

            AstNodeType::Block => {
                let mut data_type: DataType = DataType::DataTypeUnknown;
                for i in 0..ast_node.block_items.len() {
                    // DEBUG
                    if self.debug {
                        println!("BlockItem {}:", i+1);
                    }
                    let index = ast_node.block_items.len()-1-i;
                    let block_item_id = ast_node.block_items[index];
                    data_type = self.visit(block_item_id, node_map, expected_return_data_type);
                }
                return data_type;
            }

            AstNodeType::BlockItem => {
                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    return self.visit(left_node_id, node_map, expected_return_data_type);
                }
                todo!();
            }

            AstNodeType::Conditional => {
                todo!();
            }

            AstNodeType::Compound => {
                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    return self.visit(left_node_id, node_map, expected_return_data_type);
                }
                todo!();
            }

            AstNodeType::While => {
                // LHS - expression_ast_node, condition, e.g. a < 10
                if let Some(expression_node_id) = ast_node.lhs {
                    self.visit(expression_node_id, node_map, expected_return_data_type);
                }

                // RHS - statement
                if let Some(statement_node_id) = ast_node.rhs {
                    self.visit(statement_node_id, node_map, expected_return_data_type);
                }

                todo!();
            }

            AstNodeType::DoWhile => {
                todo!();
            }

            AstNodeType::For => {
                // LHS - initialization, e.g.: a = 0
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map, expected_return_data_type);
                }

                // expression - expression_ast_node, condition, e.g. a < 10
                if let Some(expression_node_id) = ast_node.expression {
                    self.visit(expression_node_id, node_map, expected_return_data_type);
                }

                // RHS - post, e.g.: a = a + 1
                if let Some(right_node_id) = ast_node.rhs {
                    self.visit(right_node_id, node_map, expected_return_data_type);
                }

                // BLOCK_ITEMS - instructions and declarations
                for i in 0..ast_node.block_items.len() {
                    let idx = ast_node.block_items.len()-1-i;
                    let block_item_id = ast_node.block_items[idx];
                    self.visit(block_item_id, node_map, expected_return_data_type);
                }

                todo!();
            }

            AstNodeType::FunctionCall => {
                // DEBUG
                if self.debug {
                    println!("ast_node.id: {}, ast_node: {:?}", ast_node.id, ast_node);
                }

                let function_name = ast_node.string_val.clone();

                // DEBUG
                if self.debug {
                    println!("FunctionCall to function using identifier: {:?}", function_name);
                }

                // make sure, the identifier is a function name/call and not a variable
                // Check argument count
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

                        return existing_symbol_table_entry.data_type;
                    }

                } else {
                    panic!("[ERR] Symbol '{}' is not contained in the symbol table! Cannot call undefined symbol!", &function_name);
                }
            }

            AstNodeType::StorageClassSpecifier => {
                todo!();
            }

            AstNodeType::Pointer => {
                todo!();
            }

            AstNodeType::Switch => {
                todo!();
            }

            AstNodeType::Case => {
                todo!();
            }

            AstNodeType::Default => {
                todo!();
            }

            AstNodeType::Break => {
                todo!();
            }

            AstNodeType::Continue => {
                todo!();
            }

            AstNodeType::EmptyStatement => {
                todo!();
            }

            AstNodeType::SingleInit => {
                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    return self.visit(left_node_id, node_map, expected_return_data_type);
                }
                todo!();
            }

            AstNodeType::CompoundInit => {
                todo!();
            }

            AstNodeType::Subscript => {

                // RHS contains the type
                if let Some(right_node_id) = ast_node.rhs {

                    self.visit(right_node_id, node_map, expected_return_data_type);

                    let right_node = node_map.get(&right_node_id).unwrap();
                    let rhs_type = self.retrieve_data_type_from_node_type(right_node, node_map);

                    ast_node.set_analyzed_data_type(rhs_type.clone());

                    // update changed node
                    node_map.insert(ast_node.id, ast_node);

                    return rhs_type;
                }

                todo!();
            }

            AstNodeType::MemberDeclaration => {
                todo!();
            }

            AstNodeType::Arrow | AstNodeType::Dot => {

                // DEBUG
                // if self.debug {
                    println!("-----------------------------------------------------------");
                    println!("{} {:?}", ast_node.id, ast_node);
                // }

                let node = node_map.get(&ast_node.id).unwrap();

                let rhs_node_id = node.rhs.unwrap();
                println!("rhs_node_id: {}", &rhs_node_id);
                let rhs_node = node_map.get(&rhs_node_id).unwrap();

                let field_name = rhs_node.string_val.clone();

                let lhs_node_id = node.lhs.unwrap();
                println!("lhs_node_id: {}", &lhs_node_id);
                let lhs_node = node_map.get(&lhs_node_id).unwrap();

                println!("{:?}", lhs_node);

                // resolve variable name to symbol
                let lhs_symbol = self.symbol_table.borrow_mut().retrieve(&lhs_node.string_val);

                println!("{:?}", lhs_symbol);

                // retrieve data type of lhs_symbol
                let lhs_data_type = lhs_symbol.data_type;

                println!("lhs_data_type: {}", &lhs_data_type);

                let mut struct_decl: StructDecl = StructDecl::new();

                // retrieve data type of the field which is accessed within the struct
                match lhs_data_type {

                    DataType::DataTypePointer(dest_data_type) => {

                        println!("dest_data_type: {}", &dest_data_type);
                        // let struct_decl = dest_data_type;

                        match *dest_data_type {
                            DataType::DataTypeStruct(struct_decl_val) => {
                                println!("struct_decl: {:?}", struct_decl_val);
                                struct_decl = struct_decl_val;

                                let struct_decl_member = struct_decl.members.iter().find(|f| f.member_name == field_name);

                                return struct_decl_member.unwrap().data_type.clone();
                            }
                            _ => {
                                panic!("Unhandled ptr destination data type: {:?}", dest_data_type);
                            }
                        }
                    }

                    DataType::DataTypeStruct(struct_decl_val) => {
                        println!("struct_decl: {:?}", struct_decl_val);
                        struct_decl = struct_decl_val;
                        let struct_decl_member = struct_decl.members.iter().find(|f| f.member_name == field_name);
                        return struct_decl_member.unwrap().data_type.clone();
                    }

                    _ => {
                        panic!("Unhandled data type: {:?}", lhs_data_type);
                    }
                }
            }

            AstNodeType::AssignmentOperator => {
                todo!();
            }

            AstNodeType::Unknown => {
                todo!();
            }

            AstNodeType::AddAssignment => {
                todo!();
            }

            AstNodeType::Typedef => {
                // DEBUG
                // if self.debug {
                    println!("-----------------------------------------------------------");
                    println!("{} {:?}", ast_node.id, ast_node);
                // }

                let lhs_node_id = ast_node.lhs.unwrap();
                println!("lhs_node_id: {}", &lhs_node_id);
                let lhs_node = node_map.get(&lhs_node_id).unwrap();

                println!("{:?}", lhs_node.string_val);

                // let type_name = lhs_node.string_val.split(' ').collect::<Vec<_>>().get(0).unwrap();
                let (type_name, rest) = lhs_node.string_val.split_once(char::is_whitespace).unwrap();

                let ts = self.type_system.borrow_mut();
                let data_type = ts.get(type_name);

                println!("{:?}", data_type);

                return data_type.unwrap().clone();
            }

            _ => {
                todo!("TODO: Handle DataType: '{:?}'", ast_node.node_type);
            }
        }
    }
}