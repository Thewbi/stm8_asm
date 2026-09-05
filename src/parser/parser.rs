use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap};

use std::fs::File;

use std::hash::Hash;
use std::io::BufReader;
use std::io::BufRead;

use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;

use std::str::FromStr;

use crate::c_ast::ast_node_id_counter::AST_NODE_ID_COUNTER;
use crate::common::data_type::DataType;

use crate::parser::rule::Rule;
use crate::parser::rule::RuleElement;

use crate::parser::parser::ParseStackElementType::StateId;

use crate::parser::grammar_state::GrammarState;

use crate::c_ast::ast_node::AstNode;
use crate::c_ast::ast_node::AstNodeType;
use crate::c_ast::ast_node::AstNodeOperatorType;

use std::{
    sync::atomic::{AtomicUsize, Ordering}
};

pub struct Transition<T>(pub usize, pub RuleElement<T>);

static DEBUG_NODE_COUNTER: AtomicUsize = AtomicUsize::new(0);
// pub static AST_NODE_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct DebugNode {
    pub id: usize,
    pub label: String,
}

impl DebugNode {
    pub fn new(id_param: usize, label_param: String) -> Self {
        DebugNode {id: id_param, label: label_param}
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseTableCell<T> {

    // ACTION-Part
    Shift(T),
    Reduce(T),
    Accept,

    // GOTO-Part
    Goto(T),
}

impl<T: Display> fmt::Debug for ParseTableCell<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match &self {

            // ACTION-Part
            ParseTableCell::Shift(target) => {
                write!(f, "Shift {}", target).expect("Write failed!");
            },
            ParseTableCell::Reduce(target) => {
                write!(f, "Reduce {}", target).expect("Write failed!");
            },
            ParseTableCell::Accept=> {
                write!(f, "Accept").expect("Write failed!");
            },

            // GOTO-Part
            ParseTableCell::Goto(target) => {
                write!(f, "Goto {}", target).expect("Write failed!");
            },
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct ParseStackElement<T: std::fmt::Display> {
    pub element_type: ParseStackElementType<T>,
    pub data: String,
}

impl<T: Display + std::fmt::Debug> fmt::Debug for ParseStackElement<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match &self.element_type {

            ParseStackElementType::RuleElement(RuleElement::Terminal(rule_element_data)) => {
                write!(f, "{:?}", rule_element_data).expect("Write failed!");
            }

            ParseStackElementType::RuleElement(RuleElement::NonTerminal(rule_element_data)) => {
                write!(f, "{:?}", rule_element_data).expect("Write failed!");
            }

            StateId(state_id) => {
                write!(f, "{:?}", state_id);
            }

            _ => {
                panic!();
            }

        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum ParseStackElementType<T: std::fmt::Display> {
    RuleElement(RuleElement<T>),
    StateId(usize),
}

pub struct Parser<T> {
    pub parse_table: HashMap::<usize, HashMap::<RuleElement<T>, ParseTableCell<usize>>>,
    pub stack: Vec::<ParseStackElement<String>>,
    pub collapse_nodes: bool,
    pub print_rule_reduction: bool,

    // TYPEDEF HANDLING
    pub typedef_found: bool,
    pub last_type_specifier: String,
    pub last_source_type: String,
    pub is_typedef_active: bool,
    pub lexer_produce_type_name: bool,

    // Type table
    pub defined_types: Vec::<String>,

    // AST
    pub construct_ast: bool,
    pub ast_stack: Vec::<usize>,
    pub child_counter: usize,
    pub parameter_counter: usize,
    pub switch_case_default_counter: usize,
    pub direct_declarator_counter: usize,
    pub struct_field_counter: usize,
}

impl Parser<String> {

    pub fn new(parse_table_param: HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>) -> Self {

        let mut parser = Parser {
            parse_table: parse_table_param,
            stack: Vec::<ParseStackElement<String>>::new(),
            collapse_nodes: false, // if true, nodes that point to a single node only are not output to the dot graph

            // print_rule_reduction: true,
            print_rule_reduction: false, // outputs all rules as they are reduced!

            // TYPEDEF HANDLING
            typedef_found: false,
            last_type_specifier: String::new(),
            last_source_type: String::new(),
            is_typedef_active: false,
            lexer_produce_type_name: false,

            // Types
            defined_types: Vec::<String>::new(),

            // AST
            construct_ast: true,
            //construct_ast: false,
            ast_stack: Vec::<usize>::new(),
            child_counter: 0,
            parameter_counter: 0,
            switch_case_default_counter: 0,
            direct_declarator_counter: 0,
            struct_field_counter: 0,
        };

        let t1 = ParseStackElementType::<String>::StateId(0);
        let e1 = ParseStackElement { element_type: t1, data: String::from("") };
        parser.stack.push(e1);

        parser
    }

    // removes the current node from the stack and replaces it by a new node, inserting a transition line and a line for the new node.
    pub fn node_to_node(&mut self,
        label: &str,
        rule_id: usize,
        string_buffer: &mut String,
        debug_node_stack: &mut Vec::<DebugNode>
    ) -> usize {

        // leave this function empty for a compact Tree which could be an AST.
        // Uncomment the code for a full parse tree that contains all production rules!

        if !self.collapse_nodes {

            // create new node id
            // create new node with node id and label
            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
            let debug_node = DebugNode::new(debug_node_id, String::from(label));
            // print new node into string buffer. e.g.    0 [label="test"]
            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, rule_id, String::from(label)).as_str());

            // take old node from stack
            let old_debug_node = debug_node_stack.pop().unwrap();
            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
            string_buffer.push_str(format!(" {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

            // push new node to stack
            debug_node_stack.push(debug_node);

            return debug_node_id;
        }

        0usize
    }

    // Given some input symbol, the current stack of parse elements looks at the topmost stack element.
    // A stack element can either can either be a rule or a state id.
    //
    // If the topmost stack element is a state_id, retrieves the parse table row for that state from the parse table.
    // Retrieve the entry that the parse table row stores for the current input.
    // The entry can either be GOTO, SHIFT, REDUCE or no entry is available in the parse table row!
    // SHIFT:   the old stack elements remain unchanged but the input is pushed.
    //          Afterwards the new state id is pushed. The SHIFT stack element internally stores that next state id.
    // REDUCE:  the stack entry of type reduce contains the id of the production rule to reduce.
    //          For each element on the RHS of the production rule a pair of { state id and terminal } is removed from the
    //          parse stack and the LHS of the reduced rule is pushed. (no state id is pushed!)
    //
    // If the topmost stack element is a production rule, then the state id stored below that production rule on the stack
    // is retrieved. Starting with this state id, a parse trable row is retrieved from the parse table and the rule,
    // from the top of the stack, is resolved from within the parse table row. This means the new input is not consumed at all!
    //
    // If the parse table row contains no entry for the current input, ???
    //
    // ...
    pub fn consume(&mut self,
        input: RuleElement<String>,
        terminal_value: &String,
        rule_map: &BTreeMap<usize, Rule<String>>,
        string_buffer: &mut String,
        debug_node_stack: &mut Vec::<DebugNode>,
        step: usize,
        node_map: &mut Box<HashMap<usize, AstNode>>
    ) -> bool {

        // let debug = true;
        let debug = false;

        if debug {
            println!("");
            println!("");
            println!("");
            println!(".:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.");
            println!("[Parser::consume] Step: {}", step);
            println!("[Parser::consume] Stack Before: {:?}", self.stack);
            println!(".:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.");
        }

        // if self.lexer_produce_type_name {
        //     println!("test {:?}", input);
        //     input = RuleElement::Terminal(String::from("TYPE_NAME"));
        //     self.lexer_produce_type_name = false;
        // }

        // peek at the topmost element
        let parse_stack_element = self.stack.pop().unwrap();
        self.stack.push(parse_stack_element.clone());

        // match between RuleElement or StateId
        match &parse_stack_element.element_type {

            ParseStackElementType::RuleElement(rule_element) => {
                if debug {
                    println!("[Parser::consume] RuleElement {:?}", rule_element);
                }

                let stack_len = self.stack.len();

                if debug {
                    println!("[Parser::consume] stack_len {:?}", stack_len);
                }

                let stack_content = &self.stack[stack_len - 2];

                match &stack_content.element_type {

                    ParseStackElementType::StateId(current_state_id) => {

                        if debug {
                            println!("[Parser::consume] StateId: {}", current_state_id);
                        }

                        // retrieve parse table row for the newly retrieved state id
                        let parse_table_row = self.parse_table.get(&current_state_id).unwrap();

                        // DEBUG
                        // let contains_key: bool = parse_table_row.contains_key(&rule_element);
                        // println!("State: {}, parse_table_row: {:?}, input: {:?}, contains_key: {:?}", current_state_id, parse_table_row, input, contains_key);

                        let idk = parse_table_row.get(&rule_element).unwrap();
                        match idk {

                            ParseTableCell::Goto(next_state_id) => {
                                if debug {
                                    println!("[Parser::consume] GOTO: {:?}", *next_state_id);
                                }

                                // PUSH new state id
                                // let pse = ParseStackElement::<String>::StateId(*next_state_id);

                                let t3 = ParseStackElementType::<String>::StateId(*next_state_id);
                                let e3 = ParseStackElement { element_type: t3, data: String::from("") };
                                self.stack.push(e3);
                            }

                            _ => {
                                panic!("[Parser::consume] test");
                            }
                        }
                    }

                    ParseStackElementType::RuleElement(rule_element) => {
                        panic!("[Parser::consume] RuleElement");
                    }
                }

                false
            }

            ParseStackElementType::StateId(current_state_id) => {

                if debug {
                    println!("[Parser::consume] StateId: {}", current_state_id);
                }

                // retrieve the parse table row for the current state
                let parse_table_row = self.parse_table.get(&current_state_id).unwrap();

                // if debug {
                //     println!("{:?}", parse_table_row);
                // }

                if debug {
                    // state id
                    println!("[Parser::consume] StateId: {}", current_state_id);

                    // DEBUG - entire state with indetification and normal rules (Spammy!)
                    // let state = grammar_state_hashmap.get(&current_state_id).unwrap();
                    // println!("[Parser::consume] State: {:?}", state);

                    // parse table row
                    println!("[Parser::consume] parse_table_row: {:?}", parse_table_row);

                    // input symbol
                    println!("[Parser::consume] Input: {:?}", input);

                    // check if the parse table is broken or not!
                    let contains_key: bool = parse_table_row.contains_key(&input);
                    println!("[Parser::consume] Contains Key: {:?}", contains_key);
                }

                // // DEBUG
                // println!("*******************************************");
                // for (key, value) in parse_table_row.into_iter() {
                //     println!("{:?} / {:?}", key, value);
                //     println!("{:?}", *key == input);
                // }
                // println!("*******************************************");

                // decide between ACTION (shift / reduce) and GOTO
                // if the parser row has no cell for the input, execute GOTO using the stack
                if !parse_table_row.contains_key(&input) {

                    // peek top element
                    let stack_top_element = self.stack.pop().unwrap();
                    self.stack.push(stack_top_element.clone());

                    if debug {
                        println!("stack_top_element: {:?}", stack_top_element);
                    }

                    match &stack_top_element.element_type {

                        ParseStackElementType::StateId(current_state_id) => {
                            panic!("[Parser::consume] StateId: {}", current_state_id);
                        }

                        ParseStackElementType::RuleElement(rule_element) => {
                            if debug {
                                println!("[Parser::consume] RuleElement");
                            }

                            let command = parse_table_row.get(&rule_element).unwrap();
                            match command {

                                ParseTableCell::Goto(state_id) => {
                                    if debug {
                                        println!("[Parser::consume] GOTO: {:?}", *state_id);
                                    }

                                    // PUSH new state id
                                    // let pse = ParseStackElement::<String>::StateId(*state_id);

                                    let t4 = ParseStackElementType::<String>::StateId(*state_id);
                                    let e4 = ParseStackElement { element_type: t4, data: String::from("") };

                                    self.stack.push(e4);

                                    if debug {
                                        println!(".:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.");
                                        println!("[Parser::consume] Step: {}", step);
                                        println!("[Parser::consume] Stack After: {:?}", self.stack);
                                        println!(".:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.");
                                    }

                                    false
                                }

                                ParseTableCell::Shift(state_id) => {
                                    if debug {
                                        println!("[Parser::consume] SHIFT: {:?}", *state_id);
                                    }

                                    // PUSH new state id
                                    // let pse = ParseStackElement::<String>::StateId(*state_id);

                                    let t5 = ParseStackElementType::<String>::StateId(*state_id);
                                    let e5 = ParseStackElement { element_type: t5, data: String::from("") };
                                    self.stack.push(e5);

                                    if debug {
                                        println!(".:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.");
                                        println!("[Parser::consume] Step: {}", step);
                                        println!("[Parser::consume] Stack After: {:?}", self.stack);
                                        println!(".:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.");
                                    }

                                    false
                                }

                                _ => {
                                    panic!("[Parser::consume] test {:?}", command);
                                }
                            }
                        }
                    }

                } else {

                    // retrieve the entry from the parse table for for the current input
                    let parser_step = parse_table_row.get(&input).expect("Parse Table is broken!");
                    match parser_step {

                        ParseTableCell::Shift(next_state_id) => {
                            if debug {
                                println!("[Parser::consume] shift {}", next_state_id);
                            }

                            // push the input symbol
                            let t1 = ParseStackElementType::<String>::RuleElement(input);
                            let e1 = ParseStackElement { element_type: t1, data: terminal_value.clone() };
                            self.stack.push(e1);

                            // push the next state id
                            let t2 = ParseStackElementType::<String>::StateId(*next_state_id);
                            let e2 = ParseStackElement { element_type: t2, data: String::from("") };
                            self.stack.push(e2);

                            if debug {
                                println!(".:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.");
                                println!("[Parser::consume] Step: {}", step);
                                println!("[Parser::consume] Stack After: {:?}", self.stack);
                                println!(".:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.");
                            }

                            true
                        }

                        ParseTableCell::Reduce(rule_id) => {
                            let found = rule_map.contains_key(&rule_id);

                            // retrieve rule by id
                            let found_rule = rule_map.get(&rule_id).unwrap();

                            if !found {
                                panic!("[Parser::consume] Rule not found!");
                            } else {
                                if debug {
                                    println!("[Parser::consume] rule: {:?}", found_rule);
                                }

                                //if debug {
                                if self.print_rule_reduction {
                                    print!("[Parser::consume()] REDUCING RULE: ");
                                    found_rule.print_rule_simple();
                                    println!("");
                                }
                                //}

                                //
                                // pop elements from the stack and transfer them into another array for inspection later
                                //

                                // WARNING: The elements are popped in reverse!
                                let mut rule_reverse = Vec::new();
                                for rhs in &found_rule.rhs {

                                    self.stack.pop(); // state id
                                    let temp = self.stack.pop(); // terminal / nonterminal

                                    rule_reverse.push(temp);
                                }

                                //
                                // further take actions based on rule reduced
                                //

                                let callback_rules = true;
                                // let callback_rules = false;
                                if callback_rules {
                                    if debug {
                                        for terminal_rev in rule_reverse.iter().rev() {
                                            print!(", TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                        }
                                        println!("");
                                    }

                                    match found_rule.original_id {

                                        // primary_expression -> IDENTIFIER
                                        1 => {

                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;

                                                // println!("TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                            }

                                            // create new node - primary_expression
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("primary_expression"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("primary_expression")).as_str());

                                            // IDENTIFIER
                                            let identifier_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let identifier_node = DebugNode::new(identifier_node_id, value.clone());
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", identifier_node_id, identifier_node_id, value.clone()).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, identifier_node.id).as_str());

                                            // push new node
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - primary_expression -> IDENTIFIER
                                            //

                                            if self.construct_ast {

                                                let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
                                                let mut ast_node: AstNode = AstNode::new(ast_node_id);
                                                ast_node.node_type = AstNodeType::Identifier;
                                                ast_node.string_val = value;

                                                self.ast_stack.push(ast_node_id);

                                                node_map.insert(ast_node_id, ast_node);
                                            }
                                        }

                                        // primary_expression -> HEX_NUMERIC
                                        2 => {
                                            // put new node on the stack
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                //println!("TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                            }

                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("primary_expression"));
                                            debug_node_stack.push(debug_node);

                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("primary_expression")).as_str());

                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, value.clone());
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", type_node_id, value.clone()).as_str());

                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            //
                                            // AST - primary_expression -> HEX_NUMERIC
                                            //

                                            if self.construct_ast {

                                                // println!("{:?}", debug_node_id);

                                                // page 252 - every expression needs a type
                                                let data_type_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
                                                let mut data_type_ast_node: AstNode = AstNode::new(data_type_ast_node_id);
                                                data_type_ast_node.node_type = AstNodeType::DataType;
                                                data_type_ast_node.string_val = String::from("undefined_type:primary_expression");

                                                // println!("{:?}", value);

                                                let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
                                                let mut ast_node: AstNode = AstNode::new(ast_node_id);

                                                // https://stackoverflow.com/questions/32381414/converting-a-hexadecimal-string-to-a-decimal-integer
                                                let value_without_prefix = value.trim_start_matches("0x");
                                                let res = match u64::from_str_radix(&value_without_prefix, 16) {
                                                    Ok(result) => {
                                                        // println!("Ok");
                                                        // println!("{:?} {:?}", result, (2u64.pow(32)-1));

                                                        if result > (2u64.pow(32)-1) {
                                                            ast_node.node_type = AstNodeType::ConstULong;
                                                            data_type_ast_node.string_val = String::from("ULong");
                                                            data_type_ast_node.analyzed_data_type = DataType::DataTypeUnsignedLong;
                                                        } else {
                                                            ast_node.node_type = AstNodeType::ConstUInt;
                                                            data_type_ast_node.string_val = String::from("UInt");
                                                            data_type_ast_node.analyzed_data_type = DataType::DataTypeUnsignedInt;
                                                        }
                                                    }
                                                    Err(e) => {
                                                        println!("Error: {:?}", e);
                                                    }
                                                };

                                                ast_node.string_val = value;
                                                ast_node.data_type = Some(data_type_ast_node.id);

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(data_type_ast_node.id, data_type_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // primary_expression -> NUMERIC
                                        3 => {
                                            // retrieve value
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                // println!("TerminalValue: '{}'", value);
                                            }

                                            // create new node - primary_expression
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("primary_expression"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("primary_expression")).as_str());

                                            let literal_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let literal_node = DebugNode::new(literal_node_id, value.clone());
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", literal_node_id, value.clone()).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, literal_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            // println!("Rule: {:?}", found_rule.original_id);
                                            // println!("DebugNodeId: {:?}", debug_node_id);

                                            //
                                            // AST - primary_expression -> NUMERIC
                                            //

                                            if self.construct_ast {

                                                // page 252 - every expression needs a type
                                                let data_type_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
                                                let mut data_type_ast_node: AstNode = AstNode::new(data_type_ast_node_id);
                                                data_type_ast_node.node_type = AstNodeType::DataType;
                                                data_type_ast_node.string_val = String::from("undefined_type:primary_expression");

                                                let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
                                                let mut ast_node: AstNode = AstNode::new(ast_node_id);

                                                // println!("{:?}", value);
                                                let res = match value.parse::<u64>() {
                                                    Ok(result) => {
                                                        // println!("Ok");
                                                        // println!("{:?} {:?}", result, (2u64.pow(32)-1));

                                                        if result > (2u64.pow(32)-1) {
                                                            ast_node.node_type = AstNodeType::ConstULong;
                                                            data_type_ast_node.string_val = String::from("ULong");
                                                            data_type_ast_node.analyzed_data_type = DataType::from_str("ulong").unwrap();
                                                        } else {
                                                            ast_node.node_type = AstNodeType::ConstUInt;
                                                            data_type_ast_node.string_val = String::from("UInt");
                                                            data_type_ast_node.analyzed_data_type = DataType::from_str("uint").unwrap();
                                                        }

                                                        // ast_node.node_type = AstNodeType::ConstInt;
                                                    }
                                                    Err(e) => {
                                                        println!("Error");
                                                    }
                                                };

                                                ast_node.string_val = value;
                                                ast_node.analyzed_data_type = data_type_ast_node.analyzed_data_type.clone();
                                                ast_node.data_type = Some(data_type_ast_node.id);

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(data_type_ast_node.id, data_type_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // primary_expression -> FLOAT_NUMERIC
                                        4 => {
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;

                                                // println!("TerminalValue: '{}'", value);
                                            }

                                            // do not pop from the stack as this is a leave for a int numeric literal value

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("primary_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, "primary_expression").as_str());

                                            // push new node to stack

                                            let literal_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let literal_node = DebugNode::new(literal_node_id, value.clone());
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", literal_node_id, value.clone()).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, literal_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            // println!("Rule: {:?}", found_rule.original_id);
                                            // println!("DebugNodeId: {:?}", debug_node_id);

                                            //
                                            // AST - primary_expression -> NUMERIC
                                            //

                                            if self.construct_ast {

                                                // println!("{:?}", value);

                                                // page 252 - every expression needs a type
                                                let mut data_type_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                data_type_ast_node.node_type = AstNodeType::DataType;
                                                data_type_ast_node.string_val = String::from("undefined_type:primary_expression");
                                                data_type_ast_node.string_val = String::from("Double");

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::ConstDouble;
                                                ast_node.string_val = value;
                                                ast_node.data_type = Some(data_type_ast_node.id);

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(data_type_ast_node.id, data_type_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // primary_expression -> STRING_LITERAL
                                        5 => {
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                // println!("TerminalValue: '{}'", value);
                                            }

                                            // do not pop from the stack as this is a leave for a int numeric literal value

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("primary_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, value).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // primary_expression -> OPENING_BRACKET expression CLOSING_BRACKET
                                        6 => {
                                            self.node_to_node("primary_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // postfix_expression -> primary_expression
                                        7 => {
                                            self.node_to_node("postfix_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // postfix_expression -> postfix_expression OPENING_ANGULAR_BRACKET expression CLOSING_ANGULAR_BRACKET
                                        8 => {

                                            //
                                            // LHS
                                            //

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("postfix_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("postfix_expression")).as_str());

                                            //
                                            // RHS
                                            //

                                            // take old node from stack - postfix_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // [
                                            // create new node id
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from("["));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from("[")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());

                                            // take old node from stack - expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // ]
                                            // create new node id
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from("]"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from("]")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - postfix_expression -> postfix_expression OPENING_ANGULAR_BRACKET expression CLOSING_ANGULAR_BRACKET
                                            //

                                            if self.construct_ast {

                                                let index_numeric_ast_node = self.ast_stack.pop().unwrap();

                                                let pointer_ast_node = self.ast_stack.pop().unwrap();

                                                let mut subscript_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                subscript_ast_node.node_type = AstNodeType::Subscript;
                                                subscript_ast_node.lhs = Some(pointer_ast_node);
                                                subscript_ast_node.rhs = Some(index_numeric_ast_node);

                                                self.ast_stack.push(subscript_ast_node.id);

                                                node_map.insert(subscript_ast_node.id, subscript_ast_node);
                                            }
                                        }

                                        // postfix_expression -> postfix_expression OPENING_BRACKET CLOSING_BRACKET
                                        9 => {
                                            self.node_to_node("postfix_expression", found_rule.original_id, string_buffer, debug_node_stack);

                                            //
                                            // AST - primary_expression -> IDENTIFIER
                                            //

                                            if self.construct_ast {

                                                // see also rule 10

                                                let primary_ast_node = self.ast_stack.pop().unwrap();

                                                let mut function_call_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                function_call_ast_node.node_type = AstNodeType::FunctionCall;
                                                function_call_ast_node.string_val = node_map.get(&primary_ast_node).unwrap().string_val.clone();

                                                let mut exp_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                exp_ast_node.node_type = AstNodeType::Expression;
                                                exp_ast_node.operator_type = AstNodeOperatorType::FunctionCall;
                                                exp_ast_node.lhs = Some(function_call_ast_node.id);

                                                self.ast_stack.push(exp_ast_node.id);

                                                node_map.insert(function_call_ast_node.id, function_call_ast_node);
                                                node_map.insert(exp_ast_node.id, exp_ast_node);
                                            }
                                        }

                                        // postfix_expression -> postfix_expression OPENING_BRACKET argument_expression_list CLOSING_BRACKET
                                        10 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("postfix_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("postfix_expression")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - postfix_expression -> postfix_expression OPENING_BRACKET argument_expression_list CLOSING_BRACKET
                                            //

                                            if self.construct_ast {

                                                // see also rule 9

                                                let mut function_call_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                function_call_ast_node.node_type = AstNodeType::FunctionCall;

                                                //
                                                // Parameter List
                                                //

                                                while self.parameter_counter > 0 {

                                                    // one parameter is processed
                                                    self.parameter_counter = self.parameter_counter - 1;

                                                    let parameter_ast_node = self.ast_stack.pop().unwrap();
                                                    function_call_ast_node.parameters.push(parameter_ast_node);
                                                }

                                                // identifier
                                                let identifier_ast_node = self.ast_stack.pop().unwrap();
                                                function_call_ast_node.string_val = node_map.get(&identifier_ast_node).unwrap().string_val.clone();

                                                let mut exp_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                exp_ast_node.node_type = AstNodeType::Expression;
                                                exp_ast_node.operator_type = AstNodeOperatorType::FunctionCall;
                                                exp_ast_node.lhs = Some(function_call_ast_node.id);

                                                self.ast_stack.push(exp_ast_node.id);

                                                node_map.insert(function_call_ast_node.id, function_call_ast_node);
                                                node_map.insert(exp_ast_node.id, exp_ast_node);
                                            }
                                        }

                                        // postfix_expression -> postfix_expression DOT IDENTIFIER
                                        11 => {

                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                // println!("TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                            }

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("postfix_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("postfix_expression")).as_str());

                                            // take old node from stack - postfix_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // .
                                            // create new node id
                                            // create new node with node id and label
                                            let inc_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let inc_node = DebugNode::new(inc_node_id, String::from("."));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", inc_node_id, inc_node_id, String::from(".")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, inc_node.id).as_str());

                                            // IDENTIFIER
                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, value.clone());
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", type_node_id, type_node_id, value.clone()).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - postfix_expression DOT IDENTIFIER
                                            //

                                            if self.construct_ast {

                                                // example: person
                                                let struct_identifier_ast_node = self.ast_stack.pop().unwrap();

                                                // // page 496 - Dot(exp structure, identifier member)
                                                // let mut dot_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                // dot_ast_node.node_type = AstNodeType::Dot;
                                                // // data_type_ast_node.string_val = String::from("undefined_type:unary_expression");

                                                // // operator
                                                // let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                // operator_ast_node.node_type = AstNodeType::Operator;
                                                // operator_ast_node.operator_type = AstNodeOperatorType::Increment;

                                                // field - e.g. age of struct person
                                                let member_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
                                                let mut member_ast_node: AstNode = AstNode::new(member_ast_node_id);
                                                member_ast_node.node_type = AstNodeType::Identifier;
                                                member_ast_node.operator_type = AstNodeOperatorType::Dot;
                                                member_ast_node.string_val = value.clone();

                                                // let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                // ast_node.node_type = AstNodeType::Unary;
                                                // ast_node.operator = Some(operator_ast_node.id);
                                                // ast_node.operator_type = AstNodeOperatorType::Increment;
                                                // ast_node.lhs = Some(expression_ast_node.id);

                                                let ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
                                                let mut ast_node: AstNode = AstNode::new(ast_node_id);
                                                ast_node.node_type = AstNodeType::Dot;
                                                ast_node.operator_type = AstNodeOperatorType::Dot;
                                                ast_node.lhs = Some(struct_identifier_ast_node); // structure
                                                ast_node.rhs = Some(member_ast_node.id); // structure-field / member
                                                // ast_node.data_type = Some(Box::new(data_type_ast_node));

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(member_ast_node.id, member_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // postfix_expression -> postfix_expression PTR_OP IDENTIFIER
                                        //
                                        // e.g. head_p->data
                                        12 => {

                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;

                                                // println!("TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                            }

                                            // println!("value: {}", value);

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("postfix_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", debug_node_id, debug_node_id, String::from("postfix_expression")).as_str());

                                            // postfix_expression - take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // PTR_OP
                                            // create new node id
                                            // create new node with node id and label
                                            let ptr_op_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let ptr_op_node = DebugNode::new(ptr_op_node_id, String::from("PTR_OP"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", ptr_op_node_id, String::from("PTR_OP")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, ptr_op_node.id).as_str());

                                            // // IDENTIFIER - take old node from stack
                                            // let identifier_node = debug_node_stack.pop().unwrap();
                                            // // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            // string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, identifier_node.id).as_str());

                                            // IDENTIFIER
                                            let identifier_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let identifier_node = DebugNode::new(identifier_node_id, value.clone());
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", identifier_node_id, identifier_node_id, value.clone()).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, identifier_node.id).as_str());

                                            // // push new node
                                            // debug_node_stack.push(identifier_node);

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - postfix_expression DOT IDENTIFIER
                                            //

                                            if self.construct_ast {

                                                // example: person
                                                let struct_identifier_ast_node = self.ast_stack.pop().unwrap();

                                                // // page 496 - Dot(exp structure, identifier member)
                                                // let mut dot_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                // dot_ast_node.node_type = AstNodeType::Dot;
                                                // // data_type_ast_node.string_val = String::from("undefined_type:unary_expression");

                                                // // operator
                                                // let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                // operator_ast_node.node_type = AstNodeType::Operator;
                                                // operator_ast_node.operator_type = AstNodeOperatorType::Increment;

                                                // field - e.g. age of struct person
                                                let mut member_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                member_ast_node.node_type = AstNodeType::Identifier;
                                                member_ast_node.operator_type = AstNodeOperatorType::Arrow;
                                                member_ast_node.string_val = value.clone();

                                                // let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                // ast_node.node_type = AstNodeType::Unary;
                                                // ast_node.operator = Some(operator_ast_node.id);
                                                // ast_node.operator_type = AstNodeOperatorType::Increment;
                                                // ast_node.lhs = Some(expression_ast_node.id);

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Arrow;
                                                ast_node.operator_type = AstNodeOperatorType::Arrow;
                                                ast_node.lhs = Some(struct_identifier_ast_node); // structure
                                                ast_node.rhs = Some(member_ast_node.id); // structure-field / member
                                                // ast_node.data_type = Some(Box::new(data_type_ast_node));

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(member_ast_node.id, member_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // postfix_expression -> postfix_expression INC_OP
                                        13 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("postfix_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("postfix_expression")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();

                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // ++
                                            // create new node id
                                            // create new node with node id and label
                                            let inc_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let inc_node = DebugNode::new(inc_node_id, String::from("++"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", inc_node_id, inc_node_id, String::from("++")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, inc_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - postfix_expression -> postfix_expression INC_OP
                                            //

                                            if self.construct_ast {

                                                let expression_ast_node = self.ast_stack.pop().unwrap();

                                                // page 252 - every expression needs a type
                                                let mut data_type_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                data_type_ast_node.node_type = AstNodeType::DataType;
                                                data_type_ast_node.string_val = String::from("undefined_type:unary_expression");

                                                // operator
                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::Increment;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Unary;
                                                ast_node.operator_type = AstNodeOperatorType::Increment;
                                                ast_node.lhs = Some(operator_ast_node.id); // operator
                                                ast_node.rhs = Some(expression_ast_node); // operand
                                                ast_node.data_type = Some(data_type_ast_node.id);

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(data_type_ast_node.id, data_type_ast_node);
                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // postfix_expression -> postfix_expression DEC_OP
                                        14 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("postfix_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("postfix_expression")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // --
                                            // create new node id
                                            // create new node with node id and label
                                            let inc_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let inc_node = DebugNode::new(inc_node_id, String::from("--"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", inc_node_id, String::from("--")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, inc_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - postfix_expression -> postfix_expression INC_OP
                                            //

                                            if self.construct_ast {

                                                let expression_ast_node = self.ast_stack.pop().unwrap();

                                                // page 252 - every expression needs a type
                                                let mut data_type_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                data_type_ast_node.node_type = AstNodeType::DataType;
                                                data_type_ast_node.string_val = String::from("undefined_type:postfix_expression");

                                                // operator
                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::Decrement;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Unary;
                                                ast_node.operator_type = AstNodeOperatorType::Decrement;
                                                ast_node.lhs = Some(operator_ast_node.id); // operator
                                                ast_node.rhs = Some(expression_ast_node); // operand
                                                ast_node.data_type = Some(data_type_ast_node.id);

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(data_type_ast_node.id, data_type_ast_node);
                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // argument_expression_list -> assignment_expression
                                        15 => {
                                            self.node_to_node("argument_expression_list", found_rule.original_id, string_buffer, debug_node_stack);

                                            //
                                            // AST - argument_expression_list -> assignment_expression
                                            //

                                            if self.construct_ast {
                                                self.parameter_counter = self.parameter_counter + 1;
                                            }
                                        }

                                        // argument_expression_list -> argument_expression_list COMMA assignment_expression
                                        16 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("argument_expression_list"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("argument_expression_list")).as_str());

                                            // ,
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from(","));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from(",")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - argument_expression_list -> argument_expression_list COMMA assignment_expression
                                            //

                                            if self.construct_ast {
                                                self.parameter_counter = self.parameter_counter + 1;
                                            }
                                        }

                                        // unary_expression -> postfix_expression
                                        17 => {
                                            self.node_to_node("unary_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // unary_expression -> INC_OP unary_expression
                                        18 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("unary_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("unary_expression")).as_str());

                                            // INC_OP
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("INC_OP"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", equals_node_id, String::from("INC_OP")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // take old node from stack - unary_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - unary_expression -> INC_OP unary_expression
                                            //

                                            if self.construct_ast {

                                                let expression_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::PrefixOperator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::PrefixIncrement;
                                                operator_ast_node.string_val = String::from("++");

                                                // page 252 - every expression needs a type
                                                let mut data_type_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                data_type_ast_node.node_type = AstNodeType::DataType;
                                                data_type_ast_node.string_val = String::from("undefined_type:unary_expression");

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Unary;
                                                ast_node.lhs = Some(operator_ast_node.id); // operator
                                                ast_node.rhs = Some(expression_ast_node); // operand
                                                ast_node.data_type = Some(data_type_ast_node.id);

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(data_type_ast_node.id, data_type_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // unary_expression -> DEC_OP unary_expression
                                        19 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("unary_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("unary_expression")).as_str());

                                            // INC_OP
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("DEC_OP"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", equals_node_id, String::from("DEC_OP")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // take old node from stack - unary_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - unary_expression -> INC_OP unary_expression
                                            //

                                            if self.construct_ast {

                                                let expression_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::PrefixOperator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::PrefixDecrement;
                                                operator_ast_node.string_val = String::from("--");

                                                // page 252 - every expression needs a type
                                                let mut data_type_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                data_type_ast_node.node_type = AstNodeType::DataType;
                                                data_type_ast_node.string_val = String::from("undefined_type:unary_expression");

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Unary;
                                                ast_node.lhs = Some(operator_ast_node.id); // operator
                                                ast_node.rhs = Some(expression_ast_node); // operand
                                                ast_node.data_type = Some(data_type_ast_node.id);

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(data_type_ast_node.id, data_type_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // unary_expression -> unary_operator cast_expression
                                        20 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("unary_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("unary_expression")).as_str());

                                            // RHS (value that the operator is applied to) - take old node from stack
                                            let rhs_debug_node = debug_node_stack.pop().unwrap();
                                            // println!("{:?}", rhs_debug_node);
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, rhs_debug_node.id).as_str());

                                            // unary operator - take old node from stack
                                            let unary_operator_debug_node = debug_node_stack.pop().unwrap();
                                            // println!("{:?}", unary_operator_debug_node);
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, unary_operator_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - unary_expression -> unary_operator cast_expression
                                            //

                                            if self.construct_ast {

                                                let expression_ast_node = self.ast_stack.pop().unwrap();
                                                let data_type = node_map.get(&expression_ast_node).unwrap().analyzed_data_type.clone();
                                                // println!("{:?}", expression_ast_node);
                                                let operator_ast_node = self.ast_stack.pop().unwrap();
                                                // println!("{:?}", operator_ast_node);

                                                // page 252 - every expression needs a type
                                                let mut data_type_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                data_type_ast_node.node_type = AstNodeType::DataType;
                                                data_type_ast_node.string_val = String::from("undefined_type:unary_expression");

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Unary;
                                                ast_node.operator_type = node_map.get(&operator_ast_node).unwrap().operator_type.clone();
                                                ast_node.lhs = Some(operator_ast_node); // operator
                                                ast_node.rhs = Some(expression_ast_node); // operand
                                                ast_node.data_type = Some(data_type_ast_node.id);

                                                // Nora Sandler, page 254, as a type take the type of the inner node
                                                // ast_node.analyzed_data_type = DataType::from_str("ulong").unwrap();
                                                ast_node.analyzed_data_type = data_type;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(data_type_ast_node.id, data_type_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // unary_expression -> SIZEOF unary_expression
                                        21 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("unary_operator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("unary_operator")).as_str());

                                            // SIZEOF
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("SIZEOF"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", equals_node_id, String::from("SIZEOF")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // take old node from stack - type_name
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            // println!("{:?}", debug_node_id);

                                            //
                                            // AST - unary_expression -> SIZEOF unary_expression
                                            //

                                            if self.construct_ast {

                                                let expression_ast_node = self.ast_stack.pop().unwrap();

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Operator;
                                                ast_node.operator_type = AstNodeOperatorType::SizeOf;
                                                ast_node.expression = Some(expression_ast_node);

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // unary_expression -> SIZEOF OPENING_BRACKET type_name CLOSING_BRACKET
                                        22 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("unary_operator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("unary_operator")).as_str());

                                            // SIZEOF
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("SIZEOF"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", equals_node_id, String::from("SIZEOF")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // OPENING_BRACKET
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("OPENING_BRACKET"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from("OPENING_BRACKET")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack - type_name
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // CLOSING_BRACKET
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("CLOSING_BRACKET"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from("CLOSING_BRACKET")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - unary_expression -> SIZEOF OPENING_BRACKET type_name CLOSING_BRACKET
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Operator;
                                                ast_node.operator_type = AstNodeOperatorType::SizeOf;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // unary_operator -> AMPERSAND
                                        23 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("unary_operator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("unary_operator")).as_str());

                                            // AMPERSAND
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("&"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", equals_node_id, String::from("&")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - unary_operator -> AMPERSAND
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Operator;
                                                ast_node.operator_type = AstNodeOperatorType::AddrOf;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // unary_operator -> ASTERISK
                                        24 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, String::from("unary_operator"), found_rule.original_id));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} Rule:{}\"]\n", debug_node_id, String::from("unary_operator"), found_rule.original_id).as_str());

                                            // ASTERISK
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("*"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", equals_node_id, String::from("*")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - unary_operator -> ASTERISK
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Operator;
                                                ast_node.operator_type = AstNodeOperatorType::Dereference;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // unary_operator -> MINUS
                                        26 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, String::from("unary_operator"), found_rule.original_id));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("unary_operator")).as_str());

                                            // MINUS
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("-"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", equals_node_id, String::from("-")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - unary_operator -> MINUS
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Operator;
                                                ast_node.operator_type = AstNodeOperatorType::Negate;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // unary_operator -> TILDE
                                        27 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, String::from("unary_operator"), found_rule.original_id));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("unary_operator")).as_str());

                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("~"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", equals_node_id, String::from("~")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - unary_operator -> TILDE
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Operator;
                                                ast_node.operator_type = AstNodeOperatorType::Complement;
                                                // Nora Sandler, page 254, Not operator ends up being a int datatype
                                                // all other operators use the data type of the inner node
                                                // ast_node.analyzed_data_type = DataType::from_str("int").unwrap();

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // unary_operator -> EXCLAMATION_MARK
                                        28 => {

                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, String::from("unary_operator"), found_rule.original_id));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("unary_operator")).as_str());

                                            // break
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("!"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", equals_node_id, String::from("!")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - unary_operator -> TILDE
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Operator;
                                                ast_node.operator_type = AstNodeOperatorType::Not;
                                                // Nora Sandler, page 254, Not operator ends up being a int datatype
                                                // all other operators use the data type of the inner node
                                                ast_node.analyzed_data_type = DataType::from_str("int").unwrap();

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // cast_expression -> OPENING_BRACKET type_name CLOSING_BRACKET cast_expression
                                        29 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("cast_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            // string_buffer.push_str(format!("{:?} [label=\"{} Rule:{} {}\"]\n", debug_node_id, String::from("cast_expression")).as_str());
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("cast_expression")).as_str());

                                            // OPENING_BRACKET
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("("));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", equals_node_id, String::from("(")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // type_name - take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // CLOSING_BRACKET
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from(")"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", equals_node_id, String::from(")")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // cast_expression - take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - cast_expression -> OPENING_BRACKET type_name CLOSING_BRACKET cast_expression
                                            //

                                            if self.construct_ast {

                                                let expression_ast_node = self.ast_stack.pop().unwrap();
                                                let type_ast_node_id = self.ast_stack.pop().unwrap();
                                                let type_ast_node = node_map.get(&type_ast_node_id).unwrap();

                                                // page 252 - every expression needs a type
                                                let mut data_type_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                data_type_ast_node.node_type = AstNodeType::DataType;
                                                data_type_ast_node.string_val = String::from("undefined_type:primary_expression");

                                                let mut cast_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                cast_ast_node.node_type = AstNodeType::Expression;
                                                cast_ast_node.operator_type = AstNodeOperatorType::Cast;
                                                cast_ast_node.analyzed_data_type = DataType::from_str(&type_ast_node.string_val).unwrap();
                                                cast_ast_node.lhs = Some(type_ast_node_id);
                                                cast_ast_node.rhs = Some(expression_ast_node);
                                                cast_ast_node.data_type = Some(data_type_ast_node.id);

                                                self.ast_stack.push(cast_ast_node.id);

                                                node_map.insert(data_type_ast_node.id, data_type_ast_node);
                                                node_map.insert(cast_ast_node.id, cast_ast_node);
                                            }
                                        }

                                        // cast_expression -> unary_expression
                                        30 => {
                                            self.node_to_node("cast_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // multiplicative_expression -> cast_expression ASTERISK multiplicative_expression
                                        31 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("additive_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("additive_expression")).as_str());

                                            // take old node from stack - multiplicative_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // MUL
                                            // create new node id
                                            // create new node with node id and label
                                            let mul_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let mus_node = DebugNode::new(mul_node_id, String::from("MUL"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", mul_node_id, String::from("MUL")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, mus_node.id).as_str());

                                            // take old node from stack - cast_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - multiplicative_expression -> cast_expression ASTERISK multiplicative_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::Multiplication;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::Multiplication;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // multiplicative_expression -> cast_expression SLASH multiplicative_expression
                                        32 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("multiplicative_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("multiplicative_expression")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // /
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("/"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from("/")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - multiplicative_expression -> cast_expression SLASH multiplicative_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::Division;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::Division;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // multiplicative_expression -> cast_expression PERCENT multiplicative_expression
                                        33 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("multiplicative_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("multiplicative_expression")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // %
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("%"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from("%")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - multiplicative_expression -> cast_expression SLASH multiplicative_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::Remainder;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::Remainder;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // multiplicative_expression -> cast_expression
                                        34 => {
                                            self.node_to_node("multiplicative_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // additive_expression -> multiplicative_expression PLUS additive_expression
                                        35 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("additive_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("additive_expression")).as_str());

                                            // take old node from stack - multiplicative_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // PLUS
                                            // create new node id
                                            // create new node with node id and label
                                            let plus_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let plus_node = DebugNode::new(plus_node_id, String::from("PLUS"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", plus_node_id, String::from("PLUS")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, plus_node.id).as_str());

                                            // take old node from stack - additive_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - additive_expression -> multiplicative_expression PLUS additive_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::Addition;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::Addition;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // additive_expression -> multiplicative_expression MINUS additive_expression
                                        36 => {

                                            // additive_expression
                                            //
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("additive_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("additive_expression")).as_str());

                                            // take old node from stack - multiplicative_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // MINUS
                                            // create new node id
                                            // create new node with node id and label
                                            let minus_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let minus_node = DebugNode::new(minus_node_id, String::from("MINUS"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", minus_node_id, String::from("MINUS")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, minus_node.id).as_str());

                                            // take old node from stack - additive_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - additive_expression -> multiplicative_expression MINUS additive_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::Subtraction;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::Subtraction;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // additive_expression -> multiplicative_expression
                                        37 => {
                                            self.node_to_node("additive_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // shift_expression -> additive_expression LEFT_OP shift_expression
                                        38 => {
                                            // shift_expression
                                            //
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("shift_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("shift_expression")).as_str());

                                            // take old node from stack - additive_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // LEFT_OP
                                            // create new node id
                                            // create new node with node id and label
                                            let minus_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let minus_node = DebugNode::new(minus_node_id, String::from("LEFT_SHIFT <<"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", minus_node_id, String::from("LEFT_SHIFT <<")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, minus_node.id).as_str());

                                            // take old node from stack - shift_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - shift_expression -> additive_expression LEFT_OP shift_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::LeftShift;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::LeftShift;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // shift_expression -> additive_expression RIGHT_OP shift_expression
                                        39 => {
                                            // shift_expression
                                            //
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("shift_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("shift_expression")).as_str());

                                            // take old node from stack - additive_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // RIGHT_OP
                                            // create new node id
                                            // create new node with node id and label
                                            let minus_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let minus_node = DebugNode::new(minus_node_id, String::from("RIGHT_SHIFT >>"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", minus_node_id, String::from("RIGHT_SHIFT >>")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, minus_node.id).as_str());

                                            // take old node from stack - shift_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - shift_expression -> additive_expression RIGHT_OP shift_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::RightShift;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::RightShift;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // shift_expression -> additive_expression
                                        40 => {
                                            self.node_to_node("shift_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // relational_expression -> shift_expression LT relational_expression
                                        41 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("relational_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("relational_expressions")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // <
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("<"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from("<")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - relational_expression -> shift_expression LT relational_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::LessThan;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::LessThan;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // relational_expression -> shift_expression GT relational_expression
                                        42 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("relational_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("relational_expressions")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // <
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from(">"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from(">")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - relational_expression -> shift_expression GT relational_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::GreaterThan;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::GreaterThan;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // relational_expression -> shift_expression LE_OP relational_expression
                                        43 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("relational_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("relational_expressions")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // <=
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("<="));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from("<=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - relational_expression -> shift_expression LE_OP relational_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::LessThanOrEqual;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::LessThanOrEqual;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // relational_expression -> shift_expression GE_OP relational_expression
                                        44 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("relational_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("relational_expressions")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // <=
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from(">="));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from(">=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - relational_expression -> shift_expression LE_OP relational_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::GreaterThanOrEqual;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::GreaterThanOrEqual;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // relational_expression -> shift_expression
                                        45 => {
                                            self.node_to_node("relational_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // equality_expression -> relational_expression EQ_OP equality_expression
                                        46 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("equality_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("equality_expression")).as_str());

                                            // relational_expression - take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // ==
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("=="));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", lessthan_node_id, lessthan_node_id, String::from("==")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // equality_expression - take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - relational_expression -> shift_expression EQ_OP relational_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::Equal;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::Equal;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // equality_expression -> relational_expression NE_OP equality_expression
                                        47 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("equality_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("equality_expression")).as_str());

                                            // relational_expression - take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // !=
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("!="));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", lessthan_node_id, lessthan_node_id, String::from("!=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // equality_expression - take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - relational_expression -> shift_expression NE_OP relational_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::NotEqual;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::NotEqual;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // equality_expression -> relational_expression
                                        48 => {
                                            self.node_to_node("equality_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // and_expression -> equality_expression AMPERSAND and_expression
                                        49 => {
                                            // and_expression
                                            //
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("and_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("and_expression")).as_str());

                                            // take old node from stack - equality_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // AMPERSAND
                                            // create new node id
                                            // create new node with node id and label
                                            let minus_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let minus_node = DebugNode::new(minus_node_id, String::from("AND &"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", minus_node_id, String::from("AND &")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, minus_node.id).as_str());

                                            // take old node from stack - and_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - and_expression -> equality_expression AMPERSAND and_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::And;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::And;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // and_expression -> equality_expression
                                        50 => {
                                            self.node_to_node("and_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // exclusive_or_expression -> and_expression CIRCUMFLEX exclusive_or_expression
                                        51 => {
                                            // exclusive_or_expression
                                            //
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("inclusive_or_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("inclusive_or_expression")).as_str());

                                            // take old node from stack - and_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // CIRCUMFLEX
                                            // create new node id
                                            // create new node with node id and label
                                            let minus_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let minus_node = DebugNode::new(minus_node_id, String::from("XOR ^"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", minus_node_id, String::from("XOR ^")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, minus_node.id).as_str());

                                            // take old node from stack - exclusive_or_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - and_expression -> equality_expression AMPERSAND and_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::Xor;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::Xor;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // exclusive_or_expression -> and_expression
                                        52 => {
                                            self.node_to_node("exclusive_or_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // inclusive_or_expression -> exclusive_or_expression OR inclusive_or_expression
                                        53 => {
                                            // inclusive_or_expression
                                            //
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("inclusive_or_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("inclusive_or_expression")).as_str());

                                            // take old node from stack - exclusive_or_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // AMPERSAND
                                            // create new node id
                                            // create new node with node id and label
                                            let minus_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let minus_node = DebugNode::new(minus_node_id, String::from("OR |"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", minus_node_id, String::from("OR |")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, minus_node.id).as_str());

                                            // take old node from stack - inclusive_or_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - and_expression -> equality_expression AMPERSAND and_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::Or;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::Or;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // inclusive_or_expression -> exclusive_or_expression
                                        54 => {
                                            self.node_to_node("inclusive_or_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // logical_and_expression -> inclusive_or_expression AND_OP logical_and_expression
                                        55 => {
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("logical_and_expression"));
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("logical_and_expression")).as_str());

                                            // take old node from stack - exclusive_or_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // logical_and_expression - 2x AMPERSAND
                                            // create new node id
                                            // create new node with node id and label
                                            let logical_and_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let logical_and_node = DebugNode::new(logical_and_node_id, String::from("AND &&"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", logical_and_node_id, String::from("AND &&")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, logical_and_node.id).as_str());

                                            // take old node from stack - inclusive_or_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, logical_and_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - and_expression -> equality_expression AMPERSAND and_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::LogicalAnd;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::LogicalAnd;
                                                ast_node.analyzed_data_type = DataType::from_str("int").unwrap();

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // logical_and_expression -> inclusive_or_expression
                                        56 => {
                                            self.node_to_node("logical_and_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // logical_or_expression -> logical_and_expression OR_OP logical_or_expression
                                        57 => {
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("logical_or_expression"));
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("logical_or_expression")).as_str());

                                            // take old node from stack - exclusive_or_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // logical_or_expression - 2x |
                                            // create new node id
                                            // create new node with node id and label
                                            let logical_or_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let logical_or_node = DebugNode::new(logical_or_node_id, String::from("LOGICAL_OR ||"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", logical_or_node_id, String::from("LOGICAL_OR ||")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, logical_or_node.id).as_str());

                                            // take old node from stack - inclusive_or_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, logical_or_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - and_expression -> equality_expression AMPERSAND and_expression
                                            //

                                            if self.construct_ast {

                                                let rhs_ast_node = self.ast_stack.pop().unwrap();
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                let mut operator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                operator_ast_node.node_type = AstNodeType::Operator;
                                                operator_ast_node.operator_type = AstNodeOperatorType::LogicalOr;

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Binary;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator = Some(operator_ast_node.id);
                                                ast_node.operator_type = AstNodeOperatorType::LogicalOr;
                                                ast_node.analyzed_data_type = DataType::from_str("int").unwrap();

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(operator_ast_node.id, operator_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // logical_or_expression -> logical_and_expression
                                        58 => {
                                            self.node_to_node("logical_or_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // conditional_expression -> logical_or_expression QUESTION_MARK expression COLON conditional_expression
                                        59 => {

                                            // new node
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("conditional_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("conditional_expression")).as_str());

                                            // logical_or_expression - take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // ?
                                            // create new node id
                                            // create new node with node id and label
                                            let operator_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let operator_node = DebugNode::new(operator_node_id, String::from("?"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", operator_node_id, operator_node_id, String::from("?")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, operator_node.id).as_str());

                                            // true_expression - take old node from stack
                                            let true_expression_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, true_expression_node.id).as_str());

                                            // :
                                            // create new node id
                                            // create new node with node id and label
                                            let colon_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let colon_node = DebugNode::new(colon_node_id, String::from(":"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", colon_node_id, colon_node_id, String::from(":")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, colon_node.id).as_str());

                                            // false_expression - take old node from stack
                                            let false_expression = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, false_expression.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - conditional_expression -> logical_or_expression QUESTION_MARK expression COLON conditional_expression
                                            //

                                            if self.construct_ast {

                                                let first_ast_node = self.ast_stack.pop().unwrap(); // false-statement
                                                let second_ast_node = self.ast_stack.pop().unwrap(); // true-statement
                                                let third_ast_node = self.ast_stack.pop().unwrap(); // expression-statement

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Conditional;
                                                ast_node.lhs = Some(second_ast_node); // true-statement
                                                ast_node.rhs = Some(first_ast_node); // false-statement
                                                ast_node.expression = Some(third_ast_node); // expression-statement

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // conditional_expression -> logical_or_expression
                                        60 => {
                                            self.node_to_node("conditional_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // assignment_expression -> unary_expression assignment_operator assignment_expression
                                        61 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("assignment_expression"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("assignment_expression")).as_str());

                                            // take old node from stack - unary_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack - assignment_operator
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack - assignment_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - assignment_expression -> unary_expression assignment_operator assignment_expression
                                            //

                                            if self.construct_ast {

                                                // source value element (variable or expression or literal)
                                                let rhs_ast_node = self.ast_stack.pop().unwrap();

                                                // operator (= equals token for assignment operator)
                                                let operator_ast_node_id = self.ast_stack.pop().unwrap();
                                                let operator_ast_node = node_map.get(&operator_ast_node_id).unwrap();

                                                // dest value element
                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                // create new expression AST Node
                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Expression;
                                                ast_node.lhs = Some(lhs_ast_node);
                                                ast_node.rhs = Some(rhs_ast_node);
                                                ast_node.operator_type = operator_ast_node.operator_type.clone();

                                                // DEBUG
                                                if debug {
                                                    println!("{:?}", ast_node);
                                                }

                                                assert!(ast_node.lhs.is_some());
                                                assert!(ast_node.rhs.is_some());

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // assignment_expression -> conditional_expression
                                        62 => {
                                            self.node_to_node("assignment_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // assignment_operator -> EQUALS_SIGN
                                        63 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("assignment_operator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("assignment_operator")).as_str());

                                            // =
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("="));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", equals_node_id, equals_node_id, String::from("=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - assignment_operator -> EQUALS_SIGN
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::AssignmentOperator;
                                                ast_node.operator_type = AstNodeOperatorType::Assignment;
                                                ast_node.string_val = String::from("=");

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // assignment_operator -> MUL_ASSIGN
                                        64 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("assignment_operator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("assignment_operator")).as_str());

                                            // =
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("*="));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", equals_node_id, equals_node_id, String::from("*=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - assignment_operator -> MUL_ASSIGN
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::AssignmentOperator;
                                                ast_node.operator_type = AstNodeOperatorType::MulAssignment;
                                                ast_node.string_val = String::from("*=");

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // assignment_operator -> DIV_ASSIGN
                                        65 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("assignment_operator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("assignment_operator")).as_str());

                                            // =
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("/="));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", equals_node_id, equals_node_id, String::from("/=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - assignment_operator -> DIV_ASSIGN
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::AssignmentOperator;
                                                ast_node.operator_type = AstNodeOperatorType::DivAssignment;
                                                ast_node.string_val = String::from("/=");

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // assignment_operator -> MOD_ASSIGN
                                        66 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("assignment_operator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("assignment_operator")).as_str());

                                            // =
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("%="));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", equals_node_id, equals_node_id, String::from("%=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - assignment_operator -> MOD_ASSIGN
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::AssignmentOperator;
                                                ast_node.operator_type = AstNodeOperatorType::ModAssignment;
                                                ast_node.string_val = String::from("%=");

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // assignment_operator -> ADD_ASSIGN
                                        67 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("assignment_operator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("assignment_operator")).as_str());

                                            // +=
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("+="));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", equals_node_id, equals_node_id, String::from("+=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - assignment_operator -> ADD_ASSIGN
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::AssignmentOperator;
                                                ast_node.operator_type = AstNodeOperatorType::AddAssignment;
                                                ast_node.string_val = String::from("+=");

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // assignment_operator -> SUB_ASSIGN
                                        68 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("assignment_operator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("assignment_operator")).as_str());

                                            // +=
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("-="));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", equals_node_id, equals_node_id, String::from("-=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - assignment_operator -> SUB_ASSIGN
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::AssignmentOperator;
                                                ast_node.operator_type = AstNodeOperatorType::SubAssignment;
                                                ast_node.string_val = String::from("-=");

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // assignment_operator -> LEFT_ASSIGN
                                        69 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("assignment_operator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("assignment_operator")).as_str());

                                            // +=
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("<<="));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", equals_node_id, equals_node_id, String::from("<<=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - assignment_operator -> LEFT_ASSIGN
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::AssignmentOperator;
                                                ast_node.operator_type = AstNodeOperatorType::LeftShiftAssignment;
                                                ast_node.string_val = String::from("<<=");

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // assignment_operator -> RIGHT_ASSIGN
                                        70 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("assignment_operator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("assignment_operator")).as_str());

                                            // +=
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from(">>="));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", equals_node_id, equals_node_id, String::from(">>=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - assignment_operator -> RIGHT_ASSIGN
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::AssignmentOperator;
                                                ast_node.operator_type = AstNodeOperatorType::RightShiftAssignment;
                                                ast_node.string_val = String::from(">>=");

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // assignment_operator -> AND_ASSIGN
                                        71 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("assignment_operator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("assignment_operator")).as_str());

                                            // +=
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("&="));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", equals_node_id, equals_node_id, String::from("&=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - assignment_operator -> AND_ASSIGN
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::AssignmentOperator;
                                                ast_node.operator_type = AstNodeOperatorType::BinaryAndAssignment;
                                                ast_node.string_val = String::from("&=");

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // assignment_operator -> XOR_ASSIGN
                                        72 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("assignment_operator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("assignment_operator")).as_str());

                                            // +=
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("^="));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", equals_node_id, equals_node_id, String::from("^=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - assignment_operator -> XOR_ASSIGN
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::AssignmentOperator;
                                                ast_node.operator_type = AstNodeOperatorType::BinaryXorAssignment;
                                                ast_node.string_val = String::from("|=");

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // assignment_operator -> OR_ASSIGN
                                        73 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("assignment_operator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("assignment_operator")).as_str());

                                            // +=
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("|="));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", equals_node_id, equals_node_id, String::from("|=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - assignment_operator -> OR_ASSIGN
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::AssignmentOperator;
                                                ast_node.operator_type = AstNodeOperatorType::BinaryOrAssignment;
                                                ast_node.string_val = String::from("|=");

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // expression -> assignment_expression
                                        74 => {
                                            self.node_to_node("expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // constant_expression -> conditional_expression
                                        76 => {
                                            self.node_to_node("constant_expression", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // declaration -> declaration_specifiers init_declarator_list SEMICOLON
                                        77 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("declaration"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("declaration")).as_str());

                                            // take old node from stack - declaration_specifiers
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack - init_declarator_list
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // ;
                                            // create new node id
                                            // create new node with node id and label
                                            let semicolon_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let semicolon_node = DebugNode::new(semicolon_node_id, String::from(";"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", semicolon_node_id, String::from(";")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, semicolon_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            // println!("debug_node_id: {}", debug_node_id);

                                            //
                                            // AST - [77] - declaration -> declaration_specifiers init_declarator_list SEMICOLON
                                            //

                                            if self.construct_ast {

                                                //
                                                // Can be a variable or function.
                                                //
                                                // example: int a = 1;
                                                // example: int foo(int a);
                                                //

                                                // Format:
                                                // 1st pop: declarator (function declaration)
                                                // 2nd pop: data type of function return or variable
                                                // 3rd pop: initializer SingleInit or CompoundInit

                                                let declarator_ast_node_id = self.ast_stack.pop().unwrap(); // name/identifier and optional initialization value
                                                let declarator_ast_node = node_map.get(&declarator_ast_node_id).unwrap();
                                                // println!("{:?}", declarator_ast_node);

                                                let data_type_ast_node_id = self.ast_stack.pop().unwrap(); // (return value) data type
                                                let data_type_ast_node = node_map.get(&data_type_ast_node_id).unwrap();
                                                // println!("{:?}", data_type_ast_node);

                                                let mut object_declaration_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                match declarator_ast_node.node_type {

                                                    AstNodeType::FunctionDeclaration => {

                                                        if let Some(func_name) = declarator_ast_node.function_name_ast_node.as_ref() {
                                                            object_declaration_ast_node.function_name_ast_node = declarator_ast_node.function_name_ast_node;
                                                        }

                                                        //object_declaration_ast_node.parameters = std::mem::take(&mut declarator_ast_node.parameters);
                                                        object_declaration_ast_node.parameters = declarator_ast_node.parameters.clone();

                                                        //
                                                        // TYPE (function return type)
                                                        //
                                                        object_declaration_ast_node.analyzed_data_type = data_type_ast_node.analyzed_data_type.clone();

                                                        // println!("{:?}", data_type_ast_node.parameters);
                                                        object_declaration_ast_node.node_type = AstNodeType::FunctionDeclaration;
                                                        object_declaration_ast_node.rhs = Some(data_type_ast_node.id);

                                                        // peek for storage class specifier
                                                        if self.ast_stack.len() > 0 {
                                                            let storage_class_specifier_ast_node_id = self.ast_stack.pop().unwrap();
                                                            let storage_class_specifier_ast_node = node_map.get(&storage_class_specifier_ast_node_id).unwrap();
                                                            if storage_class_specifier_ast_node.node_type == AstNodeType::StorageClassSpecifier {
                                                                // println!("storage_class: {:?}", storage_class_specifier_ast_node.string_val);
                                                                if storage_class_specifier_ast_node.string_val == "STATIC" {
                                                                    object_declaration_ast_node.is_static = true;
                                                                    object_declaration_ast_node.storage_class = Some(storage_class_specifier_ast_node.id);
                                                                } else if storage_class_specifier_ast_node.string_val == "EXTERN" {
                                                                    object_declaration_ast_node.is_extern = true;
                                                                    object_declaration_ast_node.storage_class = Some(storage_class_specifier_ast_node.id);
                                                                }
                                                            } else {
                                                                self.ast_stack.push(storage_class_specifier_ast_node.id);
                                                            }
                                                        }

                                                        self.ast_stack.push(object_declaration_ast_node.id);
                                                    }

                                                    _ => {

                                                        object_declaration_ast_node.node_type = AstNodeType::VariableDeclaration;
                                                        // println!("{:?}", object_declaration_ast_node.lhs);

                                                        //
                                                        // TYPE
                                                        //
                                                        object_declaration_ast_node.analyzed_data_type = data_type_ast_node.analyzed_data_type.clone();

                                                        object_declaration_ast_node.lhs = Some(data_type_ast_node.id);
                                                        object_declaration_ast_node.rhs = Some(declarator_ast_node.id);

                                                        // peek for initializer expression
                                                        if self.ast_stack.len() > 0 {

                                                            let initializer_expression_ast_node_id = self.ast_stack.pop().unwrap();
                                                            let initializer_expression_ast_node = node_map.get(&initializer_expression_ast_node_id).unwrap();

                                                            match initializer_expression_ast_node.node_type {
                                                                AstNodeType::SingleInit | AstNodeType::CompoundInit => {
                                                                    object_declaration_ast_node.expression = Some(initializer_expression_ast_node_id);
                                                                }
                                                                _ => {
                                                                    // put it back after pop to simulate peek
                                                                    self.ast_stack.push(initializer_expression_ast_node_id);
                                                                }
                                                            }
                                                        }

                                                        // peek for storage class specifier
                                                        if self.ast_stack.len() > 0 {
                                                            let storage_class_specifier_ast_node_id = self.ast_stack.pop().unwrap();
                                                            let storage_class_specifier_ast_node = node_map.get(&storage_class_specifier_ast_node_id).unwrap();
                                                            if storage_class_specifier_ast_node.node_type == AstNodeType::StorageClassSpecifier {
                                                                // println!("storage_class: {:?}", storage_class_specifier_ast_node.string_val);
                                                                if storage_class_specifier_ast_node.string_val == "STATIC" {
                                                                    object_declaration_ast_node.is_static = true;
                                                                    object_declaration_ast_node.storage_class = Some(storage_class_specifier_ast_node_id);
                                                                } else if storage_class_specifier_ast_node.string_val == "EXTERN" {
                                                                    object_declaration_ast_node.is_extern = true;
                                                                    object_declaration_ast_node.storage_class = Some(storage_class_specifier_ast_node_id);
                                                                }
                                                            } else {
                                                                // push node back to simulate peek
                                                                self.ast_stack.push(storage_class_specifier_ast_node_id);
                                                            }
                                                        }

                                                        let mut declaration_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                        declaration_ast_node.node_type = AstNodeType::Declaration;
                                                        declaration_ast_node.lhs = Some(object_declaration_ast_node.id);

                                                        // wrap declaration into block_item
                                                        let mut block_item_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                        block_item_ast_node.node_type = AstNodeType::BlockItem;
                                                        block_item_ast_node.lhs = Some(declaration_ast_node.id);

                                                        self.ast_stack.push(block_item_ast_node.id);

                                                        node_map.insert(declaration_ast_node.id, declaration_ast_node);
                                                        node_map.insert(block_item_ast_node.id, block_item_ast_node);
                                                    }
                                                }

                                                node_map.insert(object_declaration_ast_node.id, object_declaration_ast_node);
                                            }
                                        }

                                        // declaration -> declaration_specifiers SEMICOLON
                                        78 => {
                                            self.node_to_node("declaration", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // declaration_specifiers -> storage_class_specifier declaration_specifiers
                                        79 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("declaration_specifiers"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("declaration_specifiers")).as_str());

                                            // take old node from stack - storage_class_specifier
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack - declaration_specifiers
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // TYPEDEF HANDLING
                                            //

                                            if self.typedef_found {

                                                // insert a new type mapping into the types table
                                                self.defined_types.push(self.last_type_specifier.clone());

                                                // reset
                                                self.typedef_found = false;
                                                self.last_type_specifier = String::new();
                                                self.last_source_type = String::new();
                                            }
                                        }

                                        // declaration_specifiers -> type_specifier declaration_specifiers
                                        81 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("declaration_specifiers"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("declaration_specifiers")).as_str());

                                            // type_specifier
                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // declaration_specifiers
                                            // take old node from stack
                                            let declarator_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, declarator_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            // println!("{:?}", debug_node_id);

                                            //
                                            // AST - declaration_specifiers -> type_specifier declaration_specifiers
                                            //

                                            if self.construct_ast {

                                                // TODO: check if the node in the hash_map is really updated or not!

                                                let ast_node_id = self.ast_stack.pop().unwrap();
                                                let mut ast_node = node_map.get(&ast_node_id).unwrap().clone();

                                                // add UInt, ULong
                                                let temp_string = ast_node.string_val.clone();
                                                let mut new_type_string = String::from("U");
                                                new_type_string.push_str(&temp_string);
                                                ast_node.string_val = new_type_string;

                                                self.ast_stack.push(ast_node_id);

                                                // node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // declaration_specifiers -> type_specifier
                                        82 => {
                                            self.node_to_node("declaration_specifiers", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // init_declarator_list -> init_declarator COMMA init_declarator_list
                                        85 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("init_declarator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("init_declarator")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // ,
                                            // create new node id
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from(","));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from(",")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // init_declarator_list -> init_declarator
                                        86 => {
                                            self.node_to_node("init_declarator_list", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // init_declarator -> declarator EQUALS_SIGN initializer
                                        87 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("init_declarator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("init_declarator")).as_str());

                                            // take old node from stack - declarator
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // = EQUALS_SIGN
                                            // create new node id
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from("("));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from("=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());

                                            // take old node from stack - initializer
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - init_declarator -> declarator EQUALS_SIGN initializer
                                            //

                                            if self.construct_ast {

                                                if self.direct_declarator_counter > 1 {

                                                    let mut compound_init_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    compound_init_ast_node.node_type = AstNodeType::CompoundInit;

                                                    for i in 0..self.direct_declarator_counter {

                                                        let initializer_ast_node = self.ast_stack.pop().unwrap();

                                                        let mut expr_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                        expr_ast_node.node_type = AstNodeType::Expression;
                                                        expr_ast_node.lhs = Some(initializer_ast_node);
                                                        // println!("{:?}", expr_ast_node);

                                                        compound_init_ast_node.block_items.push(expr_ast_node.id);

                                                        self.direct_declarator_counter = self.direct_declarator_counter - 1;

                                                        node_map.insert(expr_ast_node.id, expr_ast_node);
                                                    }

                                                    let identifier_ast_node = self.ast_stack.pop().unwrap();
                                                    // println!("{:?}", identifier_ast_node);

                                                    let datatype_ast_node = self.ast_stack.pop().unwrap();
                                                    // println!("{:?}", datatype_ast_node);

                                                    // Format:
                                                    // 1st pop: identifier
                                                    // 2nd pop: data type of function return or variable
                                                    // 3rd pop: initializer expression

                                                    self.ast_stack.push(compound_init_ast_node.id); // 3rd pop: initializer expression

                                                    // let lhs_ast_node = self.ast_stack.pop().unwrap(); // name, identifier
                                                    self.ast_stack.push(datatype_ast_node); // 2nd pop: data type of function return or variable

                                                    // let rhs_ast_node = self.ast_stack.pop().unwrap(); // value expression
                                                    self.ast_stack.push(identifier_ast_node); // 1st pop: identifier

                                                    node_map.insert(compound_init_ast_node.id, compound_init_ast_node);

                                                } else if self.direct_declarator_counter == 1 {

                                                    let initializer_ast_node = self.ast_stack.pop().unwrap();

                                                    let mut expr_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    expr_ast_node.node_type = AstNodeType::Expression;
                                                    expr_ast_node.lhs = Some(initializer_ast_node);
                                                    // println!("{:?}", expr_ast_node);

                                                    let mut single_init_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    single_init_ast_node.node_type = AstNodeType::SingleInit;
                                                    single_init_ast_node.lhs = Some(expr_ast_node.id);

                                                    self.direct_declarator_counter = self.direct_declarator_counter - 1;

                                                    let identifier_ast_node = self.ast_stack.pop().unwrap();
                                                    // println!("{:?}", identifier_ast_node);

                                                    let datatype_ast_node = self.ast_stack.pop().unwrap();
                                                    // println!("{:?}", datatype_ast_node);

                                                    // Format:
                                                    // 1st pop: identifier
                                                    // 2nd pop: data type of function return or variable
                                                    // 3rd pop: initializer expression

                                                    self.ast_stack.push(single_init_ast_node.id); // 3rd pop: initializer expression

                                                    // let lhs_ast_node = self.ast_stack.pop().unwrap(); // name, identifier
                                                    self.ast_stack.push(datatype_ast_node); // 2nd pop: data type of function return or variable

                                                    // let rhs_ast_node = self.ast_stack.pop().unwrap(); // value expression
                                                    self.ast_stack.push(identifier_ast_node); // 1st pop: identifier

                                                    node_map.insert(expr_ast_node.id, expr_ast_node);
                                                    node_map.insert(single_init_ast_node.id, single_init_ast_node);
                                                }
                                            }
                                        }

                                        // init_declarator -> declarator
                                        88 => {
                                            self.node_to_node("init_declarator", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // storage_class_specifier -> TYPEDEF
                                        89 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("storage_class_specifier"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("storage_class_specifier")).as_str());

                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("TYPEDEF"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", type_node_id, type_node_id, String::from("TYPEDEF")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            debug_node_stack.push(debug_node);

                                            //
                                            // TYPEDEF HANDLING
                                            //

                                            self.typedef_found = true;
                                            self.is_typedef_active = true;
                                        }

                                        // storage_class_specifier -> EXTERN
                                        90 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("storage_class_specifier"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("storage_class_specifier")).as_str());

                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("EXTERN"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", type_node_id, type_node_id, String::from("EXTERN")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - storage_class_specifier -> EXTERN
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::StorageClassSpecifier;
                                                ast_node.string_val = String::from("EXTERN");

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // storage_class_specifier -> STATIC
                                        91 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("storage_class_specifier"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("storage_class_specifier")).as_str());

                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("STATIC"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", type_node_id, type_node_id, String::from("STATIC")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - storage_class_specifier -> STATIC
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::StorageClassSpecifier;
                                                ast_node.string_val = String::from("STATIC");

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // type_specifier -> VOID
                                        94 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("type_specifier"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("type_specifier")).as_str());

                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("VOID"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", type_node_id, String::from("VOID")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - type_specifier -> VOID
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::DataType;
                                                ast_node.string_val = String::from("void");
                                                ast_node.analyzed_data_type = DataType::from_str("void").unwrap();

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // type_specifier -> CHAR
                                        95 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("type_specifier"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("type_specifier")).as_str());

                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("CHAR"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", type_node_id, String::from("CHAR")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - type_specifier -> CHAR
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::DataType;
                                                ast_node.string_val = String::from("char");
                                                ast_node.analyzed_data_type = DataType::from_str("char").unwrap();

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // type_specifier -> SHORT
                                        96 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("type_specifier"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("type_specifier")).as_str());

                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("SHORT"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}| {}\"]\n", type_node_id, type_node_id, String::from("SHORT")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            debug_node_stack.push(debug_node);

                                            //
                                            // TYPEDEF HANDLING
                                            //

                                            if self.is_typedef_active {
                                                self.lexer_produce_type_name = true;
                                                self.is_typedef_active = false;
                                            }

                                            self.last_source_type = String::from("short");

                                            //
                                            // AST - type_specifier -> SHORT
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::DataType;
                                                ast_node.string_val = String::from("short");
                                                ast_node.analyzed_data_type = DataType::from_str("short").unwrap();

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // type_specifier -> INT
                                        97 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("type_specifier"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("type_specifier")).as_str());

                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("INT"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}| {}\"]\n", type_node_id, type_node_id, String::from("INT")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            debug_node_stack.push(debug_node);

                                            //
                                            // TYPEDEF HANDLING
                                            //

                                            if self.is_typedef_active {
                                                self.lexer_produce_type_name = true;
                                                self.is_typedef_active = false;
                                            }

                                            self.last_source_type = String::from("int");

                                            //
                                            // AST - type_specifier -> INT
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::DataType;
                                                ast_node.string_val = String::from("int");
                                                ast_node.analyzed_data_type = DataType::from_str("int").unwrap();

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // type_specifier -> LONG
                                        98 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("type_specifier"));
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("type_specifier")).as_str());

                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("LONG"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}| {}\"]\n", type_node_id, type_node_id, String::from("LONG")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            debug_node_stack.push(debug_node);

                                            //
                                            // TYPEDEF HANDLING
                                            //

                                            if self.is_typedef_active {
                                                self.lexer_produce_type_name = true;
                                                self.is_typedef_active = false;
                                            }

                                            self.last_source_type = String::from("long");

                                            //
                                            // AST - type_specifier -> LONG
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::DataType;
                                                ast_node.string_val = String::from("long");
                                                //
                                                // TYPE
                                                //
                                                // Nora Sandler, page 252, add datatype to AST nodes
                                                ast_node.analyzed_data_type = DataType::from_str("long").unwrap();

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // type_specifier -> FLOAT
                                        99 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("type_specifier"));
                                            debug_node_stack.push(debug_node);
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("type_specifier")).as_str());

                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("FLOAT"));
                                            // debug_node_stack.push(debug_node);
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", type_node_id, type_node_id, String::from("FLOAT")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            //
                                            // TYPEDEF HANDLING - if a typedef is used over a type, the parser needs to
                                            // insert the new type into the list of user-defined types and it needs to
                                            // turn IDENTIFIER lookahead token into TYPE_NAME token to satisfy the grammar.
                                            //

                                            if self.is_typedef_active {
                                                self.lexer_produce_type_name = true;
                                                self.is_typedef_active = false;
                                            }

                                            self.last_source_type = String::from("float");

                                            //
                                            // AST - type_specifier -> FLOAT
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::DataType;
                                                ast_node.string_val = String::from("float");

                                                //
                                                // TYPE
                                                //
                                                // Nora Sandler, page 252, add datatype to AST nodes
                                                ast_node.analyzed_data_type = DataType::from_str("float").unwrap();

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // type_specifier -> DOUBLE
                                        100 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("type_specifier"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", debug_node_id, debug_node_id, String::from("type_specifier")).as_str());

                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("DOUBLE"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}| {}\"]\n", type_node_id, type_node_id, String::from("DOUBLE")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            debug_node_stack.push(debug_node);

                                            //
                                            // TYPEDEF HANDLING
                                            //

                                            if self.is_typedef_active {
                                                self.lexer_produce_type_name = true;
                                                self.is_typedef_active = false;
                                            }

                                            self.last_source_type = String::from("double");

                                            //
                                            // AST - type_specifier -> DOUBLE
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::DataType;
                                                ast_node.string_val = String::from("double");

                                                //
                                                // TYPE
                                                //
                                                // Nora Sandler, page 252, add datatype to AST nodes
                                                ast_node.analyzed_data_type = DataType::from_str("double").unwrap();

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // type_specifier -> SIGNED
                                        101 => {
                                            // retrieve terminal of SIGNED
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                // println!("[Parser::consume()] [108] TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                            }

                                            // create new node
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("type_specifier"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", debug_node_id, debug_node_id, String::from("type_specifier")).as_str());

                                            // signed
                                            let identifier_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let identifier_node = DebugNode::new(identifier_node_id, value.clone());
                                            // debug_node_stack.push(debug_node);
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", identifier_node_id, identifier_node_id, value.clone()).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, identifier_node.id).as_str());

                                            debug_node_stack.push(debug_node);
                                        }

                                        // type_specifier -> UNSIGNED
                                        102 => {
                                            // retrieve terminal of UNSIGNED
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                // println!("[Parser::consume()] [108] TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                            }

                                            // create new node
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("type_specifier"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", debug_node_id, debug_node_id, String::from("type_specifier")).as_str());

                                            // unsigned
                                            let identifier_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let identifier_node = DebugNode::new(identifier_node_id, value.clone());
                                            // debug_node_stack.push(debug_node);
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", identifier_node_id, identifier_node_id, value.clone()).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, identifier_node.id).as_str());

                                            debug_node_stack.push(debug_node);

                                            // println!("{:?}", debug_node_id);
                                        }

                                        // type_specifier -> struct_or_union_specifier
                                        103 => {
                                            self.node_to_node("type_specifier", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // type_specifier -> enum_specifier

                                        // type_specifier -> TYPE_NAME
                                        105 => {
                                            // retrieve terminal TYPE_NAME
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                // println!("[Parser::consume()] [108] TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                            }

                                            // create new node
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("type_specifier"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", debug_node_id, debug_node_id, String::from("type_specifier")).as_str());

                                            // IDENTIFIER
                                            let identifier_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let identifier_node = DebugNode::new(identifier_node_id, value.clone());
                                            // debug_node_stack.push(debug_node);
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", identifier_node_id, identifier_node_id, value.clone()).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, identifier_node.id).as_str());

                                            debug_node_stack.push(debug_node);

                                            //
                                            // TYPEDEF HANDLING
                                            //

                                            // Store the type_specifier so that if a typedef is detected later,
                                            // the parser remembers the name of the newly created type
                                            self.last_type_specifier = value.clone();

                                            self.lexer_produce_type_name = false;
                                        }

                                        // struct_or_union_specifier -> struct_or_union IDENTIFIER OPENING_CURLY_BRACKET struct_declaration_list CLOSING_CURLY_BRACKET
                                        106 => {

                                            // retrieve struct name
                                            // the first value is a whitespace character
                                            // the second value is the struct name
                                            let mut value_index = 0;
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {

                                                value = terminal_rev.clone().unwrap().data;
                                                // println!("[Parser::consume()] [106] TerminalValue: '{}'", terminal_rev.clone().unwrap().data);

                                                // break at index 1 because after index 1 the CURLY brackets appear
                                                if value_index == 1 {
                                                    break;
                                                }

                                                value_index = value_index + 1;
                                            }

                                            //
                                            // LHS
                                            //

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("struct_or_union_specifier"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("struct_or_union_specifier")).as_str());

                                            //
                                            // RHS
                                            //

                                            // take old node from stack - struct_or_union
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // IDENTIFIER

                                            // OPENING_CURLY_BRACKET

                                            // take old node from stack - struct_declaration_list
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // CLOSING_CURLY_BRACKET

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - struct_or_union_specifier -> struct_or_union IDENTIFIER OPENING_CURLY_BRACKET struct_declaration_list CLOSING_CURLY_BRACKET
                                            //

                                            if self.construct_ast {

                                                let mut struct_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                struct_ast_node.node_type = AstNodeType::StructureDeclaration;
                                                struct_ast_node.string_val = value.clone();
                                                for i in 0..self.struct_field_counter {
                                                    let member_declaration_ast_node = self.ast_stack.pop().unwrap();
                                                    struct_ast_node.block_items.push(member_declaration_ast_node)
                                                }
                                                self.struct_field_counter = 0;

                                                // struct declaration
                                                let struct_declaration_ast_node = self.ast_stack.pop().unwrap();
                                                struct_ast_node.lhs = Some(struct_declaration_ast_node);

                                                self.ast_stack.push(struct_ast_node.id);

                                                node_map.insert(struct_ast_node.id, struct_ast_node);
                                            }
                                        }

                                        // struct_or_union_specifier -> struct_or_union IDENTIFIER
                                        108 => {

                                            // retrieve terminal IDENTIFIER
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;

                                                // println!("[Parser::consume()] [108] TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                                // println!("");
                                            }

                                            // let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            // let debug_node = DebugNode::new(debug_node_id, format!("primary_expression '{}'\n", value));
                                            // debug_node_stack.push(debug_node);
                                            // string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, format!("primary_expression '{}'", value)).as_str());

                                            // create new node
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("struct_or_union_specifier"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("struct_or_union_specifier")).as_str());

                                            // IDENTIFIER
                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, value.clone());
                                            // debug_node_stack.push(debug_node);
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", type_node_id, type_node_id, value.clone()).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            // struct_or_union
                                            // take old node from stack
                                            let declarator_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, declarator_node.id).as_str());

                                            debug_node_stack.push(debug_node);

                                            //
                                            // TYPEDEF HANDLING
                                            //

                                            if self.is_typedef_active {
                                                self.lexer_produce_type_name = true;
                                                self.is_typedef_active = false;
                                            }

                                            self.last_source_type = value.clone();

                                            //
                                            // AST - struct_or_union_specifier -> struct_or_union IDENTIFIER
                                            //

                                            // example: struct Person p1;

                                            if self.construct_ast {

                                                let type_ast_node = self.ast_stack.pop().unwrap();

                                                let mut identifier_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                identifier_ast_node.node_type = AstNodeType::Identifier;
                                                identifier_ast_node.string_val = value.clone();

                                                let mut data_type_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                data_type_ast_node.node_type = AstNodeType::Structure;
                                                data_type_ast_node.lhs = Some(type_ast_node);
                                                data_type_ast_node.rhs = Some(identifier_ast_node.id);

                                                self.ast_stack.push(data_type_ast_node.id);

                                                node_map.insert(identifier_ast_node.id, identifier_ast_node);
                                                node_map.insert(data_type_ast_node.id, data_type_ast_node);
                                            }
                                        }

                                        // struct_or_union -> STRUCT
                                        109 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("struct_or_union"));
                                            debug_node_stack.push(debug_node);
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("struct_or_union")).as_str());

                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("STRUCT"));
                                            // debug_node_stack.push(debug_node);
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", type_node_id, type_node_id, found_rule.original_id, String::from("STRUCT")).as_str());

                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            //
                                            // AST - struct_or_union -> STRUCT
                                            //

                                            if self.construct_ast {

                                                let mut struct_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                struct_ast_node.node_type = AstNodeType::DataType;
                                                struct_ast_node.string_val = String::from("STRUCT");

                                                self.ast_stack.push(struct_ast_node.id);

                                                node_map.insert(struct_ast_node.id, struct_ast_node);
                                            }
                                        }

                                        // struct_declaration_list -> struct_declaration_list struct_declaration
                                        111 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("struct_declaration_list"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("struct_declaration_list")).as_str());

                                            // struct_declaration_list
                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // struct_declaration
                                            // take old node from stack
                                            let declarator_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, declarator_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // struct_declaration_list -> struct_declaration
                                        112 => {
                                            self.node_to_node("struct_declaration_list", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // struct_declaration -> specifier_qualifier_list struct_declarator_list SEMICOLON
                                        113 => {

                                            //
                                            // LHS
                                            //

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("struct_declaration"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("struct_declaration")).as_str());

                                            //
                                            // RHS
                                            //

                                            // take old node from stack - specifier_qualifier_list
                                            let field_type_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, field_type_node.id).as_str());

                                            // take old node from stack - struct_declarator_list
                                            let field_identifier_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, field_identifier_node.id).as_str());

                                            // SEMICOLON

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - struct_declaration -> specifier_qualifier_list struct_declarator_list SEMICOLON
                                            //

                                            if self.construct_ast {

                                                let type_ast_node = self.ast_stack.pop().unwrap();
                                                let identifier_ast_node = self.ast_stack.pop().unwrap();

                                                let mut member_declaration_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                // member_declaration_ast_node.string_val = type_ast_node.string_val.clone();
                                                member_declaration_ast_node.node_type = AstNodeType::MemberDeclaration;
                                                member_declaration_ast_node.lhs = Some(type_ast_node);
                                                member_declaration_ast_node.rhs = Some(identifier_ast_node);

                                                self.struct_field_counter = self.struct_field_counter + 1;

                                                self.ast_stack.push(member_declaration_ast_node.id);

                                                node_map.insert(member_declaration_ast_node.id, member_declaration_ast_node);
                                            }
                                        }

                                        // specifier_qualifier_list -> type_specifier
                                        115 => {
                                            self.node_to_node("specifier_qualifier_list", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // struct_declarator_list -> struct_declarator
                                        118 => {
                                            self.node_to_node("struct_declarator_list", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // struct_declarator -> declarator
                                        120 => {
                                            self.node_to_node("struct_declarator", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // declarator -> pointer direct_declarator
                                        132 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("declarator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("declarator")).as_str());

                                            // take old node from stack - pointer
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack - direct_declarator
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            // println!("{:?}", debug_node_id);

                                            //
                                            // AST - declarator -> pointer direct_declarator
                                            //

                                            if self.construct_ast {

                                                // referenced type
                                                let referenced_type_ast_node_id = self.ast_stack.pop().unwrap();
                                                let referenced_type_ast_node = node_map.get(&referenced_type_ast_node_id).unwrap();

                                                let mut pointer_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                pointer_ast_node.node_type = AstNodeType::Pointer;
                                                pointer_ast_node.string_val = referenced_type_ast_node.string_val.clone();
                                                pointer_ast_node.rhs = Some(referenced_type_ast_node.id);

                                                self.ast_stack.push(pointer_ast_node.id);

                                                node_map.insert(pointer_ast_node.id, pointer_ast_node);
                                            }
                                        }

                                        // declarator -> direct_declarator
                                        133 => {
                                            self.node_to_node("declarator", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // direct_declarator -> IDENTIFIER
                                        134 => {
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                            }

                                            // // DEBUG
                                            // println!("{:?}", value);

                                            // direct_declarator
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("direct_declarator"));
                                            debug_node_stack.push(debug_node);
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("direct_declarator")).as_str());

                                            // IDENTIFIER
                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from(value.clone()));
                                            string_buffer.push_str(format!("{:?} [label=\"{}| {}\"]\n", type_node_id, type_node_id, String::from(value.clone())).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            //
                                            // AST - direct_declarator -> IDENTIFIER
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Identifier;
                                                ast_node.string_val = value.clone();

                                                // // DEBUG
                                                // println!("{:?}", &ast_node.string_val);

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // direct_declarator -> direct_declarator OPENING_ANGULAR_BRACKET constant_expression CLOSING_ANGULAR_BRACKET
                                        136 => {

                                            // direct_declarator
                                            //
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("direct_declarator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("direct_declarator")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // [
                                            // create new node id
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from("["));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from("[")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());

                                            // take old node from stack - constant_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // ]
                                            // create new node id
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from("]"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from("]")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            // println!("debug_node_id: {:?}", debug_node_id);

                                            //
                                            // AST - direct_declarator -> direct_declarator OPENING_ANGULAR_BRACKET constant_expression CLOSING_ANGULAR_BRACKET
                                            //

                                            if self.construct_ast {

                                                // array size
                                                let size_ast_node = self.ast_stack.pop().unwrap();

                                                // array name
                                                let identifier_ast_node = self.ast_stack.pop().unwrap();

                                                // array element data type
                                                let array_element_data_type_ast_node_id = self.ast_stack.pop().unwrap();
                                                let array_element_data_type_ast_node = node_map.get(&array_element_data_type_ast_node_id).unwrap();

                                                let node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
                                                let mut array_datatype_ast_node: AstNode = AstNode::new(node_id);
                                                array_datatype_ast_node.node_type = AstNodeType::Array;
                                                // this is where the dot printer (ast_node.rs) takes the data type from
                                                array_datatype_ast_node.string_val = array_element_data_type_ast_node.string_val.clone();
                                                array_datatype_ast_node.data_type = Some(array_element_data_type_ast_node.id);
                                                array_datatype_ast_node.lhs = Some(size_ast_node);

                                                // Format:
                                                // 1st pop: identifier
                                                // 2nd pop: data type of function return or variable
                                                // 3rd pop: initializer SingleInit or CompoundInit

                                                self.ast_stack.push(array_datatype_ast_node.id);
                                                self.ast_stack.push(identifier_ast_node);

                                                node_map.insert(array_datatype_ast_node.id, array_datatype_ast_node);
                                            }
                                        }

                                        // direct_declarator -> direct_declarator OPENING_ANGULAR_BRACKET CLOSING_ANGULAR_BRACKET
                                        137 => {

                                            // direct_declarator
                                            //
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("direct_declarator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("direct_declarator")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // [
                                            // create new node id
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from("["));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from("[")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());

                                            // ]
                                            // create new node id
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from("]"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from("]")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - direct_declarator -> direct_declarator OPENING_ANGULAR_BRACKET CLOSING_ANGULAR_BRACKET
                                            //

                                            if self.construct_ast {

                                                let mut direct_declarator_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                direct_declarator_ast_node.node_type = AstNodeType::VariableDeclaration;

                                                // TODO
                                                direct_declarator_ast_node.analyzed_data_type = DataType::from_str("double").unwrap();

                                                self.ast_stack.push(direct_declarator_ast_node.id);

                                                self.direct_declarator_counter = self.direct_declarator_counter + 1;

                                                node_map.insert(direct_declarator_ast_node.id, direct_declarator_ast_node);
                                            }
                                        }

                                        // direct_declarator -> direct_declarator OPENING_BRACKET parameter_type_list CLOSING_BRACKET
                                        138 => {

                                            // direct_declarator
                                            //
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("direct_declarator"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("direct_declarator")).as_str());

                                            // take old node from stack - direct_declarator
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // (
                                            // create new node id
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from("("));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from("(")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());

                                            // take old node from stack - parameter_type_list
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // )
                                            // create new node id
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from(")"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from(")")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - direct_declarator -> direct_declarator OPENING_BRACKET parameter_type_list CLOSING_BRACKET
                                            //

                                            if self.construct_ast {

                                                // [direct_declarator] (138, this rule) is used in both full function definitions
                                                // and half function prototypes
                                                //
                                                // For function definitions as well as for function prototypes
                                                // the [direct_declarator] contains
                                                // - function name and paremeter list (type + name of each param)
                                                // - does not contain return type
                                                // - does not contain the body
                                                //
                                                // For full function definitions, the return type along with the body
                                                // is contained in the parent [function_definition]
                                                //
                                                // For prototypes, the return type is contained in the parent [declaration] (77).

                                                // println!("debug_node_id: {}", debug_node_id);

                                                let mut function_declaration_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                function_declaration_ast_node.node_type = AstNodeType::FunctionDeclaration;

                                                //
                                                // Parameter List
                                                //

                                                while self.parameter_counter > 0 {

                                                    // one parameter is processed
                                                    self.parameter_counter = self.parameter_counter - 1;

                                                    let parameter_ast_node = self.ast_stack.pop().unwrap();
                                                    function_declaration_ast_node.parameters.push(parameter_ast_node);
                                                }

                                                //
                                                // Function Name
                                                //

                                                let identifier_ast_node = self.ast_stack.pop().unwrap();
                                                function_declaration_ast_node.function_name_ast_node = Some(identifier_ast_node);

                                                //
                                                // Body Block
                                                //
                                                // body block does not exist for declarations (without implementation)
                                                //

                                                function_declaration_ast_node.lhs = None;

                                                function_declaration_ast_node.analyzed_data_type = DataType::from_str("double").unwrap();

                                                // if let Some(block) = function_declaration_ast_node.lhs.as_ref() {
                                                //     println!("test");
                                                // }

                                                self.ast_stack.push(function_declaration_ast_node.id);

                                                node_map.insert(function_declaration_ast_node.id, function_declaration_ast_node);
                                            }
                                        }

                                        // direct_declarator -> direct_declarator OPENING_BRACKET CLOSING_BRACKET
                                        140 => {
                                            // create new node
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("direct_declarator"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("direct_declarator")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!(" {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // (
                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("("));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", type_node_id, type_node_id, String::from("(")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            // )
                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from(")"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", type_node_id, type_node_id, String::from(")")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST
                                            //

                                            if self.construct_ast {

                                                let identifier_ast_node_id = self.ast_stack.pop().unwrap();
                                                let identifier_ast_node = node_map.get(&identifier_ast_node_id).unwrap();

                                                let mut function_definition_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                function_definition_ast_node.node_type = AstNodeType::FunctionDeclaration;
                                                function_definition_ast_node.string_val = identifier_ast_node.string_val.clone();
                                                // TODO
                                                function_definition_ast_node.analyzed_data_type = DataType::from_str("double").unwrap();

                                                let mut function_name_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                function_name_ast_node.node_type = AstNodeType::Identifier;
                                                function_name_ast_node.string_val = identifier_ast_node.string_val.clone();

                                                function_definition_ast_node.function_name_ast_node = Some(function_name_ast_node.id);

                                                // // DEBUG
                                                // println!("{:?}", &function_definition_ast_node.string_val);

                                                self.ast_stack.push(function_definition_ast_node.id);

                                                node_map.insert(function_definition_ast_node.id, function_definition_ast_node);
                                                node_map.insert(function_name_ast_node.id, function_name_ast_node);
                                            }
                                        }

                                        // pointer -> ASTERISK
                                        141 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("pointer"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("pointer")).as_str());

                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("*"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", type_node_id, type_node_id, String::from("*")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            debug_node_stack.push(debug_node);
                                        }

                                        // parameter_type_list -> parameter_list
                                        147 => {
                                            self.node_to_node("parameter_type_list", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // parameter_list -> parameter_declaration
                                        149 => {
                                            self.node_to_node("parameter_list", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // parameter_list -> parameter_list COMMA parameter_declaration
                                        150 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("parameter_list"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("parameter_list")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // ,
                                            // create new node id
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from(","));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from(",")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // parameter_declaration -> declaration_specifiers declarator
                                        151 => {
                                            // parameter_declaration
                                            //
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("parameter_declaration"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} RuleId: {} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("parameter_declaration")).as_str());

                                            // declaration_specifiers
                                            //
                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // declarator
                                            //
                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST
                                            //

                                            if self.construct_ast {

                                                self.parameter_counter = self.parameter_counter + 1;

                                                // parameter name
                                                let identifier_ast_node = self.ast_stack.pop().unwrap();

                                                // DEBUG
                                                // println!("{:?}", identifier_ast_node);

                                                // parameter DataType
                                                let data_type_ast_node_id = self.ast_stack.pop().unwrap();
                                                let data_type_ast_node = node_map.get(&data_type_ast_node_id).unwrap();

                                                // DEBUG
                                                // println!("{:?}", data_type_ast_node);

                                                let mut parameter_declaration_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                parameter_declaration_ast_node.node_type = AstNodeType::ParameterDeclaration;
                                                parameter_declaration_ast_node.lhs = Some(identifier_ast_node);
                                                //
                                                // TYPE
                                                //
                                                parameter_declaration_ast_node.analyzed_data_type = data_type_ast_node.analyzed_data_type.clone();
                                                parameter_declaration_ast_node.rhs = Some(data_type_ast_node.id);

                                                self.ast_stack.push(parameter_declaration_ast_node.id);

                                                node_map.insert(parameter_declaration_ast_node.id, parameter_declaration_ast_node);
                                            }
                                        }

                                        // parameter_declaration -> declaration_specifiers
                                        153 => {
                                            self.node_to_node("parameter_declaration", found_rule.original_id, string_buffer, debug_node_stack);

                                            //
                                            // AST - parameter_declaration -> declaration_specifiers
                                            //

                                            if self.construct_ast {

                                                self.parameter_counter = self.parameter_counter + 1;

                                                let mut identifier_ast_node = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                identifier_ast_node.node_type = AstNodeType::Identifier;
                                                identifier_ast_node.string_val = String::from("Unnamed!");

                                                // parameter DataType
                                                let data_type_ast_node = self.ast_stack.pop().unwrap();

                                                let mut parameter_declaration_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                parameter_declaration_ast_node.node_type = AstNodeType::ParameterDeclaration;
                                                parameter_declaration_ast_node.lhs = Some(identifier_ast_node.id);
                                                parameter_declaration_ast_node.rhs = Some(data_type_ast_node);

                                                self.ast_stack.push(parameter_declaration_ast_node.id);

                                                node_map.insert(identifier_ast_node.id, identifier_ast_node);
                                                node_map.insert(parameter_declaration_ast_node.id, parameter_declaration_ast_node);
                                            }
                                        }

                                        // type_name -> specifier_qualifier_list
                                        156 => {
                                            self.node_to_node("parameter_declaration", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // statement -> labeled_statement
                                        175 => {
                                            self.node_to_node("statement", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // initializer -> assignment_expression
                                        170 => {
                                            self.node_to_node("initializer", found_rule.original_id, string_buffer, debug_node_stack);

                                            //
                                            // AST - initializer -> assignment_expression
                                            //

                                            self.direct_declarator_counter = self.direct_declarator_counter + 1;
                                        }

                                        // initializer -> OPENING_CURLY_BRACKET initializer_list CLOSING_CURLY_BRACKET
                                        171 => {
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("initializer"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("initializer")).as_str());

                                            // {
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from("{"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from("{")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());

                                            // take old node from stack - initializer_list
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // }
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from("}"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from("}")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // initializer_list -> initializer
                                        173 => {
                                            self.node_to_node("initializer_list", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // initializer_list -> initializer COMMA initializer_list
                                        174 => {
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("initializer_list"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("initializer_list")).as_str());

                                            // take old node from stack - initializer
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // ,
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from(","));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from(",")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());

                                            // take old node from stack - initializer_list
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // statement -> compound_statement
                                        176 => {
                                            self.node_to_node("statement", found_rule.original_id, string_buffer, debug_node_stack);

                                            //
                                            // AST - statement -> compound_statement
                                            //

                                            if self.construct_ast {

                                                let compound_ast_node = self.ast_stack.pop().unwrap();

                                                let mut statement_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                statement_ast_node.node_type = AstNodeType::Statement;
                                                statement_ast_node.lhs = Some(compound_ast_node);

                                                self.ast_stack.push(statement_ast_node.id);

                                                node_map.insert(statement_ast_node.id, statement_ast_node);
                                            }
                                        }

                                        // statement -> expression_statement
                                        177 => {
                                            let debug_node_id = self.node_to_node("statement", found_rule.original_id, string_buffer, debug_node_stack);

                                            // println!("debug_node_id: {}", debug_node_id);

                                            //
                                            // AST - statement -> expression_statement
                                            //

                                            if self.construct_ast {

                                                let exp_ast_node_id = self.ast_stack.pop().unwrap();
                                                let exp_ast_node = node_map.get(&exp_ast_node_id).unwrap();
                                                // println!("{:?}", exp_ast_node);

                                                if exp_ast_node.node_type == AstNodeType::Expression {

                                                    let mut statement_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    statement_ast_node.node_type = AstNodeType::Statement;
                                                    statement_ast_node.lhs = Some(exp_ast_node_id);

                                                    // wrap declaration into block_item
                                                    let mut block_item_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    block_item_ast_node.node_type = AstNodeType::BlockItem;
                                                    block_item_ast_node.lhs = Some(statement_ast_node.id);

                                                    self.ast_stack.push(block_item_ast_node.id);

                                                    node_map.insert(statement_ast_node.id, statement_ast_node);
                                                    node_map.insert(block_item_ast_node.id, block_item_ast_node);

                                                } else {

                                                    self.ast_stack.push(exp_ast_node.id);

                                                }
                                            }
                                        }

                                        // statement -> selection_statement
                                        178 => {
                                            self.node_to_node("statement", found_rule.original_id, string_buffer, debug_node_stack);

                                            //
                                            // AST - statement -> selection_statement
                                            //

                                            if self.construct_ast {

                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                // statement
                                                let mut statement_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                statement_ast_node.node_type = AstNodeType::Statement;
                                                statement_ast_node.lhs = Some(lhs_ast_node);

                                                self.ast_stack.push(statement_ast_node.id);

                                                node_map.insert(statement_ast_node.id, statement_ast_node);
                                            }
                                        }

                                        // statement -> iteration_statement
                                        179 => {
                                            self.node_to_node("statement", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // statement -> jump_statement
                                        180 => {
                                            let debug_node_id = self.node_to_node("statement", found_rule.original_id, string_buffer, debug_node_stack);

                                            // println!("debug_node_id: {}", debug_node_id);

                                            //
                                            // AST - statement -> jump_statement
                                            //

                                            if self.construct_ast {

                                                let lhs_ast_node = self.ast_stack.pop().unwrap();

                                                // statement
                                                let mut statement_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                statement_ast_node.node_type = AstNodeType::Statement;
                                                statement_ast_node.lhs = Some(lhs_ast_node);

                                                self.ast_stack.push(statement_ast_node.id);

                                                node_map.insert(statement_ast_node.id, statement_ast_node);
                                            }
                                        }

                                        // labeled_statement -> CASE constant_expression COLON statement
                                        182 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("labeled_statement"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("labeled_statement")).as_str());

                                            // CASE
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("case"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", lessthan_node_id, lessthan_node_id, String::from("case")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack - constant_expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // COLON
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from(":"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", lessthan_node_id, lessthan_node_id, String::from(":")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack - statement
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            // println!("{:?}", debug_node_id);

                                            //
                                            // AST node - labeled_statement -> CASE constant_expression COLON statement
                                            //

                                            if self.construct_ast {

                                                // take the statement that this case contains from the stack
                                                let body_ast_node = self.ast_stack.pop().unwrap();

                                                // take the expression or constant that acts as differentiator for this case from the stack
                                                let differentiator_ast_node = self.ast_stack.pop().unwrap();

                                                // case node
                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Case;
                                                ast_node.block_items.push(body_ast_node);
                                                ast_node.expression = Some(differentiator_ast_node);

                                                self.ast_stack.push(ast_node.id);

                                                self.switch_case_default_counter = self.switch_case_default_counter + 1;

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // labeled_statement -> DEFAULT COLON statement
                                        183 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("labeled_statement"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("labeled_statement")).as_str());

                                            // DEFAULT
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("default"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", lessthan_node_id, lessthan_node_id, String::from("default")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // COLON
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from(":"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", lessthan_node_id, lessthan_node_id, String::from(":")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            // println!("{:?}", debug_node_id);

                                            //
                                            // AST node - labeled_statement -> DEFAULT COLON statement
                                            //

                                            if self.construct_ast {

                                                let body_ast_node = self.ast_stack.pop().unwrap();

                                                // loop {
                                                //     let temp_node = self.ast_stack.pop().unwrap();
                                                //     if temp_node.node_type == AstNodeType::EmptyStatement {
                                                //         break;
                                                //     }
                                                // }

                                                // default node
                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Default;
                                                ast_node.block_items.push(body_ast_node);

                                                self.ast_stack.push(ast_node.id);

                                                self.switch_case_default_counter = self.switch_case_default_counter + 1;

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // compound_statement -> declaration_or_statement_list
                                        // compound_statement -> OPENING_CURLY_BRACKET declaration_or_statement_list CLOSING_CURLY_BRACKET
                                        184 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("compound_statement"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("compound_statement")).as_str());

                                            // take old node from stack - declaration_or_statement_list
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - compound_statement -> declaration_or_statement_list
                                            // AST - compound_statement -> OPENING_CURLY_BRACKET declaration_or_statement_list CLOSING_CURLY_BRACKET
                                            //

                                            if self.construct_ast {

                                                // println!("debug_node_id: {}", debug_node_id);

                                                // create a block since a compound AST node needs to have a block
                                                let mut block_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                block_ast_node.node_type = AstNodeType::Block;

                                                // take all Statements, Declarations or BlockItem nodes and insert them into the block
                                                while self.child_counter > 0 {

                                                    self.child_counter = self.child_counter - 1;

                                                    let body_ast_node_id = self.ast_stack.pop().unwrap();
                                                    let body_ast_node = node_map.get(&body_ast_node_id).unwrap();
                                                    match body_ast_node.node_type {

                                                        // AstNodeType::BlockItem => {
                                                        //     block_ast_node.block_items.push(Box::new(body_ast_node));
                                                        // }

                                                        AstNodeType::Declaration | AstNodeType::Statement => {
                                                            let mut block_item_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                            block_item_ast_node.node_type = AstNodeType::BlockItem;
                                                            block_item_ast_node.lhs = Some(body_ast_node.id);

                                                            block_ast_node.block_items.push(block_item_ast_node.id);

                                                            node_map.insert(block_item_ast_node.id, block_item_ast_node);
                                                        }

                                                        // AstNodeType::Default | AstNodeType::Case => {
                                                        //     block_ast_node.block_items.push(Box::new(body_ast_node));
                                                        // }

                                                        // AstNodeType::EmptyStatement => {
                                                        //     block_ast_node.block_items.push(Box::new(body_ast_node));
                                                        // }

                                                        _ => {
                                                            // println!("{:?}", body_ast_node);
                                                            // self.ast_stack.push(body_ast_node);
                                                            block_ast_node.block_items.push(body_ast_node.id);
                                                        }
                                                    }
                                                }

                                                // create compound AST node - insert block
                                                let mut compound_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                compound_ast_node.node_type = AstNodeType::Compound;
                                                compound_ast_node.lhs = Some(block_ast_node.id);

                                                // println!("{:?}", compound_ast_node);

                                                self.ast_stack.push(compound_ast_node.id);

                                                node_map.insert(block_ast_node.id, block_ast_node);
                                                node_map.insert(compound_ast_node.id, compound_ast_node);
                                            }
                                        }

                                        // compound_statement -> OPENING_CURLY_BRACKET CLOSING_CURLY_BRACKET
                                        185 => {
                                            //debug_node_stack.pop();

                                            // push a dummy compound statement otherwise the stack is empty and the other rule code crashes
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("compound_statement {{}}"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", debug_node_id, debug_node_id, String::from("compound_statement {{}}")).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST
                                            //

                                            if self.construct_ast {

                                                // create a block since a compound AST node needs to have a block
                                                let mut block_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                block_ast_node.node_type = AstNodeType::Block;

                                                // no block items

                                                // create compound AST node
                                                let mut compound_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                compound_ast_node.node_type = AstNodeType::Compound;
                                                compound_ast_node.lhs = Some(block_ast_node.id);

                                                self.ast_stack.push(compound_ast_node.id);

                                                node_map.insert(compound_ast_node.id, compound_ast_node);
                                                node_map.insert(block_ast_node.id, block_ast_node);
                                            }
                                        }

                                        // declaration_or_statement_list -> declaration declaration_or_statement_list
                                        186 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("declaration_or_statement_list"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("declaration_or_statement_list")).as_str());

                                            // declaration - take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // declaration_or_statement_list - take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            // println!("{:?}", debug_node_id);

                                            //
                                            // AST - declaration_or_statement_list -> declaration declaration_or_statement_list
                                            //

                                            if self.construct_ast {

                                                // the child counter is used by rule 184 which will take that many
                                                // children from the ast stack and insert them into the compound element
                                                self.child_counter = self.child_counter + 1;
                                                // println!("child_counter: {}", self.child_counter);

                                                let wrap_into_block_item = false;
                                                if wrap_into_block_item {

                                                    let second_ast_node = self.ast_stack.pop().unwrap(); // declaration_or_statement_list
                                                    let first_ast_node = self.ast_stack.pop().unwrap(); // declaration

                                                    // wrap declaration into BlockItem
                                                    let mut block_item_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    block_item_ast_node.node_type = AstNodeType::BlockItem;
                                                    block_item_ast_node.lhs = Some(first_ast_node);

                                                    self.ast_stack.push(block_item_ast_node.id);
                                                    self.ast_stack.push(second_ast_node);

                                                    node_map.insert(block_item_ast_node.id, block_item_ast_node);
                                                }
                                            }
                                        }

                                        // declaration_or_statement_list -> declaration
                                        187 => {
                                            self.node_to_node("declaration_or_statement_list", found_rule.original_id, string_buffer, debug_node_stack);

                                            //
                                            // AST
                                            //

                                            if self.construct_ast {

                                                self.child_counter = self.child_counter + 1;
                                                // println!("child_counter: {}", self.child_counter);
                                            }
                                        }

                                        // declaration_or_statement_list -> statement declaration_or_statement_list
                                        188 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("declaration_or_statement_list"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("declaration_or_statement_list")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            // println!("{:?}", debug_node_id);

                                            //
                                            // AST - declaration_or_statement_list -> statement declaration_or_statement_list
                                            //

                                            if self.construct_ast {

                                                // affects: case-statements, default-statement,

                                                // the child counter is used by the 184 rule which constructs compound statements
                                                self.child_counter = self.child_counter + 1;
                                                // println!("child_counter: {}", self.child_counter);

                                                // loop {
                                                //     let temp_node = self.ast_stack.pop().unwrap();
                                                //     println!("{:?}", temp_node);
                                                //     if temp_node.node_type == AstNodeType::EmptyStatement {
                                                //         break;
                                                //     }
                                                // }
                                            }
                                        }

                                        // declaration_or_statement_list -> statement
                                        189 => {
                                            self.node_to_node("declaration_or_statement_list", found_rule.original_id, string_buffer, debug_node_stack);

                                            //
                                            // AST
                                            //

                                            if self.construct_ast {

                                                self.child_counter = self.child_counter + 1;
                                                // println!("child_counter: {}", self.child_counter);
                                            }
                                        }

                                        // expression_statement -> expression SEMICOLON
                                        190 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("expression_statement"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("expression_statement")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // ;
                                            // create new node id
                                            // create new node with node id and label
                                            let semicolon_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let semicolon_node = DebugNode::new(semicolon_node_id, String::from(";"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", semicolon_node_id, semicolon_node_id, String::from(";")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, semicolon_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // expression_statement -> SEMICOLON
                                        191 => {

                                            // this affects switch-case and for loops. For loops expect at least a single body statement
                                            let output_empty_statement = true;
                                            if output_empty_statement {
                                                // create new node id
                                                // create new node with node id and label
                                                let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                                let debug_node = DebugNode::new(debug_node_id, String::from("expression_statement"));
                                                // print new node into string buffer. e.g.    0 [label="test"]
                                                string_buffer.push_str(format!("{:?} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("expression_statement")).as_str());

                                                // ;
                                                // create new node id
                                                // create new node with node id and label
                                                let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                                let equals_node = DebugNode::new(equals_node_id, String::from(";"));
                                                string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", equals_node_id, equals_node_id, String::from(";")).as_str());
                                                string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                                // push new node to stack
                                                debug_node_stack.push(debug_node);
                                            }

                                            //
                                            // AST - expression_statement -> SEMICOLON
                                            //

                                            if self.construct_ast {

                                                if output_empty_statement {
                                                    let mut empty_statement_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    empty_statement_ast_node.node_type = AstNodeType::EmptyStatement;

                                                    self.ast_stack.push(empty_statement_ast_node.id);

                                                    node_map.insert(empty_statement_ast_node.id, empty_statement_ast_node);
                                                }
                                            }
                                        }

                                        // selection_statement -> IF OPENING_BRACKET expression CLOSING_BRACKET statement
                                        192 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("selection_statement"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("selection_statement")).as_str());

                                            // if
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("if"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", lessthan_node_id, lessthan_node_id, String::from("if")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - selection_statement -> IF OPENING_BRACKET expression CLOSING_BRACKET statement
                                            //

                                            if self.construct_ast {

                                                let first_ast_node = self.ast_stack.pop().unwrap(); // compound > block > block_items/statements
                                                let second_ast_node_id = self.ast_stack.pop().unwrap(); // binary expression for if
                                                let second_ast_node = node_map.get(&second_ast_node_id).unwrap();

                                                // println!("{:?}", first_ast_node);
                                                // println!("{:?}", second_ast_node);

                                                if second_ast_node.node_type == AstNodeType::Binary {

                                                    // e.g. if (1 < 2)

                                                    let mut expression_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    expression_ast_node.node_type = AstNodeType::Expression;
                                                    expression_ast_node.lhs = Some(second_ast_node.id);

                                                    let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    ast_node.node_type = AstNodeType::If;
                                                    ast_node.lhs = Some(first_ast_node);
                                                    ast_node.expression = Some(expression_ast_node.id);

                                                    self.ast_stack.push(ast_node.id);

                                                    node_map.insert(expression_ast_node.id, expression_ast_node);
                                                    node_map.insert(ast_node.id, ast_node);

                                                } else if second_ast_node.node_type == AstNodeType::Identifier {

                                                    // e.g. if (var_test)

                                                    let mut expression_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    expression_ast_node.node_type = AstNodeType::Expression;
                                                    expression_ast_node.lhs = Some(second_ast_node.id);

                                                    let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    ast_node.node_type = AstNodeType::If;
                                                    ast_node.lhs = Some(first_ast_node);
                                                    ast_node.expression = Some(expression_ast_node.id);

                                                    self.ast_stack.push(ast_node.id);

                                                    node_map.insert(expression_ast_node.id, expression_ast_node);
                                                    node_map.insert(ast_node.id, ast_node);

                                                } else {

                                                    panic!("I forgot, what this case was meant for! I do not know of an example that triggers this part!");

                                                    /*
                                                    let third_ast_node = self.ast_stack.pop().unwrap();

                                                    let mut expression_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    expression_ast_node.node_type = AstNodeType::Expression;
                                                    expression_ast_node.lhs = Some(Box::new(third_ast_node));

                                                    let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    ast_node.node_type = AstNodeType::If;
                                                    ast_node.lhs = Some(expression_ast_node.id);
                                                    ast_node.rhs = Some(Box::new(second_ast_node));
                                                    ast_node.expression = Some(Box::new(first_ast_node));

                                                    self.ast_stack.push(ast_node.id);
                                                    */
                                                }
                                            }
                                        }

                                        // selection_statement -> IF OPENING_BRACKET expression CLOSING_BRACKET statement ELSE statement
                                        193 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("selection_statement"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("selection_statement")).as_str());

                                            // if
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("if"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", lessthan_node_id, lessthan_node_id, String::from("if")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // if-branch

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // else-branch

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();

                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());
                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - selection_statement -> IF OPENING_BRACKET expression CLOSING_BRACKET statement ELSE statement
                                            //

                                            if self.construct_ast {

                                                let first_ast_node = self.ast_stack.pop().unwrap();
                                                let second_ast_node_id = self.ast_stack.pop().unwrap();
                                                let second_ast_node = node_map.get(&second_ast_node_id).unwrap();

                                                if second_ast_node.node_type == AstNodeType::Binary {

                                                    // build AST node for expression
                                                    let mut expression_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    expression_ast_node.node_type = AstNodeType::Expression;
                                                    expression_ast_node.lhs = Some(second_ast_node.id);

                                                    // build AST node for IF
                                                    let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    ast_node.node_type = AstNodeType::If;
                                                    ast_node.lhs = Some(first_ast_node);
                                                    ast_node.expression = Some(expression_ast_node.id);

                                                    self.ast_stack.push(ast_node.id);

                                                    node_map.insert(expression_ast_node.id, expression_ast_node);
                                                    node_map.insert(ast_node.id, ast_node);

                                                } else {

                                                    let third_ast_node = self.ast_stack.pop().unwrap();

                                                    // build AST node for expression
                                                    let mut expression_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    expression_ast_node.node_type = AstNodeType::Expression;
                                                    expression_ast_node.lhs = Some(third_ast_node);

                                                    // build AST node for IF
                                                    let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                    ast_node.node_type = AstNodeType::If;
                                                    ast_node.lhs = Some(second_ast_node.id);
                                                    ast_node.rhs = Some(first_ast_node);
                                                    ast_node.expression = Some(expression_ast_node.id);

                                                    self.ast_stack.push(ast_node.id);

                                                    node_map.insert(expression_ast_node.id, expression_ast_node);
                                                    node_map.insert(ast_node.id, ast_node);
                                                }
                                            }
                                        }

                                        // selection_statement -> SWITCH OPENING_BRACKET expression CLOSING_BRACKET statement
                                        194 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("selection_statement"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("selection_statement")).as_str());

                                            // SWITCH
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("switch"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", lessthan_node_id, lessthan_node_id, String::from("switch")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // (
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("("));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", lessthan_node_id, lessthan_node_id, String::from("(")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // )
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from(")"));
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", lessthan_node_id, lessthan_node_id, String::from(")")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - selection_statement -> SWITCH OPENING_BRACKET expression CLOSING_BRACKET statement
                                            //

                                            if self.construct_ast {

                                                // println!("{:?}", debug_node_id);

                                                // loop {
                                                //     let temp_node = self.ast_stack.pop().unwrap();
                                                //     println!("{:?}", temp_node);
                                                //     if temp_node.node_type == AstNodeType::EmptyStatement {
                                                //         break;
                                                //     }
                                                // }

                                                let statement_ast_node_id = self.ast_stack.pop().unwrap();
                                                let statement_ast_node = node_map.get(&statement_ast_node_id).unwrap();
                                                // println!("{:?}", statement_ast_node);
                                                let compound_ast_node_id = statement_ast_node.lhs.unwrap();
                                                let compound_ast_node = node_map.get(&compound_ast_node_id).unwrap();

                                                // let compound_ast_node = statement_ast_node.lhs.unwrap();
                                                // let mut block_ast_node = compound_ast_node.lhs.unwrap();

                                                let switch_expression_ast_node = self.ast_stack.pop().unwrap();

                                                // switch
                                                let mut switch_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                switch_ast_node.node_type = AstNodeType::Switch;
                                                switch_ast_node.expression = Some(switch_expression_ast_node);

                                                let mut temp_items = Vec::<AstNode>::new();

                                                let mut case_default: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));

                                                let mut is_first: bool = true;

                                                let all_statements_ast_node_id = compound_ast_node.lhs.unwrap();
                                                let all_statements_ast_node = node_map.get(&all_statements_ast_node_id).unwrap();
                                                for i in 0..all_statements_ast_node.block_items.len() {

                                                    let temp_node_id = all_statements_ast_node.block_items[i];
                                                    let temp_node = node_map.get(&temp_node_id).unwrap();
                                                    match temp_node.node_type {

                                                        AstNodeType::Case | AstNodeType::Default => {

                                                            // if there is an old case or default from an older iteration
                                                            if !is_first {

                                                                // put the old case or default into the switch
                                                                //switch_ast_node.block_items.push(Box::new(case_default));
                                                                temp_items.push(case_default);
                                                            }

                                                            is_first = false;

                                                            // temp_node.block_items.push(temp_node.lhs.expect("REASON"));
                                                            //let lhs_item = temp_node.lhs.unwrap();

                                                            case_default = temp_node.clone();
                                                        }

                                                        _ => {
                                                            // temp_items.push(*temp_node);

                                                            // println!("{:?}", case_default.lhs);
                                                            case_default.block_items.push(temp_node_id);
                                                        }
                                                    }

                                                    if all_statements_ast_node.block_items.len() == 0 {
                                                        break;
                                                    }
                                                }

                                                //switch_ast_node.block_items.push(Box::new(case_default));
                                                // temp_items.push(case_default);

                                                for i in 0..temp_items.len() {
                                                    let temp_node = temp_items.pop().unwrap();
                                                    // let temp_lhs = temp_node.lhs.unwrap();
                                                    // temp_node.block_items.push(temp_lhs);
                                                    switch_ast_node.block_items.push(temp_node.id);
                                                }

                                                switch_ast_node.block_items.push(case_default.id);

                                                self.ast_stack.push(switch_ast_node.id);

                                                node_map.insert(switch_ast_node.id, switch_ast_node);
                                                node_map.insert(case_default.id, case_default);
                                            }
                                        }

                                        // iteration_statement -> WHILE OPENING_BRACKET expression CLOSING_BRACKET statement
                                        195 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("iteration_statement (WHILE)"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("iteration_statement WHILE")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - iteration_statement -> WHILE OPENING_BRACKET expression CLOSING_BRACKET statement
                                            //

                                            if self.construct_ast {

                                                let expression_ast_node = self.ast_stack.pop().unwrap();
                                                let statement_ast_node = self.ast_stack.pop().unwrap();

                                                // while
                                                let mut while_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                while_ast_node.node_type = AstNodeType::While;
                                                while_ast_node.lhs = Some(statement_ast_node);
                                                while_ast_node.rhs = Some(expression_ast_node);
                                                while_ast_node.string_val = "WHILE: insert label here!".to_string();

                                                // statement
                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Statement;
                                                ast_node.lhs = Some(while_ast_node.id);

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(while_ast_node.id, while_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // iteration_statement -> DO statement WHILE OPENING_BRACKET expression CLOSING_BRACKET SEMICOLON
                                        196 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("selection_statement"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("selection_statement")).as_str());

                                            // DO
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("do"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from("do")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // WHILE
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("WHILE"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from("WHILE")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // (
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("("));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from("(")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // )
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from(")"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from(")")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // ;
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from(";"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from(";")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - iteration_statement -> DO statement WHILE OPENING_BRACKET expression CLOSING_BRACKET SEMICOLON
                                            //

                                            if self.construct_ast {

                                                let body_ast_node = self.ast_stack.pop().unwrap();
                                                let expression_ast_node = self.ast_stack.pop().unwrap();

                                                // for
                                                let mut for_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                for_ast_node.node_type = AstNodeType::DoWhile;
                                                for_ast_node.expression = Some(expression_ast_node); // condition, e.g. a < 10
                                                for_ast_node.string_val = "DO_WHILE: insert label here!".to_string();

                                                for_ast_node.block_items.push(body_ast_node);

                                                // statement
                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Statement;
                                                ast_node.lhs = Some(for_ast_node.id);

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(for_ast_node.id, for_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // iteration_statement -> FOR OPENING_BRACKET expression_statement expression_statement expression CLOSING_BRACKET statement
                                        198 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("iteration_statement (FOR)"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("iteration_statement FOR")).as_str());

                                            // take old node from stack - expression_statement - (initializer)
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack - expression_statement - (continuation predicate)
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack - expression - (post loop update)
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack - statement (body)
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - iteration_statement -> FOR OPENING_BRACKET
                                            //          expression_statement expression_statement expression CLOSING_BRACKET statement
                                            //

                                            if self.construct_ast {

                                                let body_ast_node = self.ast_stack.pop().unwrap(); // (body)
                                                // println!("body: {:?}", body_ast_node);

                                                let post_ast_node = self.ast_stack.pop().unwrap(); // (post loop update)

                                                let expression_ast_node = self.ast_stack.pop().unwrap(); // (continuation predicate)

                                                let init_ast_node = self.ast_stack.pop().unwrap(); // (initializer)
                                                // println!("init: {:?}", init_ast_node);

                                                // for
                                                let mut for_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                for_ast_node.node_type = AstNodeType::For;
                                                for_ast_node.lhs = Some(init_ast_node); // initialization, e.g.: a = 0
                                                for_ast_node.expression = Some(expression_ast_node); // condition, e.g. a < 10
                                                for_ast_node.rhs = Some(post_ast_node); // post: e.g.: a = a + 1
                                                for_ast_node.string_val = "FOR: insert label here!".to_string();

                                                for_ast_node.block_items.push(body_ast_node);

                                                // statement
                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Statement;
                                                ast_node.lhs = Some(for_ast_node.id);

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(for_ast_node.id, for_ast_node);
                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // jump_statement -> BREAK
                                        201 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("jump_statement"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("jump_statement")).as_str());

                                            // break
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("break"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", equals_node_id, String::from("break")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - jump_statement -> BREAK
                                            //

                                            if self.construct_ast {

                                                let mut break_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                break_ast_node.node_type = AstNodeType::Break;

                                                self.ast_stack.push(break_ast_node.id);

                                                node_map.insert(break_ast_node.id, break_ast_node);
                                            }
                                        }

                                        // jump_statement -> RETURN expression SEMICOLON
                                        202 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("jump_statement RETURN"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("jump_statement RETURN")).as_str());

                                            // take old node from stack - expression
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - jump_statement -> RETURN expression SEMICOLON
                                            //

                                            if self.construct_ast {

                                                let expression_ast_node = self.ast_stack.pop().unwrap();

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Return;
                                                ast_node.lhs = Some(expression_ast_node);

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // jump_statement -> RETURN SEMICOLON
                                        203 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("jump_statement RETURN"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("jump_statement RETURN")).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // AST - jump_statement -> RETURN SEMICOLON
                                            //

                                            if self.construct_ast {

                                                let mut ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                ast_node.node_type = AstNodeType::Return;

                                                self.ast_stack.push(ast_node.id);

                                                node_map.insert(ast_node.id, ast_node);
                                            }
                                        }

                                        // translation_unit -> translation_unit external_declaration
                                        204 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("translation_unit"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("translation_unit")).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // translation_unit -> external_declaration
                                        205 => {
                                            self.node_to_node("translation_unit", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // external_declaration -> function_definition
                                        206 => {
                                            self.node_to_node("external_declaration", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // external_declaration -> declaration
                                        207 => {
                                            self.node_to_node("external_declaration", found_rule.original_id, string_buffer, debug_node_stack);
                                        }

                                        // function_definition -> declaration_specifiers declarator compound_statement
                                        208 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("function_definition"));
                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, found_rule.original_id, String::from("function_definition")).as_str());

                                            // declaration_specifiers
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // declarator
                                            let declarator_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, declarator_node.id).as_str());

                                            // compound_statement
                                            let compound_statement_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, compound_statement_node.id).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            // println!("debug_node_id: {}", debug_node_id);

                                            //
                                            // AST - function_definition -> declaration_specifiers declarator compound_statement
                                            //

                                            if self.construct_ast {

                                                let mut function_definition_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                function_definition_ast_node.node_type = AstNodeType::FunctionDeclaration;
                                                // function_definition_ast_node.analyzed_data_type = a<.analyzed_data_type.clone();

                                                // // function body statement (= block)
                                                // let mut function_body_block_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                // function_body_block_ast_node.node_type = AstNodeType::Block;

                                                // function name
                                                let mut function_name: String = String::new();

                                                let mut done = false;
                                                while !done {

                                                    let sub_ast_node_id = self.ast_stack.pop().unwrap();
                                                    let sub_ast_node = node_map.get(&sub_ast_node_id).unwrap();
                                                    match sub_ast_node.node_type {

                                                        // AstNodeType::Identifier => {
                                                        //     function_name = sub_ast_node.string_val;
                                                        // }

                                                        AstNodeType::BlockItem => {
                                                            function_definition_ast_node.block_items.push(sub_ast_node.id);
                                                        }

                                                        AstNodeType::Compound => {
                                                            // // wrap statement into BlockItem
                                                            // let mut block_item_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                            // block_item_ast_node.node_type = AstNodeType::BlockItem;
                                                            // block_item_ast_node.lhs = Some(Box::new(sub_ast_node));

                                                            // function_body_block_ast_node.block_items.push(Box::new(block_item_ast_node));

                                                            // LHS contains the block, the block contains all statements as block_items
                                                            function_definition_ast_node.lhs = sub_ast_node.lhs;
                                                        }

                                                        AstNodeType::Declaration => {
                                                            // nop
                                                        }

                                                        AstNodeType::DataType => {
                                                            //
                                                            // Return Value
                                                            //

                                                            function_definition_ast_node.analyzed_data_type = sub_ast_node.analyzed_data_type.clone();

                                                            // Assumption: return-value data-type of function
                                                            // return-value data-type of function
                                                            function_definition_ast_node.rhs = Some(sub_ast_node.id);
                                                            done = true;
                                                        }

                                                        AstNodeType::Statement => {
                                                            // wrap statement into BlockItem
                                                            let mut block_item_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                            block_item_ast_node.node_type = AstNodeType::BlockItem;
                                                            block_item_ast_node.lhs = Some(sub_ast_node.id);

                                                            function_definition_ast_node.block_items.push(block_item_ast_node.id);

                                                            node_map.insert(block_item_ast_node.id, block_item_ast_node);
                                                        }

                                                        AstNodeType::FunctionDeclaration => {
                                                            if let Some(function_name_ast_node_id) = sub_ast_node.function_name_ast_node {

                                                                let function_name_ast_node = node_map.get(&function_name_ast_node_id).unwrap();
                                                                function_name = function_name_ast_node.string_val.clone();
                                                            }
                                                            //function_definition_ast_node.parameters = std::mem::take(&mut sub_ast_node.parameters);
                                                            function_definition_ast_node.parameters = sub_ast_node.parameters.clone();

                                                            // function_name = sub_ast_node.string_val;

                                                            // there should be a non-empty name for the function to call
                                                            assert!(function_name.len() > 0);
                                                        }

                                                        _ => {
                                                            function_definition_ast_node.block_items.push(sub_ast_node.id);
                                                        }
                                                    }
                                                }

                                                // // LHS contains the block, the block contains all statements as block_items
                                                // function_definition_ast_node.lhs = Some(Box::new(function_body_block_ast_node));

                                                //
                                                // function name
                                                //

                                                // there should be a non-empty name for the function to call
                                                assert!(function_name.len() > 0);

                                                let mut function_name_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
                                                function_name_ast_node.string_val = function_name.clone();
                                                function_name_ast_node.node_type = AstNodeType::Identifier;

                                                function_definition_ast_node.function_name_ast_node = Some(function_name_ast_node.id);

                                                self.ast_stack.push(function_definition_ast_node.id);

                                                node_map.insert(function_name_ast_node.id, function_name_ast_node);
                                                node_map.insert(function_definition_ast_node.id, function_definition_ast_node);
                                            }
                                        }

                                        _ => {
                                            panic!("{}", format!("Unhandled rule {:?}!\n", found_rule.original_id).as_str());
                                        }
                                    }

                                }

                                //
                                // Update the parse stack
                                //

                                // push the LHS that the rule has been reduced to
                                let t6 = ParseStackElementType::<String>::RuleElement(found_rule.lhs.clone());
                                let e6 = ParseStackElement { element_type: t6, data: String::from("") };
                                self.stack.push(e6);

                                // push the LHS that the rule has been reduced to
                                //self.stack.push(ParseStackElement::<String>::RuleElement(found_rule.lhs));
                            }

                            if debug {
                                println!(".:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.");
                                println!("[Parser::consume] Step: {}", step);
                                println!("[Parser::consume] Stack After: {:?}", self.stack);
                                println!(".:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.");
                            }

                            false
                        }

                        ParseTableCell::Accept => {
                            if debug {
                                println!("[Parser::consume] ACCEPT !!!!");
                            }

                            if debug {
                                println!(".:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.");
                                println!("[Parser::consume] Step: {}", step);
                                println!("[Parser::consume] Stack After: {:?}", self.stack);
                                println!(".:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.:.");
                            }

                            true
                        }

                        _ => {
                            panic!("[Parser::consume] NIY!");
                        }

                    }
                }
            }
        }
    }

    // passes a token (parameter: terminal_token_rule_element) to the parser.
    // The parser will now perform one or more steps until it is ready for the next token.
    // it will consume the token which completes a production rule.
    // Then it will potentially perform many reduction operations because one rule reduction
    // may lead to another and so on ...
    pub fn provide_input(&mut self,
        rule_map: &BTreeMap<usize, Rule<String>>,
        step: &mut usize,
        terminal_token_rule_element: &RuleElement<String>,
        terminal_value: &String,
        string_buffer: &mut String,
        debug_node_stack: &mut Vec::<DebugNode>,
        node_map: &mut Box<HashMap::<usize, AstNode>>
    ) -> usize {

        // let debug = true;
        let debug = false;

        let mut consumed = false;
        while !consumed {

            if debug {
                // println!("");
                println!("[provide_input] Step: {}, TerminalValue: {}", *step, terminal_value);
            }

            if self.lexer_produce_type_name {
                println!("test {:?}", terminal_token_rule_element);

                // terminal_token_rule_element = &RuleElement::Terminal(String::from("TYPE_NAME"));

                consumed = self.consume(RuleElement::Terminal(String::from("TYPE_NAME")),
                    &terminal_value,
                    &rule_map,
                    string_buffer,
                    debug_node_stack,
                    *step,
                    node_map
                );

            } else {

                consumed = self.consume(terminal_token_rule_element.clone(),
                    &terminal_value,
                    &rule_map,
                    string_buffer,
                    debug_node_stack,
                    *step,
                    node_map
            );

            }

            *step = *step + 1;
        }

        *step
    }
}

pub fn output_parse_table_to_csv(
    parse_table_string_buffer: &mut String,
    parse_table: &HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>,
    grammar_state_hashmap: &BTreeMap<usize, GrammarState<String>>,
    rules: &mut Vec::<Rule::<String>>,
    rule_ids: &mut Vec::<usize>,
    rule_map: &mut BTreeMap::<usize, Rule<String>>
) {

    let debug: bool = false;

    for i in 0..parse_table.len() {

        let parse_table_row = &parse_table[&i];

        // DEBUG - output parse table row
        // println!("{}) {:?}", i, parse_table_row);

        // write state id
        parse_table_string_buffer.push_str(format!("{}", i).as_str());

        // https://stackoverflow.com/questions/45724517/how-to-iterate-through-a-hashmap-print-the-key-value-and-remove-the-value-in-ru
        for (rule_element, parse_table_cell) in &*parse_table_row {

            // DEBUG
            println!("{:?} / {:?}", rule_element, parse_table_cell);

            parse_table_string_buffer.push_str(";");

            match rule_element {

                RuleElement::Terminal(term) => {
                    // println!("{}", term);

                    // rule element
                    parse_table_string_buffer.push_str(term);
                    parse_table_string_buffer.push_str(".");
                }

                RuleElement::NonTerminal(non_term) => {
                    // println!("{}", non_term);

                    // rule element
                    parse_table_string_buffer.push_str(non_term);
                    parse_table_string_buffer.push_str(".");
                }

                // RuleElement::Epsilon => {
                //     println!("#");

                //     // rule element
                //     parse_table_string_buffer.push_str("#");
                //     parse_table_string_buffer.push_str(".");
                // }

                RuleElement::Closure => {
                    // println!("#");

                    // rule element
                    parse_table_string_buffer.push_str("#");
                    parse_table_string_buffer.push_str(".");
                }

                _ => {
                    // println!("{:?}", rule_element);
                    panic!("test");
                }
            }

            match parse_table_cell {

                ParseTableCell::Shift(state_id) => {
                    // println!("Shift {}", state_id);

                    // operation
                    parse_table_string_buffer.push_str("S");
                    parse_table_string_buffer.push_str(format!("{}", state_id).as_str());
                }

                ParseTableCell::Reduce(rule_id) => {
                    // println!("Reduce {}", rule_id);

                    // retrieve the state
                    let state = grammar_state_hashmap.get(&i).unwrap();

                    // retrieve the rule from the state
                    let mut found_rule = Rule::<String>::new(0);

                    let mut found = false;

                    // start search with identification rules
                    for i in 0..state.identification_rules.len() {
                        if state.identification_rules[i].id == *rule_id {
                            if debug {
                                println!("[Parser::consume] rule: {:?}", state.identification_rules[i]);
                            }
                            found_rule = state.identification_rules[i].clone();
                            found = true;
                        }
                    }

                    if !found {
                        // now, finish the search with the normal rules
                        for i in 0..state.rules.len() {
                            if state.rules[i].id == *rule_id {
                                if debug {
                                    println!("[Parser::consume] rule: {:?}", state.rules[i]);
                                }
                                found_rule = state.rules[i].clone();
                                found = true;
                            }
                        }
                    }

                    if found {

                        // turn rule into it's original id
                        let current_original_rule_id = found_rule.original_id;

                        // if the rule is not already contained in the set of rule
                        if !rule_ids.contains(&current_original_rule_id) {

                            rule_map.insert(current_original_rule_id, found_rule.clone());

                            // remember rule
                            rule_ids.push(current_original_rule_id);

                            // insert rule for output later
                            rules.push(found_rule.clone());
                        }

                        // operation
                        parse_table_string_buffer.push_str("R");
                        parse_table_string_buffer.push_str(format!("{}", current_original_rule_id).as_str());

                    } else {
                        panic!("Cannot find rule!");
                    }
                }

                ParseTableCell::Accept => {
                    // println!("Accept #");

                    // operation
                    parse_table_string_buffer.push_str("A");
                    parse_table_string_buffer.push_str(format!("{}", 0).as_str());
                }

                ParseTableCell::Goto(state_id) => {
                    // println!("Goto {}", state_id);

                    // operation
                    parse_table_string_buffer.push_str("G");
                    parse_table_string_buffer.push_str(format!("{}", state_id).as_str());
                }

            }

            // parse_table_string_buffer.push_str(";");
        }

        // rule element
        parse_table_string_buffer.push_str("\n");
    }
}

pub fn read_parse_table_from_csv(filename: &str, parse_table: &mut HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>) {

    let file = File::open(filename).expect("Reading file failed!");
    let reader = BufReader::new(file);

    // DEBUG - output the lines read from the parse table file
    for line_result in reader.lines() {

        if let Ok(line) = line_result {

            // DEBUG
            // println!("{:?}", line);

            // File Format:
            //
            // <State_id>;
            // 247;COLON.R61;CLOSING_ANGULAR_BRACKET.R61;ELSE.R61;unary_operator.G79;relational_expression.G100

            let row_split: Vec<_> = line.split(';').collect();

            // DEBUG
            // println!("{:?}", row_split);

            let state_id_as_tring = row_split[0];
            let mut operations = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();

            for i in 1..row_split.len() {

                let entry_split: Vec<_> = row_split[i].split('.').collect();
                let rule_element_as_string = entry_split[0];
                let parse_table_cell_as_string = entry_split[1];

                let mut temp_rule_element = RuleElement::<String>::Terminal(String::from(""));

                // DEBUG
                // println!("{:?}", entry_split);
                // println!("{:?}", rule_element_as_string);
                // println!("{:?}", parse_table_cell_as_string);

                if rule_element_as_string == "#" {
                    temp_rule_element = RuleElement::<String>::Closure;
                } else {
                    let is_uppercase = rule_element_as_string.chars().all( |c| c.is_uppercase() || c == '_' || c == '#' );
                    if is_uppercase {
                        temp_rule_element = RuleElement::<String>::Terminal(String::from(rule_element_as_string));
                    } else {
                        temp_rule_element = RuleElement::<String>::NonTerminal(String::from(rule_element_as_string));
                    }
                }

                let mut parse_table_rule = ParseTableCell::<usize>::Accept;

                if parse_table_cell_as_string.starts_with("R") {

                    let temp = parse_table_cell_as_string[1..].parse().unwrap();
                    parse_table_rule = ParseTableCell::<usize>::Reduce(temp);

                } else if parse_table_cell_as_string.starts_with("S") {

                    let temp = parse_table_cell_as_string[1..].parse().unwrap();
                    parse_table_rule = ParseTableCell::<usize>::Shift(temp);

                } else if parse_table_cell_as_string.starts_with("G") {

                    let temp = parse_table_cell_as_string[1..].parse().unwrap();
                    parse_table_rule = ParseTableCell::<usize>::Goto(temp);

                } else {
                    parse_table_rule = ParseTableCell::<usize>::Accept;
                }

                operations.insert(temp_rule_element, parse_table_rule);
            }

            parse_table.insert(state_id_as_tring.parse().unwrap(), operations);
        }
    }
}

// DEBUG
//println!("{:?}", identifier_ast_node);

// function_declaration_ast_node.string_val = identifier_ast_node.string_val;

// let mut function_name_ast_node: AstNode = AstNode::new(AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
// function_name_ast_node.node_type = AstNodeType::Identifier;
// function_name_ast_node.string_val = identifier_ast_node.string_val;

// function_declaration_ast_node.function_name_ast_node = Some(Box::new(function_name_ast_node));
