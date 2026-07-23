#![allow(
dead_code,
unused_imports,
unused_must_use,
unused_variables,
unused_assignments
)]

use std::collections::{HashMap, HashSet, BTreeSet, BTreeMap};
use std::hash::Hash;
use std::{
    sync::atomic::{AtomicUsize, Ordering}
};
use std::fs;
use std::fs::File;

use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;

mod regex;
use crate::regex::infix_postfix_converter::InfixPostfixConverter;
use crate::regex::regex_building_block::RegexBuildingBlock;
use crate::regex::arena::Arena;
use crate::regex::arena::NodeId;
use crate::regex::arena::Node;

use crate::regex::enfa::Input;
use crate::regex::enfa::Fragment;

use crate::regex::enfa::EpsilonNfa;
use crate::regex::enfa::State;

mod parser;
use crate::parser::parser::ParseTableCell;
use crate::parser::parser::Transition;
use crate::parser::parser::Parser;
use crate::parser::parser::DebugNode;
use crate::parser::propagation::perform_propagation;
use crate::parser::first::compute_first_original;
use crate::parser::build_parse_table::build_parse_table;
use crate::parser::perform_lalr_1::perform_lalr_1;
use crate::parser::nullable_sets::compute_nullable_sets;
use crate::parser::validate_grammar::validate_grammar;
use crate::parser::print_rules::print_rules;
use crate::parser::rule::Rule;
use crate::parser::rule::RuleElement;

mod lexer;
use crate::lexer::lexer::Lexer;
use crate::lexer::lexer::IDENTIFIER_TOKEN_ID;
use crate::lexer::lexer::WHITESPACE_TOKEN_ID;
use crate::lexer::lexer::NEWLINE_TOKEN_ID;

mod example_lexers;
use crate::example_lexers::c_lexer::produce_c_lexer;

mod example_grammars;
use crate::example_grammars::c_full::produce_grammar_c_full;
use crate::example_grammars::c_full_if_else::produce_grammar_c_full_if_else;
use crate::example_grammars::c_full_if_else_2::produce_grammar_c_full_if_else_2;
use crate::example_grammars::c_full_if_else_3::produce_grammar_c_full_if_else_3;
use crate::example_grammars::c_full_if_else_4::produce_grammar_c_full_if_else_4;
use crate::example_grammars::c_full_5::produce_grammar_c_full_5;
use crate::example_grammars::left_recursive::produce_grammar_left_recursive;
use crate::example_grammars::grammar_1::produce_grammar_1;
use crate::example_grammars::grammar_2::produce_grammar_2;
use crate::example_grammars::grammar_3::produce_grammar_3;
use crate::example_grammars::grammar_4::produce_grammar_4;
use crate::example_grammars::grammar_5::produce_grammar_5;
use crate::example_grammars::grammar_6::produce_grammar_6;

use crate::RuleElement::Terminal;

mod example_input;
use crate::example_input::input::provide_sourcode_input;

// https://stackoverflow.com/questions/32935808/generate-sequential-ids-for-each-instance-of-a-struct
static RULE_COUNTER: AtomicUsize = AtomicUsize::new(0);
static STATE_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn main() {

    let debug = false;

    println!("start");

    let mut parse_table = HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>::new();

    //
    // You only need to generate the LALR_1 parser if you have not done so before
    // or if you have changed the grammar. Otherwise, the application will read the generated
    // parse table and rules from the files parse_table.txt and rule_table.txt
    //

    let generate_lalr_1 = false;
    if generate_lalr_1 {

        let mut grammar_rules = Vec::<Rule<String>>::new();

        //
        // Select one of the Grammars
        //
        
        // let g_result = produce_grammar_1(&mut grammar_rules); // has epsilon rules (wont work)
        // let g_result = produce_grammar_2(&mut grammar_rules);
        // let g_result = produce_grammar_3(&mut grammar_rules); // shows # is not propagated
        // let g_result = produce_grammar_4(&mut grammar_rules);
        // let g_result = produce_grammar_5(&mut grammar_rules);
        // let g_result = produce_grammar_6(&mut grammar_rules); // contains arrows that point backwords
        // let g_result = produce_grammar_c_full(&mut grammar_rules);
        // let g_result = produce_grammar_c_full_if_else(&mut grammar_rules);
        // let g_result = produce_grammar_c_full_if_else_2(&mut grammar_rules);
        // let g_result = produce_grammar_c_full_if_else_3(&mut grammar_rules);
        // let g_result = produce_grammar_c_full_5(&mut grammar_rules);
        // let g_result = produce_grammar_left_recursive(&mut grammar_rules);
        let g_result = produce_grammar_c_full_if_else_4(&mut grammar_rules);

        let rule_1 = g_result.0;
        let augmented_start_symbol = g_result.1;

        //
        // Validate the Grammar
        //

        validate_grammar(&mut grammar_rules);

        //
        // Print all Rules (in the order in which they use each other)
        //

        print_rules(grammar_rules.clone(), &rule_1);

        //
        // Build Nullable Set
        //

        let mut nullable = BTreeMap::<RuleElement::<String>, bool>::new();
        compute_nullable_sets(&mut grammar_rules, &mut nullable);

        //
        // Build First Set
        //

        let mut first = BTreeMap::<RuleElement::<String>, Vec::<RuleElement::<String>>>::new();
        compute_first_original(&grammar_rules, &nullable, &mut first);

        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        println!("LALR(1) generation channel algorithm starts                                      ");
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

        let mut rule_ids = Vec::<usize>::new();
        let mut rule_id_to_state_id_map = HashMap::<usize, usize>::new();
        let mut rule_channel_map = HashMap::<usize, Vec::<Transition<String>>>::new();

        let mut grammar_state_hashmap = perform_lalr_1(&rule_1, 
            &mut rule_ids, 
            &mut rule_id_to_state_id_map, 
            &mut rule_channel_map, 
            &grammar_rules, 
            &first, 
            &nullable);

        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

        //
        // Propagation Cycles
        //

        println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        perform_propagation(&rule_ids, &rule_id_to_state_id_map, &rule_channel_map, &mut grammar_state_hashmap);
        println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");

        println!("");
        println!("*********************************************************************************");
        println!("RESULT - FINISHED - READY - RESULT - FINISHED - READY - RESULT - FINISHED - READY");
        println!("*********************************************************************************");

        //
        // DEBUG - print all states for comparison with online tools (e.g. https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html)
        //

        // let debug_grammar_states = true;
        let debug_grammar_states = false;
        if debug_grammar_states {
            // DEBUG output all states - inb4 huge wall of text incoming!
            for (key, value) in &grammar_state_hashmap {
                println!("");
                println!("{} / {:?}", key, value);
                println!("");
            }
        }

        //
        // Building the Parse Table from the LALR(1) DFA
        //

        println!("");
        println!("*********************************************************************************");
        println!("Building the Parse Table from the LALR(1) DFA                                    ");
        println!("*********************************************************************************");
        build_parse_table(&mut parse_table, &mut grammar_state_hashmap, &rule_channel_map, &augmented_start_symbol, &rule_id_to_state_id_map);    
        println!("*********************************************************************************");

        // //
        // // Output Parse Table into grammar_state_hashmap file
        // //

        // // write grammar_state_hashmap to file because to pop RHS from the stack, the parser
        // // needs to know all states and rules in that state!

        // // BTreeMap<usize, GrammarState<String>>
        // // maps from state_id to GrammarState

        // for (grammar_state_id, grammar_state) in &grammar_state_hashmap {

        //     println!("");
        //     println!("{} / {:?}", grammar_state_id, grammar_state);
        //     println!("");
        // }


        let mut rule_map = BTreeMap::<usize, Rule<String>>::new();

        //
        // Output Parse Table into CSV file
        //

        let mut parse_table_string_buffer = String::new();

        let mut rule_ids = Vec::<usize>::new();
        let mut rules = Vec::<Rule::<String>>::new();

        for i in 0..parse_table.len() {

            let parse_table_row = &parse_table[&i];

            // DEBUG - output parse table row
            println!("{}) {:?}", i, parse_table_row);

            // write state id
            parse_table_string_buffer.push_str(format!("{}", i).as_str());        

            // https://stackoverflow.com/questions/45724517/how-to-iterate-through-a-hashmap-print-the-key-value-and-remove-the-value-in-ru
            for (rule_element, parse_table_cell) in &*parse_table_row {

                // DEBUG
                println!("{:?} / {:?}", rule_element, parse_table_cell);

                parse_table_string_buffer.push_str(";");

                match rule_element {

                    RuleElement::Terminal(term) => {
                        println!("{}", term);

                        // rule element
                        parse_table_string_buffer.push_str(term);
                        parse_table_string_buffer.push_str(".");
                    }

                    RuleElement::NonTerminal(non_term) => {
                        println!("{}", non_term);

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
                        println!("#");

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
                        println!("Shift {}", state_id);

                        // operation
                        parse_table_string_buffer.push_str("S");
                        parse_table_string_buffer.push_str(format!("{}", state_id).as_str());
                    }
                    ParseTableCell::Reduce(rule_id) => {
                        println!("Reduce {}", rule_id);

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
                        println!("Accept #");

                        // operation
                        parse_table_string_buffer.push_str("A");
                        parse_table_string_buffer.push_str(format!("{}", 0).as_str());
                    }
                    ParseTableCell::Goto(state_id) => {
                        println!("Goto {}", state_id);

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

        // 1. Create or overwrite the file
        let file = File::create("parse_table.txt").expect("Create file parse_table.txt failed!");
        
        // 2. Wrap the file in a BufWriter
        let mut writer = BufWriter::new(file);

        // 3. Write data
        write!(writer, "{}", parse_table_string_buffer);

        // 4. Explicitly flush the remaining data to disk
        writer.flush().expect("flush failed!");






        let mut rules_string_buffer = String::new();

        for i in 0..rules.len() {

            let temp_rule = &rules[i];

            // // rule id
            // rules_string_buffer.push_str(format!("{}", temp_rule.id).as_str());
            // rules_string_buffer.push_str(";");

            // rule original_id
            rules_string_buffer.push_str(format!("{}", temp_rule.original_id).as_str());
            // rules_string_buffer.push_str(";");

            // rule LHS
            rules_string_buffer.push_str(";");
            rules_string_buffer.push_str(format!("{:?}", temp_rule.lhs).as_str());

            // rule RHS
            for j in 0..temp_rule.rhs.len() {
                rules_string_buffer.push_str(";");
                rules_string_buffer.push_str(format!("{:?}", temp_rule.rhs[j]).as_str());
            }

            rules_string_buffer.push_str("\n");
        }

        // 1. Create or overwrite the file
        let file = File::create("rule_table.txt").expect("Create file rule_table.txt failed!");
        
        // 2. Wrap the file in a BufWriter
        let mut writer = BufWriter::new(file);

        // 3. Write data
        write!(writer, "{}", rules_string_buffer);

        // 4. Explicitly flush the remaining data to disk
        writer.flush().expect("flush failed!");
    }

    //
    // reading back the rule_map from file
    //

    println!("");
    println!("*********************************************************************************");
    println!("Reading back the rule_map from file                                              ");
    println!("*********************************************************************************");

    let mut rule_map = BTreeMap::<usize, Rule<String>>::new();

    let file = File::open("rule_table.txt").expect("Reading file failed!");
    let reader = BufReader::new(file);

    // DEBUG - output the lines read from the parse table file
    for line in reader.lines() {

        if let Ok(line) = line {
            
            // println!("{:?}", line);

            // 79;declaration_specifiers;storage_class_specifier;declaration_specifiers

            let row_split: Vec<_> = line.split(';').collect();

            // println!("{:?}", row_split);

            let rule_id_as_tring = row_split[0];
            let lhs_as_tring = row_split[1];

            let mut rule = Rule::<String>::new(rule_id_as_tring.parse().unwrap());
            rule.lhs = RuleElement::<String>::NonTerminal(String::from(lhs_as_tring));

            for i in 2..row_split.len() {

                let split_element = row_split[i];

                let mut temp_rule_element = RuleElement::<String>::Terminal(String::from(""));

                if split_element == "#" {
                    temp_rule_element = RuleElement::<String>::Closure;
                } else {
                    let is_uppercase = split_element.chars().all( |c| c.is_uppercase() || c == '_' || c == '#' );
                    if is_uppercase {
                        temp_rule_element = RuleElement::<String>::Terminal(String::from(split_element));
                    } else {
                        temp_rule_element = RuleElement::<String>::NonTerminal(String::from(split_element));
                    }
                }

                rule.rhs.push(temp_rule_element);
            }

            rule_map.insert(rule.id, rule);
        }
    }

    println!("*********************************************************************************");

    //
    // reading back the parse table from file
    //

    println!("");
    println!("*********************************************************************************");
    println!("Reading back the parse table from file                                           ");
    println!("*********************************************************************************");

    //let mut parse_table = HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>::new();
    parse_table.clear();

    let file = File::open("parse_table.txt").expect("Reading file failed!");
    let reader = BufReader::new(file);

    // DEBUG - output the lines read from the parse table file
    for line_result in reader.lines() {

        if let Ok(line) = line_result {

            // println!("{:?}", line);

            // <State_id>;
            // 247;COLON.R61;CLOSING_ANGULAR_BRACKET.R61;ELSE.R61;unary_operator.G79;relational_expression.G100

            let row_split: Vec<_> = line.split(';').collect();

            // println!("{:?}", row_split);

            let state_id_as_tring = row_split[0];
            let mut operations = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();

            for i in 1..row_split.len() {

                let entry_split: Vec<_> = row_split[i].split('.').collect();
                let rule_element_as_string = entry_split[0];
                let parse_table_cell_as_string = entry_split[1];

                let mut temp_rule_element = RuleElement::<String>::Terminal(String::from(""));

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

    println!("*********************************************************************************");

    //
    // Build the Lexer
    //

    println!("");
    println!("*********************************************************************************");
    println!("Building the Lexer (This may take some time ...)                                 ");
    println!("*********************************************************************************");

    let dfa = produce_c_lexer();

    println!("*********************************************************************************");

    //
    // Process some input
    //

    let str = provide_sourcode_input();

    // DEBUG
    println!("");
    println!("");
    println!("Input: {}", str);

    //
    // Driving the parser against input
    //

    println!("");
    println!("*********************************************************************************");
    println!("Driving the parser against input                                                 ");
    println!("*********************************************************************************");

    // https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html (do not add a rule S' -> S into the webapp)
    // https://jsmachines.sourceforge.net/machines/lalr1.html (Add augmented start rule)

    // How to drive the parser:
    //
    // The parser starts in the start state S' (= state-id 0)
    // When the parser sees a nonterminal in a state, it will consult the parse table.
    // If the ACTION is shift(x), the nonterminal is placed on to the stack and the parser enters state-id x
    // If the ACTION is a reduce(x), 
    //      - the rule is retrieved, 
    //      - the symbols on the RHS of the rule are popped from the stack
    //      - the rule's LHS is pushed onto the stack
    //      - the parser enters state-id x
    // If the ACTION is a GOTO(x), the parser enters state-id x
    // If the parser enters the ACCEPT state, then parsing stops successfully.

    // init
    let mut parser: Parser<String> = Parser::<String>::new(parse_table);
    let mut lexer: Lexer = Lexer::new(dfa);

    let mut step: usize = 1;

    let mut current_character: char = 'x';
    let mut lookahead_character: char = 'y';
    let mut has_lookahead_character = false;

    //
    // DOT Graph
    //
    // DEBUG outputting parse tree as DOT graph
    //

    let mut debug_node_string_buffer = String::from("");
    let mut debug_node_stack = Vec::<DebugNode>::new();

    //
    // Feed the input source code to the lexer
    //

    for character in str.chars() {

        current_character = lookahead_character;
        lookahead_character = character;

        // the very first iteration is here to load the lookahead character
        if !has_lookahead_character {
            has_lookahead_character = true;
            continue;
        }

        // TODO: the lookahead character is not used at all!
        // Remove it! It makes the parser loop more complicated
        lexer.consume_character(current_character, 
            lookahead_character, 
            &mut step, 
            &mut parser, 
            // &grammar_state_hashmap,
            &rule_map,
            &mut debug_node_string_buffer, 
            &mut debug_node_stack);
    }

    // consume the lookahead from the very last cycle as a normal input. Specify dummy lookahead character.
    lexer.consume_character(lookahead_character, 
        'x', 
        &mut step, 
        &mut parser, 
        // &grammar_state_hashmap,
        &mut rule_map,
        &mut debug_node_string_buffer, 
        &mut debug_node_stack);

    // let lexer_debug: bool = false;
    // if lexer_debug {
    //     println!("[LEXER] Emitting '{}'. Token-Id: {}, Token-Name: {}", token_string_buffer, dfa.states[&current_state_id].token_id, dfa.states[&current_state_id].token_name);
    //     println!("");
    // }

    // provide the last token to the parser
    lexer.parser_provide_input(&mut parser, 
        &mut step, 
        // &grammar_state_hashmap,
        &rule_map,
        &RuleElement::Terminal(lexer.dfa.states[&lexer.current_state_id].token_name.clone()), 
        &mut debug_node_string_buffer, 
        &mut debug_node_stack);

    // provide EOI (End of Input) to the parser
    lexer.parser_provide_input(&mut parser, 
        &mut step, 
        // &grammar_state_hashmap,
        &rule_map,
        &RuleElement::Closure, 
        &mut debug_node_string_buffer, 
        &mut debug_node_stack);

    // print the parse tree to a dot file for online debugging (https://dreampuf.github.io/GraphvizOnline)
    // let output_parse_tree_as_dot_to_console: bool = true;
    let output_parse_tree_as_dot_to_console: bool = false;
    if output_parse_tree_as_dot_to_console {

        println!("");
        println!("https://dreampuf.github.io/GraphvizOnline");
        println!("");
        println!("digraph {{");
        print!("{}", debug_node_string_buffer);
        println!("}}");
        println!("");
    }

    let output_parse_tree_as_dot_to_file: bool = true;
    if output_parse_tree_as_dot_to_file {

        // 1. Create or overwrite the file
        let file = File::create("parse_tree.dot").expect("Create file failed!");
        
        // 2. Wrap the file in a BufWriter
        let mut writer = BufWriter::new(file);

        // 3. Write data
        write!(writer, "{}", "digraph {{");
        write!(writer, "{}", debug_node_string_buffer);
        write!(writer, "{}", "}}");

        // 4. Explicitly flush the remaining data to disk
        writer.flush().expect("flush failed!");
    }
    
    println!("end");
}