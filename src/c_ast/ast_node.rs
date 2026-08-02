use std::fmt;
use std::fmt::Display;

use std::{
    sync::atomic::{AtomicUsize, Ordering}
};

static AST_NODE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Just to be fair, I asked AI on how to design a recursive data structure
// which can be iterated over recursively several times without beeing consume
// and AI delivered ...
//
// I would not have been able to do this without the use of AI :(

#[derive(Debug, PartialEq)]
pub enum AstNodeType {
    Program,
    ConstInt,
    ConstLong,
    ConstUInt,
    ConstULong,
    ConstDouble,
    Structure,
    Array,
    Expression,
    Identifier,
    Return,
    If,
    Unary,
    Binary,
    Operator,
    DataType,
    Declaration, // variable declaration or function declaration
    FunctionDeclaration,
    VariableDeclaration,
    StructureDeclaration,
    // StructDeclaration,
    ParameterDeclaration,
    Statement,
    Block,
    BlockItem,
    Conditional, // elvis operator
    Compound,
    While,
    DoWhile,
    For,
    FunctionCall,
    StorageClassSpecifier,
    Pointer,
    Switch,
    Case,
    Default,
    Break,
    Continue,
    EmptyStatement,
    SingleInit,
    CompoundInit,
    Subscript,
    MemberDeclaration,
    Dot,
    Arrow,
    Unknown,
}

#[derive(Debug)]
pub enum AstNodeOperatorType {
    Negate,
    Complement,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    LessThan,
    GreaterThan,
    Assignment,
    FunctionCall,
    NotApplicable,
    Cast,
    Dereference,
    AddrOf,
    Increment,
    Decrement,
    Dot,
    Arrow,
    SizeOf,
}

pub struct AstNode {
    pub node_type: AstNodeType,
    pub lhs: Option<Box<AstNode>>,
    pub rhs: Option<Box<AstNode>>,
    pub data_type: Option<Box<AstNode>>,
    pub expression: Option<Box<AstNode>>,
    pub operator: Option<Box<AstNode>>,
    pub operator_type: AstNodeOperatorType,
    pub string_val: String,
    pub block_items: Vec<Box<AstNode>>,
    pub parameters: Vec<Box<AstNode>>,
    pub storage_class: Option<Box<AstNode>>,
    pub is_extern: bool,
    pub is_static: bool,
    pub indent: usize,
}

impl fmt::Debug for AstNode {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match &self.node_type {

            AstNodeType::Compound => {
                println!("Compound");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("{:?}", left_node);
                }
            }

            AstNodeType::Block => {
                println!("Block");
                for i in 0..self.block_items.len() {
                    print!("{:?}", self.block_items[i]);
                }
            }

            AstNodeType::BlockItem => {
                println!("BlockItem");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("{:?}", left_node);
                }
            }

            AstNodeType::Statement => {
                println!("Statement");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("{:?}", left_node);
                }
            }

            AstNodeType::Return => {
                println!("Return");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("{:?}", left_node);
                }
            }

            AstNodeType::ConstInt | AstNodeType::ConstLong => {
                print!("Constant ");
                println!("{:?}", self.string_val);
            }

            AstNodeType::Declaration => {
                println!("Declaration");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("{:?}", left_node);
                }
            }

            AstNodeType::ParameterDeclaration => {
                println!("ParameterDeclaration");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("{:?}", left_node);
                }
            }            

            AstNodeType::VariableDeclaration => {
                print!("VariableDeclaration ");
                println!("{:?}", self.string_val);

                // data type
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("{:?}", left_node);
                }
                // identifier
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("{:?}", right_node);
                }
            }

            AstNodeType::DataType => {
                print!("DataType ");
                println!("'{:?}'", self.string_val);
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("{:?}", left_node);
                }
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("{:?}", right_node);
                }
            }

            AstNodeType::FunctionDeclaration => {
                println!("FunctionDeclaration");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("{:?}", left_node);
                }
                print!("{:?}", self.parameters);
            }

            AstNodeType::Identifier => {
                print!("Identifier: ");
                println!("'{:?}'", self.string_val);
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("{:?}", left_node);
                }
            }

            AstNodeType::Case => {
                print!("Case: ");
                println!("'{:?}'", self.string_val);
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("{:?}", left_node);
                }
            }

            AstNodeType::Default => {
                print!("Default: ");
                println!("'{:?}'", self.string_val);
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("{:?}", left_node);
                }
            }

            AstNodeType::Break => {
                println!("Break: ");
            }

            AstNodeType::Expression => {
                print!("Expression: ");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("LHS: {:?}", left_node);
                }
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("RHS {:?}", right_node);
                }
            }

            AstNodeType::Unary => {
                print!("Unary: ");
                println!("  Unary.OperatorType: {:?}", self.operator_type);
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("LHS: {:?}", left_node);
                }
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("RHS: {:?}", right_node);
                }
            }

            AstNodeType::Binary => {
                print!("Binary: ");
                println!("  Binary.OperatorType: {:?}", self.operator_type);
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("LHS: {:?}", left_node);
                }
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("RHS: {:?}", right_node);
                }
            }

            AstNodeType::Operator => {
                println!("Operator {:?}", self.operator);
            }

            AstNodeType::EmptyStatement => {
                println!("EmptyStatement");
            }

            AstNodeType::Subscript => {
                println!("Subscript");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("LHS: {:?}", left_node);
                }
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("RHS: {:?}", right_node);
                }
            }

            AstNodeType::Array => {
                println!("Array");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("LHS: {:?}", left_node);
                }
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("RHS: {:?}", right_node);
                }
            }

            AstNodeType::Structure => {
                println!("Struct");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("LHS: {:?}", left_node);
                }
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("RHS: {:?}", right_node);
                }
            }

            AstNodeType::Dot => {
                println!("Dot");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("LHS: {:?}", left_node);
                }
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("RHS: {:?}", right_node);
                }
            }

            AstNodeType::Arrow => {
                println!("Arrow");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("LHS: {:?}", left_node);
                }
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("RHS: {:?}", right_node);
                }
            }

            _ => {
                println!("{:?}", self.node_type);
                todo!();
            }
        }

        Ok(())
    }
}

impl AstNode {

    pub fn new() -> Self {
        let ast_node = AstNode {
            node_type: AstNodeType::Unknown,
            lhs: None,
            rhs: None,
            data_type: None,
            expression: None,
            operator: None,
            operator_type: AstNodeOperatorType::NotApplicable,
            string_val: String::from(""),
            block_items: Vec::<Box<AstNode>>::new(),
            parameters: Vec::<Box<AstNode>>::new(),
            storage_class: None,
            is_extern: false,
            is_static: false,
            indent: 0,
        };
        ast_node
    }

    //
    // JSON
    //

    // pub fn pretty_print_ast_json(&self) {
    //     match self.node_type {
    //         AstNodeType::FunctionDefinition => {
    //             self.pretty_print_ast_function_definition_json();
    //         }
    //         AstNodeType::Return => {
    //             self.pretty_print_ast_return_json();
    //         }
    //         AstNodeType::Constant => {
    //             self.pretty_print_ast_constant_json();
    //         }
    //         AstNodeType::Unary => {
    //             self.pretty_print_ast_unary_json();
    //         }
    //         _ => {
    //             panic!("Test");
    //         }
    //     }
    // }

    // fn pretty_print_ast_function_definition_json(&self) {
    //     println!("{{");
    //     println!("\"name\":\"{}\",", self.string_val);
    //     println!("\"body\": [");
    //     if let Some(left_node) = self.lhs.as_ref() {
    //         left_node.pretty_print_ast_json();
    //     }
    //     println!("]");
    //     println!("}}");
    // }

    // fn pretty_print_ast_return_json(&self) {
    //     println!("{{");
    //     println!("\"name\":\"return\",");
    //     println!("\"value\":{{");
    //     if let Some(left_node) = self.lhs.as_ref() {
    //         left_node.pretty_print_ast_json();
    //     }
    //     println!("}}");
    //     println!("}}");
    // }

    // fn pretty_print_ast_constant_json(&self) {
    //     println!("\"constant\":\"{}\"", self.string_val);
    // }

    //
    // DOT graphviz - https://dreampuf.github.io/GraphvizOnline
    //

    pub fn set_indent(&mut self, indent_param: usize) {
        self.indent = indent_param;
    }

    pub fn pretty_print_ast_dot(&self, string_buffer: &mut String) -> usize {
        self.pretty_print_ast_dot_ex(string_buffer, "")
    }

    pub fn pretty_print_ast_dot_ex(&self, string_buffer: &mut String, extended_string: &str) -> usize {

        match self.node_type {
            AstNodeType::Program => {
                self.pretty_print_ast_program_dot(string_buffer)
            }
            AstNodeType::FunctionDeclaration => {
                self.pretty_print_ast_function_declaration_dot(string_buffer)
            }
            AstNodeType::Return => {
                self.pretty_print_ast_return_dot(string_buffer)
            }
            AstNodeType::ConstInt 
            | AstNodeType::ConstLong 
            | AstNodeType::ConstUInt 
            | AstNodeType::ConstULong 
            | AstNodeType::ConstDouble => {
                self.pretty_print_ast_constant_dot_ex(string_buffer, extended_string)
            }
            AstNodeType::Expression => {
                self.pretty_print_ast_expression_dot(string_buffer)
            }
            AstNodeType::Unary => {
                self.pretty_print_ast_unary_dot(string_buffer)
            }
            AstNodeType::Binary => {
                self.pretty_print_ast_binary_dot(string_buffer)
            }
            AstNodeType::Operator => {
                self.pretty_print_ast_operator_dot(string_buffer)
            }
            AstNodeType::BlockItem => {
                self.pretty_print_ast_block_item_dot(string_buffer, extended_string)
            }
            AstNodeType::Declaration => {
                self.pretty_print_ast_declaration_dot(string_buffer)
            }
            AstNodeType::VariableDeclaration => {
                self.pretty_print_ast_variable_declaration_dot(string_buffer)
            }
            AstNodeType::ParameterDeclaration => {
                self.pretty_print_ast_variable_declaration_dot(string_buffer)
            }
            AstNodeType::Statement => {
                self.pretty_print_ast_statement_dot(string_buffer, extended_string)
            }
            AstNodeType::DataType => {
                self.pretty_print_ast_datatype_dot(string_buffer, extended_string)
            }
            AstNodeType::Identifier => {
                self.pretty_print_ast_identifier_dot(string_buffer)
            }
            AstNodeType::If => {
                self.pretty_print_ast_if_dot(string_buffer)
            }
            AstNodeType::Compound => {
                self.pretty_print_ast_compound_dot(string_buffer)
            }
            AstNodeType::Block => {
                self.pretty_print_ast_block_dot(string_buffer)
            }
            AstNodeType::While => {
                self.pretty_print_ast_while_dot(string_buffer)
            }
            AstNodeType::DoWhile => {
                self.pretty_print_ast_do_while_dot(string_buffer)
            }
            AstNodeType::For => {
                self.pretty_print_ast_for_dot(string_buffer)
            }
            AstNodeType::Conditional => {
                self.pretty_print_ast_conditional_dot(string_buffer)
            }
            AstNodeType::FunctionCall => {
                self.pretty_print_ast_function_call_dot(string_buffer)
            }
            AstNodeType::StorageClassSpecifier => {
                self.pretty_print_ast_storage_class_dot(string_buffer)
            }
            AstNodeType::Pointer => {
                self.pretty_print_ast_pointer_dot(string_buffer)
            }
            AstNodeType::Switch => {
                self.pretty_print_ast_switch_dot(string_buffer)
            }
            AstNodeType::Case => {
                self.pretty_print_ast_case_dot(string_buffer)
            }
            AstNodeType::Default => {
                self.pretty_print_ast_default_dot(string_buffer)
            }
            AstNodeType::Break => {
                self.pretty_print_ast_break_dot(string_buffer)
            }
            AstNodeType::EmptyStatement => {
                self.pretty_print_ast_empty_statement_dot(string_buffer)
            }
            AstNodeType::SingleInit => {
                self.pretty_print_ast_single_init_dot(string_buffer)
            }
            AstNodeType::CompoundInit => {
                self.pretty_print_ast_compound_init_dot(string_buffer)
            }
            AstNodeType::Subscript => {
                self.pretty_print_ast_subscript_dot(string_buffer)
            }
            AstNodeType::StructureDeclaration => {
                self.pretty_print_ast_structure_declaration_dot(string_buffer)
            }
            AstNodeType::MemberDeclaration => {
                self.pretty_print_ast_member_declaration_dot(string_buffer)
            }
            AstNodeType::Array => {
                self.pretty_print_ast_array_dot(string_buffer)
            }
            AstNodeType::Structure => {
                self.pretty_print_ast_structure_dot(string_buffer)
            }
            AstNodeType::Dot => {
                self.pretty_print_ast_dot_dot(string_buffer)
            }
            AstNodeType::Arrow => {
                self.pretty_print_ast_arrow_dot(string_buffer)
            }
            _ => {
                panic!("{}", format!("Unhandled AST node_type: {:?}", self.node_type).as_str());
            }
        }
    }

    fn pretty_print_ast_program_dot(&self, string_buffer: &mut String) -> usize {

        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Program: {}\"]", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Program\"]\n", ast_node_id, ast_node_id).as_str());

        for i in 0..self.block_items.len() {

            let block_item_ast_node_id = self.block_items[self.block_items.len()-1-i].pretty_print_ast_dot(string_buffer);

            // connect parent and child
            // println!("{} -> {}", block_ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_function_declaration_dot(&self, string_buffer: &mut String) -> usize {
        
        // create node for this AstNode and also output the name into the label
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} FunctionDeclaration: {}\"]", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} FunctionDeclaration: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // return type
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot(string_buffer);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        // create node for the function name
        let identifier_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        string_buffer.push_str(format!("{} [label=\"{} Identifier: '{}'\"]\n", identifier_ast_node_id, identifier_ast_node_id, self.string_val).as_str());
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, identifier_ast_node_id).as_str());

        // create node for the parameters
        let parameters_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} params\"]", parameters_ast_node_id, parameters_ast_node_id);
        string_buffer.push_str(format!("{} [label=\"{} params\"]\n", parameters_ast_node_id, parameters_ast_node_id).as_str());
        // println!("{} -> {}", ast_node_id, parameters_ast_node_id);
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, parameters_ast_node_id).as_str());

        // add parameters into parameters block
        for i in 0..self.parameters.len() {
            let parameter_ast_node_id = self.parameters[self.parameters.len()-1-i].pretty_print_ast_dot(string_buffer);
            // connect parent and child
            // println!("{} -> {}", parameters_ast_node_id, parameter_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", parameters_ast_node_id, parameter_ast_node_id).as_str());
        }

        // create node for the body/block
        let block_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Body/Block: {}\"]", block_ast_node_id, block_ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Body/Block: {}\"]\n", block_ast_node_id, block_ast_node_id, self.string_val).as_str());
        // connect parent and child
        // println!("{} -> {}", ast_node_id, block_ast_node_id);
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_ast_node_id).as_str());

        // add instructions and declarations into body/block
        if let Some(block) = self.lhs.as_ref() {
            for i in 0..block.block_items.len() {
                let block_item_ast_node_id = block.block_items[block.block_items.len()-1-i].pretty_print_ast_dot(string_buffer);
                // connect parent and child
                // println!("{} -> {}", block_ast_node_id, block_item_ast_node_id);
                string_buffer.push_str(format!("{} -> {}\n", block_ast_node_id, block_item_ast_node_id).as_str());
            }
        }

        // add instructions and declarations into body/block
        for i in 0..self.block_items.len() {
            let block_item_ast_node_id = self.block_items[self.block_items.len()-1-i].pretty_print_ast_dot(string_buffer);
            // connect parent and child
            // println!("{} -> {}", block_ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", block_ast_node_id, block_item_ast_node_id).as_str());
        }

        // storage class
        if let Some(storage_class_node) = self.storage_class.as_ref() {

            let storage_class_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
            // println!("{} [label=\"{} StorageClass: {}\"]", storage_class_ast_node_id, storage_class_ast_node_id, self.string_val);
            string_buffer.push_str(format!("{} [label=\"{} StorageClass: {}\"]\n", storage_class_ast_node_id, storage_class_ast_node_id, storage_class_node.string_val).as_str());

            // storage_class_ast_node_id = storage_class_node.pretty_print_ast_dot(string_buffer);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, storage_class_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_return_dot(&self, string_buffer: &mut String) -> usize {

        // print the child tree
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot(string_buffer);
        }

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Return\"]", ast_node_id, ast_node_id);
        string_buffer.push_str(format!("{} [label=\"{} Return\"]\n", ast_node_id, ast_node_id).as_str());

        // connect parent and child
        if let Some(left_node) = self.lhs.as_ref() {
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_constant_dot_ex(&self, string_buffer: &mut String, extended_string: &str) -> usize {

        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

        match self.node_type {

            AstNodeType::ConstULong => {
                // println!("{} [label=\"{} Constant({})\"]", ast_node_id, ast_node_id, self.string_val);
                string_buffer.push_str(format!("{} [label=\"{} {} ConstULong({})\"]\n", ast_node_id, ast_node_id, extended_string, self.string_val).as_str());
            }

            AstNodeType::ConstUInt => {
                string_buffer.push_str(format!("{} [label=\"{} {} ConstUInt({})\"]\n", ast_node_id, ast_node_id, extended_string, self.string_val).as_str());
            }

            AstNodeType::ConstLong => {
                string_buffer.push_str(format!("{} [label=\"{} {} ConstLong({})\"]\n", ast_node_id, ast_node_id, extended_string, self.string_val).as_str());
            }

            AstNodeType::ConstInt => {
                string_buffer.push_str(format!("{} [label=\"{} {} ConstInt({})\"]\n", ast_node_id, ast_node_id, extended_string, self.string_val).as_str());
            }

            AstNodeType::ConstDouble => {
                string_buffer.push_str(format!("{} [label=\"{} {} ConstDouble({})\"]\n", ast_node_id, ast_node_id, extended_string, self.string_val).as_str());
            }

            _ => {
                panic!("Unhandeled case!");
            }
        }

        let mut data_type_ast_node_id = 0;
        if let Some(data_type_ast_node) = self.data_type.as_ref() {
            data_type_ast_node_id = data_type_ast_node.pretty_print_ast_dot(string_buffer);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, data_type_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_expression_dot(&self, string_buffer: &mut String) -> usize {

        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

        // println!("{} [label=\"{} Exp ({:?})\"]", ast_node_id, ast_node_id, self.operator_type);
        string_buffer.push_str(format!("{} [label=\"{} Exp ({:?})\"]\n", ast_node_id, ast_node_id, self.operator_type).as_str());

        // print the child tree
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {

            match self.operator_type {

                AstNodeOperatorType::Cast => {
                    // println!("test");
                    lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, "Cast-TargetType:");
                }

                _ => {
                    // println!("test2");
                    lhs_ast_node_id = left_node.pretty_print_ast_dot(string_buffer);
                }
            }
            
            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot(string_buffer);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        let mut data_type_ast_node_id = 0;
        if let Some(data_type_ast_node) = self.data_type.as_ref() {
            data_type_ast_node_id = data_type_ast_node.pretty_print_ast_dot(string_buffer);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, data_type_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_unary_dot(&self, string_buffer: &mut String) -> usize {

        // print the child tree
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot(string_buffer);
        }
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot(string_buffer);
        }
        let mut data_type_ast_node_id = 0;
        if let Some(data_type_node) = self.data_type.as_ref() {
            data_type_ast_node_id = data_type_node.pretty_print_ast_dot(string_buffer);
        }

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Unary\"]", ast_node_id, ast_node_id);
        string_buffer.push_str(format!("{} [label=\"{} Unary\"]\n", ast_node_id, ast_node_id).as_str());

        // connect parent and child
        // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());

        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, data_type_ast_node_id).as_str());

        ast_node_id
    }

    fn pretty_print_ast_binary_dot(&self, string_buffer: &mut String) -> usize {

        // print the child tree
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot(string_buffer);
        }
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot(string_buffer);
        }

        // print the operator tree
        let mut operator_ast_node_id = 0;
        if let Some(operator_node) = self.operator.as_ref() {
            operator_ast_node_id = operator_node.pretty_print_ast_dot(string_buffer);
        }

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Binary\"]", ast_node_id, ast_node_id);
        string_buffer.push_str(format!("{} [label=\"{} Binary\"]\n", ast_node_id, ast_node_id).as_str());

        // connect parent and child
        // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        // println!("{} -> {}", ast_node_id, operator_ast_node_id);
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, operator_ast_node_id).as_str());

        ast_node_id
    }

    fn pretty_print_ast_operator_dot(&self, string_buffer: &mut String) -> usize {

        // print the child tree
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot(string_buffer);
        }

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} {:?}\"]", ast_node_id, ast_node_id, self.operator_type);
        string_buffer.push_str(format!("{} [label=\"{} {:?}\"]\n", ast_node_id, ast_node_id, self.operator_type).as_str());

        match self.operator_type {

            AstNodeOperatorType::SizeOf => {
                let mut expression_ast_node_id = 0;
                if let Some(expression_node) = self.expression.as_ref() {
                    expression_ast_node_id = expression_node.pretty_print_ast_dot(string_buffer);
                    string_buffer.push_str(format!("{} -> {}\n", ast_node_id, expression_ast_node_id).as_str());
                }
            }

            _ => {

            }
        }

        ast_node_id
    }

    fn pretty_print_ast_block_item_dot(&self, string_buffer: &mut String, extended_string: &str) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} BlockItem\"]", ast_node_id, ast_node_id);
        string_buffer.push_str(format!("{} [label=\"{} BlockItem\"]\n", ast_node_id, ast_node_id).as_str());

        // print the child tree
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, extended_string);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_declaration_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Declaration\"]", ast_node_id, ast_node_id);
        string_buffer.push_str(format!("{} [label=\"{} Declaration\"]\n", ast_node_id, ast_node_id).as_str());

        // print the child tree
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot(string_buffer);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_variable_declaration_dot(&self, string_buffer: &mut String) -> usize {

        // data type
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot(string_buffer);
            println!("left_node: {:?}", left_node);
        }
        // name
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot(string_buffer);
        }
        // expression
        let mut expression_ast_node_id = 0;
        if let Some(expression_node) = self.expression.as_ref() {
            expression_ast_node_id = expression_node.pretty_print_ast_dot(string_buffer);
        }
        // storage class
        let mut storage_class_ast_node_id = 0;
        if let Some(storage_class_node) = self.storage_class.as_ref() {
            storage_class_ast_node_id = storage_class_node.pretty_print_ast_dot(string_buffer);
        }

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} VariableDeclaration\"]", ast_node_id, ast_node_id);
        string_buffer.push_str(format!("{} [label=\"{} VariableDeclaration\"]\n", ast_node_id, ast_node_id).as_str());

        // connect parent and child
        // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());

        if rhs_ast_node_id != 0 {
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        if expression_ast_node_id != 0 {
            // println!("{} -> {}", ast_node_id, expression_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, expression_ast_node_id).as_str());
        }

        if storage_class_ast_node_id != 0 {
            // println!("{} -> {}", ast_node_id, storage_class_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, storage_class_ast_node_id).as_str());
        }

        ast_node_id
    }
    
    fn pretty_print_ast_statement_dot(&self, string_buffer: &mut String, extended_string: &str) -> usize {

        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot(string_buffer);
        }

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Statement: {} {}\"]", ast_node_id, ast_node_id, extended_string, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Statement: {} {}\"]\n", ast_node_id, ast_node_id, extended_string, self.string_val).as_str());

        // connect parent and child
        // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());

        ast_node_id
    }

    fn pretty_print_ast_datatype_dot(&self, string_buffer: &mut String, extended_string: &str) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} DataType: {}\"]", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} {} DataType: {}\"]\n", ast_node_id, ast_node_id, extended_string, self.string_val).as_str());

        // array size is stored in LHS
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, "ARRAY-Size");
            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot(string_buffer);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_identifier_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Identifier: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Identifier: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        ast_node_id
    }

    // selection_statement -> IF OPENING_BRACKET expression CLOSING_BRACKET statement
    fn pretty_print_ast_if_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} If: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} If: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // expression
        let mut expression_ast_node_id = 0;
        if let Some(expression_node) = self.expression.as_ref() {
            expression_ast_node_id = expression_node.pretty_print_ast_dot(string_buffer);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, expression_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, expression_ast_node_id).as_str());
        }
        // if
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, "if");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }
        // else
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot_ex(string_buffer, "else");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_compound_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Compound: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Compound: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot(string_buffer);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_block_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Block: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Block: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // instructions and declarations
        for i in 0..self.block_items.len() {

            let block_item_ast_node_id = self.block_items[self.block_items.len()-1-i].pretty_print_ast_dot(string_buffer);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }
    
    fn pretty_print_ast_while_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} While: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} While: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // statement_ast_node
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot(string_buffer);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }
        // expression_ast_node
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot(string_buffer);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_do_while_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} DoWhile: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} DoWhile: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // expression_ast_node
        let mut expression_ast_node_id = 0;
        if let Some(expression_node) = self.expression.as_ref() {
            expression_ast_node_id = expression_node.pretty_print_ast_dot(string_buffer);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, expression_ast_node_id).as_str());
        }

        // instructions and declarations
        for i in 0..self.block_items.len() {

            let block_item_ast_node_id = self.block_items[self.block_items.len()-1-i].pretty_print_ast_dot(string_buffer);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_for_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} For: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} For: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // initialization, e.g.: a = 0
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, "INIT");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }
        // expression_ast_node, condition, e.g. a < 10
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.expression.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot_ex(string_buffer, "CONDITION");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }
        // post, e.g.: a = a + 1
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot_ex(string_buffer, "POST");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        // instructions and declarations
        for i in 0..self.block_items.len() {

            let block_item_ast_node_id = self.block_items[self.block_items.len()-1-i].pretty_print_ast_dot(string_buffer);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_conditional_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - true-statement
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, "TRUE-Statement:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        // rhs - false-statement
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot_ex(string_buffer, "FALSE-Statement:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        // expression
        let mut expression_ast_node_id = 0;
        if let Some(expression_node) = self.expression.as_ref() {
            expression_ast_node_id = expression_node.pretty_print_ast_dot_ex(string_buffer, "Expression:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, expression_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, expression_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_function_call_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} FunctionCall: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        ast_node_id
    }

    fn pretty_print_ast_storage_class_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} StorageClass: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        ast_node_id
    }

    fn pretty_print_ast_pointer_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Pointer: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        ast_node_id
    }

    fn pretty_print_ast_switch_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Switch: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        for i in 0..self.block_items.len() {

            let block_item_ast_node_id = self.block_items[self.block_items.len()-1-i].pretty_print_ast_dot(string_buffer);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_case_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Case: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // // LHS - body statement
        // let mut lhs_ast_node_id = 0;
        // if let Some(left_node) = self.lhs.as_ref() {
        //     lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, "Case: Body-Statement:");
        //     // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
        //     string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        // }

        // expression
        let mut expression_ast_node_id = 0;
        if let Some(expression_node) = self.expression.as_ref() {
            expression_ast_node_id = expression_node.pretty_print_ast_dot_ex(string_buffer, "Discriminator-Exp:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, expression_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, expression_ast_node_id).as_str());
        }

        for i in 0..self.block_items.len() {
            let block_item_ast_node_id = self.block_items[i].pretty_print_ast_dot(string_buffer);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_default_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Default: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // // LHS - body statement
        // let mut lhs_ast_node_id = 0;
        // if let Some(left_node) = self.lhs.as_ref() {
        //     lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, "Default: Body-Statement:");
        //     // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
        //     string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        // }

        for i in 0..self.block_items.len() {
            let block_item_ast_node_id = self.block_items[i].pretty_print_ast_dot(string_buffer);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }
    
    fn pretty_print_ast_break_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Break\"]\n", ast_node_id, ast_node_id);
        string_buffer.push_str(format!("{} [label=\"{} Break\"]\n", ast_node_id, ast_node_id).as_str());

        ast_node_id
    }

    fn pretty_print_ast_empty_statement_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} EmptyStatement: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        ast_node_id
    }

    fn pretty_print_ast_single_init_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} SingleInit: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - true-statement
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, "Value");
            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_compound_init_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} CompoundInit: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        for i in 0..self.block_items.len() {
            let block_item_ast_node_id = self.block_items[i].pretty_print_ast_dot(string_buffer);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_subscript_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Subscript: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - pointer 
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, "pointer:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        // rhs - index
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot_ex(string_buffer, "index:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_structure_declaration_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} StructDecl: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        for i in 0..self.block_items.len() {
            let block_item_ast_node_id = self.block_items[i].pretty_print_ast_dot(string_buffer);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_member_declaration_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} MemberDecl: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - type
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, "type:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        // rhs - identifier
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot_ex(string_buffer, "identifier:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_array_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Array: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - type
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, "LHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        // rhs - identifier
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot_ex(string_buffer, "RHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_structure_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Struct: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - type
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, "LHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        // rhs - identifier
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot_ex(string_buffer, "RHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_dot_dot(&self, string_buffer: &mut String) -> usize {

        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Dot: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - type
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, "LHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        // rhs - identifier
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot_ex(string_buffer, "RHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_arrow_dot(&self, string_buffer: &mut String) -> usize {
        
        // create node for this AstNode
        let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Arrow: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - type
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, "LHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        // rhs - identifier
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = right_node.pretty_print_ast_dot_ex(string_buffer, "RHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }
    
}