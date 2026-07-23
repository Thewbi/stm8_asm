use std::collections::HashMap;
use std::collections::BTreeMap;

use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;

use crate::parser::rule::Rule;
use crate::parser::rule::RuleElement;

use crate::parser::parser::ParseStackElementType::StateId;

use crate::parser::grammar_state::GrammarState;

use std::{
    sync::atomic::{AtomicUsize, Ordering}
};

pub struct Transition<T>(pub usize, pub RuleElement<T>);

static DEBUG_NODE_COUNTER: AtomicUsize = AtomicUsize::new(0);

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

    // TYPEDEF HANDLING
    pub typedef_found: bool,
    pub last_type_specifier: String,
    pub last_source_type: String,
    pub is_typedef_active: bool,
    pub lexer_produce_type_name: bool,

    // Type table
    pub defined_types: Vec::<String>,
}

impl Parser<String> {

    pub fn new(parse_table_param: HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>) -> Self {

        let mut parser = Parser {
            parse_table: parse_table_param,
            stack: Vec::<ParseStackElement<String>>::new(),
            collapse_nodes: false,

            // TYPEDEF HANDLING
            typedef_found: false,
            last_type_specifier: String::new(),
            last_source_type: String::new(),
            is_typedef_active: false,
            lexer_produce_type_name: false,

            // Types
            defined_types: Vec::<String>::new(),
        };

        let t1 = ParseStackElementType::<String>::StateId(0);
        let e1 = ParseStackElement { element_type: t1, data: String::from("") };
        parser.stack.push(e1);

        parser
    }

    // removes the current node from the stack and replaces it by a new node, inserting a transition line and a line for the new node.
    pub fn node_to_node(&mut self,
        label: &str,
        string_buffer: &mut String,
        debug_node_stack: &mut Vec::<DebugNode>) {

        // leave this function empty for a compact Tree which could be an AST.
        // Uncomment the code for a full parse tree that contains all production rules!

        if label == "point_t" {
            println!("Test");
        }

        if !self.collapse_nodes {

            // create new node id
            // create new node with node id and label
            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
            let debug_node = DebugNode::new(debug_node_id, String::from(label));

            // print new node into string buffer. e.g.    0 [label="test"]
            string_buffer.push_str(format!("{:?} [label=\"{:?} {}\"]\n", debug_node_id, debug_node_id, String::from(label)).as_str());

            // take old node from stack
            let old_debug_node = debug_node_stack.pop().unwrap();

            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
            string_buffer.push_str(format!(" {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

            // push new node to stack
            debug_node_stack.push(debug_node);
        }
        
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
        // grammar_state_hashmap: &BTreeMap<usize, GrammarState<String>>,
        rule_map: &BTreeMap<usize, Rule<String>>,
        string_buffer: &mut String,
        debug_node_stack: &mut Vec::<DebugNode>,
        step: usize) -> bool {

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
                            // println!("[Parser::consume] reducing rule_id: {}", rule_id);

                            // println!("[Parser::consume] rule_map: {:?}", rule_map);

                            //let mut found_rule = Rule::<String>::new(0);

                            let found = rule_map.contains_key(&rule_id);

                            // TODO retrieve rule by id
                            let found_rule = rule_map.get(&rule_id).unwrap();

                            /*
                            let state = grammar_state_hashmap.get(&current_state_id).unwrap();

                            // DEBUG
                            // println!("[Parser::consume] reduce State: {:?}, rule_id: {:?}", state, rule_id);

                            let mut found_rule = Rule::<String>::new(0);

                            let mut found = false;
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
                            */

                            if !found {
                                panic!("[Parser::consume] Rule not found!");
                            } else {
                                if debug {
                                    println!("[Parser::consume] rule: {:?}", found_rule);
                                }

                                // if debug {
                                    print!("[Parser::consume()] REDUCING RULE: ");
                                    found_rule.print_rule_simple();
                                    println!("");
                                // }

                                
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

                                        // translation_unit -> translation_unit external_declaration
                                        204 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("translation_unit"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("translation_unit")).as_str());



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
                                            self.node_to_node("translation_unit", string_buffer, debug_node_stack);
                                        }

                                        // external_declaration -> function_definition
                                        206 => {
                                            self.node_to_node("external_declaration", string_buffer, debug_node_stack);
                                        }

                                        // external_declaration -> declaration
                                        207 => {
                                            self.node_to_node("external_declaration", string_buffer, debug_node_stack);
                                        }

                                        // function_definition -> declaration_specifiers declarator compound_statement
                                        208 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("function_definition"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("function_definition")).as_str());




                                            // declaration_specifiers
                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());


                                            
                                            

                                            // declarator
                                            // take old node from stack
                                            let declarator_node = debug_node_stack.pop().unwrap();
                                            // if let Some(declarator_node) = debug_node_stack.pop() {
                                                // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                                string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, declarator_node.id).as_str());
                                            // }



                                            // compound_statement
                                            // take old node from stack
                                            let compound_statement_node = debug_node_stack.pop().unwrap();
                                            // if let Some(compound_statement_node) = debug_node_stack.pop() {
                                                // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                                string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, compound_statement_node.id).as_str());
                                            // }




                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // declaration_specifiers -> type_specifier declaration_specifiers
                                        81 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("declaration_specifiers"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", debug_node_id, debug_node_id, String::from("declaration_specifiers")).as_str());




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
                                        }

                                        // declaration_specifiers -> type_specifier
                                        82 => {
                                            // // create new node id
                                            // // create new node with node id and label
                                            // let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            // let debug_node = DebugNode::new(debug_node_id, String::from("declaration_specifiers"));

                                            // // print new node into string buffer. e.g.    0 [label="test"]
                                            // string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("declaration_specifiers")).as_str());

                                            // // take old node from stack
                                            // let old_debug_node = debug_node_stack.pop().unwrap();

                                            // // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            // string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // // push new node to stack
                                            // debug_node_stack.push(debug_node);
                                            self.node_to_node("declaration_specifiers", string_buffer, debug_node_stack);
                                        }

                                        // declaration_specifiers -> storage_class_specifier declaration_specifiers
                                        79 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("declaration_specifiers"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", debug_node_id, debug_node_id, String::from("declaration_specifiers")).as_str());



                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());


                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());



                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                            //
                                            // TYPEDEF HANDLING
                                            //

                                            if self.typedef_found {

                                                // TODO: 
                                                // insert a new type mapping into the types table
                                                self.defined_types.push(self.last_type_specifier.clone());

                                                // reset
                                                self.typedef_found = false;
                                                self.last_type_specifier = String::new();
                                                self.last_source_type = String::new();
                                            }
                                        }

                                        // storage_class_specifier -> TYPEDEF
                                        89 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("storage_class_specifier"));

                                            debug_node_stack.push(debug_node);

                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", debug_node_id, debug_node_id, String::from("storage_class_specifier")).as_str());



                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("TYPEDEF"));
                                            // debug_node_stack.push(debug_node);
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", type_node_id, type_node_id, String::from("TYPEDEF")).as_str());


                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());


                                            //
                                            // TYPEDEF HANDLING
                                            //

                                            self.typedef_found = true;
                                            self.is_typedef_active = true;
                                        }

                                        // type_specifier -> VOID
                                        94 => {
                                            // // push node onto stack
                                            // let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            // let debug_node = DebugNode::new(debug_node_id, String::from("type_specifier VOID"));

                                            // // print node into string buffer. e.g.    0 [label="test"]
                                            // string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("type_specifier VOID")).as_str());
                                        
                                            // debug_node_stack.push(debug_node);

                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("type_specifier"));

                                            debug_node_stack.push(debug_node);

                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("type_specifier")).as_str());



                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("VOID"));
                                            // debug_node_stack.push(debug_node);
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", type_node_id, String::from("VOID")).as_str());


                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());
                                        }

                                        // type_specifier -> CHAR

                                        // type_specifier -> SHORT

                                        // type_specifier -> INT
                                        97 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("type_specifier"));

                                            debug_node_stack.push(debug_node);

                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("type_specifier")).as_str());



                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("INT"));
                                            // debug_node_stack.push(debug_node);
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", type_node_id, String::from("INT")).as_str());


                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());
                                        
                                            //
                                            // TYPEDEF HANDLING
                                            //

                                            if self.is_typedef_active {
                                                self.lexer_produce_type_name = true;
                                                self.is_typedef_active = false;
                                            }

                                            self.last_source_type = String::from("int");
                                        }

                                        // type_specifier -> LONG

                                        // type_specifier -> FLOAT
                                        99 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("type_specifier"));

                                            debug_node_stack.push(debug_node);

                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("type_specifier")).as_str());



                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("FLOAT"));
                                            // debug_node_stack.push(debug_node);
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", type_node_id, String::from("FLOAT")).as_str());


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
                                        }

                                        // type_specifier -> DOUBLE

                                        // type_specifier -> SIGNED

                                        // type_specifier -> UNSIGNED

                                        // type_specifier -> struct_or_union_specifier
                                        103 => {
                                            self.node_to_node("type_specifier", string_buffer, debug_node_stack);
                                        }

                                        // type_specifier -> enum_specifier

                                        // type_specifier -> TYPE_NAME
                                        105 => {
                                            // retrieve terminal TYPE_NAME
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                println!("[Parser::consume()] [108] TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                            }

                                            // let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            // let debug_node = DebugNode::new(debug_node_id, format!("primary_expression '{}'\n", value));
                                            // debug_node_stack.push(debug_node);
                                            // string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, format!("primary_expression '{}'", value)).as_str());


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
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("struct_or_union_specifier"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", debug_node_id, debug_node_id, String::from("struct_or_union_specifier")).as_str());


                                            // struct_or_union
                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());



                                            // struct_or_union
                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());


                                            // // IDENTIFIER
                                            // let mut value = String::from("");
                                            // for terminal_rev in rule_reverse.iter().rev() {
                                            //     value = terminal_rev.clone().unwrap().data;
                                            //     println!("TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                            // }


                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // struct_or_union_specifier -> struct_or_union IDENTIFIER
                                        108 => {

                                            // retrieve terminal IDENTIFIER
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                println!("[Parser::consume()] [108] TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                                println!("");
                                            }

                                            // let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            // let debug_node = DebugNode::new(debug_node_id, format!("primary_expression '{}'\n", value));
                                            // debug_node_stack.push(debug_node);
                                            // string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, format!("primary_expression '{}'", value)).as_str());


                                            // create new node
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("struct_or_union_specifier"));
                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", debug_node_id, debug_node_id, String::from("struct_or_union_specifier")).as_str());


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
                                        }

                                        // struct_or_union -> STRUCT
                                        109 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("struct_or_union"));

                                            debug_node_stack.push(debug_node);

                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", debug_node_id, debug_node_id, String::from("struct_or_union")).as_str());



                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("STRUCT"));
                                            // debug_node_stack.push(debug_node);
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", type_node_id, type_node_id, String::from("STRUCT")).as_str());


                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());
                                        }

                                        // struct_declaration_list -> struct_declaration_list struct_declaration
                                        111 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("struct_declaration_list"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", debug_node_id, debug_node_id, String::from("struct_declaration_list")).as_str());


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
                                            self.node_to_node("struct_declaration_list", string_buffer, debug_node_stack);
                                        }

                                        // struct_declaration -> specifier_qualifier_list struct_declarator_list SEMICOLON
                                        113 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("struct_declaration"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{} {}\"]\n", debug_node_id, debug_node_id, String::from("struct_declaration")).as_str());


                                            // specifier_qualifier_list
                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // struct_declarator_list
                                            // take old node from stack
                                            let declarator_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, declarator_node.id).as_str());


                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // specifier_qualifier_list -> type_specifier
                                        115 => {
                                            self.node_to_node("specifier_qualifier_list", string_buffer, debug_node_stack);
                                        }

                                        // struct_declarator_list -> struct_declarator
                                        118 => {
                                            self.node_to_node("struct_declarator_list", string_buffer, debug_node_stack);
                                        }

                                        // struct_declarator -> declarator
                                        120 => {
                                            self.node_to_node("struct_declarator", string_buffer, debug_node_stack);
                                        }

                                        // declarator -> pointer direct_declarator
                                        132 => {
                                            
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("declarator"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("declarator")).as_str());





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

                                        }

                                        // declarator -> direct_declarator
                                        133 => {
                                            self.node_to_node("declarator", string_buffer, debug_node_stack);
                                        }

                                        // direct_declarator -> IDENTIFIER
                                        134 => {
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                println!("TerminalValue: '{}'", value);
                                            }

                                            // let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            // let debug_node = DebugNode::new(debug_node_id, format!("direct_declarator '{}'\n", value));
                                            // debug_node_stack.push(debug_node);
                                            // string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, format!("direct_declarator '{}'", value)).as_str());
                                        



                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("direct_declarator"));

                                            debug_node_stack.push(debug_node);

                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("direct_declarator")).as_str());



                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from(value.clone()));
                                            // debug_node_stack.push(debug_node);
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", type_node_id, String::from(value.clone())).as_str());


                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());



                                            // //
                                            // // TYPEDEF HANDLING
                                            // //

                                            // // Store the type_specifier so that if a typedef is detected later,
                                            // // the parser remembers the name of the newly created type
                                            // self.last_type_specifier = value.clone();
                                        }

                                        // direct_declarator -> direct_declarator OPENING_ANGULAR_BRACKET CLOSING_ANGULAR_BRACKET
                                        137 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("direct_declarator"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("direct_declarator")).as_str());

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
                                        }

                                        // direct_declarator -> direct_declarator OPENING_BRACKET parameter_type_list CLOSING_BRACKET
                                        138 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("direct_declarator"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("direct_declarator")).as_str());





                                            // take old node from stack
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
                                            


                                            // take old node from stack
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
                                        }

                                        // direct_declarator -> direct_declarator OPENING_BRACKET CLOSING_BRACKET
                                        140 => {
                                            self.node_to_node("direct_declarator", string_buffer, debug_node_stack);
                                        }

                                        // pointer -> ASTERISK
                                        141 => {
                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("pointer"));

                                            debug_node_stack.push(debug_node);

                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("pointer")).as_str());



                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, String::from("*"));
                                            // debug_node_stack.push(debug_node);
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", type_node_id, String::from("*")).as_str());


                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());
                                        }

                                        // parameter_type_list -> parameter_list
                                        147 => {
                                            self.node_to_node("parameter_type_list", string_buffer, debug_node_stack);
                                        }

                                        // parameter_list -> parameter_declaration
                                        149 => {
                                            self.node_to_node("parameter_list", string_buffer, debug_node_stack);
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
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("parameter_declaration"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("parameter_declaration")).as_str());

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
                                        }

                                        // compound_statement -> OPENING_CURLY_BRACKET declaration_or_statement_list CLOSING_CURLY_BRACKET
                                        184 => {

                                            // TEST THIS

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("compound_statement"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("compound_statement")).as_str());



                                            // declaration_or_statement_list
                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // compound_statement -> OPENING_CURLY_BRACKET CLOSING_CURLY_BRACKET
                                        185 => {
                                            //debug_node_stack.pop();

                                            // push a dummy compound statement otherwise the stack is empty and the other rule code crashes
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("NOP"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("NOP")).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // declaration_or_statement_list -> declaration declaration_or_statement_list
                                        186 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("declaration_or_statement_list"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("declaration_or_statement_list")).as_str());


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
                                        }

                                        // declaration_or_statement_list -> declaration
                                        187 => {
                                            self.node_to_node("declaration_or_statement_list", string_buffer, debug_node_stack);
                                        }

                                        // declaration_or_statement_list -> statement declaration_or_statement_list
                                        188 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("declaration_or_statement_list"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("declaration_or_statement_list")).as_str());


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
                                        }

                                        // declaration_or_statement_list -> statement
                                        189 => {
                                            self.node_to_node("declaration_or_statement_list", string_buffer, debug_node_stack);
                                        }

                                        // constant_expression -> conditional_expression
                                        76 => {
                                            self.node_to_node("constant_expression", string_buffer, debug_node_stack);
                                        }

                                        // declaration -> declaration_specifiers init_declarator_list SEMICOLON
                                        77 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("declaration"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("declaration")).as_str());



                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());


                                            // take old node from stack
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
                                        }

                                        // declaration -> declaration_specifiers SEMICOLON
                                        78 => {
                                            self.node_to_node("declaration", string_buffer, debug_node_stack);
                                        }

                                        // init_declarator_list -> init_declarator COMMA init_declarator_list
                                        85 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("init_declarator"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("init_declarator")).as_str());





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
                                            self.node_to_node("init_declarator_list", string_buffer, debug_node_stack);
                                        }

                                        // init_declarator -> declarator EQUALS_SIGN initializer
                                        87 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("init_declarator"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("init_declarator")).as_str());





                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();

                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());


                                            // =
                                            // create new node id
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from("("));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from("=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());
                                            


                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();

                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());



                                            // // )
                                            // // create new node id
                                            // // create new node with node id and label
                                            // let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            // let comma_node = DebugNode::new(comma_node_id, String::from(")"));
                                            // string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from(")")).as_str());
                                            // string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());




                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // init_declarator -> declarator
                                        88 => {
                                            self.node_to_node("init_declarator", string_buffer, debug_node_stack);
                                        }

                                        // initializer -> assignment_expression
                                        170 => {
                                            self.node_to_node("initializer", string_buffer, debug_node_stack);
                                        }

                                        // initializer -> OPENING_CURLY_BRACKET initializer_list CLOSING_CURLY_BRACKET
                                        171 => {
                                            //self.node_to_node("initializer", string_buffer, debug_node_stack);

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("init_declarator"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("init_declarator")).as_str());


                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();

                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());


                                            // {
                                            // create new node id
                                            // create new node with node id and label
                                            let comma_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let comma_node = DebugNode::new(comma_node_id, String::from("{"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", comma_node_id, String::from("{")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, comma_node.id).as_str());
                                            

                                            // }
                                            // create new node id
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
                                            self.node_to_node("initializer_list", string_buffer, debug_node_stack);
                                        }

                                        // initializer_list -> initializer COMMA initializer_list
                                        174 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("init_declarator"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("init_declarator")).as_str());


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

                                        // statement -> compound_statement
                                        176 => {
                                            self.node_to_node("statement", string_buffer, debug_node_stack);
                                        }

                                        // statement -> expression_statement
                                        177 => {
                                            self.node_to_node("statement", string_buffer, debug_node_stack);
                                        }

                                        // statement -> selection_statement
                                        178 => {
                                            self.node_to_node("statement", string_buffer, debug_node_stack);
                                        }

                                        // statement -> iteration_statement
                                        179 => {
                                            self.node_to_node("statement", string_buffer, debug_node_stack);
                                        }

                                        // statement -> jump_statement
                                        180 => {
                                            self.node_to_node("statement", string_buffer, debug_node_stack);
                                        }

                                        // expression_statement -> expression SEMICOLON
                                        190 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("expression_statement"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("expression_statement")).as_str());



                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();

                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
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
                                        }

                                        // expression_statement -> SEMICOLON
                                        191 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("expression_statement"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("expression_statement")).as_str());


                                            // ;
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from(";"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", equals_node_id, String::from(";")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());


                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // selection_statement -> IF OPENING_BRACKET expression CLOSING_BRACKET statement
                                        192 => {

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
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from("if")).as_str());
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
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from("if")).as_str());
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
                                        }

                                        // expression -> assignment_expression
                                        74 => {
                                            self.node_to_node("expression", string_buffer, debug_node_stack);
                                        }

                                        // assignment_expression -> unary_expression assignment_operator assignment_expression
                                        61 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("assignment_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("assignment_expression")).as_str());



                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());


                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // assignment_expression -> conditional_expression
                                        62 => {
                                            self.node_to_node("assignment_expression", string_buffer, debug_node_stack);
                                        }

                                        // postfix_expression -> postfix_expression DOT IDENTIFIER
                                        11 => {

                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                println!("TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                            }


                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("postfix_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("postfix_expression")).as_str());


                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();

                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());



                                            // .
                                            // create new node id
                                            // create new node with node id and label
                                            let inc_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let inc_node = DebugNode::new(inc_node_id, String::from("."));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", inc_node_id, String::from(".")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, inc_node.id).as_str());


                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, value.clone());
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", type_node_id, value.clone()).as_str());

                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());


                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // unary_expression -> postfix_expression
                                        17 => {
                                            self.node_to_node("unary_expression", string_buffer, debug_node_stack);
                                        }

                                        // cast_expression -> unary_expression
                                        30 => {
                                            self.node_to_node("cast_expression", string_buffer, debug_node_stack);
                                        }

                                        // postfix_expression -> primary_expression
                                        7 => {
                                            self.node_to_node("postfix_expression", string_buffer, debug_node_stack);
                                        }

                                        // postfix_expression -> postfix_expression OPENING_BRACKET argument_expression_list CLOSING_BRACKET
                                        10 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("postfix_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("postfix_expression")).as_str());

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
                                        }

                                        // postfix_expression -> postfix_expression INC_OP
                                        13 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("postfix_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("postfix_expression")).as_str());


                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();

                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());



                                            // ++
                                            // create new node id
                                            // create new node with node id and label
                                            let inc_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let inc_node = DebugNode::new(inc_node_id, String::from("++"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", inc_node_id, String::from("++")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, inc_node.id).as_str());


                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // postfix_expression -> postfix_expression DEC_OP
                                        14 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("postfix_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("postfix_expression")).as_str());


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
                                        }

                                        // argument_expression_list -> assignment_expression
                                        15 => {
                                            self.node_to_node("argument_expression_list", string_buffer, debug_node_stack);
                                        }

                                        // argument_expression_list -> argument_expression_list COMMA assignment_expression
                                        16 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("argument_expression_list"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("argument_expression_list")).as_str());




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
                                        }

                                        // primary_expression -> IDENTIFIER
                                        1 => {

                                            // TODO: put new node on the stack
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                println!("TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                            }

                                            // let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            // let debug_node = DebugNode::new(debug_node_id, format!("primary_expression '{}'\n", value));
                                            // debug_node_stack.push(debug_node);
                                            // string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, format!("primary_expression '{}'", value)).as_str());
                                        
                                            
                                            

                                            // create new node
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("primary_expression"));

                                            

                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("primary_expression")).as_str());



                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, value.clone());
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", type_node_id, value.clone()).as_str());

                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());

                                            debug_node_stack.push(debug_node);
                                        }

                                        // primary_expression -> HEX_NUMERIC
                                        2 => {

                                            // TODO: put new node on the stack
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                println!("TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                            }

                                            // let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            // let debug_node = DebugNode::new(debug_node_id, format!("primary_expression '{}'\n", value));
                                            // debug_node_stack.push(debug_node);
                                            // string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, format!("primary_expression '{}'", value)).as_str());
                                        
                                            
                                            

                                            // push node onto stack
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("primary_expression"));

                                            debug_node_stack.push(debug_node);

                                            // print node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("primary_expression")).as_str());



                                            let type_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let type_node = DebugNode::new(type_node_id, value.clone());
                                            // debug_node_stack.push(debug_node);
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", type_node_id, value.clone()).as_str());


                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, type_node.id).as_str());
                                        }

                                        // primary_expression -> NUMERIC
                                        3 => {
                                            // TODO: put new node on the stack
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                println!("TerminalValue: '{}'", value);
                                            }

                                            // do not pop from the stack as this is a leave for a int numeric literal value

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("primary_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, value).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // primary_expression -> FLOAT_NUMERIC
                                        4 => {
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                println!("TerminalValue: '{}'", value);
                                            }

                                            // do not pop from the stack as this is a leave for a int numeric literal value

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("primary_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, value).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // primary_expression -> STRING_LITERAL
                                        5 => {
                                            let mut value = String::from("");
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                value = terminal_rev.clone().unwrap().data;
                                                println!("TerminalValue: '{}'", value);
                                            }

                                            // do not pop from the stack as this is a leave for a int numeric literal value

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("primary_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label={}]\n", debug_node_id, value).as_str());

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // primary_expression -> OPENING_BRACKET expression CLOSING_BRACKET
                                        6 => {
                                            self.node_to_node("primary_expression", string_buffer, debug_node_stack);
                                        }

                                        // assignment_operator -> EQUALS_SIGN
                                        63 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("assignment_operator"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("assignment_operator")).as_str());


                                            // =
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("="));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", equals_node_id, String::from("=")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());


                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // conditional_expression -> logical_or_expression
                                        60 => {
                                            self.node_to_node("conditional_expression", string_buffer, debug_node_stack);
                                        }

                                        // logical_or_expression -> logical_and_expression
                                        58 => {
                                            self.node_to_node("logical_or_expression", string_buffer, debug_node_stack);
                                        }

                                        // logical_and_expression -> inclusive_or_expression
                                        56 => {
                                            self.node_to_node("logical_and_expression", string_buffer, debug_node_stack);
                                        }

                                        // inclusive_or_expression -> exclusive_or_expression
                                        54 => {
                                            self.node_to_node("inclusive_or_expression", string_buffer, debug_node_stack);
                                        }

                                        // exclusive_or_expression -> and_expression
                                        52 => {
                                            self.node_to_node("exclusive_or_expression", string_buffer, debug_node_stack);
                                        }

                                        // and_expression -> equality_expression
                                        50 => {
                                            self.node_to_node("and_expression", string_buffer, debug_node_stack);
                                        }

                                        // equality_expression -> relational_expression
                                        48 => {
                                            self.node_to_node("equality_expression", string_buffer, debug_node_stack);
                                        }

                                        // relational_expression -> shift_expression LT relational_expression
                                        41 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("relational_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("relational_expressions")).as_str());




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
                                        }

                                        // relational_expression -> shift_expression LE_OP relational_expression
                                        43 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("relational_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("relational_expressions")).as_str());




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
                                        }

                                        // relational_expression -> shift_expression
                                        45 => {
                                            self.node_to_node("relational_expression", string_buffer, debug_node_stack);
                                        }

                                        // shift_expression -> additive_expression
                                        40 => {
                                            self.node_to_node("shift_expression", string_buffer, debug_node_stack);
                                        }

                                        // additive_expression -> multiplicative_expression PLUS additive_expression
                                        35 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("additive_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("additive_expression")).as_str());

                                            // take old node from stack
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




                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();

                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());




                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // additive_expression -> multiplicative_expression MINUS additive_expression
                                        36 => {
                                            // TODO: put new node on the stack
                                            for terminal_rev in rule_reverse.iter().rev() {
                                                println!("TerminalValue: '{}'", terminal_rev.clone().unwrap().data);
                                            }

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("additive_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("additive_expression")).as_str());

                                            // take old node from stack
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




                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();

                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());



                                            
                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // additive_expression -> multiplicative_expression
                                        37 => {
                                            self.node_to_node("additive_expression", string_buffer, debug_node_stack);
                                        }

                                        // multiplicative_expression -> cast_expression ASTERISK multiplicative_expression
                                        31 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("additive_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("additive_expression")).as_str());

                                            // take old node from stack
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




                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();

                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());



                                            
                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // multiplicative_expression -> cast_expression SLASH multiplicative_expression
                                        32 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("multiplicative_expression"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("multiplicative_expression")).as_str());




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
                                        }

                                        // multiplicative_expression -> cast_expression
                                        34 => {
                                            self.node_to_node("multiplicative_expression", string_buffer, debug_node_stack);
                                        }

                                        // jump_statement -> RETURN expression SEMICOLON
                                        202 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("jump_statement"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("jump_statement RETURN")).as_str());



                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();

                                            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());



                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // iteration_statement -> WHILE OPENING_BRACKET expression CLOSING_BRACKET statement
                                        195 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("iteration_statement WHILE"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("iteration_statement WHILE")).as_str());



                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());




                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // iteration_statement -> FOR OPENING_BRACKET expression_statement expression_statement expression CLOSING_BRACKET statement
                                        198 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("iteration_statement FOR"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("iteration_statement FOR")).as_str());



                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());



                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // labeled_statement -> CASE constant_expression COLON statement
                                        182 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("labeled_statement"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("labeled_statement")).as_str());



                                            // CASE
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("case"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from("case")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());



                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());


                                            // COLON
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from(":"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from(":")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());



                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());






                                            // push new node to stack
                                            debug_node_stack.push(debug_node);

                                        }

                                        // statement -> labeled_statement
                                        175 => {
                                            self.node_to_node("statement", string_buffer, debug_node_stack);
                                        }

                                        // jump_statement -> BREAK
                                        201 => {
                                            // self.node_to_node("jump_statment", string_buffer, debug_node_stack);
                                            
                                            
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("jump_statement"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("jump_statement")).as_str());


                                            // break
                                            // create new node id
                                            // create new node with node id and label
                                            let equals_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let equals_node = DebugNode::new(equals_node_id, String::from("break"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", equals_node_id, String::from("break")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, equals_node.id).as_str());


                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // labeled_statement -> DEFAULT COLON statement
                                        183 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("labeled_statement"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("labeled_statement")).as_str());



                                            // DEFAULT
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("default"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from("default")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());



                                            


                                            // COLON
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from(":"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from(":")).as_str());
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, lessthan_node.id).as_str());



                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());



                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // selection_statement -> SWITCH OPENING_BRACKET expression CLOSING_BRACKET statement
                                        194 => {

                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("selection_statement"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("selection_statement")).as_str());



                                            // SWITCH
                                            // create new node id
                                            // create new node with node id and label
                                            let lessthan_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let lessthan_node = DebugNode::new(lessthan_node_id, String::from("switch"));
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", lessthan_node_id, String::from("switch")).as_str());
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




                                            // take old node from stack
                                            let old_debug_node = debug_node_stack.pop().unwrap();
                                            string_buffer.push_str(format!("  {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());


                                            

                                            // push new node to stack
                                            debug_node_stack.push(debug_node);
                                        }

                                        // iteration_statement -> DO statement WHILE OPENING_BRACKET expression CLOSING_BRACKET SEMICOLON
                                        196 => {
                                            // create new node id
                                            // create new node with node id and label
                                            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                            let debug_node = DebugNode::new(debug_node_id, String::from("selection_statement"));

                                            // print new node into string buffer. e.g.    0 [label="test"]
                                            string_buffer.push_str(format!("{:?} [label=\"{}\"]\n", debug_node_id, String::from("selection_statement")).as_str());



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
                            println!("[Parser::consume] ACCEPT !!!!");

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
        // grammar_state_hashmap: &BTreeMap<usize, GrammarState<String>>,
        rule_map: &BTreeMap<usize, Rule<String>>,
        step: &mut usize, 
        terminal_token_rule_element: &RuleElement<String>,
        terminal_value: &String,
        string_buffer: &mut String,
        debug_node_stack: &mut Vec::<DebugNode>) -> usize {

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
                    // &grammar_state_hashmap,
                    &rule_map,
                    string_buffer, 
                    debug_node_stack,
                    *step);

            } else {

                consumed = self.consume(terminal_token_rule_element.clone(), 
                    &terminal_value, 
                    // &grammar_state_hashmap,
                    &rule_map,
                    string_buffer, 
                    debug_node_stack,
                    *step);

            }

            *step = *step + 1;
        }

        *step
    }
}