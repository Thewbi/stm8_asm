use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::cell::RefCell;

use crate::AstNode;
use crate::AstNodeType;

use crate::c_ast::ast_node::AstNodeOperatorType;
use crate::c_ast::identifier_resolution_node::IdentifierResolutionNode;

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
// 1. c_ast/IdentifierResolutionVisitor - checks for duplicate or undeclared variable names
// 2. tacky/TackyVisitor - Generate TACKY (from AST)
// 3. asm_ast/AsmAstConversionVisitor - Converts the AST into a ASM AST for assembly with a precursory form of mnenomics
// 4. asm_ast/AsmAstFixupVisitor - replacing pseudo operands/variables with stack addresses
// 5. asm_ast/AsmAstMasmEmitterVisitor / asm_ast/AsmAstASEmitterVisitor / ...
//

pub struct IdentifierResolutionVisitor {
    variable_naming_source: Rc<RefCell<VariableNamingSource>>, // https://www.youtube.com/watch?v=8O0Nt9qY_vo
    debug: bool,
    identifier_resolution_node: IdentifierResolutionNode,
}

impl IdentifierResolutionVisitor {

    pub fn new(variable_naming_source_param: Rc<RefCell<VariableNamingSource>>) -> IdentifierResolutionVisitor {
        IdentifierResolutionVisitor {
            variable_naming_source: variable_naming_source_param,
            debug: false,
            identifier_resolution_node: IdentifierResolutionNode::new(),
        }
    }

    pub fn visit(&mut self,
        ast_node: &AstNode,
        node_map: &HashMap::<usize, AstNode>)
        // -> IdentifierResolutionNode
        -> HashMap::<usize, AstNode>
    {
        self.visit_ex(ast_node, true, node_map)
    }

    pub fn visit_ex(&mut self,
        ast_node: &AstNode,
        build_new_scope: bool,
        node_map: &HashMap::<usize, AstNode>
    )
        // -> IdentifierResolutionNode
        -> HashMap::<usize, AstNode>
    {

        // DEBUG
        if self.debug {
            println!("[visit_ex()] {:?}", ast_node.node_type);
        }

        let mut node_map = node_map.clone();

        match ast_node.node_type {

            AstNodeType::Program => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // visit all items in the body of the program
                for i in 0..ast_node.block_items.len() {

                    let index = ast_node.block_items.len()-1-i;
                    // let rc = ast_node.block_items[index].as_ref();
                    // self.visit(&mut rc.borrow_mut());

                    self.visit(&node_map.get(&ast_node.block_items[index]).unwrap(), &node_map);
                }
            }

            AstNodeType::ConstInt => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // convert AstNode to IdentifierResolutionNode
                // let mut semantic_analysis_node: IdentifierResolutionNode = IdentifierResolutionNode::new();
                // semantic_analysis_node.node_type = ast_node.node_type.clone();
                // semantic_analysis_node.string_val = ast_node.string_val.clone();

                self.identifier_resolution_node.node_type = ast_node.node_type.clone();
                self.identifier_resolution_node.string_val = ast_node.string_val.clone();

                // return semantic_analysis_node;
            }

            AstNodeType::ConstLong => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // let mut semantic_analysis_node: IdentifierResolutionNode = IdentifierResolutionNode::new();
                self.identifier_resolution_node.node_type = ast_node.node_type.clone();
                self.identifier_resolution_node.string_val = ast_node.string_val.clone();

                // return semantic_analysis_node;
            }

            AstNodeType::ConstUInt => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // let mut semantic_analysis_node: IdentifierResolutionNode = IdentifierResolutionNode::new();
                self.identifier_resolution_node.node_type = ast_node.node_type.clone();
                self.identifier_resolution_node.string_val = ast_node.string_val.clone();

                // return semantic_analysis_node;
            }

            AstNodeType::ConstULong => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // let mut semantic_analysis_node: IdentifierResolutionNode = IdentifierResolutionNode::new();
                self.identifier_resolution_node.node_type = ast_node.node_type.clone();
                self.identifier_resolution_node.string_val = ast_node.string_val.clone();

                // return semantic_analysis_node;
            }

            AstNodeType::ConstDouble => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // let mut semantic_analysis_node: IdentifierResolutionNode = IdentifierResolutionNode::new();
                self.identifier_resolution_node.node_type = ast_node.node_type.clone();
                self.identifier_resolution_node.string_val = ast_node.string_val.clone();

                // return semantic_analysis_node;
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
                    println!("AstNodeType: {:?}", ast_node.node_type);
                    println!("{:?}", ast_node);
                }

                let mut node_map_clone = node_map.clone();

                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    let mut left_node = node_map_clone.get(&left_node_id).unwrap().clone();

                    // replace user-choosen variable name by unique variable name
                    // let replaced_var_name_node: IdentifierResolutionNode = self.visit(&left_node, &node_map_clone);
                    self.visit(&left_node, &node_map_clone);
                    let replaced_var_name = self.identifier_resolution_node.string_val.clone();
                    left_node.string_val = replaced_var_name;

                    node_map_clone.insert(left_node.id, left_node);

                    // // DEBUG
                    // println!("{:?}", replaced_var_name);
                }

                // RHS
                if let Some(right_node_id) = ast_node.rhs {
                    let mut right_node = node_map_clone.get(&right_node_id).unwrap().clone();

                    // replace user-choosen variable name by unique variable name
                    // let replaced_var_name_node: IdentifierResolutionNode = self.visit(&right_node, &node_map_clone);
                    self.visit(&right_node, &node_map_clone);
                    let replaced_var_name = self.identifier_resolution_node.string_val.clone();
                    right_node.string_val = replaced_var_name;

                    node_map_clone.insert(right_node.id, right_node);
                }

                return node_map_clone;
            }

            AstNodeType::Identifier => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // let mut identifier_resolution_node = IdentifierResolutionNode::new();
                self.identifier_resolution_node.node_type = ast_node.node_type.clone();
                let user_choosen_variable_name = ast_node.string_val.clone();
                // DEBUG
                // println!("user_choosen_variable_name: {:?}", user_choosen_variable_name);

                //
                // Replace Variable Name
                //

                self.identifier_resolution_node.string_val = match self.variable_naming_source.borrow_mut().get_replaced_variable_name(&user_choosen_variable_name) {
                    Ok(var_name) => var_name,
                    Err(e) => {
                        panic!("{}", e);
                    }
                };

                // return identifier_resolution_node;
            }

            AstNodeType::Return => {
                // DEBUG
                if self.debug {
                    println!("AstNodeType.RETURN: {:?}", ast_node);
                }

                if let Some(left_node_id) = ast_node.lhs {

                    let mut node_map_clone = node_map.clone();

                    let mut left_node = node_map_clone.get(&left_node_id).unwrap().clone();

                    // let semant_node: IdentifierResolutionNode = self.visit(&left_node, &node_map_clone);
                    self.visit(&left_node, &node_map_clone);

                    // println!("{:?}", semant_node);

                    let user_choosen_variable_name = left_node.string_val.clone();
                    let replaced_variable_name = self.identifier_resolution_node.string_val.clone();

                    // DEBUG
                    println!("user_choosen_variable_name: '{}', replaced_variable_name: '{}'",
                        user_choosen_variable_name, replaced_variable_name);

                    left_node.string_val = replaced_variable_name.clone();

                    node_map_clone.insert(left_node.id, left_node);

                    return node_map_clone;
                }
            }

            AstNodeType::If => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // Important: replace variable names in potential if expressions

                // Expression
                if let Some(expression_node_id) = ast_node.expression {
                    self.visit(node_map.get(&expression_node_id).unwrap(), &node_map);
                }

                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(node_map.get(&left_node_id).unwrap(), &node_map);
                }

                // RHS
                if let Some(right_node_id) = ast_node.rhs {
                    self.visit(node_map.get(&right_node_id).unwrap(), &node_map);
                }
            }

            AstNodeType::Unary => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                    println!("AstNode: {:?}", ast_node);
                }

                // let mut node = IdentifierResolutionNode::new();
                self.identifier_resolution_node.node_type = ast_node.node_type.clone();

                let mut node_map_clone = HashMap::<usize, AstNode>::new();

                if let Some(rhs_node_id) = ast_node.rhs {

                    let mut rhs_node = node_map_clone.get(&rhs_node_id).unwrap().clone();

                    let user_choosen_variable_name = rhs_node.string_val.clone();

                    let replaced_var_name = match self.variable_naming_source.borrow_mut().get_replaced_variable_name(&user_choosen_variable_name) {
                        Ok(var_name) => var_name,
                        Err(e) => {
                            panic!("{}", e);
                        }
                    };

                    // DEBUG
                    println!("user_choosen_variable_name: '{:?}', replaced_var_name: '{:?}'", user_choosen_variable_name, replaced_var_name);

                    rhs_node.string_val = replaced_var_name.clone();

                    node_map_clone.insert(rhs_node.id, rhs_node);

                    self.identifier_resolution_node.string_val = replaced_var_name.clone();
                }

                return node_map_clone;
            }

            AstNodeType::Binary => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                let mut node_map_clone = HashMap::<usize, AstNode>::new();

                // LHS
                if let Some(left_node_id) = ast_node.lhs {

                    let left_node = node_map.get(&left_node_id).unwrap();

                    if self.debug {
                        println!("{:?}", left_node);
                        println!("{:?}", left_node.string_val);
                    }

                    //let lhs_result: IdentifierResolutionNode = self.visit(left_node, node_map);
                    node_map_clone = self.visit(left_node, &node_map_clone);

                    // // DEBUG
                    // if self.debug {
                    //     println!("{:?}", lhs_result);
                    // }

                    match self.identifier_resolution_node.node_type {

                        AstNodeType::Identifier => {
                            //let is_defined = self.variable_naming_source.borrow_mut().is_variable_name_defined(&lhs_result.string_val);

                            // check the original user-choosen variable name
                            let is_defined = self.variable_naming_source.borrow_mut().is_variable_name_defined(&left_node.string_val);
                            if !is_defined {
                                panic!("Variable \"{}\" is not defined!", &self.identifier_resolution_node.string_val);
                            }
                            //let new_name = match self.variable_naming_source.borrow_mut().get_replaced_variable_name(&lhs_result.string_val) {
                            let new_name = match self.variable_naming_source.borrow_mut().get_replaced_variable_name(&left_node.string_val) {
                                Ok(var_name) => var_name,
                                Err(e) => {
                                    panic!("{}", e);
                                }
                            };
                            if self.debug {
                                println!("{:?}", new_name);
                            }

                            let mut left_node_clone = left_node.clone();
                            left_node_clone.string_val = new_name;

                            node_map_clone.insert(left_node_clone.id, left_node_clone);
                        }
                        _ => {

                        }
                    }
                }

                // RHS
                if let Some(right_node_id) = ast_node.rhs {
                    let right_node = node_map.get(&right_node_id).unwrap();

                    //let rhs_result: IdentifierResolutionNode = self.visit(right_node, node_map);

                    node_map_clone = self.visit(right_node, &node_map_clone);

                    // DEBUG
                    if self.debug {
                        println!("{:?}", self.identifier_resolution_node);
                    }

                    match self.identifier_resolution_node.node_type {
                        AstNodeType::Identifier => {

                            let mut right_node_clone = right_node.clone();
                            right_node_clone.string_val = self.identifier_resolution_node.string_val.clone();

                            node_map_clone.insert(right_node_clone.id, right_node_clone);
                        }
                        _ => {

                        }
                    }
                }

                return node_map_clone;
            }

            AstNodeType::Operator => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::PrefixOperator => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::DataType => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Declaration => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(node_map.get(&left_node_id).unwrap(), &node_map);
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
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

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
                    let user_define_function_name
                        = self.variable_naming_source.borrow_mut().new_function_declaration(&function_name.string_val);

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

                let mut node_map_clone = HashMap::<usize, AstNode>::new();

                // parameters - are visited in the same context (= same varname_map on the stack) as the function declaration itself
                for i in 0..ast_node.parameters.len() {
                    // let index = ast_node.parameters.len()-1-i;
                    // self.visit(&mut ast_node.parameters[index]);

                    let id = ast_node.parameters[i];
                    let node = node_map.get(&id).unwrap();

                    node_map_clone = self.visit(&node, &node_map);
                }

                //
                // body of function == block with statements as block_items
                // is visited in the same context (= same varname_map on the stack) as the function declaration and it's parameters
                //

                if let Some(left_node_id) = ast_node.lhs {

                    let left_node = node_map.get(&left_node_id).unwrap();

                    if self.debug {
                        print!("{:?}", left_node);
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
                    self.visit_ex(left_node, build_new_scope, &node_map);

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

                // data type
                if let Some(left_node_id) = ast_node.lhs {
                    // DEBUG
                    if self.debug {
                        print!("{:?}", left_node_id);
                    }
                }

                let mut user_define_varname = String::new();

                // identifier (RHS)
                let mut varname = String::from("");
                if let Some(right_node_id) = ast_node.rhs {

                    let mut right_node = node_map.get(&right_node_id).unwrap().clone();

                    // DEBUG
                    if self.debug {
                        print!("{:?}", right_node);
                    }

                    varname = right_node.string_val.clone();

                    // next: implement page 105

                    // The naming source checks if the name is used already (which is not allowed)
                    // throws panic if the variable name is already used.
                    // To remember if a variable is used, it maintains a data structure of variables
                    user_define_varname = self.variable_naming_source.borrow_mut().new_user_defined_var(&varname);

                    // DEBUG
                    if self.debug {
                        println!("Test {}", user_define_varname);
                    }

                    // replace the variable name by the unique variable name from the scope
                    // Subsequent steps such as TACKY generation will use unique names instead
                    // of duplicate names separated by scopes
                    right_node.string_val = user_define_varname.clone();

                    node_map.insert(right_node.id, right_node);
                }

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
                    self.visit(expression_node, &node_map);
                }

                // TODO: return VariableDeclaration node

                // let mut node = IdentifierResolutionNode::new();
                self.identifier_resolution_node.node_type = AstNodeType::VariableDeclaration;

                // the variable name is changed to a unique variable name to handle/represent nested scopes
                // Subsequent steps such as TACKY generation will not know about scopes as the variable
                // names have already been made unique by the linearization applied here!
                self.identifier_resolution_node.string_val = user_define_varname;

                // return node;
            }

            AstNodeType::StructureDeclaration => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::ParameterDeclaration => {
                if self.debug {
                    println!("AstNode-Id: {}, AstNodeType: {:?}", ast_node.id, ast_node.node_type);
                }

                // parameter's data type (RHS)
                if let Some(right_node_id) = ast_node.rhs {

                    let right_node = node_map.get(&right_node_id).unwrap();

                    // data type
                    let data_type = right_node.string_val.clone();

                    // DEBUG
                    // println!("DataType: '{:?}'", data_type);

                    // void parameters are not inserted into the name map
                    if data_type == "void" {
                        // return IdentifierResolutionNode::new();
                    }
                }

                // parameter's identifier (LHS)
                let mut param_name = String::from("");
                if let Some(left_node_id) = ast_node.lhs {

                    let left_node = node_map.get(&left_node_id).unwrap();

                    // DEBUG
                    if self.debug {
                        print!("{:?}", left_node);
                    }

                    param_name = left_node.string_val.clone();

                    // insert parameter into the current scope
                    // (causing potential conflicts with local variables defined inside the function body)

                    // The naming source first checks if the name is used already (which is not allowed)
                    // throws panic if the variable name is already used.
                    // To remember if a variable is used, it maintains a data structure of variables
                    let user_define_varname = self.variable_naming_source.borrow_mut().new_user_defined_var(&param_name);

                    // let mut node = IdentifierResolutionNode::new();
                    self.identifier_resolution_node.node_type = AstNodeType::ParameterDeclaration;
                    self.identifier_resolution_node.string_val = user_define_varname;

                    // return node;
                }
            }

            AstNodeType::Statement => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                    println!("{:?}", ast_node);
                }

                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(node_map.get(&left_node_id).unwrap(), &node_map);
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
                    let block_item = node_map.get(&ast_node.block_items[index]).unwrap();
                    self.visit(&block_item, &node_map);
                }

                if build_new_scope {
                    // DEBUG
                    // println!("[AstNodeType::Block EXIT_SCOPE]");
                    self.variable_naming_source.borrow_mut().exit_scope();
                }
            }

            AstNodeType::BlockItem => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // LHS
                if let Some(left_node_id_id) = ast_node.lhs {
                    self.visit(node_map.get(&left_node_id_id).unwrap(), &node_map);
                }
            }

            AstNodeType::Conditional => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Compound => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(node_map.get(&left_node_id).unwrap(), &node_map);
                }
            }

            AstNodeType::DoWhile => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::While | AstNodeType::For => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // LHS - initialization, e.g.: a = 0
                let lhs_ast_node_id = 0;
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(node_map.get(&left_node_id).unwrap(), &node_map);
                }

                // Expression - expression_ast_node, condition, e.g. a < 10
                let rhs_ast_node_id = 0;
                if let Some(expression_node_id) = ast_node.expression {
                    self.visit(node_map.get(&expression_node_id).unwrap(), &node_map);
                }

                // RHS - post, e.g.: a = a + 1
                let rhs_ast_node_id = 0;
                if let Some(right_node_id) = ast_node.rhs {

                    let mut right_node = node_map.get(&right_node_id).unwrap().clone();

                    // let replaced_var_name_node: IdentifierResolutionNode = self.visit(right_node, node_map);
                    // let replaced_var_name = replaced_var_name_node.string_val;
                    node_map = self.visit(&right_node, &node_map);
                    let replaced_var_name = self.identifier_resolution_node.string_val.clone();

                    // DEBUG
                    println!("{:?}", replaced_var_name);

                    right_node.string_val = replaced_var_name;

                    node_map.insert(right_node.id, right_node);
                }

                // BLOCK_ITEMS - instructions and declarations
                for i in 0..ast_node.block_items.len() {
                    let idx = ast_node.block_items.len()-1-i;
                    let mut block_item = node_map.get(&ast_node.block_items[idx]).unwrap();
                    self.visit(&mut block_item, &node_map);
                }
            }

            AstNodeType::FunctionCall => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // let mut node = IdentifierResolutionNode::new();
                self.identifier_resolution_node.node_type = AstNodeType::FunctionCall;
                self.identifier_resolution_node.string_val = ast_node.string_val.clone();

                // return node;
            }

            AstNodeType::StorageClassSpecifier => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Pointer => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Switch => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Case => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Default => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Break => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Continue => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::EmptyStatement => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::SingleInit => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }

                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(node_map.get(&left_node_id).unwrap(), &node_map);
                }
            }

            AstNodeType::CompoundInit => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Subscript => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::MemberDeclaration => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Dot => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Arrow => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::AssignmentOperator => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
            }

            AstNodeType::Cast => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
                // LHS
                if let Some(left_node_id) = ast_node.lhs {
                    self.visit(node_map.get(&left_node_id).unwrap(), &node_map);
                }
                // RHS
                if let Some(right_node_id) = ast_node.rhs {
                    self.visit(node_map.get(&right_node_id).unwrap(), &node_map);
                }
            }

            AstNodeType::Unknown => {
                if self.debug {
                    println!("AstNodeType: {:?}", ast_node.node_type);
                }
                panic!("Unknown");
            }
        }

        // return IdentifierResolutionNode::new();
        return node_map.clone();
    }
}


// match self.variable_naming_source.borrow_mut().get_replaced_variable_name(&user_choosen_variable_name) {
                    //     Ok(var_name) => {
                    //         // DEBUG
                    //         // println!("user_choosen_variable_name: {:?} resolved into {:?}", user_choosen_variable_name, var_name);
                    //         left_node.string_val = var_name.clone();
                    //         var_name
                    //     }
                    //     Err(e) => {
                    //         // println!("test");
                    //         //panic!("{}", e);
                    //         e
                    //     }
                    // };

// let mut node = IdentifierResolutionNode::new();
                // node.node_type = ast_node.node_type.clone();

                // if let Some(mut left_node) = ast_node.lhs.as_mut() {
                //     print!("{:?}", left_node);

                //     let user_choosen_variable_name = left_node.string_val.clone();

                //     node.string_val = match self.variable_naming_source.borrow_mut().get_replaced_variable_name(&user_choosen_variable_name) {
                //         Ok(var_name) => {
                //             // DEBUG
                //             println!("user_choosen_variable_name: {:?} resolved into {:?}", user_choosen_variable_name, node.string_val);

                //             left_node.string_val = node.string_val;

                //             var_name
                //         }
                //         Err(e) => {
                //             //panic!("{}", e);
                //         }
                //     };

                // }

                // return node;

                // // LHS
                // if let Some(left_node) = ast_node.lhs.as_mut() {

                //     println!("AstNodeType.RETURN.LHS: {:?}", left_node);

                //     let semant_node: IdentifierResolutionNode = self.visit(left_node);
                //     left_node.string_val = semant_node.string_val.clone();
                // }

                // // RHS
                // if let Some(right_node) = ast_node.rhs.as_mut() {
                //     let semant_node: IdentifierResolutionNode = self.visit(right_node);

                //     right_node.string_val = semant_node.string_val.clone();
                // }

                // let is_defined = self.variable_naming_source.borrow_mut().is_variable_name_defined(&rhs_result.string_val);
                            // if !is_defined {
                            //     panic!("Variable \"{}\" is not defined!", &rhs_result.string_val);
                            // }

                            // let new_name = self.variable_naming_source.borrow_mut().get_replaced_variable_name(&rhs_result.string_val);
                            // if self.debug {
                            //     println!("{:?}", new_name);
                            // }
                            // right_node.string_val = new_name;

// ast_node.rhs = Some(Box::new(data_type_ast_node));
                //    object_declaration_ast_node.lhs = Some(Box::new(identifier_ast_node));
                //    object_declaration_ast_node.string_val = ident;
                // data_type (return value)