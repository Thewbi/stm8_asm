use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;

use crate::AstNode;
use crate::AstNodeType;

use crate::c_ast::ast_node::AstNodeOperatorType;
use crate::c_ast::identifier_resolution_node::IdentifierResolutionNode;

use crate::common::data_type::DataType;
use crate::common::data_type::DataType::DataTypeStruct;
use crate::common::struct_decl::StructDecl;
use crate::common::struct_decl::StructDeclMember;

use crate::common::symbol_table;
use crate::common::symbol_table::SymbolTable;
use crate::tacky::tacky::BinaryOperator;

use crate::common::variable_naming_source::VariableNamingSource;

//
// Nora Sandler, page 104, maps user-defined variable names to unique variable names
//
// * checks for undefined variables
// * checks for duplicate variable names
// * tells the VariableNamingSource that a new scope has been entered or exited
// * ??? replaces parameter names by "pseudo_{}" which is then later used to move arguments to
//   function calls from ABI registers onto the stack in the AsmAstConversionVisitor
//
// 1. src\c_ast\identifier_resolution_visitor.rs - checks for duplicate or undeclared variable names
// 2. src\c_ast\type_checking_visitor.rs - performs type checking. The rules that are checked are documented in the file itself
// 3. src\tacky\tacky_visitor.rs - Generate TACKY (from AST)
// 4. asm_ast/AsmAstConversionVisitor - Converts the AST into a ASM AST for assembly with a precursory form of mnenomics
// 5. asm_ast/AsmAstFixupVisitor - replacing pseudo operands/variables with stack addresses
// 6. src\asm_ast\asm_ast_masm_emitter_visitor.rs / asm_ast/AsmAstASEmitterVisitor / ...
//

pub struct IdentifierResolutionVisitor {
    // https://www.youtube.com/watch?v=8O0Nt9qY_vo
    variable_naming_source: Rc<RefCell<VariableNamingSource>>,
    structure_naming_source: Rc<RefCell<VariableNamingSource>>,
    symbol_table: Rc<RefCell<SymbolTable>>,
    // struct_variable_option: Option<_>,
    is_in_dot_or_arrow: bool,
    debug: bool,
}

impl IdentifierResolutionVisitor {

    pub fn new(variable_naming_source_param: Rc<RefCell<VariableNamingSource>>,
        structure_naming_source_param: Rc<RefCell<VariableNamingSource>>,
        symbol_table_param: Rc<RefCell<SymbolTable>>,) -> IdentifierResolutionVisitor
    {
        IdentifierResolutionVisitor {
            variable_naming_source: variable_naming_source_param,
            structure_naming_source: structure_naming_source_param,
            symbol_table: symbol_table_param,
            // struct_variable_option: None,
            is_in_dot_or_arrow: false,
            debug: false,
        }
    }

    pub fn visit(&mut self,
        ast_node_id: usize,
        node_map: &mut Box<HashMap<usize, AstNode>>) -> IdentifierResolutionNode
    {
        self.visit_ex(ast_node_id, node_map, true)
    }

    pub fn visit_ex(&mut self,
        ast_node_id: usize,
        node_map: &mut Box<HashMap<usize, AstNode>>,
        build_new_scope: bool) -> IdentifierResolutionNode
    {
        // DEBUG
        if self.debug {
            println!("AstNodeId: {}", ast_node_id);
        }

        let mut ast_node = AstNode::new(0);

        {
            ast_node = node_map.get(&ast_node_id).unwrap().clone();
        }

        // // DEBUG
        // if self.debug {
        //     println!("[visit_ex()] {:?}", ast_node.node_type);
        // }

        match ast_node.node_type {

            AstNodeType::Program => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // visit all items in the body of the program
                for i in 0..ast_node.block_items.len() {
                    let index = ast_node.block_items.len()-1-i;
                    self.visit(ast_node.block_items[index], node_map);
                }
            }

            AstNodeType::ConstInt => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                let mut semantic_analysis_node: IdentifierResolutionNode = IdentifierResolutionNode::new();
                semantic_analysis_node.node_type = ast_node.node_type.clone();
                semantic_analysis_node.string_val = ast_node.string_val.clone();

                return semantic_analysis_node;
            }

            AstNodeType::ConstLong => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                let mut semantic_analysis_node: IdentifierResolutionNode = IdentifierResolutionNode::new();
                semantic_analysis_node.node_type = ast_node.node_type.clone();
                semantic_analysis_node.string_val = ast_node.string_val.clone();

                return semantic_analysis_node;
            }

            AstNodeType::ConstUInt => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                let mut semantic_analysis_node: IdentifierResolutionNode = IdentifierResolutionNode::new();
                semantic_analysis_node.node_type = ast_node.node_type.clone();
                semantic_analysis_node.string_val = ast_node.string_val.clone();

                return semantic_analysis_node;
            }

            AstNodeType::ConstULong => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                let mut semantic_analysis_node: IdentifierResolutionNode = IdentifierResolutionNode::new();
                semantic_analysis_node.node_type = ast_node.node_type.clone();
                semantic_analysis_node.string_val = ast_node.string_val.clone();

                return semantic_analysis_node;
            }

            AstNodeType::ConstDouble => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                let mut semantic_analysis_node: IdentifierResolutionNode = IdentifierResolutionNode::new();
                semantic_analysis_node.node_type = ast_node.node_type.clone();
                semantic_analysis_node.string_val = ast_node.string_val.clone();

                return semantic_analysis_node;
            }

            AstNodeType::ConstString => {
                let mut semantic_analysis_node: IdentifierResolutionNode = IdentifierResolutionNode::new();
                semantic_analysis_node.node_type = ast_node.node_type.clone();
                semantic_analysis_node.string_val = ast_node.string_val.clone();

                return semantic_analysis_node;
            }

            AstNodeType::Structure => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Array => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Expression => {
                // DEBUG
                if self.debug {
                    println!("[Expression] AstNodeId: {}, AstNodeType: {:?}", ast_node.id, ast_node.node_type);
                }

                assert!(ast_node.lhs.is_some());

                // LHS
                if let Some(left_node_id) = ast_node.lhs {

                    // replace user-choosen variable name by unique variable name
                    let replaced_var_name_node: IdentifierResolutionNode =
                        self.visit(left_node_id, node_map);

                    let initial_var_name = replaced_var_name_node.initial_string_val.clone();
                    let replaced_var_name = replaced_var_name_node.string_val.clone();

                    let mut left_node = node_map.get(&left_node_id).unwrap().clone();
                    left_node.string_val = replaced_var_name;
                    left_node.initial_string_val = initial_var_name;

                    // DEBUG
                    if self.debug {
                        println!("[IdentifierResolutionVisitor] NodeId: {}, Initial:{:?}, Replaced:{:?}",
                            &left_node.id, &replaced_var_name_node.initial_string_val, &replaced_var_name_node.string_val);
                    }

                    node_map.insert(left_node.id, left_node);
                }

                // RHS
                if let Some(right_node_id) = ast_node.rhs {
                    let replaced_var_name_node: IdentifierResolutionNode = self.visit(right_node_id, node_map);
                    let replaced_var_name = replaced_var_name_node.string_val;
                    // DEBUG
                    if self.debug {
                        println!("{:?}", replaced_var_name);
                    }

                    // update the node
                    let mut right_node = node_map.get(&right_node_id).unwrap().clone();
                    right_node.string_val = replaced_var_name;

                    // node has changed, replace it in the node map
                    node_map.insert(right_node.id, right_node);
                }
            }

            AstNodeType::Identifier => {
                // DEBUG
                // if self.debug {
                    println!("[Identifier] AstNodeId: {}, AstNodeType: {:?}", ast_node.id, ast_node.node_type);
                // }

                let mut indentifier_resolution_node = IdentifierResolutionNode::new();
                indentifier_resolution_node.node_type = ast_node.node_type.clone();

                let user_choosen_variable_name = ast_node.string_val.clone();

                // DEBUG
                // if self.debug {
                    println!("[Identifier] user_choosen_variable_name: '{}'", &user_choosen_variable_name);
                // }

                if self.is_in_dot_or_arrow {

                    self.is_in_dot_or_arrow = false;

                    // remember original name for convenient debugging
                    indentifier_resolution_node.initial_string_val = user_choosen_variable_name.clone();

                    // store new varible name
                    indentifier_resolution_node.string_val = user_choosen_variable_name.clone();

                } else {

                    // replace original variable name with new variable name
                    let replaced_variable_name = match self.variable_naming_source.borrow_mut()
                        .get_replaced_variable_name(&user_choosen_variable_name)
                    {
                        Ok(var_name) => var_name,
                        Err(e) => {
                            let res = self.structure_naming_source.borrow_mut()
                                .get_replaced_variable_name(&user_choosen_variable_name);
                            match res {
                                Ok(var_name) => var_name,
                                Err(e) => {
                                    panic!("{}", e);
                                }
                            }
                        }
                    };

                    // if self.debug {
                        println!("[Identifier] replaced_variable_name: '{}'", &replaced_variable_name);
                        println!("[Identifier] [NAMING-1] Name lookup: old: '{}', new: '{}'",
                            &user_choosen_variable_name, replaced_variable_name);
                    // }

                    // remember original name for convenient debugging
                    indentifier_resolution_node.initial_string_val = user_choosen_variable_name;

                    // store new varible name
                    indentifier_resolution_node.string_val = replaced_variable_name.clone();

                    // replace name and update node inside node map
                    ast_node.string_val = replaced_variable_name.clone();
                    node_map.insert(ast_node.id, ast_node);
                }

                // DEBUG
                if self.debug {
                    println!("node.initial_string_val: {:?} replaced_variable_name: {:?}",
                        &indentifier_resolution_node.initial_string_val, &indentifier_resolution_node.string_val);
                }

                return indentifier_resolution_node;
            }

            AstNodeType::Return => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType.RETURN: {:?}", ast_node);
                }
                if let Some(left_node_id) = ast_node.lhs {

                    let semant_node: IdentifierResolutionNode = self.visit(left_node_id, node_map);

                    // DEBUG
                    if self.debug {
                        println!("{:?}", semant_node);
                    }

                    let mut left_node = node_map.get(&left_node_id).unwrap().clone();

                    let user_choosen_variable_name = left_node.string_val.clone();
                    let replaced_variable_name = semant_node.string_val.clone();

                    // DEBUG
                    if self.debug {
                        println!("user_choosen_variable_name: '{}', replaced_variable_name: '{}'", user_choosen_variable_name, replaced_variable_name);
                    }

                    left_node.string_val = replaced_variable_name.clone();

                    node_map.insert(left_node.id, left_node);
                }
            }

            AstNodeType::If => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // Important: replace variable names in potential if expressions

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
                // DEBUG
                // if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                    println!("AstNodeId: {:?}, AstNode: {:?}", ast_node.id, ast_node);
                // }

                let mut identifier_resolution_node = IdentifierResolutionNode::new();
                identifier_resolution_node.node_type = ast_node.node_type.clone();

                if let Some(right_node_id) = ast_node.rhs {

                    let rhs_identifier_resolution_node: IdentifierResolutionNode
                        = self.visit(right_node_id, node_map);

                    // let mut right_node = node_map.get(&right_node_id).unwrap().clone();
                    // let user_choosen_variable_name = right_node.string_val.clone();

                    //let user_choosen_variable_name = rhs_identifier_resolution_node.string_val;

                    // // DEBUG
                    // // if self.debug {
                    //     println!("user_choosen_variable_name: '{}'", user_choosen_variable_name);
                    // // }

                    // let res = self.variable_naming_source.borrow_mut()
                    //     .get_replaced_variable_name(&user_choosen_variable_name);
                    // let replaced_var_name = match res {
                    //     Ok(var_name) => var_name,
                    //     Err(e) => {
                    //         let res = self.structure_naming_source.borrow_mut()
                    //             .get_replaced_variable_name(&user_choosen_variable_name);
                    //         match res {
                    //             Ok(var_name) => var_name,
                    //             Err(e) => {
                    //                 panic!("{}", e);
                    //             }
                    //         }
                    //     }
                    // };

                    // // DEBUG
                    // if self.debug {
                    //     println!("user_choosen_variable_name: '{:?}', replaced_var_name: '{:?}'", user_choosen_variable_name, replaced_var_name);
                    // }

                    // right_node.string_val = replaced_var_name.clone();
                    // node_map.insert(right_node.id, right_node);

                    //identifier_resolution_node.string_val = replaced_var_name.clone();

                    identifier_resolution_node.string_val = rhs_identifier_resolution_node.string_val.clone();
                }

                return identifier_resolution_node;
            }

            AstNodeType::Binary => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // LHS - this part will replace the original LHS name by a unique name
                // LHS is the LHS of the binary operation
                if let Some(left_node_id) = ast_node.lhs {

                    // DEBUG
                    if self.debug {
                        println!("left_node_id: {:?}", left_node_id);
                    }

                    // naming map before
                    if self.debug {
                        self.variable_naming_source.borrow_mut().print_variable_naming_source();
                    }

                    let lhs_result = self.visit(left_node_id, node_map);

                    // DEBUG
                    if self.debug {
                        println!("[IdentifierResolutionVisitor] NodeId: {}, Initial:{:?}, Replaced:{:?}",
                            &left_node_id, &lhs_result.initial_string_val, &lhs_result.string_val);
                    }

                    // naming map after
                    if self.debug {
                        self.variable_naming_source.borrow_mut().print_variable_naming_source();
                    }

                    let mut left_node = node_map.get(&left_node_id).unwrap().clone();

                    // DEBUG
                    if self.debug {
                        println!("left_node: {:?}", left_node);
                        println!("left_node.string_val: {:?}", left_node.string_val);
                    }

                    // DEBUG
                    if self.debug {
                        println!("{:?}", lhs_result);
                    }

                    match lhs_result.node_type {

                        AstNodeType::Identifier => {

                            if self.debug {
                                println!("123: {}", left_node.string_val);
                            }

                            // check the original user-choosen variable name
                            let is_defined = self.variable_naming_source.borrow_mut().is_variable_name_defined(&left_node.string_val);
                            if !is_defined {
                                panic!("Variable \"{}\" is not defined!", &left_node.string_val);
                            }

                            let new_name = match self.variable_naming_source.borrow_mut().get_replaced_variable_name(&left_node.string_val) {
                                Ok(var_name) => var_name,
                                Err(e) => {
                                    panic!("{}", e);
                                }
                            };

                            // DEBUG
                            if self.debug {
                                println!("{:?}", new_name);
                            }

                            left_node.string_val = new_name;

                            // node has changed, replace the node in the node map
                            node_map.insert(left_node.id, left_node);
                        }
                        _ => {

                        }
                    }
                }

                // RHS - this part will replace the original LHS name by a unique name
                // RHS is the LHS of the binary operation
                if let Some(right_node_id) = ast_node.rhs {

                    let rhs_result = self.visit(right_node_id, node_map);

                    // DEBUG
                    if self.debug {
                        println!("{:?}", rhs_result);
                    }

                    match rhs_result.node_type {

                        AstNodeType::Identifier => {
                            let mut right_node = node_map.get(&right_node_id).unwrap().clone();
                            right_node.string_val = rhs_result.string_val.clone();

                            // the node has changed, update the node
                            node_map.insert(right_node.id, right_node);
                        }
                        _ => {

                        }
                    }
                }
            }

            AstNodeType::Operator => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::PrefixOperator => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::DataType => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Declaration => {
                // DEBUG
                if self.debug {
                    println!("AstNodeId: {}, AstNodeType: {:?}", ast_node.id, ast_node.node_type);
                }

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
                // DEBUG
                if self.debug {
                    println!("[FunctionDeclaration] AstNodeId: {}, AstNodeType: {:?}", ast_node.id, ast_node.node_type);
                }

                // ast_node.rhs = Some(Box::new(data_type_ast_node));
                //    object_declaration_ast_node.lhs = Some(Box::new(identifier_ast_node));
                //    object_declaration_ast_node.string_val = ident;
                // data_type (return value)

                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.rhs);
                }

                //
                // function name
                //

                // if self.debug {
                //     println!("AstNodeType: {:?}", ast_node.string_val);
                // }

                // &ast_node.function_name_ast_node.unwrap().string_val

                if let Some(function_name_id) = ast_node.function_name_ast_node {
                    let function_name = node_map.get(&function_name_id).unwrap();
                    let user_define_function_name = self.variable_naming_source.borrow_mut().new_function_declaration(&function_name.string_val);

                    // // DEBUG
                    // println!("user_define_function_name: {:?}", user_define_function_name);

                    // TODO continue here with function declarations
                }

                //
                // Function Body
                //

                // DEBUG
                // println!("[AstNodeType::FunctionDeclaration ENTER_SCOPE]");
                self.variable_naming_source.borrow_mut().enter_scope();

                // parameters - are visited in the same context (= same varname_map on the stack) as the function declaration itself
                for i in 0..ast_node.parameters.len() {
                    // let index = ast_node.parameters.len()-1-i;
                    // self.visit(&mut ast_node.parameters[index]);

                    self.visit(ast_node.parameters[i], node_map);
                }

                //
                // body of function == block with statements as block_items
                // is visited in the same context (= same varname_map on the stack) as the function declaration and it's parameters
                //

                if let Some(left_node_id) = ast_node.lhs {

                    // DEBUG
                    if self.debug {
                        println!("{:?}", left_node_id);
                    }

                    // println!("[AstNodeType::FunctionDeclaration ENTER_SCOPE]");
                    // self.variable_naming_source.borrow_mut().enter_scope();
                    // println!("AstNodeType: {:?}", ast_node.lhs);

                    // for the special case of function declarations with body,
                    // the function declaration will create a new scope and insert
                    // the function's name and parameters into that scope and the body
                    // of that function declaration needs to be processed within that
                    // scope instead of creating a new scope because local variables
                    // in the body of the function declarations need to clash with
                    // parameter names as it is not valid to shadow parameters using
                    // local variables!
                    let build_new_scope = false;
                    self.visit_ex(left_node_id, node_map, build_new_scope);

                    // println!("[AstNodeType::FunctionDeclaration EXIT_SCOPE]");
                    // self.variable_naming_source.borrow_mut().exit_scope();
                }

                // DEBUG
                // println!("[AstNodeType::FunctionDeclaration EXIT_SCOPE]");
                self.variable_naming_source.borrow_mut().exit_scope();
            }

            AstNodeType::VariableDeclaration => {
                // DEBUG
                if self.debug {
                    println!("AstNodeId: {:?}, AstNodeType: {:?}", ast_node.id, ast_node.node_type);
                    println!("AstNode: {:?}", ast_node);
                }

                let mut is_struct: bool = false;

                // data type (LHS)
                if let Some(left_node_id) = ast_node.lhs {
                    // DEBUG
                    if self.debug {
                        println!("{:?}", left_node_id);
                    }

                    let lhs_node = node_map.get(&left_node_id).unwrap();
                    if self.debug {
                        println!("left_node_id: {}, lhs_node.string_val: '{}'", left_node_id, lhs_node.string_val);
                    }
                    is_struct = lhs_node.node_type == AstNodeType::Structure;
                    // match lhs_node.data_type.unwrap() {
                    //     DataType::DataTypeStruct(_) => {
                    //         is_struct = true;
                    //     }
                    //     _ => {

                    //     }
                    // }
                }

                let mut replaced_varname = String::new();

                // identifier (RHS)
                let mut user_defined_varname = String::from("");
                if let Some(right_node_id) = ast_node.rhs {

                    // DEBUG
                    if self.debug {
                        println!("{:?}", right_node_id);
                    }

                    // next: implement page 105

                    let mut right_node = node_map.get(&right_node_id).unwrap().clone();
                    user_defined_varname = right_node.string_val.clone();

                    // why does borrow_mut() work but borrow() does not work?
                    //let symbol_table_entry_option = self.symbol_table.borrow_mut().get(&varname);
                    //if let Some(symbol_table_entry) = symbol_table_entry_option {
                        // if symbol_table_entry.is_struct {
                        if is_struct {
                            // the naming source checks if the name is used already (which is not allowed)
                            // throws panic if the variable name is already used.
                            // To remember if a variable is used, it maintains a data structure of variables
                            replaced_varname = self.structure_naming_source.borrow_mut().
                                new_user_defined_var(&user_defined_varname);
                        } else {
                            // the naming source checks if the name is used already (which is not allowed)
                            // throws panic if the variable name is already used.
                            // To remember if a variable is used, it maintains a data structure of variables
                            replaced_varname = self.variable_naming_source.borrow_mut().
                                new_user_defined_var(&user_defined_varname);
                        }
                    //}

                    // DEBUG
                    if self.debug {
                        println!("[NAMING-2] Name replaced: old: {}, new: {}", user_defined_varname, replaced_varname);
                    }

                    // replace the variable name by the unique variable name from the scope.
                    // Subsequent steps such as TACKY generation will use unique names instead
                    // of duplicate names separated by scopes
                    right_node.initial_string_val = user_defined_varname.clone();
                    right_node.string_val = replaced_varname.clone();

                    // need a reinsert to update the node in the node_map because.
                    // The updat is required since the node's name was changed
                    node_map.insert(right_node.id, right_node);
                }

                // initialization expression
                //
                // perform semantic analysis of the initialization expression
                // replace variable names with the unique variable names from
                // the map stored inside the variable_naming_source
                if let Some(expression_node_id) = ast_node.expression {
                    // DEBUG
                    if self.debug {
                        println!("{:?}", expression_node_id);
                    }
                    self.visit(expression_node_id, node_map);
                }

                // TODO: return VariableDeclaration node

                let mut node = IdentifierResolutionNode::new();
                node.node_type = AstNodeType::VariableDeclaration;

                // the variable name is changed to a unique variable name to handle/represent nested scopes
                // Subsequent steps such as TACKY generation will not know about scopes as the variable
                // names have already been made unique by the linearization applied here!
                node.string_val = replaced_varname;

                return node;
            }

            AstNodeType::StructureDeclaration => {
                // DEBUG
                if self.debug {
                    println!("[StructureDeclaration] AstNodeId: {}, AstNodeType: {:?}", ast_node.id, ast_node.node_type);
                }

                let struct_tag = ast_node.string_val.clone();

                let define_struct = false;
                if define_struct {

                    let mut struct_decl = StructDecl::new();
                    struct_decl.tag = struct_tag.clone();

                    for i in 0..ast_node.block_items.len() {

                        let member_node_id = &ast_node.block_items[i];

                        let member_declaration_node = node_map.get(&member_node_id).unwrap().clone();

                        let member_type_node_id = member_declaration_node.lhs.unwrap();
                        let member_name_node_id = member_declaration_node.rhs.unwrap();

                        let member_type_node = node_map.get(&member_type_node_id).unwrap().clone();
                        let member_name_node = node_map.get(&member_name_node_id).unwrap().clone();

                        let mut struct_member = StructDeclMember::new();
                        struct_member.member_name = member_name_node.string_val.clone();
                        struct_member.member_type = member_type_node.string_val.clone();

                        struct_decl.members.push(struct_member);
                    }

                    // DEBUG
                    if self.debug {
                        println!("{:?}", &struct_decl);
                    }
                }

/*
                //
                // replace user-defined tag by compiler-defined name
                //

                // the naming source checks if the name is used already (which is not allowed)
                // throws panic if the variable name is already used.
                // To remember if a variable is used, it maintains a data structure of variables
                let compiler_defined_struct_name = self.structure_naming_source.borrow_mut().
                    new_user_defined_var(&struct_tag);

                // DEBUG
                // if self.debug {
                    println!("[NAMING-3] Name replaced: old: '{}', new: '{}'",
                        &struct_tag, compiler_defined_struct_name);
                // }

                // replace the variable name by the unique variable name from the scope.
                // Subsequent steps such as TACKY generation will use unique names instead
                // of duplicate names separated by scopes
                ast_node.initial_string_val = ast_node.string_val.clone();
                ast_node.string_val = compiler_defined_struct_name.clone();

                // need a reinsert to update the node in the node_map because.
                // The updat is required since the node's name was changed
                node_map.insert(ast_node.id, ast_node);
 */
            }

            AstNodeType::ParameterDeclaration => {
                // DEBUG
                if self.debug {
                    println!("AstNode-Id: {}, AstNodeType: {:?}", ast_node.id, ast_node.node_type);
                }

                // parameter's data type (RHS)
                if let Some(right_node_id) = ast_node.rhs {

                    // data type
                    let right_node = node_map.get(&right_node_id).unwrap();
                    let data_type = right_node.string_val.clone();

                    // void parameters are not inserted into the name map
                    if data_type == "void" {
                        return IdentifierResolutionNode::new();
                    }
                }

                // parameter's identifier (LHS)
                let mut param_name = String::from("");
                if let Some(left_node_id) = ast_node.lhs {

                    // DEBUG
                    if self.debug {
                        println!("{:?}", left_node_id);
                    }

                    let left_node = node_map.get(&left_node_id).unwrap();
                    param_name = left_node.string_val.clone();

                    // insert parameter into the current scope
                    // (causing potential conflicts with local variables defined inside the function body)

                    // the naming source first checks if the name is used already (which is not allowed)
                    // throws panic if the variable name is already used.
                    // To remember if a variable is used, it maintains a data structure of variables
                    let user_define_varname = self.variable_naming_source.borrow_mut()
                        .new_user_defined_var(&param_name);

                    let mut node = IdentifierResolutionNode::new();
                    node.node_type = AstNodeType::ParameterDeclaration;
                    node.string_val = user_define_varname;

                    return node;
                }
            }

            AstNodeType::VariadicParameterDeclaration => {
                // DEBUG
                if self.debug {
                    println!("AstNode-Id: {}, AstNodeType: {:?}", ast_node.id, ast_node.node_type);
                }

                let mut node = IdentifierResolutionNode::new();
                node.node_type = AstNodeType::VariadicParameterDeclaration;
                node.string_val = String::from("...");

                return node;
            }

            AstNodeType::Statement => {
                // DEBUG
                if self.debug {
                    println!("[Statement] AstNodeType: {:?}", ast_node.node_type);
                    println!("{:?}", ast_node);
                }

                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map);
                }
            }

            AstNodeType::Block => {
                // DEBUG
                if self.debug {
                    println!("AstNodeId: {:?}, AstNodeType: {:?}, build_new_scope: {}", ast_node.id, ast_node.node_type, build_new_scope);
                }

                // for the special case of function declarations with body,
                // the function declaration will create a new scope and insert
                // the function's name and parameters into that scope and the body
                // of that function declaration needs to be processed within that
                // scope instead of creating a new scope because local variables
                // in the body of the function declarations need to clash with
                // parameter names as it is not valid to shadow parameters using
                // local variables!
                if build_new_scope {
                    // DEBUG
                    // println!("[AstNodeType::Block ENTER_SCOPE]");
                    self.variable_naming_source.borrow_mut().enter_scope();
                }

                for i in 0..ast_node.block_items.len() {

                    // DEBUG
                    if self.debug {
                        println!("BlockItem {}:", i+1);
                    }

                    let index = ast_node.block_items.len()-1-i;
                    self.visit(ast_node.block_items[index], node_map);
                }

                if build_new_scope {
                    // DEBUG
                    // println!("[AstNodeType::Block EXIT_SCOPE]");
                    self.variable_naming_source.borrow_mut().exit_scope();
                }
            }

            AstNodeType::BlockItem => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map);
                }
            }

            AstNodeType::Conditional => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Compound => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map);
                }
            }

            AstNodeType::DoWhile => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::While | AstNodeType::For => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // LHS - initialization, e.g.: a = 0
                if let Some(left_node_id) = ast_node.lhs {
                    let replaced_var_name_node = self.visit(left_node_id, node_map);
                    let replaced_var_name = replaced_var_name_node.string_val;

                    // DEBUG
                    if self.debug {
                        println!("{:?}", replaced_var_name);
                    }

                    let mut left_node = node_map.get(&left_node_id).unwrap().clone();
                    left_node.string_val = replaced_var_name;

                    node_map.insert(left_node.id, left_node);
                }

                // Expression - expression_ast_node, condition, e.g. a < 10
                if let Some(expression_node_id) = ast_node.expression {
                    self.visit(expression_node_id, node_map);
                }

                // RHS - post, e.g.: a = a + 1
                if let Some(right_node_id) = ast_node.rhs {
                    let replaced_var_name_node = self.visit(right_node_id, node_map);
                    let replaced_var_name = replaced_var_name_node.string_val;

                    // DEBUG
                    if self.debug {
                        println!("{:?}", replaced_var_name);
                    }

                    let mut right_node = node_map.get(&right_node_id).unwrap().clone();
                    right_node.string_val = replaced_var_name;

                    node_map.insert(right_node.id, right_node);
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
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                let mut node = IdentifierResolutionNode::new();
                node.node_type = AstNodeType::FunctionCall;
                node.string_val = ast_node.string_val.clone();

                return node;
            }

            AstNodeType::StorageClassSpecifier => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Pointer => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Switch => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Case => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Default => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Break => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Continue => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::EmptyStatement => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::SingleInit => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(left_node_id, node_map);
                }
            }

            AstNodeType::CompoundInit => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Subscript => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                    println!("{:?}", ast_node);
                }

                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    let replaced_var_name_node: IdentifierResolutionNode = self.visit(left_node_id, node_map);

                    let initial_var_name = replaced_var_name_node.initial_string_val.clone();
                    let replaced_var_name = replaced_var_name_node.string_val.clone();

                    let mut left_node = node_map.get(&left_node_id).unwrap().clone();
                    left_node.string_val = replaced_var_name;
                    left_node.initial_string_val = initial_var_name;

                    // DEBUG
                    if self.debug {
                        println!("[IdentifierResolutionVisitor] NodeId: {}, Initial:{:?}, Replaced:{:?}",
                            &left_node.id, &replaced_var_name_node.initial_string_val, &replaced_var_name_node.string_val);
                    }

                    node_map.insert(left_node.id, left_node);
                }

                // RHS
                if let Some(right_node_id) = ast_node.rhs {
                    let replaced_var_name_node: IdentifierResolutionNode = self.visit(right_node_id, node_map);

                    let initial_var_name = replaced_var_name_node.initial_string_val.clone();
                    let replaced_var_name = replaced_var_name_node.string_val.clone();

                    let mut right_node = node_map.get(&right_node_id).unwrap().clone();
                    right_node.string_val = replaced_var_name;
                    right_node.initial_string_val = initial_var_name;

                    // DEBUG
                    if self.debug {
                        println!("[IdentifierResolutionVisitor] NodeId: {}, Initial:{:?}, Replaced:{:?}",
                            &right_node.id, &replaced_var_name_node.initial_string_val, &replaced_var_name_node.string_val);
                    }

                    node_map.insert(right_node.id, right_node);
                }
            }

            AstNodeType::MemberDeclaration => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::MemberTypeDeclaration => {

            }

            AstNodeType::Dot | AstNodeType::Arrow => {
                // DEBUG
                // if self.debug {
                    println!("[Dot] AstNodeId: {}, AstNodeType: {:?}", ast_node.id, ast_node.node_type);
                // }

                // LHS
                if let Some(left_node_id) = ast_node.lhs {

                    // visit left node and retrieve exchanged name
                    let indentifier_resolution_node = self.visit(left_node_id, node_map);

                    println!("{:?}", indentifier_resolution_node);

                    // put exchanged name into node and update node in the node_map
                    let mut left_node = node_map.get(&left_node_id).unwrap().clone();
                    left_node.string_val = indentifier_resolution_node.string_val.clone();
                    left_node.initial_string_val = indentifier_resolution_node.initial_string_val.clone();

                    // update node in map after field change
                    node_map.insert(left_node.id, left_node);
                }

                // DO NOT VISIT RHS after DOT or ARROW because the names of fields
                // of structs are not replaced
                // RHS
                if let Some(right_node_id) = ast_node.rhs {
                    self.is_in_dot_or_arrow = true;
                    self.visit(right_node_id, node_map);
                    self.is_in_dot_or_arrow = false;
                }
            }

            // AstNodeType::Arrow => {
            //     // DEBUG
            //     //if self.debug {
            //         println!("[Arrow] AstNodeId: {}, AstNodeType: {:?}", ast_node.id, ast_node.node_type);
            //     //}

            //     self.is_in_dot_or_arrow = true;

            //     // LHS
            //     if let Some(left_node_id) = ast_node.lhs {
            //         let indentifier_resolution_node = self.visit(left_node_id, node_map);
            //     }

            //     // DO NOT VISIT RHS after DOT or ARROW because the names of fields
            //     // of structs are not replaced
            //     // RHS
            //     if let Some(right_node_id) = ast_node.rhs {
            //         self.is_in_dot_or_arrow = true;
            //         self.visit(right_node_id, node_map);
            //         self.is_in_dot_or_arrow = false;
            //     }
            // }

            AstNodeType::AssignmentOperator => {
                // DEBUG
                if self.debug {
                    println!("[AssignmentOperator] AstNodeType: {:?}", ast_node.node_type);
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

            AstNodeType::Cast => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", &ast_node.node_type);
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

            AstNodeType::AddAssignment => {
                todo!();
            }

            AstNodeType::Typedef => {
            }

            AstNodeType::TypeSpecifier(type_specifier_value) => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType.TypeSpecifier: {:?}", type_specifier_value);
                }
            }

            AstNodeType::Unknown => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
                panic!("Unknown");
            }
        }

        return IdentifierResolutionNode::new();
    }
}
