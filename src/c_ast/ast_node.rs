use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

use std::str::FromStr;

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::common::data_type::DataType;

// Just to be fair, I asked AI on how to design a recursive data structure in rust
// which can be iterated over recursively several times without being consumed
// and AI delivered ...
//
// I would not have been able to do this without the use of AI :(

//
// DOT graphviz - https://dreampuf.github.io/GraphvizOnline
//

#[derive(Clone, Debug, PartialEq)]
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
    PrefixOperator,
    DataType,
    Declaration, // variable declaration or function declaration
    FunctionDeclaration,
    VariableDeclaration,
    StructureDeclaration,
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
    AssignmentOperator,
    Cast,
    Unknown,
}

#[derive(Clone, Debug)]
pub enum AstNodeOperatorType {
    Negate,
    Complement,
    Not,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Assignment,
    AddAssignment,
    SubAssignment,
    MulAssignment,
    DivAssignment,
    ModAssignment,
    FunctionCall,
    Cast,
    Dereference,
    AddrOf,
    Increment,
    PrefixIncrement,
    Decrement,
    PrefixDecrement,
    Dot,
    Arrow,
    SizeOf,
    And,
    Or,
    Xor,
    LeftShift,
    RightShift,
    BinaryAndAssignment,
    BinaryOrAssignment,
    BinaryXorAssignment,
    LeftShiftAssignment,
    RightShiftAssignment,
    LogicalAnd,
    LogicalOr,
    NotApplicable,
}

impl fmt::Display for AstNodeOperatorType {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {

            // binary
            AstNodeOperatorType::Equal => write!(f, "E"),
            AstNodeOperatorType::NotEqual => write!(f, "NE"),
            AstNodeOperatorType::LessThan => write!(f, "L"),
            AstNodeOperatorType::LessThanOrEqual => write!(f, "LE"),
            AstNodeOperatorType::GreaterThan => write!(f, "G"),
            AstNodeOperatorType::GreaterThanOrEqual => write!(f, "GE"),

            // pointers / memory
            AstNodeOperatorType::Dereference => write!(f, "Deref *"),

            _ => todo!(),
        }
    }
}

#[derive(Clone)]
pub struct AstNode {
    pub id: usize,
    pub node_type: AstNodeType,
    pub parent: Option<usize>,
    pub lhs: Option<usize>,
    pub rhs: Option<usize>,
    pub parent_id: Option<usize>, // id of parent AstNode
    pub data_type: Option<usize>,
    pub analyzed_data_type: DataType,
    pub expression: Option<usize>,
    pub operator: Option<usize>,
    pub operator_type: AstNodeOperatorType,
    pub string_val: String,
    pub block_items: Vec<usize>,
    pub parameters: Vec<usize>,
    pub storage_class: Option<usize>,
    pub is_extern: bool,
    pub is_static: bool,
    pub indent: usize,

    pub function_name_ast_node: Option<usize>,
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

            AstNodeType::CompoundInit => {
                println!("CompoundInit");

                // if let Some(left_node) = self.lhs.as_ref() {
                //     print!("{:?}", left_node);
                // }

                for i in 0..self.block_items.len() {
                    let temp_node_id = self.block_items[i];
                    // let temp_node = node_map.get(&temp_node_id).unwrap();
                    print!("{:?}", temp_node_id);
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

            AstNodeType::ConstInt |
            AstNodeType::ConstLong |
            AstNodeType::ConstUInt |
            AstNodeType::ConstULong |
            AstNodeType::ConstDouble => {
                print!("Constant");
                print!(" {:?}", self.string_val);
                println!(" [{:?}]", self.analyzed_data_type);
            }

            AstNodeType::Declaration => {
                println!("Declaration");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("{:?}", left_node);
                }
            }

            AstNodeType::ParameterDeclaration => {
                println!("ParameterDeclaration");
                // name
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("{:?}", left_node);
                }
                // data type
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("{:?}", right_node);
                }
            }

            AstNodeType::VariableDeclaration => {
                println!("VariableDeclaration");
                // println!("{:?}", self.string_val);

                // data type
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("{:?}", left_node);
                }
                // identifier
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("{:?}", right_node);
                }
                // initialization expression
                if let Some(expression_node) = self.expression.as_ref() {
                    print!("{:?}", expression_node);
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
                println!("Identifier: '{}'", self.string_val);
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
                println!("Expression: node_id:{}", self.id);
                if let Some(left_node) = self.lhs.as_ref() {
                    println!("LHS: {:?}", left_node);
                }
                if let Some(right_node) = self.rhs.as_ref() {
                    println!("RHS {:?}", right_node);
                }
            }

            AstNodeType::Unary => {
                println!("Unary: node_id:{} [{}]", self.id, self.analyzed_data_type);
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

            AstNodeType::PrefixOperator => {
                println!("PrefixOperator {:?}", self.operator);
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
                println!("Array Node-Id:{}", self.id);
                if let Some(left_node_id) = self.lhs.as_ref() {
                    print!("LHS: {:?}", left_node_id);
                }
                if let Some(right_node_id) = self.rhs.as_ref() {
                    print!("RHS: {:?}", right_node_id);
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

            AstNodeType::SingleInit => {
                println!("SingleInit");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("LHS: {:?}", left_node);
                }
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("RHS: {:?}", right_node);
                }
            }

            AstNodeType::StructureDeclaration => {
                println!("StructureDeclaration");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("LHS: {:?}", left_node);
                }
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("RHS: {:?}", right_node);
                }
            }

            AstNodeType::MemberDeclaration => {
                println!("MemberDeclaration");
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("LHS: {:?}", left_node);
                }
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("RHS: {:?}", right_node);
                }
            }

            AstNodeType::While | AstNodeType::For => {
                println!("While/For");

                // LHS - initialization, e.g.: a = 0
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("LHS: {:?}", left_node);
                }
                // expression - expression_ast_node, condition, e.g. a < 10
                if let Some(expression_node) = self.expression.as_ref() {
                    print!("{:?}", expression_node);
                }
                // RHS - post, e.g.: a = a + 1
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("RHS: {:?}", right_node);
                }
                // BLOCK_ITEMS - instructions and declarations
                for i in 0..self.block_items.len() {
                    let block_item_ast_node = &self.block_items[self.block_items.len()-1-i];
                    print!("BLOCK_ITEM: {:?}", block_item_ast_node);
                }
            }

            AstNodeType::FunctionCall => {
                println!("FunctionCall: to function: \"{}\"", self.string_val);

                // // LHS - function name
                // if let Some(left_node) = self.lhs.as_ref() {
                //     println!("{:?}", left_node.string_val);
                // }

                // // RHS -
                // if let Some(right_node) = self.rhs.as_ref() {
                //     println!("{:?}", right_node.string_val);
                // }

                // PARAMETERS / ARGUMENTS
                for i in 0..self.parameters.len() {
                    let parameter_ast_node = &self.parameters[self.parameters.len()-1-i];
                    println!("ARGUMENT_{}: {:?}", i, parameter_ast_node);
                }
            }

            AstNodeType::If => {
                println!("If");

                // expression - expression_ast_node, condition/predicate, e.g. a < 10
                if let Some(expression_node) = self.expression.as_ref() {
                    print!("Predicate: {:?}", expression_node);
                }

                // LHS - true-case
                if let Some(left_node) = self.lhs.as_ref() {
                    print!("TRUE-Branch: {:?}", left_node);
                }

                // RHS - false-case
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("FALSE-Branch: {:?}", right_node);
                }
            }

            AstNodeType::Pointer => {
                println!("Pointer");

                // // LHS - true-case
                // if let Some(left_node) = self.lhs.as_ref() {
                //     print!("TRUE-Branch: {:?}", left_node);
                // }

                // RHS - user defined variable name
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("User Choosen Variable Name: {:?}", right_node);
                }

                println!("Replaced Variable Name: {:?}", self.string_val);
            }

            AstNodeType::Cast => {
                println!("Cast");

                // type
                if let Some(left_node_id) = self.lhs {
                    // let left_node = node_map.get(left_node_id).unwrap();
                    print!("{:?}", left_node_id);
                }
                // identifier
                if let Some(right_node) = self.rhs.as_ref() {
                    print!("{:?}", right_node);
                }
            }

            _ => {
                panic!("Unhandled node type: {:?}", self.node_type);
            }
        }

        Ok(())
    }
}

impl AstNode {

    pub fn new(id_param: usize) -> Self {

        let ast_node = AstNode {
            id: id_param,
            node_type: AstNodeType::Unknown,
            parent: None,
            lhs: None,
            rhs: None,
            parent_id: None,
            data_type: None,
            analyzed_data_type: DataType::DataTypeUnknown,
            expression: None,
            operator: None,
            operator_type: AstNodeOperatorType::NotApplicable,
            string_val: String::from(""),
            block_items: Vec::<usize>::new(),
            parameters: Vec::<usize>::new(),
            storage_class: None,
            is_extern: false,
            is_static: false,
            indent: 0,

            function_name_ast_node: None,
        };

        ast_node
    }

    pub fn serialize(&self, node_map: &Box<HashMap<usize, AstNode>>) -> String {
        let mut lhs_string = String::new();
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_string = node_map.get(left_node).unwrap().serialize(node_map);
        }
        let mut rhs_string = String::new();
        if let Some(right_node) = self.rhs.as_ref() {
           rhs_string = node_map.get(right_node).unwrap().serialize(node_map);
        }
        let mut expression_string = String::new();
        if let Some(expression_node) = self.expression.as_ref() {
           expression_string = node_map.get(expression_node).unwrap().serialize(node_map);
        }

        // lhs_string.push_str(" ");
        // lhs_string.push_str(&self.id.to_string());

        match self.operator_type {

            AstNodeOperatorType::AddrOf => {
                lhs_string.push_str(" = &");
            }

            AstNodeOperatorType::Dereference => {
                lhs_string.push_str(" = *");
            }

            _ => {

            }
        }

        lhs_string.push_str(" ");
        lhs_string.push_str(&self.string_val);

        lhs_string.push_str(" ");
        lhs_string.push_str(&rhs_string);

        lhs_string.push_str(" ");
        lhs_string.push_str(&expression_string);

        return lhs_string;
    }

    pub fn set_indent(&mut self, indent_param: usize) {
        self.indent = indent_param;
    }

    pub fn pretty_print_ast_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {
        self.pretty_print_ast_dot_ex(string_buffer, node_map, "")
    }

    pub fn pretty_print_ast_dot_ex(&self,
        string_buffer: &mut String,
        node_map: &Box<HashMap<usize, AstNode>>,
        extended_string: &str)
        -> usize {
        match self.node_type {
            AstNodeType::Program => {
                self.pretty_print_ast_program_dot(string_buffer, node_map)
            }
            AstNodeType::FunctionDeclaration => {
                self.pretty_print_ast_function_declaration_dot(string_buffer, node_map)
            }
            AstNodeType::Return => {
                self.pretty_print_ast_return_dot(string_buffer, node_map)
            }
            AstNodeType::ConstInt
            | AstNodeType::ConstLong
            | AstNodeType::ConstUInt
            | AstNodeType::ConstULong
            | AstNodeType::ConstDouble => {
                self.pretty_print_ast_constant_dot_ex(string_buffer, node_map, extended_string)
            }
            AstNodeType::Expression => {
                self.pretty_print_ast_expression_dot(string_buffer, node_map)
            }
            AstNodeType::Unary => {
                self.pretty_print_ast_unary_dot(string_buffer, node_map)
            }
            AstNodeType::Binary => {
                self.pretty_print_ast_binary_dot(string_buffer, node_map)
            }
            AstNodeType::Operator => {
                self.pretty_print_ast_operator_dot(string_buffer, node_map)
            }
            AstNodeType::PrefixOperator => {
                self.pretty_print_ast_prefix_operator_dot(string_buffer, node_map)
            }
            AstNodeType::BlockItem => {
                self.pretty_print_ast_block_item_dot(string_buffer, node_map, extended_string)
            }
            AstNodeType::Declaration => {
                self.pretty_print_ast_declaration_dot(string_buffer, node_map)
            }
            AstNodeType::VariableDeclaration => {
                self.pretty_print_ast_variable_declaration_dot(string_buffer, node_map, false)
            }
            AstNodeType::ParameterDeclaration => {
                self.pretty_print_ast_variable_declaration_dot(string_buffer, node_map, true)
            }
            AstNodeType::Statement => {
                self.pretty_print_ast_statement_dot(string_buffer, node_map, extended_string)
            }
            AstNodeType::DataType => {
                self.pretty_print_ast_datatype_dot(string_buffer, node_map, extended_string)
            }
            AstNodeType::Identifier => {
                self.pretty_print_ast_identifier_dot(string_buffer, node_map)
            }
            AstNodeType::If => {
                self.pretty_print_ast_if_dot(string_buffer, node_map)
            }
            AstNodeType::Compound => {
                self.pretty_print_ast_compound_dot(string_buffer, node_map)
            }
            AstNodeType::Block => {
                self.pretty_print_ast_block_dot(string_buffer, node_map)
            }
            AstNodeType::While => {
                self.pretty_print_ast_while_dot(string_buffer, node_map)
            }
            AstNodeType::DoWhile => {
                self.pretty_print_ast_do_while_dot(string_buffer, node_map)
            }
            AstNodeType::For => {
                self.pretty_print_ast_for_dot(string_buffer, node_map)
            }
            AstNodeType::Conditional => {
                self.pretty_print_ast_conditional_dot(string_buffer, node_map)
            }
            AstNodeType::FunctionCall => {
                self.pretty_print_ast_function_call_dot(string_buffer, node_map)
            }
            AstNodeType::StorageClassSpecifier => {
                self.pretty_print_ast_storage_class_dot(string_buffer, node_map)
            }
            AstNodeType::Pointer => {
                self.pretty_print_ast_pointer_dot(string_buffer, node_map)
            }
            AstNodeType::Switch => {
                self.pretty_print_ast_switch_dot(string_buffer, node_map)
            }
            AstNodeType::Case => {
                self.pretty_print_ast_case_dot(string_buffer, node_map)
            }
            AstNodeType::Default => {
                self.pretty_print_ast_default_dot(string_buffer, node_map)
            }
            AstNodeType::Break => {
                self.pretty_print_ast_break_dot(string_buffer, node_map)
            }
            AstNodeType::EmptyStatement => {
                self.pretty_print_ast_empty_statement_dot(string_buffer, node_map)
            }
            AstNodeType::SingleInit => {
                self.pretty_print_ast_single_init_dot(string_buffer, node_map)
            }
            AstNodeType::CompoundInit => {
                self.pretty_print_ast_compound_init_dot(string_buffer, node_map)
            }
            AstNodeType::Subscript => {
                self.pretty_print_ast_subscript_dot(string_buffer, node_map)
            }
            AstNodeType::StructureDeclaration => {
                self.pretty_print_ast_structure_declaration_dot(string_buffer, node_map)
            }
            AstNodeType::MemberDeclaration => {
                self.pretty_print_ast_member_declaration_dot(string_buffer, node_map)
            }
            AstNodeType::Array => {
                self.pretty_print_ast_array_dot(string_buffer, node_map)
            }
            AstNodeType::Structure => {
                self.pretty_print_ast_structure_dot(string_buffer, node_map)
            }
            AstNodeType::Dot => {
                self.pretty_print_ast_dot_dot(string_buffer, node_map)
            }
            AstNodeType::Arrow => {
                self.pretty_print_ast_arrow_dot(string_buffer, node_map)
            }
            AstNodeType::Cast => {
                self.pretty_print_ast_cast_dot(string_buffer, node_map)
            }
            _ => {
                panic!("{}", format!("Unhandled AST node_type: {:?}", self.node_type).as_str());
            }
        }
    }

    fn pretty_print_ast_program_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Program: {}\"]", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Program\"]\n", ast_node_id, ast_node_id).as_str());

        for i in 0..self.block_items.len() {

            let id = self.block_items[self.block_items.len()-1-i];
            let block_item_ast_node_id = node_map.get(&id).unwrap().pretty_print_ast_dot(string_buffer, node_map);

            // connect parent and child
            // println!("{} -> {}", block_ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_function_declaration_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode and also output the name into the label
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} FunctionDeclaration: {}\"]", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} FunctionDeclaration: {} [{}]\"]\n", ast_node_id, ast_node_id, self.string_val, self.analyzed_data_type).as_str());

        // return type
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        // create node for the function name
        // let identifier_ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // string_buffer.push_str(format!("{} [label=\"{} Identifier: '{}'\"]\n", identifier_ast_node_id, identifier_ast_node_id, self.string_val).as_str());
        // string_buffer.push_str(format!("{} -> {}\n", ast_node_id, identifier_ast_node_id).as_str());
        if let Some(function_name_ast_node) = self.function_name_ast_node.as_ref() {
            let function_name_ast_node_id = node_map.get(&function_name_ast_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, function_name_ast_node_id).as_str());
        }

        // // create node for the parameters
        // let parameters_ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        // // println!("{} [label=\"{} params\"]", parameters_ast_node_id, parameters_ast_node_id);
        // string_buffer.push_str(format!("{} [label=\"{} params\"]\n", parameters_ast_node_id, parameters_ast_node_id).as_str());
        // // println!("{} -> {}", ast_node_id, parameters_ast_node_id);
        // string_buffer.push_str(format!("{} -> {}\n", ast_node_id, parameters_ast_node_id).as_str());

        // add parameters into parameters block
        for i in 0..self.parameters.len() {
            let parameter_ast_node_id = node_map.get(&self.parameters[self.parameters.len()-1-i]).unwrap().pretty_print_ast_dot(string_buffer, node_map);

            // connect parent and child
            // println!("{} -> {}", parameters_ast_node_id, parameter_ast_node_id);
            //string_buffer.push_str(format!("{} -> {}\n", parameters_ast_node_id, parameter_ast_node_id).as_str());

            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, parameter_ast_node_id).as_str());
        }

        if let Some(block) = self.lhs.as_ref() {

            // create node for the body/block
            // let block_ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
            let block = node_map.get(&block).unwrap();
            let block_ast_node_id = block.id;

            // println!("{} [label=\"{} Body/Block: {}\"]", block_ast_node_id, block_ast_node_id, self.string_val);
            string_buffer.push_str(format!("{} [label=\"{} Body/Block: {}\"]\n", block_ast_node_id, block_ast_node_id, self.string_val).as_str());
            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_ast_node_id).as_str());

            // add instructions and declarations into body/block
            for i in 0..block.block_items.len() {
                let block_item_ast_node_id = node_map.get(&block.block_items[block.block_items.len()-1-i]).unwrap().pretty_print_ast_dot(string_buffer, node_map);
                // connect parent and child
                // println!("{} -> {}", block_ast_node_id, block_item_ast_node_id);
                string_buffer.push_str(format!("{} -> {}\n", block_ast_node_id, block_item_ast_node_id).as_str());
            }
        }

        // storage class
        if let Some(storage_class_node) = self.storage_class.as_ref() {

            let storage_class_node = node_map.get(&storage_class_node).unwrap();
            let storage_class_ast_node_id = storage_class_node.id;

            // println!("{} [label=\"{} StorageClass: {}\"]", storage_class_ast_node_id, storage_class_ast_node_id, self.string_val);
            string_buffer.push_str(format!("{} [label=\"{} StorageClass: {}\"]\n", storage_class_ast_node_id, storage_class_ast_node_id, storage_class_node.string_val).as_str());

            // storage_class_ast_node_id = storage_class_node.pretty_print_ast_dot(string_buffer);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, storage_class_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_return_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // print the child tree
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
        }

        // create node for this AstNode
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Return\"]", ast_node_id, ast_node_id);
        string_buffer.push_str(format!("{} [label=\"{} Return [{}]\"]\n", ast_node_id, ast_node_id, self.analyzed_data_type).as_str());

        // connect parent and child
        if let Some(left_node) = self.lhs.as_ref() {
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_constant_dot_ex(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>, extended_string: &str) -> usize {

        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        match self.node_type {

            AstNodeType::ConstULong => {
                // println!("{} [label=\"{} Constant({})\"]", ast_node_id, ast_node_id, self.string_val);
                string_buffer.push_str(format!("{} [label=\"{} {} ConstULong({}) [{}]\"]\n", ast_node_id, ast_node_id, extended_string, self.string_val, self.analyzed_data_type).as_str());
            }

            AstNodeType::ConstUInt => {
                string_buffer.push_str(format!("{} [label=\"{} {} ConstUInt({}) [{}]\"]\n", ast_node_id, ast_node_id, extended_string, self.string_val, self.analyzed_data_type).as_str());
            }

            AstNodeType::ConstLong => {
                string_buffer.push_str(format!("{} [label=\"{} {} ConstLong({}) [{}]\"]\n", ast_node_id, ast_node_id, extended_string, self.string_val, self.analyzed_data_type).as_str());
            }

            AstNodeType::ConstInt => {
                string_buffer.push_str(format!("{} [label=\"{} {} ConstInt({}) [{}]\"]\n", ast_node_id, ast_node_id, extended_string, self.string_val, self.analyzed_data_type).as_str());
            }

            AstNodeType::ConstDouble => {
                string_buffer.push_str(format!("{} [label=\"{} {} ConstDouble({}) [{}]\"]\n", ast_node_id, ast_node_id, extended_string, self.string_val, self.analyzed_data_type).as_str());
            }

            _ => {
                panic!("Unhandled case!");
            }
        }

        let mut data_type_ast_node_id = 0;
        if let Some(data_type_ast_node) = self.data_type.as_ref() {
            data_type_ast_node_id = node_map.get(&data_type_ast_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, data_type_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_expression_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Exp ({:?})\"]", ast_node_id, ast_node_id, self.operator_type);
        string_buffer.push_str(format!("{} [label=\"{} Exp ({:?}) [{}]\"]\n", ast_node_id, ast_node_id, self.operator_type, self.analyzed_data_type).as_str());

        // print the child tree
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {

            match self.operator_type {

                AstNodeOperatorType::Cast => {
                    // println!("test");
                    lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "Cast-TargetType:");
                }

                _ => {
                    // println!("test2");
                    lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
                }
            }

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        let mut data_type_ast_node_id = 0;
        if let Some(data_type_ast_node) = self.data_type.as_ref() {
            data_type_ast_node_id = node_map.get(&data_type_ast_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, data_type_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_unary_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // print the child tree

        // operator
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
        }

        // operand
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
        }

        // data type
        let mut data_type_ast_node_id = 0;
        if let Some(data_type_node) = self.data_type.as_ref() {
            data_type_ast_node_id = node_map.get(&data_type_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
        }

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Unary\"]", ast_node_id, ast_node_id);
        string_buffer.push_str(format!("{} [label=\"{} Unary [{}]\"]\n", ast_node_id, ast_node_id, self.analyzed_data_type).as_str());

        // connect parent and child

        // operator
        // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());

        // operand
        // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());

        // data type
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, data_type_ast_node_id).as_str());

        ast_node_id
    }

    fn pretty_print_ast_binary_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // print the child tree
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
        }
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
        }

        // print the operator tree
        let mut operator_ast_node_id = 0;
        if let Some(operator_node) = self.operator.as_ref() {
            operator_ast_node_id = node_map.get(&operator_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
        }

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Binary\"]", ast_node_id, ast_node_id);
        string_buffer.push_str(format!("{} [label=\"{} Binary [{}]\"]\n", ast_node_id, ast_node_id, self.analyzed_data_type).as_str());

        // connect parent and child
        // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        // println!("{} -> {}", ast_node_id, operator_ast_node_id);
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, operator_ast_node_id).as_str());

        ast_node_id
    }

    fn pretty_print_ast_operator_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // print the child tree
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
        }

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} {:?}\"]", ast_node_id, ast_node_id, self.operator_type);
        string_buffer.push_str(format!("{} [label=\"{} {:?}\"]\n", ast_node_id, ast_node_id, self.operator_type).as_str());

        match self.operator_type {

            AstNodeOperatorType::SizeOf => {
                let mut expression_ast_node_id = 0;
                if let Some(expression_node) = self.expression.as_ref() {
                    expression_ast_node_id = node_map.get(&expression_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
                    string_buffer.push_str(format!("{} -> {}\n", ast_node_id, expression_ast_node_id).as_str());
                }
            }

            _ => {

            }
        }

        ast_node_id
    }

    fn pretty_print_ast_prefix_operator_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // print the child tree
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
        }

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} {:?}\"]", ast_node_id, ast_node_id, self.operator_type);
        string_buffer.push_str(format!("{} [label=\"{} PREFIX {:?}\"]\n", ast_node_id, ast_node_id, self.operator_type).as_str());

        ast_node_id
    }

    fn pretty_print_ast_block_item_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>, extended_string: &str) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} BlockItem\"]", ast_node_id, ast_node_id);
        string_buffer.push_str(format!("{} [label=\"{} BlockItem\"]\n", ast_node_id, ast_node_id).as_str());

        // print the child tree
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, extended_string);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_declaration_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Declaration\"]", ast_node_id, ast_node_id);
        string_buffer.push_str(format!("{} [label=\"{} Declaration\"]\n", ast_node_id, ast_node_id).as_str());

        // print the child tree
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_variable_declaration_dot(&self,
        string_buffer: &mut String,
        node_map: &Box<HashMap<usize, AstNode>>,
        is_parameter: bool)
        -> usize
    {
        // data type
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
        }
        // name
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
        }
        // expression
        let mut expression_ast_node_id = 0;
        if let Some(expression_node) = self.expression.as_ref() {
            expression_ast_node_id = node_map.get(&expression_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
        }
        // storage class
        let mut storage_class_ast_node_id = 0;
        if let Some(storage_class_node) = self.storage_class.as_ref() {
            storage_class_ast_node_id = node_map.get(&storage_class_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
        }

        // create node for this AstNode
        let ast_node_id = self.id;

        let ast_node = node_map.get(&ast_node_id).unwrap();

        // either print parameter or variable to the dot output
        if is_parameter {
            string_buffer.push_str(format!("{} [label=\"{} ParameterDeclaration [{}] \"]\n", ast_node_id, ast_node_id, self.analyzed_data_type).as_str());
        } else {
            string_buffer.push_str(format!("{} [label=\"{} VariableDeclaration [{}] \"]\n", ast_node_id, ast_node_id, self.analyzed_data_type).as_str());
            // match ast_node.node_type {
            //     AstNodeType::Array => {
            //         string_buffer.push_str(format!("{} [label=\"{} VariableDeclaration [Array-of-{}] \"]\n", ast_node_id, ast_node_id, self.analyzed_data_type).as_str());
            //     }
            //     _ => {
            //         string_buffer.push_str(format!("{} [label=\"{} VariableDeclaration [{}] \"]\n", ast_node_id, ast_node_id, self.analyzed_data_type).as_str());
            //     }
            // }
        }

        // connect parent and child
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());

        if rhs_ast_node_id != 0 {
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

    fn pretty_print_ast_statement_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>, extended_string: &str) -> usize {

        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
        }

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Statement: {} {}\"]", ast_node_id, ast_node_id, extended_string, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Statement: {} {}\"]\n", ast_node_id, ast_node_id, extended_string, self.string_val).as_str());

        // connect parent and child
        // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
        string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());

        ast_node_id
    }

    fn pretty_print_ast_datatype_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>, extended_string: &str) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} DataType: {}\"]", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} {} DataType: {}\"]\n", ast_node_id, ast_node_id, extended_string, self.string_val).as_str());

        // array size is stored in LHS
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "ARRAY-Size");
            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_identifier_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Identifier: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Identifier: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        ast_node_id
    }

    fn pretty_print_ast_if_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // DESCRIPTION
        //
        // selection_statement -> IF OPENING_BRACKET expression CLOSING_BRACKET statement

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} If: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} If: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // expression
        let mut expression_ast_node_id = 0;
        if let Some(expression_node) = self.expression.as_ref() {
            expression_ast_node_id = node_map.get(&expression_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, expression_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, expression_ast_node_id).as_str());
        }
        // if
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "if");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }
        // else
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "else");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_compound_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Compound: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Compound: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_block_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Block: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Block: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // instructions and declarations
        for i in 0..self.block_items.len() {

            let block_item_ast_node_id = node_map.get(&self.block_items[self.block_items.len()-1-i]).unwrap().pretty_print_ast_dot(string_buffer, node_map);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_while_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} While: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} While: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // statement_ast_node
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }
        // expression_ast_node
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_do_while_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} DoWhile: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} DoWhile: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // expression_ast_node
        let mut expression_ast_node_id = 0;
        if let Some(expression_node) = self.expression.as_ref() {
            expression_ast_node_id = node_map.get(&expression_node).unwrap().pretty_print_ast_dot(string_buffer, node_map);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, expression_ast_node_id).as_str());
        }

        // instructions and declarations
        for i in 0..self.block_items.len() {

            let block_item_ast_node_id = node_map.get(&self.block_items[self.block_items.len()-1-i]).unwrap().pretty_print_ast_dot(string_buffer, node_map);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_for_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} For: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} For: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // LHS - initialization, e.g.: a = 0
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "INIT");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }
        // Expression - expression_ast_node, condition, e.g. a < 10
        let mut expression_ast_node_id = 0;
        if let Some(expression_node) = self.expression.as_ref() {
            expression_ast_node_id = node_map.get(&expression_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "CONDITION");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, expression_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, expression_ast_node_id).as_str());
        }
        // RHS - post, e.g.: a = a + 1
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "POST");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        // BLOCK_ITEMS - instructions and declarations
        for i in 0..self.block_items.len() {

            let block_item_ast_node_id = node_map.get(&self.block_items[self.block_items.len()-1-i]).unwrap().pretty_print_ast_dot(string_buffer, node_map);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_conditional_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - true-statement
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "TRUE-Statement:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        // rhs - false-statement
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "FALSE-Statement:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        // expression
        let mut expression_ast_node_id = 0;
        if let Some(expression_node) = self.expression.as_ref() {
            expression_ast_node_id = node_map.get(&expression_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "Expression:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, expression_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, expression_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_function_call_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        string_buffer.push_str(format!("{} [label=\"{} FunctionCall: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // parameters
        let parameter_ast_node_id = 0;
        for i in 0..self.parameters.len() {

            let parameter_ast_node_id = node_map.get(&self.parameters[self.parameters.len()-1-i]).unwrap().pretty_print_ast_dot(string_buffer, node_map);

            // connect parent and child
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, parameter_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_storage_class_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} StorageClass: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        ast_node_id
    }

    fn pretty_print_ast_pointer_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Pointer: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        ast_node_id
    }

    fn pretty_print_ast_switch_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Switch: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        for i in 0..self.block_items.len() {

            let block_item_ast_node_id = node_map.get(&self.block_items[self.block_items.len()-1-i]).unwrap().pretty_print_ast_dot(string_buffer, node_map);

            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_case_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

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
            expression_ast_node_id = node_map.get(&expression_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "Discriminator-Exp:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, expression_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, expression_ast_node_id).as_str());
        }

        for i in 0..self.block_items.len() {
            let block_item_ast_node_id = node_map.get(&self.block_items[i]).unwrap().pretty_print_ast_dot(string_buffer, node_map);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_default_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

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
            let block_item_ast_node_id = node_map.get(&self.block_items[i]).unwrap().pretty_print_ast_dot(string_buffer, node_map);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_break_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Break\"]\n", ast_node_id, ast_node_id);
        string_buffer.push_str(format!("{} [label=\"{} Break\"]\n", ast_node_id, ast_node_id).as_str());

        ast_node_id
    }

    fn pretty_print_ast_empty_statement_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} EmptyStatement: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        ast_node_id
    }

    fn pretty_print_ast_single_init_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} SingleInit: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - true-statement
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "Value");
            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_compound_init_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} CompoundInit: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        for i in 0..self.block_items.len() {
            let block_item_ast_node_id = node_map.get(&self.block_items[i]).unwrap().pretty_print_ast_dot(string_buffer, node_map);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_subscript_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Subscript: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - pointer
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "pointer:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        // rhs - index
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "index:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_structure_declaration_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} StructDecl: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        for i in 0..self.block_items.len() {
            let block_item_ast_node_id = node_map.get(&self.block_items[i]).unwrap().pretty_print_ast_dot(string_buffer, node_map);
            // connect parent and child
            // println!("{} -> {}", ast_node_id, block_item_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, block_item_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_member_declaration_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} MemberDecl: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - type
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "type:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        // rhs - identifier
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "identifier:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_array_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Array: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - type
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "LHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        // rhs - identifier
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "RHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_structure_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Struct: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - type
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "LHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        // rhs - identifier
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "RHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_dot_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Dot: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - type
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "LHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        // rhs - identifier
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "RHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_arrow_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap<usize, AstNode>>) -> usize {

        // create node for this AstNode
        // let ast_node_id = DOT_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Arrow: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - type
        let mut lhs_ast_node_id = 0;
        if let Some(left_node) = self.lhs.as_ref() {
            lhs_ast_node_id = node_map.get(&left_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "LHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        // rhs - identifier
        let mut rhs_ast_node_id = 0;
        if let Some(right_node) = self.rhs.as_ref() {
            rhs_ast_node_id = node_map.get(&right_node).unwrap().pretty_print_ast_dot_ex(string_buffer, node_map, "RHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        ast_node_id
    }

    fn pretty_print_ast_cast_dot(&self, string_buffer: &mut String, node_map: &Box<HashMap::<usize, AstNode>>) -> usize {

        // create node for this AstNode
        let ast_node_id = self.id;

        // println!("{} [label=\"{} Conditional: {}\"]\n", ast_node_id, ast_node_id, self.string_val);
        string_buffer.push_str(format!("{} [label=\"{} Cast: {}\"]\n", ast_node_id, ast_node_id, self.string_val).as_str());

        // lhs - type
        let mut lhs_ast_node_id = 0;
        if let Some(left_node_id) = self.lhs.as_ref() {

            let left_node = node_map.get(left_node_id).unwrap();
            lhs_ast_node_id = left_node.pretty_print_ast_dot_ex(string_buffer, node_map, "LHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, lhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, lhs_ast_node_id).as_str());
        }

        // rhs - identifier
        let mut rhs_ast_node_id = 0;
        if let Some(right_node_id) = self.rhs.as_ref() {

            let right_node = node_map.get(&right_node_id).unwrap();
            rhs_ast_node_id = right_node.pretty_print_ast_dot_ex(string_buffer, node_map, "RHS:");

            // connect parent and child
            // println!("{} -> {}", ast_node_id, rhs_ast_node_id);
            string_buffer.push_str(format!("{} -> {}\n", ast_node_id, rhs_ast_node_id).as_str());
        }

        // draw arrow to parent
        if let Some(parent_id) = self.parent_id {
            string_buffer.push_str(format!("{} -> {}\n", self.id, parent_id).as_str());
        }

        ast_node_id
    }
}