// filename: main_lexeme_test.rs

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
use crate::regex::enfa::FragmentStack;
use crate::regex::enfa::recurse_postfix_build_fragment_stack;
use crate::regex::enfa::enfa_copy;
use crate::regex::enfa::enfa_to_dot_directed_graph;
use crate::regex::enfa::enfa_to_dfa;

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
use crate::parser::grammar_state::GrammarState;

mod lexer;
use crate::lexer::lexer::Lexer;
use crate::lexer::lexer::IDENTIFIER_TOKEN_ID;
use crate::lexer::lexer::WHITESPACE_TOKEN_ID;
use crate::lexer::lexer::NEWLINE_TOKEN_ID;

mod example_lexers;
use crate::example_lexers::c_lexer::produce_c_lexer;
use crate::example_lexers::common::add_token_definition;

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

    //
    // The commented section is a fast way to test the lexer
    //

/**/
    //
    // Phase 0 - 
    //

    let mut combined_fragment = Fragment::new(RegexBuildingBlock::Or);

    //
    // Pre-Build alphabet
    //

    // complete alphabet has to be known in advance
    let mut alphabet = HashSet::<RegexBuildingBlock>::new();

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('a'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('A'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('b'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('B'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('c'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('C'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('d'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('D'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('e'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('E'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('f'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('F'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('g'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('G'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('h'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('H'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('i'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('I'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('j'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('J'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('k'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('K'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('l'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('L'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('m'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('M'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('n'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('N'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('o'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('O'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('p'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('P'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('q'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('Q'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('r'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('R'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('s'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('S'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('t'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('T'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('u'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('U'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('v'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('V'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('w'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('W'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('x'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('X'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('y'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('Y'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('z'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('Z'));

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('0'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('1'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('2'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('3'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('4'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('5'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('6'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('7'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('8'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('9'));

    // alphabet.insert(RegexBuildingBlock::CharacterLiteral(' ')); // WHITESPACE, SPACE

    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('_'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('<'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('>'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('{'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('}'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('('));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral(')'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('['));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral(']'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('+'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('-'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('*'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('/'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('%'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('&'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('^')); // Used in regex as NOT operator
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('|'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('!'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral(';'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral(','));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('~'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('?'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('.'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('='));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('"'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral(':'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('\\'));

    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('\n'));

    //
    // Phase 1 - build all regexes
    //

    //
    // identifier

    // provide a regex in infix notation and let the converter produce a postfix notation
    // The result is stored within the state of the converter instance, this is why the converter can be reset
    let mut converter = InfixPostfixConverter::new();

    //
    // string_literal (token-id: 610)
    // converter.infix_to_postfix("\"(a|A|b|B|c|C|d|D)+\"");
    // converter.infix_to_postfix("\"^[\",\"]+\"");
    // converter.infix_to_postfix("^a");
    // converter.infix_to_postfix("^(a)");
    // let result = converter.infix_to_postfix("\"^\"");
    // converter.infix_to_postfix("^(\")");
    // converter.infix_to_postfix("//^(a)");
    // converter.infix_to_postfix("\"^(\")\"");

    // D			[0-9]
    // L			[a-zA-Z_]
    // H			[a-fA-F0-9]
    // E			[Ee][+-]?{D}+
    // FS			(f|F|l|L)
    // IS			(u|U|l|L)*
    //
    // {D}+{E}{FS}?		        { count(); return(CONSTANT); }
    // {D}*"."{D}+({E})?{FS}?	{ count(); return(CONSTANT); }
    // {D}+"."{D}*({E})?{FS}?	{ count(); return(CONSTANT); }

    // let result = converter.infix_to_postfix("(0|1|2|3|4|5|6|7|8|9)*");

    // let result = converter.infix_to_postfix("(0|1|2)*.(0|1|2)+((e|E))?(f|F|l|L)?");
    // let result = converter.infix_to_postfix("(0|1|2)*.(0|1|2)+(e|E)(\\+|\\-)?(0|1|2)+(f|F|l|L)?");
    // let result = converter.infix_to_postfix("(0|1|2)*.(0|1|2)+((e|E)(\\+|\\-)?(0|1|2)+)?(f|F|l|L)?");

    // let result = converter.infix_to_postfix("(0|1|2)+");
    // let result = converter.infix_to_postfix("((e|E)(0|1|2))+");
    // let result = converter.infix_to_postfix("((0|1|2)(e|E))+");
    // let result = converter.infix_to_postfix("((0|1|2)(e|E)(3|4|5))+");
    // let result = converter.infix_to_postfix("((e|E)(0|1|2)+)");

    // let result = converter.infix_to_postfix("a((e)(0)+)");
    // let result = converter.infix_to_postfix("a((e|E)(0|1|2)+)");

    // let result = converter.infix_to_postfix("((e|E)(\\+|\\-))+");
    // let result = converter.infix_to_postfix("((e|E)(\\+|\\-)?)+");
    // let result = converter.infix_to_postfix("((e|E)(\\+|\\-)?(0|1|2)+)+");
    // let result = converter.infix_to_postfix("((e|E)(\\+|\\-)?(0|1|2)+)?");
    // let result = converter.infix_to_postfix("(0|1|2)+(e|E)(0|1|2)+");
    // let result = converter.infix_to_postfix("(0|1|2)+((e|E)(0|1|2)+)");
    // let result = converter.infix_to_postfix("(0|1|2)+((e|E)(\\+|\\-)?(0|1|2)+)");
    // let result = converter.infix_to_postfix("(0|1|2)+((e|E)(\\+|\\-)?(0|1|2)+)?(f|F|l|L)?");
    // println!("{:?}", result);
    // println!("");

    // let mut fragment_stack_string_literal = FragmentStack::new();
    // recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_string_literal, &mut alphabet);
    // converter.reset();
    // let mut fragment_string_literal = fragment_stack_string_literal.stack.pop().unwrap();
    // fragment_string_literal.enfa.states.get_mut(&fragment_string_literal.end_id).unwrap().token_id = 610;
    // fragment_string_literal.enfa.states.get_mut(&fragment_string_literal.end_id).unwrap().token_name = String::from("STRING_LITERAL");

    // // DEBUG
    // enfa_to_dot_directed_graph(&mut fragment_string_literal.enfa, "string_literal_enfa_automaton.dot");

    // // insert into LEXER
    // let (start_id_string_literal, end_id_string_literal) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_string_literal.enfa, fragment_string_literal.end_id);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_string_literal);

    // // DEBUG
    // enfa_to_dot_directed_graph(&mut fragment_string_literal.enfa, "lexer_enfa_automaton.dot");

    //
    // Float Numeric - //{D}*"."{D}+({E})?{FS}?
    // add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "(0|1|2)*", "FLOAT_NUMERIC", 601);
    // add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "(0|1|2)*.", "FLOAT_NUMERIC", 601);
    // add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "(0|1|2)*.(0|1|2)+", "FLOAT_NUMERIC", 601);
    // add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "ab?c", "FLOAT_NUMERIC", 601);
    // add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "(e|E)(\\+|\\-)?", "FLOAT_NUMERIC", 601);
    // add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "(e|E)(\\+|\\-)?(0|1|2)+", "FLOAT_NUMERIC", 601);
    // add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "(0|1|2)*.(0|1|2)+((e|E)(\\+|\\-)?(0|1|2)+)?(f|F|l|L)?", "FLOAT_NUMERIC", 601);
    //add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "(0|1|2|3|4|5|6|7|8|9)*.(0|1|2|3|4|5|6|7|8|9)+((e|E)(\\+|\\-)?(0|1|2|3|4|5|6|7|8|9)+)?(f|F|l|L)?", "FLOAT_NUMERIC", 601);

    // enfa_to_dot_directed_graph(&mut combined_fragment.enfa, "float_enfa_automaton.dot");

    //add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "int", "INT", 116);

    // //
    // // Whitespace
    // // ' ' (toke-id: 15)
    // let mut fragment_stack_whitespace = FragmentStack::new();
    // add_character_literal(&mut fragment_stack_whitespace, RegexBuildingBlock::CharacterLiteral(' '), &mut alphabet);
    // // the top fragment on the fragment stack contains the root of the eNFA
    // let mut fragment_whitespace = fragment_stack_whitespace.stack.pop().unwrap();
    // fragment_whitespace.enfa.states.get_mut(&fragment_whitespace.end_id).unwrap().token_id = 15;
    // fragment_whitespace.enfa.states.get_mut(&fragment_whitespace.end_id).unwrap().token_name = String::from("WHITESPACE");
    // // insert into LEXER
    // let (start_id_numeric, end_id_numeric) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_whitespace.enfa, fragment_whitespace.end_id);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_numeric);

    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, " ", "WHITESPACE", WHITESPACE_TOKEN_ID);

    //
    // numeric (token-id: 600)
    converter.infix_to_postfix("(0|1|2|3|4|5|6|7|8|9)+");
    let mut fragment_stack_numeric = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_numeric, &mut alphabet);
    converter.reset();
    let mut fragment_numeric = fragment_stack_numeric.stack.pop().unwrap();
    fragment_numeric.enfa.states.get_mut(&fragment_numeric.end_id).unwrap().token_id = 600;
    fragment_numeric.enfa.states.get_mut(&fragment_numeric.end_id).unwrap().token_name = String::from("NUMERIC");
    // insert into LEXER
    let (start_id_numeric, end_id_numeric) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_numeric.enfa, fragment_numeric.end_id);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_numeric);

    //
    // IS			(u|U|l|L)*
    // H			[a-fA-F0-9]
    // Hex Numeric - 0[xX]{H}+{IS}?
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "0(x|X)(0|1|2|3|4|5|6|7|8|9|a|A|b|B|c|C|d|D|e|E|f|F)+(u|U|l|L)?", "HEX_NUMERIC", 602);

    // pointer operator ->
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\->", "PTR_OP", 15);

    //
    // combine all token into DFA
    //

    enfa_to_dot_directed_graph(&mut combined_fragment.enfa, "enfa_automaton.dot");

    let mut dfa = enfa_to_dfa(&mut combined_fragment.enfa, &mut alphabet);

    //enfa_to_dot_directed_graph(&mut fragment_string_literal.enfa, "dfa_automaton.dot");
    enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");

    //let str = "\"Hello World: %d\"";

    // let str = "1983";

    // let str = "1.0";
    // let str = "1.0e+100f";
    // let str = "123.4455e+100f";

    // let str = "0e31e42e5";

    // let str = "e+e-e+";
    // let str = "e+ee+";

    // let str = "e+0e+1e+2";
    // let str = "123123e+0e+1e+2F";

    // let str = "ac";
    // let str = "abc";

    //let str = "int int int";

    // let str = "0x03";

    let str = "->";

    println!("Input: {}", str);

    let mut current_state_id = dfa.start_state_id;
    // let mut last_state_id = dfa.start_state_id;
    
    let lexer_debug: bool = true;
    let mut lexer: Lexer = Lexer::new(dfa);

    let mut rule_map = BTreeMap::<usize, Rule<String>>::new();

    let mut current_character: char = 'x';
    let mut lookahead_character: char = 'y';
    let mut has_lookahead_character = false;

    let mut token_string_buffer = String::from("");

    let mut parse_table = HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>::new();
    let mut parser: Parser<String> = Parser::<String>::new(parse_table);
    let mut step: usize = 1;
    let mut grammar_state_hashmap = BTreeMap::<usize, GrammarState<String>>::new();

    // DEBUG outputting parse tree as DOT graph
    let mut debug_node_string_buffer = String::from("");
    let mut debug_node_stack = Vec::<DebugNode>::new();

    for character in str.chars() {

        current_character = lookahead_character;
        lookahead_character = character;

        if !has_lookahead_character {
            has_lookahead_character = true;
            continue;
        }

        // current_state_id = consume_character(&mut dfa, 
        //     current_state_id, 
        //     &mut token_string_buffer, 
        //     current_character, 
        //     lookahead_character, 
        //     &mut step, 
        //     &mut parser, 
        //     &grammar_state_hashmap, 
        //     &mut string_buffer, 
        //     &mut debug_node_stack);
       current_state_id = lexer.consume_character(current_character, 
            lookahead_character, 
            &mut step, 
            &mut parser, 
            &rule_map,
            &mut debug_node_string_buffer, 
            &mut debug_node_stack);
    }

    current_state_id = lexer.consume_character(
        lookahead_character, 
        'x', 
        &mut step, 
        &mut parser, 
        &rule_map, 
        &mut debug_node_string_buffer, 
        &mut debug_node_stack);

    // if lexer_debug {
    //     println!("[LEXER] Emitting '{}'. Token-Id: {}, Token-Name: {}", token_string_buffer, dfa.states[&current_state_id].token_id, dfa.states[&current_state_id].token_name);
    //     println!("");
    // }

    if lexer.dfa.is_end_state(current_state_id) {
        println!("[LEXER] Emitting '{}'. Token-Id: {}, Token-Name: {}", token_string_buffer, lexer.dfa.states[&current_state_id].token_id, lexer.dfa.states[&current_state_id].token_name);
        println!("ACCEPT!");
    } else {
        panic!("DECLINED!");
    }

    println!("test");

    ///////////////////////////////////////////////////////////////////////////////////
/*

    let mut grammar_rules = Vec::<Rule<String>>::new();

    //
    // Select one of the grammars
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
    let g_result = produce_grammar_c_full_if_else_4(&mut grammar_rules);
    // let g_result = produce_grammar_c_full_5(&mut grammar_rules);
    // let g_result = produce_grammar_left_recursive(&mut grammar_rules);

    let rule_1 = g_result.0;
    // let mut start_symbol = g_result.1;
    let augmented_start_symbol = g_result.1;



    //
    // Validating the Grammar
    //

    validate_grammar(&mut grammar_rules);



    //
    // Print all rules (in the order in which they use each other)
    //

    println!("");
    println!("https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html  (Do not add augmented start rule)");
    println!("https://jsmachines.sourceforge.net/machines/lalr1.html               (add augmented start rule)");
    println!("");
    println!("All Rules:");

    let mut unlocked_rules_lhs = Vec::new();
    unlocked_rules_lhs.push(rule_1.lhs.clone());

    let mut temp_rules = grammar_rules.clone();

    let mut printed_rules = Vec::<Rule<String>>::new();

    // DEBUG - print all rules
    let mut all_rules_printed: bool = false;
    while !all_rules_printed {

        for i in 0..temp_rules.len() {

            if unlocked_rules_lhs.contains(&temp_rules.get(i).unwrap().lhs) {

                let mut temp_rule = temp_rules.get(i).unwrap().clone();

                temp_rule.dot_idx = std::usize::MAX;
                println!("{:?}", temp_rule);
                temp_rule.dot_idx = 0;

                for rhs in temp_rule.rhs.iter() {
                    match &rhs {
                        RuleElement::NonTerminal(nt) => {
                            unlocked_rules_lhs.push(rhs.clone());
                        }
                        _ => {

                        }
                    }
                }

                printed_rules.push(temp_rule);
            }
        }

        let removed_rules = temp_rules.extract_if(.., |r| printed_rules.contains(r)).collect::<Vec<_>>();

        // println!(">> temp_rules >> {:?}", temp_rules);

        all_rules_printed = temp_rules.len() == 0;

        if temp_rules.len() > 0 && removed_rules.len() == 0 {
            // println!("Unused rules detected!");
            // println!("{:?}", temp_rules);

            all_rules_printed = true;
        }
    }

    if temp_rules.len() > 0 {
        println!("Unused rules detected!");
        println!("{:?}", temp_rules);
    }




    //
    // Nullable
    //

    let mut nullable = BTreeMap::<RuleElement::<String>, bool>::new();
    compute_nullable_sets(&mut grammar_rules, &mut nullable);




    //
    // First Set
    //

    let mut first = BTreeMap::<RuleElement::<String>, Vec::<RuleElement::<String>>>::new();
    compute_first_original(&grammar_rules, &nullable, &mut first);



    // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    // LALR(1) generation channel algorithm starts
    // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    //
    // Unfold the states (CLOSURE) and build channels between the states.
    //

    let mut rule_channel_map = HashMap::<usize, Vec::<Transition<String>>>::new();

    let mut found_start_state: bool = false;
    let mut start_state_id: usize = 0;
    //let mut found_final_state: bool = false;
    //let mut final_state_id: usize = 0;

    // build a state for the first rule
    let first_state_id = STATE_COUNTER.fetch_add(1, Ordering::SeqCst);
    let mut grammar_state: GrammarState<String> = GrammarState::new(first_state_id);
    grammar_state.identification_rules.push(rule_1);

    if !found_start_state {
        found_start_state = true;
        start_state_id = grammar_state.id;
    }

    // the e_set is a set of states that still need to be processed. 
    // The letter e in e set has no special meaning.
    // The name e set is choosen as another d set exists and e comes after d in the alphabet. 
    // The term d-set stems from a eNFA-to-DFA conversion algorithm. I used it as a convention
    let mut e_set = Vec::<usize>::new();
    let mut processed_set = Vec::<usize>::new();

    // place unfolded start state into the e-set
    e_set.push(grammar_state.id);

    // this is the set of all states of the DFA. It is gradually complemented
    let mut grammar_state_hashmap = BTreeMap::new();
    grammar_state_hashmap.insert(grammar_state.id, grammar_state);

    let unfold_debug = false;

    let mut done: bool = e_set.is_empty();
    while !done {

        // // DEBUG
        // println!("e_set: {:?}", e_set);
        // println!("processed_set: {:?}", processed_set);

        // get the next state id from the e_set (= set of states to process)
        let current_grammar_state_id = e_set.pop().expect("Need at least one state!");
        processed_set.push(current_grammar_state_id);

        // if current_grammar_state_id == 8 {
        //     println!("test");
        // }

        // unfold node (retrieve state given state id, then call unfold_grammar_state())
        if let Some(grammar_state) = grammar_state_hashmap.get_mut(&current_grammar_state_id) {
            
            if unfold_debug {
                println!("Before: {:?}", grammar_state);
            }

            // if grammar_state.id == 18 {
            //     println!("test");
            // }

            // unfold_grammar_state is probably the CLOSURE() operation
            // the rule_channel_map is extended with new entries, by this call
            grammar_state.unfold_grammar_state(&grammar_rules, &first, &nullable, &mut rule_channel_map);

            if unfold_debug {
                println!("After: {:?}", grammar_state);
                println!("");
            }

            // // DEBUG
            // println!("\n");
            // println!("----------------------------------------");
            // println!("{:?}", grammar_state);
            // println!("========================================");
        }

        //
        // For the unfolded new node, create new nodes if there is a Terminal or NonTerminal pointing to new states
        //

        // clone current state
        let curr_state = grammar_state_hashmap[&current_grammar_state_id].clone();

        // collect all rules without removing them from the current state
        let mut all_state_rules = Vec::new();
        all_state_rules.append(&mut curr_state.identification_rules.clone());
        all_state_rules.append(&mut curr_state.rules.clone());

        // iterate over all rules and collect all rules that are activated by the same symbol
        // This is done because a transition between states is defined by ALL rules activated by the same symbol!
        while all_state_rules.len() > 0 {

            // remove rules that are completely processed (dot-marker is after last symbol)
            let consumed_rules = all_state_rules.extract_if(.., |r| r.dot_idx >= r.rhs.len()).collect::<Vec<_>>();

            // // check for the end state.
            // // The end state has the dot marker after the start symbol
            // if consumed_rules.len() == 1 && let Some(last_symbol) = consumed_rules[0].rhs.last() {

            //     if *last_symbol == start_symbol {

            //         if found_final_state {
            //             eprintln!("DFA cannot have two end states! First final state: {}, This state: {}", final_state_id, current_grammar_state_id);
            //         }

            //         found_final_state = true;
            //         final_state_id = current_grammar_state_id;

            //         // // TODO output transition
            //         // println!("[CHANNELS] STATE-TRANSITION-ENDSTATE: {:?} -{:?}-> {:?}", &current_grammar_state_id, RuleElement::<String>::AcceptingStateTransition, std::usize::MAX);
            //     }
            // }

            if all_state_rules.len() == 0 {
                continue;
            }

            // get activated symbol from first rule
            let current_symbol = all_state_rules[0].rhs[all_state_rules[0].dot_idx].clone();

            // extract other rules that have the same activated symbol
            let mut rules_for_symbol = all_state_rules.extract_if(.., |r| r.dot_idx < r.rhs.len() && r.rhs[r.dot_idx] == current_symbol).collect::<Vec<_>>();

            if rules_for_symbol.len() == 0 {
                continue;
            }

            // // CREATE/FOLLOW EPSILON TRANSITIONS?
            // // if it is an epsilon, do nothing because that node will not transition to another node at all
            // match current_symbol {
            //     RuleElement::Epsilon => { continue; }
            //     _ => {}
            // }
            
            // // DEBUG - output the symbol and the rules activated by it
            // println!("{:?} {:?}", &current_symbol, &rules_for_symbol);
            // println!("");

            // TODO
            // iterate over each rule in rules_for_symbol
            //      - advance the dot in the collected rules
            //      - look for states globally in grammar_state_hashmap, that have ALL the collected, 
            //        modified rules in their identifying set AT THE SAME TIME! OF UTMOST IMPORTANT!!!!!
            //          - if no such state exists yet, create one
            //              - insert newly created state into e_set
            //              - insert newly created state into global set
            //          - if such a state exists, build transition to it

            // remove depleted rules
            let _ = rules_for_symbol.extract_if(.., |r| r.dot_idx >= r.rhs.len()).collect::<Vec<_>>();

            // if no active rules (= rules that have the dot marker within the rule) are left, abort
            if rules_for_symbol.len() == 0 {
                continue;
            }

            let mut rules_for_symbol_copy = Vec::<Rule<String>>::new();
            let mut src_rule_id = Vec::<usize>::new();

            // for all activated rules, advance the dot marker
            for rule in &mut rules_for_symbol {

                let mut rule_clone = rule.clone();
                rule_clone.id = RULE_COUNTER.fetch_add(1, Ordering::SeqCst);
                rule_clone.dot_idx = rule_clone.dot_idx + 1;

                //println!("RULE-RULE-CHANNEL: {} -> {}", rule.id, rule_clone.id);

                rules_for_symbol_copy.push(rule_clone);
                src_rule_id.push(rule.id);
            }

            // // DEBUG
            // println!("{:?}", rules_for_symbol);
            // println!("{:?}", rules_for_symbol_copy);

            // DEBUG - output the symbol and the rules activated by it
            // println!("{:?} {:?}", &current_symbol, &rules_for_symbol);

            //
            // look for existing state to point to using a transition or created a new state
            //

            let mut state_contained_already = false;
            let mut state_id: usize = 0;

            for (loop_state_id, loop_state) in &grammar_state_hashmap {

                // a state is identified via the (all rules in) identification rules set
                if loop_state.identification_rules == rules_for_symbol_copy {

                    // state found which did already exist (because it was created by a prior state)
                    state_contained_already = true;
                    state_id = *loop_state_id;

                    // the copied rules have new id's that will not match the id's
                    // for rules in the existing state. Match the rules to match 
                    // and reuse the existing rule id's to build a valid channel network
                    let mut iter_index: usize = 0;
                    for rule_copy in &mut rules_for_symbol_copy {
                        for rrule in &loop_state.identification_rules {
                            if rule_copy == rrule {

                                // // DEBUG
                                // println!("[CHANNELS] RULE-RULE-CHANNEL: {} -> {}", src_rule_id[iter_index], rrule.id);

                                //println!("{:?}", rules_for_symbol[iter_index]);
                                // rules_for_symbol[iter_index].channels.push(rrule.id);

                                // let mut curr_state = grammar_state_hashmap.get_mut(&current_grammar_state_id).unwrap();
                                // for jj in 0..curr_state.rules.len() {
                                //     if curr_state.rules[jj].id == src_rule_id[iter_index] {
                                //         curr_state.rules[jj].channels.push(rrule.id);
                                //     }
                                // }

                                // println!("{:?}", rules_for_symbol[iter_index]);

                                if !rule_channel_map.contains_key(&src_rule_id[iter_index]) {
                                    // println!("Not contained yet!");

                                    let channel_ends = Vec::<Transition<String>>::new();
                                    rule_channel_map.insert(src_rule_id[iter_index], channel_ends);
                                }

                                // retrieve the vector of first symbols for the nonterminal and extend it
                                let channel_ends = &mut rule_channel_map.get_mut(&src_rule_id[iter_index]).unwrap();

                                // add the channel                                
                                channel_ends.push(Transition(rrule.id, current_symbol.clone()));
                            }
                        }
                        iter_index = iter_index + 1;
                    }

                    // only a single state can be found
                    break;
                }
            }

            if state_contained_already {

                // if current_grammar_state_id == state_id {
                //     // // DEBUG
                //     // println!("[CHANNELS] STATE-TRANSITION-EXISTING-SELF: {:?} -{:?}-> {:?}", &current_grammar_state_id, &current_symbol, &state_id);
                // } else {
                //     // // DEBUG output transition (to already existing state)
                //     // println!("[CHANNELS] STATE-TRANSITION-EXISTING: {:?} -{:?}-> {:?}", &current_grammar_state_id, &current_symbol, &state_id);
                // }

            } else {

                // state not contained, build state, insert into e_set, insert transition

                // build new state
                let next_state_id = STATE_COUNTER.fetch_add(1, Ordering::SeqCst);

                if unfold_debug {
                    println!("next_state_id: {}", next_state_id);
                }

                //if next_state_id == 6 as usize {
                // if next_state_id == 8 as usize {
                //     println!("test");
                // }
                let mut new_grammar_state: GrammarState<String> = GrammarState::new(next_state_id);

                // // DEBUG - output rule channels
                // for rule_copy in &mut rules_for_symbol_copy {
                //     println!("RULE-RULE-CHANNEL: ? -> {}", rule_copy.id);
                // }

                // DEBUG
                // println!("Created new state: {:?}", new_grammar_state.id);

                //new_grammar_state.identification_rules.append(&mut rules_for_symbol);
                //new_grammar_state.identification_rules.append(&mut rules_for_symbol_copy);

                // copy identification rules into new target state
                let mut iter_index: usize = 0;
                for rule_copy in rules_for_symbol_copy {

                    let rule_copy_id = rule_copy.id;

                    // copy identification rules into new target state
                    new_grammar_state.identification_rules.push(rule_copy);

                    // // DEBUG
                    // println!("[CHANNELS] RULE-RULE-CHANNEL: {} -> {}", src_rule_id[iter_index], rule_copy_id);

                    // add a channel from src rule inside the old state to the identification rule in the newly created state
                    if !rule_channel_map.contains_key(&src_rule_id[iter_index]) {
                        let channel_ends = Vec::<Transition<String>>::new();
                        rule_channel_map.insert(src_rule_id[iter_index], channel_ends);
                    }
                    // retrieve the vector of first symbols for the nonterminal and extend it
                    let channel_ends = &mut rule_channel_map.get_mut(&src_rule_id[iter_index]).unwrap();
                    // add the new channel into the channels for the source rule
                    channel_ends.push(Transition(rule_copy_id, current_symbol.clone()));

                    iter_index = iter_index + 1;
                }

                // DEBUG - print new state so far:
                if unfold_debug {
                    println!("New State Before Unfold: {:?}", new_grammar_state);
                }

                // insert new state into e_set
                e_set.insert(0, new_grammar_state.id);

                // // TODO output transition (to new state)
                // println!("[CHANNELS] STATE-TRANSITION-NEW_STATE: {:?} -{:?}-> {:?}", &current_grammar_state_id, &current_symbol, &new_grammar_state.id);

                // add the new state into the map of states
                grammar_state_hashmap.insert(new_grammar_state.id, new_grammar_state);
            }
        }

        done = e_set.is_empty();
    }

    //
    // DEBUG output all channels
    //
    // rule_channel_map: rule_id to all the rule_ids the rule points to

    println!("");
    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    println!("Channels");
    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");

    // build a map from rule-id to the state-id of the state that contains this rule
    let mut rule_id_to_state_id_map = HashMap::<usize, usize>::new();
    let mut rule_ids = Vec::<usize>::new();
    for (key, value) in &grammar_state_hashmap {
        // println!("{:?} / {:?}", key, value);
        for i in 0..value.identification_rules.len() {
            let rule_id = value.identification_rules[i].id;
            rule_id_to_state_id_map.insert(rule_id, value.id);
            rule_ids.push(rule_id);
        }
        for i in 0..value.rules.len() {
            let rule_id = value.rules[i].id;
            rule_id_to_state_id_map.insert(rule_id, value.id);
            rule_ids.push(rule_id);
        }
    }
    
    // DEBUG - output all channels
    let output_channels = false;
    if output_channels {
        for (key, value) in &rule_channel_map {
            for transition in value {
                println!("Channel: {:?}:{:?} -{:?}- {:?}:{:?}", rule_id_to_state_id_map[key], key, transition.1, rule_id_to_state_id_map[&transition.0], transition.0);
            }
        }
    }

    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");



    //
    // Propagation cycles
    //

    // propagation: it is required to keep channels that connect a start rule to an end rule!
    // because lookahead symbols are pushed over channels between rules and not between states!
    //
    // DragonBook, 2nd Edition, page 273:
    //
    // "4. Make repeated passes over the kernel items in all sets. When we visit an
    // item i, we look up the kernel items to which i propagates its lookaheads,
    // using information tabulated in step (2). The current set of lookaheads
    // for i is added to those already associated with each of the items to which
    // i propagates its lookaheads. We continue making passes over the kernel
    // items until no more new lookaheads are propagated."

    

    println!("");
    println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
    println!("Propagation cycles start ...");
    println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");

    // println!("{:?}", rule_id_to_state_id_map);

    // for (key, value) in &grammar_state_hashmap {
    //     println!("{:?} / {:?}", key, value);
    // }

    // // DEBUG
    // println!("rule-channel-map: {:?}", &rule_channel_map);

    let mut change_detected = true;
    let mut iteration = 0;

    while change_detected {

        change_detected = false;

        println!("[Propagation] Iteration: {:?}", iteration);

        let mut dirty_state_ids = Vec::<usize>::new();
        let mut dirty_state_id_and_symbols = HashMap::<usize, Vec<RuleElement::<String>>>::new();

        let processed_state_ids = Vec::<usize>::new();

        // over all rules
        for rule_id in &rule_ids {

            // for rule id, resolve state that contains the rule
            let src_rule_id = rule_id;
            let src_state_id = rule_id_to_state_id_map.get(src_rule_id).unwrap();

            // if the rule has no channel attached to it, continue because no propagation is necessary
            if !rule_channel_map.contains_key(src_rule_id) {
                continue;                
            }

            // println!("Source-Rule-Id: {}", src_rule_id);

            //
            // Step 1 - retrieve all channels for the source rule
            //

            // retrieve all channels
            let dest_rule_transitions = rule_channel_map.get(src_rule_id).unwrap();

            // over all of the channels to target rules that this rule has
            //
            // transition.0 == destination rule id
            // transition.1 == symbol to transition for
            for dest_rule_id in dest_rule_transitions {

                let dest_state_id:usize = *rule_id_to_state_id_map.get(&dest_rule_id.0).unwrap();

                // // DO NOT GO BACKWARDS
                // if *src_state_id > dest_state_id {
                //     //panic!("test");
                //     continue;
                // }

                // if *src_rule_id == 68 as usize {
                //     println!("Src-RuleId: {:?}, Src-StateId: {:?} ===> Dest-RuleId: {}, Dest-StateId: {:?}",
                //         src_rule_id, src_state_id, dest_rule_id.0, dest_state_id);
                //     println!("test");
                // }

                //
                // push lookaheads to external rules
                //

                // retrieve src-rule from src-state (src-state is read-only, non-mutable clone!)
                let src_state = grammar_state_hashmap.get(src_state_id).unwrap().clone();

                // first, search rule in identification_rules then search inside the rules collection
                let mut src_rule = src_state.identification_rules.iter().filter(|r| r.id == *src_rule_id).collect::<Vec<_>>();
                if src_rule.len() == 0 {
                    src_rule = src_state.rules.iter().filter(|r| r.id == *src_rule_id).collect::<Vec<_>>();
                }
                // println!("{:?}", &src_rule.first());

                // retrieve dest-rule from dest-state
                let dest_state = grammar_state_hashmap.get_mut(&dest_state_id).unwrap();
                // println!("{:?}", dest_state);

                // let mut dest_rule = dest_state.identification_rules.iter().filter(|r| r.id == *dest_rule_id).collect::<Vec<_>>();
                // if dest_rule.len() == 0 {
                //     dest_rule = dest_state.rules.iter().filter(|r| r.id == *dest_rule_id).collect::<Vec<_>>();
                // }
                // println!("{:?}", &dest_rule.first());

                //
                // Step 2 - check if the channel points to an identification rule
                //

                // if *src_state_id > dest_state_id {
                //     continue;
                // }

                //
                // This is a very important detail. All identification rules of a state are checked for empty beta.
                // If ALL rules have empty beta, then incoming lookahead symbols will propagate into the states inner rules!
                // If one or more identification rules are not empty beta, the entire state is not empty beta and
                // hence no external propagated lookaheads will propagate into the state!
                //
                
                let mut empty_beta = true;
                for i in 0..dest_state.identification_rules.len() {
                    if dest_state.identification_rules[i].dot_idx + 1 >= dest_state.identification_rules[i].rhs.len() {
                        // println!("empty beta!");
                    } else {
                        empty_beta = false;
                    }
                }

                for i in 0..dest_state.identification_rules.len() {

                    if dest_state.identification_rules[i].id == dest_rule_id.0 {

                        // copy lookaheads into dest rule
                        for la in &src_rule.first().unwrap().lookahead {

                            // // DEBUG
                            // //if *la == RuleElement::NonTerminal(String::from(")")) {
                            // if dest_state.id == 50 && *la == RuleElement::Terminal(String::from("CLOSING_BRACKET")) {
                            //     println!("test");
                            // }

                            // if a lookahead is inserted into the identification rules where it 
                            // has not been contained already, the state becomes dirty
                            if !dest_state.identification_rules[i].lookahead.contains(&la) {
                            // if !dest_state.identification_rules[i].external_lookahead.contains(&la) {

                                // if dest_state.identification_rules[i].dot_idx + 1 >= dest_state.identification_rules[i].rhs.len() {
                                //     println!("empty beta!");
                                //     empty_beta = true;
                                // }

                                //
                                // TODO TODO TODO
                                // TODO TODO TODO
                                // TODO TODO TODO
                                // TODO TODO TODO
                                // 
                                // insert lookahead into identification rule
                                dest_state.identification_rules[i].lookahead.push(la.clone());
                                // dest_state.identification_rules[i].external_lookahead.push(la.clone());

                                //println!("{:?}", dest_state);
                                //println!("{}", dest_state.identification_rules[i].id);

                                // identification rules have been changed, a new lookahead was added.
                                // The state becomes dirty and a progation inside that state and
                                // across that state to other states needs to take place
                                if !dirty_state_ids.contains(&dest_state_id) && empty_beta {
                                    dirty_state_ids.push(dest_state_id);
                                }

                                // empty_beta = false;

                                // if !processed_state_ids.contains(&dest_state_id) {
                                //     dirty_state_ids.push(dest_state_id);
                                //     processed_state_ids.push(dest_state_id);
                                // }

                                // dirty_state_id_and_symbols.insert(dest_state_id)

                                // TO_OTHER_STATE
                                //
                                // add a channel from src rule inside the old state to the identification rule in the newly created state
                                if !dirty_state_id_and_symbols.contains_key(&dest_state_id) {
                                    let symbols = Vec::<RuleElement::<String>>::new();
                                    dirty_state_id_and_symbols.insert(dest_state_id, symbols);
                                }

                                // retrieve the vector of first symbols for the nonterminal and extend it
                                let symbols = &mut dirty_state_id_and_symbols.get_mut(&dest_state_id).unwrap();

                                // add the new channel into the channels for the source rule
                                symbols.push(la.clone());

                                //println!("({} -> {}) Dirty: {} because '{:?}' has been pushed into one of it's identifying rules", src_state_id, dest_state_id, dest_state_id, la.clone());

                                change_detected = true;
                            }
                        }
                    }
                }
            } // over all destination rules
        } // over all rules

        if debug {
            println!("[Propagation] DirtySet: {:?}", dirty_state_ids);
        }

        // for each state in the dirty set, perform inner propagation of the newly pushed symbol!
        for state_id in dirty_state_ids {

            let state = grammar_state_hashmap.get_mut(&state_id).unwrap();

            // if state.id == 52 {
            //     println!("{:?}", state);
            // }

            let mut local_rule_ids = Vec::<usize>::new();
            let mut processed_rule_ids = Vec::<usize>::new();

            // collect all rules which the identification_rules point to within the same state
            for i in 0..state.identification_rules.len() {

                let identification_rule_id = state.identification_rules[i].id;
                // println!("Rule id:{:?}", identification_rule_id);

                // CHANGED
                if !processed_rule_ids.contains(&identification_rule_id) {
                    processed_rule_ids.push(identification_rule_id);
                }

                if !rule_channel_map.contains_key(&identification_rule_id) {
                    continue;
                }

                // retrieve all channels for the current rule
                let dest_rule_transitions = rule_channel_map.get(&identification_rule_id).unwrap();
                for transition in dest_rule_transitions {
                    let target_state = rule_id_to_state_id_map.get(&transition.0).unwrap();
                    // needs to be within same state
                    if *target_state == state_id {
                        // push target rule id

                        // println!("{:?}", transition.1);

                        // TODO: I have disabled this!
                        // CHANGED
                        if !local_rule_ids.contains(&transition.0) {
                            local_rule_ids.push(transition.0);
                        }

                        // if empty_beta {
                        //     local_rule_ids.push(transition.0);
                        // }
                    }
                }
            }

            // println!("local_rule_ids:{:?}", local_rule_ids);

            let mut done = local_rule_ids.len() == 0;
            while !done {

                // go from normal rule to normal rule within the same state
                let src_rule_id = local_rule_ids[0];

                // println!("Rule id:{:?}", src_rule_id);

                local_rule_ids.drain(0..1);
                
                // CHANGED
                if !processed_rule_ids.contains(&src_rule_id) {
                    processed_rule_ids.push(src_rule_id);
                }

                // insert lookahead into rule if not contained already
                for j in 0..state.rules.len() {

                    let added_symbols_option = dirty_state_id_and_symbols.get(&state_id);
                    if let Some(added_symbols) = added_symbols_option {

                        for added_symbol in added_symbols {
                            if src_rule_id == state.rules[j].id && !state.rules[j].lookahead.contains(&added_symbol) {
                                state.rules[j].lookahead.push(added_symbol.clone());
                                change_detected = true;
                            }
                        }
                    }
                }

                //if empty_beta {

                    /* PUSH DOWN FURTHER RULES */

                    // retrieve all channels for the current rule
                    if rule_channel_map.contains_key(&src_rule_id) {
                        let dest_rule_transitions = rule_channel_map.get(&src_rule_id).unwrap();
                        for transition in dest_rule_transitions {

                            // println!("{:?} -> {:?}", src_rule_id, transition.0);

                            if processed_rule_ids.contains(&transition.0) {
                                continue;
                            }

                            let target_state = rule_id_to_state_id_map.get(&transition.0).unwrap();
                            // needs to be within same state
                            if *target_state == state_id {

                                // push target rule id
                                // CHANGED
                                if !local_rule_ids.contains(&transition.0) {
                                    local_rule_ids.push(transition.0);
                                }
                            }
                        }
                    }

                //}
            
                done = local_rule_ids.len() == 0;
            }
                
            // if state.id == 50 {
            //     println!("{:?}", state);
            // }

            // println!("done");
        }

        iteration = iteration + 1;

        // change_detected = false;
    }

    println!("Propagation cycles end after {} iterations.", iteration);

    println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");

    

    // DEBUG
    // rust iterate over hashmap
    // https://stackoverflow.com/questions/45724517/how-to-iterate-through-a-hashmap-print-the-key-value-and-remove-the-value-in-ru
    println!("");
    println!("*********************************************************************************");
    println!("RESULT - FINISHED - READY - RESULT - FINISHED - READY - RESULT - FINISHED - READY");
    println!("*********************************************************************************");

    // DEBUG - print all states for comparison with onlien tools (e.g. https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html)
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
    
    if !found_start_state {
        panic!("DFA no start state detected!");
    } else {
        println!("Start state: {}", start_state_id);
    }

    //
    // Building the Parse Table from the LALR(1) DFA
    //

    println!("");
    println!("*********************************************************************************");
    println!("Building the Parse Table from the LALR(1) DFA                                    ");
    println!("*********************************************************************************");

    let mut parse_table = HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>::new();

    build_parse_table(&mut parse_table, &mut grammar_state_hashmap, &rule_channel_map, &augmented_start_symbol, &rule_id_to_state_id_map);    

    


    //
    // Build the Lexer
    //

    //
    // Phase 0 - 
    //

    let mut combined_fragment = Fragment::new(RegexBuildingBlock::Or);

    //
    // Pre-Build alphabet
    //

    // complete alphabet has to be known in advance
    let mut alphabet = HashSet::<RegexBuildingBlock>::new();

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('a'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('A'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('b'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('B'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('c'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('C'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('d'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('D'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('e'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('E'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('f'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('F'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('g'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('G'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('h'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('H'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('i'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('I'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('j'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('J'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('k'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('K'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('l'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('L'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('m'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('M'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('n'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('N'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('o'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('O'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('p'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('P'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('q'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('Q'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('r'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('R'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('s'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('S'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('t'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('T'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('u'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('U'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('v'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('V'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('w'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('W'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('x'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('X'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('y'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('Y'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('z'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('Z'));

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('0'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('1'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('2'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('3'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('4'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('5'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('6'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('7'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('8'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('9'));

    alphabet.insert(RegexBuildingBlock::CharacterLiteral(' ')); // WHITESPACE, SPACE

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('_'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('<'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('>'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('{'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('}'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('('));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral(')'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('['));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral(']'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('+'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('-'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('*'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('/'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('%'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('&'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('^')); // Used in regex as NOT operator
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('|'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('!'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral(';'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral(','));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('~'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('?'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('.'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('='));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('"'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral(':'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('\\'));

    // "\n" | "\r\n" | "\r"
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('\n'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('\r'));
    

    //
    // Phase 1 - build all regexes
    //

    //
    // identifier

    // provide a regex in infix notation and let the converter produce a postfix notation
    // The result is stored within the state of the converter instance, this is why the converter can be reset
    let mut converter = InfixPostfixConverter::new();
    //converter.infix_to_postfix("(_|a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z)(_|a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z)+");
    converter.infix_to_postfix("(_|a|A|b|B|c|C|d|D|e|E|f|F|g|G|h|H|i|I|j|J|k|K|l|L|m|M|n|N|o|O|p|P|q|Q|r|R|s|S|t|T|u|U|v|V|w|W|x|X|y|Y|z|Z)|(_|a|A|b|B|c|C|d|D|e|E|f|F|g|G|h|H|i|I|j|J|k|K|l|L|m|M|n|N|o|O|p|P|q|Q|r|R|s|S|t|T|u|U|v|V|w|W|x|X|y|Y|z|Z)(_|a|A|b|B|c|C|d|D|e|E|f|F|g|G|h|H|i|I|j|J|k|K|l|L|m|M|n|N|o|O|p|P|q|Q|r|R|s|S|t|T|u|U|v|V|w|W|x|X|y|Y|z|Z|0|1|2|3|4|5|6|7|8|9)+");
    
    // next, from the regex-items in the postfix notation, construct a eNFA
    // This function will go through the infix character by character and extend a eNFA as it goes.
    // Once done, the eNFA will accept all input described by the regex infix notation
    let mut fragment_stack_identifier = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_identifier, &mut alphabet);
    // reset the converter
    converter.reset();
    // the top fragment on the fragment stack contains the root of the eNFA
    let mut fragment_identifier = fragment_stack_identifier.stack.pop().unwrap();
    // assign a token id to eNFA so it will assign that token id to all token it accepts
    fragment_identifier.enfa.states.get_mut(&fragment_identifier.end_id).unwrap().token_id = IDENTIFIER_TOKEN_ID;
    fragment_identifier.enfa.states.get_mut(&fragment_identifier.end_id).unwrap().token_name = String::from("IDENTIFIER");
    // insert into LEXER
    let (start_id_identifier, end_id_identifier) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_identifier.enfa, fragment_identifier.end_id);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_identifier);
    
    // DEBUG dump the graph to .dot format for viewing using https://dreampuf.github.io/GraphvizOnline
    //enfa_to_dot_directed_graph(&mut fragment_identifier.enfa, "fragment_identifier_automaton.dot");

    //
    // Float Numeric - {D}*"."{D}+({E})?{FS}?
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "(0|1|2|3|4|5|6|7|8|9)*.(0|1|2|3|4|5|6|7|8|9)+((e|E)(\\+|\\-)?(0|1|2|3|4|5|6|7|8|9)+)?(f|F|l|L)?", "FLOAT_NUMERIC", 601);

    //
    // IS			(u|U|l|L)*
    // H			[a-fA-F0-9]
    // Hex Numeric - 0[xX]{H}+{IS}?
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "0(x|X)(0|1|2|3|4|5|6|7|8|9|a|A|b|B|c|C|d|D|e|E|f|F)+(u|U|l|L)?", "HEX_NUMERIC", 602);


    //
    // numeric (token-id: 600)
    converter.infix_to_postfix("(0|1|2|3|4|5|6|7|8|9)+");
    let mut fragment_stack_numeric = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_numeric, &mut alphabet);
    converter.reset();
    let mut fragment_numeric = fragment_stack_numeric.stack.pop().unwrap();
    fragment_numeric.enfa.states.get_mut(&fragment_numeric.end_id).unwrap().token_id = 600;
    fragment_numeric.enfa.states.get_mut(&fragment_numeric.end_id).unwrap().token_name = String::from("NUMERIC");
    // insert into LEXER
    let (start_id_numeric, end_id_numeric) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_numeric.enfa, fragment_numeric.end_id);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_numeric);
    
    //
    // string_literal (token-id: 610)
    //converter.infix_to_postfix("\"(a|A|b|B|c|C|d|D)+\"");
    //converter.infix_to_postfix("\"^[\",\"]+\"");
    // converter.infix_to_postfix("^a");
    // converter.infix_to_postfix("^(a)");
    // converter.infix_to_postfix("^(\")");
    // converter.infix_to_postfix("//^(a)");
    // converter.infix_to_postfix("\"^(\")\"");
    converter.infix_to_postfix("\"^\"");
    let mut fragment_stack_string_literal = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_string_literal, &mut alphabet);
    converter.reset();
    let mut fragment_string_literal = fragment_stack_string_literal.stack.pop().unwrap();
    fragment_string_literal.enfa.states.get_mut(&fragment_string_literal.end_id).unwrap().token_id = 610;
    fragment_string_literal.enfa.states.get_mut(&fragment_string_literal.end_id).unwrap().token_name = String::from("STRING_LITERAL");
    // // DEBUG
    // enfa_to_dot_directed_graph(&mut fragment_string_literal.enfa, "string_literal_enfa_automaton.dot");
    // insert into LEXER
    let (start_id_string_literal, end_id_string_literal) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_string_literal.enfa, fragment_string_literal.end_id);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_string_literal);

    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_2);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_3);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_4);
    // // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_5);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_6);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_7);

    // DEBUG - print to dot file format for debugging with https://dreampuf.github.io/GraphvizOnline
    //enfa_to_dot_directed_graph(&mut combined_fragment.enfa, "enfa_automaton.dot");

    //
    // define operators
    //

    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "...", "ELLIPSIS", 0);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ">>=", "RIGHT_ASSIGN", 1);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "<<=", "LEFT_ASSIGN", 2);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\+=", "ADD_ASSIGN", 3); // used in Regex as Repeat(1, std::usize::MAX)
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\-=", "SUB_ASSIGN", 4); // used in Regex to build character classes
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\*=", "MUL_ASSIGN", 5); // used in Regex as Repeat(0, std::usize::MAX)
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "/=", "DIV_ASSIGN", 6);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "%=", "MOD_ASSIGN", 7);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "&=", "AND_ASSIGN", 8);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\^=", "XOR_ASSIGN", 9); // used in Regex as NEGATION operator
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\|=", "OR_ASSIGN", 10); // used in Regex as OR operator
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ">>", "RIGHT_OP", 11);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "<<", "LEFT_OP", 12);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\+\\+", "INC_OP", 13); // used in Regex as Repeat(1, std::usize::MAX)
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\-\\-", "DEC_OP", 14); // used in Regex to build character classes
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\->", "PTR_OP", 15);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "&&", "AND_OP", 16);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\|\\|", "OR_OP", 17); // used in Regex as OR operator
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "<=", "LE_OP", 18);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ">=", "GE_OP", 19);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "==", "EQ_OP", 20);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\!=", "NE_OP", 21); // ???
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ";", "SEMICOLON", 22);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\{", "OPENING_CURLY_BRACKET", 23);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\}", "CLOSING_CURLY_BRACKET", 24);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ",", "COMMA", 25);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ":", "COLON", 26);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "=", "EQUALS_SIGN", 27);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\(", "OPENING_BRACKET", 28); // used in Regex to build blocks
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\)", "CLOSING_BRACKET", 29); // used in Regex to build blocks
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\[", "OPENING_ANGULAR_BRACKET", 30); // used in Regex to build character classes
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\]", "CLOSING_ANGULAR_BRACKET", 31); // used in Regex to build character classes
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ".", "DOT", 32);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "&", "AMPERSAND", 33);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\!", "EXCLAMATION_MARK", 34); // ???
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "~", "TILDE", 35);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\-", "MINUS", 36);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\+", "PLUS", 37); // used in Regex as Repeat(1, std::usize::MAX)
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\*", "ASTERISK", 38); // used in Regex as Repeat(0, std::usize::MAX)
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "/", "SLASH", 39);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "%", "PERCENT", 40);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "<", "LT", 41);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ">", "GT", 42);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\^", "CIRCUMFLEX", 43); // used in Regex as NEGATION operator
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\|", "OR", 44); // used in Regex as OR operator
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\?", "QUESTION_MARK", 45); // used in Regex as Repeat(0, 1)

    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, " ", "WHITESPACE", WHITESPACE_TOKEN_ID);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\n|\r\n|\r", "NEWLINE", NEWLINE_TOKEN_ID);

    // //
    // // Whitespace
    // // ' ' (toke-id: 46)
    // let mut fragment_stack_whitespace = FragmentStack::new();
    // add_character_literal(&mut fragment_stack_whitespace, RegexBuildingBlock::CharacterLiteral(' '), &mut alphabet);
    // // the top fragment on the fragment stack contains the root of the eNFA
    // let mut fragment_whitespace = fragment_stack_whitespace.stack.pop().unwrap();
    // fragment_whitespace.enfa.states.get_mut(&fragment_whitespace.end_id).unwrap().token_id = WHITESPACE_TOKEN_ID;
    // fragment_whitespace.enfa.states.get_mut(&fragment_whitespace.end_id).unwrap().token_name = String::from("WHITESPACE");
    // // Add to lexer
    // let (start_id_whitespace, end_id_whitespace) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_whitespace.enfa, fragment_whitespace.end_id);
    // // add epsilon transitions to all the individual keyword eNFAs
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_whitespace);

    //
    // define keywords last so they have precedence over identifiers!
    //

    // auto        break       case        char 
    // const       continue    default     do 
    // double      else        enum        extern 
    // float       for         goto        if 
    // int         long        register    return 
    // short       signed      sizeof      static 
    // struct      switch      typedef     union 
    // unsigned    void        volatile    while

    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "auto", "AUTO", 100);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "break", "BREAK", 101);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "case", "CASE", 102);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "char", "CHAR", 103);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "const", "CONST", 104);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "continue", "CONTINUE", 105); // continue  // 105  
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "default", "DEFAULT", 106); // default     
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "do", "DO", 107); // do  
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "double", "DOUBLE", 108); // double      
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "else", "ELSE", 109);        
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "enum", "ENUM", 110); // enum      // 110  
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "extern", "EXTERN", 111); // extern 
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "float", "FLOAT", 112); // float       
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "for", "FOR", 113); // for         
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "goto", "GOTO", 114); // goto        
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "if", "IF", 115);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "int", "INT", 116);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "long", "LONG", 117);  // long        
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "register", "REGISTER", 118); // register   
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "return", "RETURN", 119);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "short", "SHORT", 120); // short   // 120    
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "signed", "SIGNED", 121); // signed      
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "sizeof", "SIZEOF", 122); // sizeof      
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "static", "STATIC", 123); // static 
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "struct", "STRUCT", 124); // struct      
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "switch", "SWITCH", 125); // switch      // 125
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "typedef", "TYPEDEF", 126); // typedef     
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "union", "UNION", 127); // union 
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "unsigned", "UNSIGNED", 128); // unsigned    
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "void", "VOID", 129);    
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "volatile", "VOLATILE", 130); // volatile    
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "while", "WHILE", 131); // while

    //
    // Phase 3 - Convert eNFA to DFA
    //

    let mut dfa = enfa_to_dfa(&mut combined_fragment.enfa, &mut alphabet);

    // DEBUG - print to dot file format for debugging with https://dreampuf.github.io/GraphvizOnline
    // enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");

    //
    // Phase 4 - lex some input
    //

    // NUMERIC PLUS NUMERIC
    // let str = "2 + 2";

    // NUMERIC PLUS NUMERIC SEMICOLON
    // let str = "2 + 2;";

    // IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET RETURN NUMERIC SEMICOLON
    // let str = "if (1 < 2) return 2;";

    // IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET RETURN NUMERIC PLUS NUMERIC SEMICOLON
    // let str = "if (1 < 2) return 2 + 2;";

    // IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_CURLY_BRACKET RETURN NUMERIC PLUS NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "if (1 < 2) { }";

    // IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_CURLY_BRACKET RETURN SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "if (1 < 2) { return; }";

    // let str = "if (1 < 2) { return 0; }";

    // let str = "if (1 < 2) { return 2 + 2; }";

    // let str = "if (1 < 2) { return 2 + 2; } if (1 < 2) { return 2 + 2; }";

    // let str = "{ if (1 < 2) { return 2 + 2; } if (1 < 2) { return 2 + 2; } }";

    // https://github.com/nlsandler/writing-a-c-compiler-tests/blob/main/tests/chapter_1/valid/return_0.c
    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "int main() { return 2; }";
    // let str = "int main() { return void; }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET INT IDENTIFIER SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "int main() { int abc; }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET INT IDENTIFIER SEMICOLON RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "int main() { int abc; return 2; }";

    // let str = "int main() { if (1 < 2) return 2; }";
    // let str = "int main() { if (1 < 2) { return 2; } }";
    
    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_CURLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "int main() { if (1 < 2) { return 2; } return 0; }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_CURLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET ELSE OPENING_CURLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET CLOSING_CURLY_BRACKET
    // let str = "int main() { if (1 < 2) { return 2; } else { return 3; } }";
    // let str = "int main() { if (1 < 2) { return; } else { return; } }";
    // let str = "int main() { if (1 < 2) {} else {} }";
 
    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IF OPENING_BRACKET VOID CLOSING_BRACKET OPENING_CURLY_BRACKET CLOSING_CURLY_BRACKET ELSE OPENING_CURLY_BRACKET CLOSING_CURLY_BRACKET CLOSING_CURLY_BRACKET
    // let str = "void main() { if (void) {} else {} }";
    // let str = "void main() { if (1) {} else {} }";
    // let str = "void main() { if (1) { return; } else {} }";
    // let str = "void main() { if (1) { return; } else {return 0; } }";
    // let str = "void main() { if (1 < 2) { return; } else {return 0; } }";
    // let str = "void main() { if (1 < 2) { return 2 + 2; } else {return 0; } }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IF OPENING_BRACKET EXPRESSION_STOP CLOSING_BRACKET OPENING_CURLY_BRACKET STATEMENT_STOP CLOSING_CURLY_BRACKET ELSE OPENING_CURLY_BRACKET STATEMENT_STOP CLOSING_CURLY_BRACKET CLOSING_CURLY_BRACKET
    // let str = "int main() { if (EXPRESSION_STOP) { STATEMENT_STOP } else { STATEMENT_STOP } }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IF OPENING_BRACKET EXPRESSION_STOP CLOSING_BRACKET STATEMENT_STOP ELSE STATEMENT_STOP CLOSING_CURLY_BRACKET
    // let str = "int main() { if (EXPRESSION_STOP) STATEMENT_STOP else STATEMENT_STOP }";

    // IF OPENING_BRACKET EXPRESSION_STOP CLOSING_BRACKET STATEMENT_STOP ELSE STATEMENT_STOP
    // let str = "if ( EXPRESSION_STOP ) STATEMENT_STOP else STATEMENT_STOP";
    // let str = "if ( void ) void else void";
    // let str = "if ( void ) return; else return;";
    // let str = "if ( void ) return; else if ( void ) return;";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_CURLY_BRACKET RETURN NUMERIC PLUS NUMERIC SEMICOLON CLOSING_CURLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "int main() { if (1 < 2) { return 2 + 2; } return 0; }";
    // let str = "int main() { int a; if (1 < 2) { return 2 + 2; } return 0; }";

    // let str = "int main() { int a; }";
    // let str = "int main() { int a = 0; }";
    // let str = "int main() { int a = 2; a *= 100; }";

    // let str = "int main() { int a = 1 + 1; }";
    // let str = "int main() { int a = 1 - 1; }";
    // let str = "int main() { int a = 1 * 1; }";
    // let str = "int main() { int a = 1 / 1; }";
    // let str = "int main() { int a = 1 % 1; }";

    // let str = "int main() { int a = 1; int b = 2; }";
    // let str = "int main() { int a = 1; int b = 2; a > b ? 1 : 2 ; }";
    // let str = "int main() { int a = 1; int b = 2; (a > b) ? 1 : 2 ; }"; // INVALID BUT SHOULD BE ALLOWED
    // let str = "int main() { int a = 1; int b = 2; a > b ? a++ : b++ ; }";
    // let str = "int main() { int a = 1; int b = 2; a > b ? a = 0 : b = 0; }"; // NOT WORKING YET

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET INT IDENTIFIER EQUALS_SIGN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "int main() { int a = 0; }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET INT IDENTIFIER IDENTIFIER EQUALS_SIGN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "int main() { int a c = 0; }"; // This is accepted for some reason!!!!!
    // let str = "int main() { mullmull a c = 0; }"; // INVALID!

    // let str = "int main() { int a = 0; if (1 < 2) { return 2 + 2; } return 0; }";
    // let str = "int main() { int a = 0; if (1 < 2) { return 2 % 2; } return 0; }";
    // let str = "int main() { int a = 0; if (1 < 2) { return 2 % 2; } else { return 2 + 2; } return 0; }";

    // let str = "int main() { int a; int b; c = a && b; }";
    // let str = "int main() { int a; int b; c = a || b; }";

    // let str = "int main() { int a; int b; c = a | b; }";
    // let str = "int main() { int a; int b; c = a ^ b; }";

    // let str = "int main() { int a; a = a << 1; }";
    // let str = "int main() { int a; a = a >> 1; }";

    // let str = "int main() { if (1 == 2) { return; } }";
    // let str = "int main() { if (1 != 2) { return; } }";

    // let str = "int main() { if (1 < 2) { return; } }";
    // let str = "int main() { if (1 <= 2) { return; } }";
    // let str = "int main() { if (1 > 2) { return; } }";
    // let str = "int main() { if (1 >= 2) { return; } }";

    // let str = "int main() { const int abc = 3; }";
    // let str = "int main() { ; }";

    // let str = "int main() { int a; int b; a = (int) b; }";

    // let str = "int main() { while (a < b) { void; } }";
    // let str = "int main() { while (1) { void; } }";
    // let str = "int main() { while (1) { if (a < b) return 0; } }";

    // let str = "int main() { do { if (a < b) return 0; } while (1); }";

    // let str = "int main() { for ( i = 0; a < 10; i++ ) { return; } }";

    // let str = "int main() { int a = (float) b; }";
    // let str = "int main() { int a = (float) * b; }";

    // let str = "int main() { switch (data) { case const_1: break; case const_2: break; default: break; } }";

    // let str = "int main() { switch (data) { case const_1: if (1 < 2) { return; } break; case const_2: { int a = (float) b; int a = (float) b; } break; default: break; } }";

    // let str = "int main() { }";

    // let str = "enum days_enum { AA, BB };";
    // let str = "enum days_enum { MON, TUE, WED, THU, FRI, SAT, SUN };";

    // struct Person {
    //     char name[50];
    //     int alter;
    //     float gehalt;
    // };
    // let str = "struct Person { int alter; float gehalt; };";
    // let str = "struct Person { int data[50]; };";
    // let str = "struct Person { char name[50]; int alter; float gehalt; };";

    // let str = "int zahlen[5];";
    // let str = "int zahlen[5]; int main() { zahlen[0] = 15; }";

    // let str = "int main() { int alter = 25; int *zeiger = &alter; }";

    // let str = "int main() { data_struct.field = 4; }";

    // let str = "int main() { data_struct->field = 4; }";

    // let str = "union Data { int i; float f; char str[20]; }; int main() { union Data data; data.i = 10; data.f = 220; }";

    // let str = "int main(int x, int y) { return x + y; }";

    // let str = "\"aaa\"";
    // let str = "int main() { char *message = \"aaa\"; }";
    // let str = "int main() { char *message = \"This is a string literal.\"; }";
    // let str = "int main() { char *message = \"This is a string literal.\"; }";
    // let str = "int main() { printf(\"This is a string literal: %d.\", 199); }";

    // let str = "int main(int argc, char **argv) { int (*say)(const char *); }";
    // let str = "int main(int argc, char **argv) { int (*say)(const char *); say = puts; }";
    // let str = "int main(int argc, char **argv) { int (*say)(const char *); say = puts; say(\"hello world\"); }";
    // let str = "int main(int argc, char **argv) { int (*say)(const char *); say = puts; say(\"hello world\"); return 0; }";

    // INT IDENTIFIER SEMICOLON
    // let str = "int abc;";

    // let str = "float abc;";
    // let str = "float abc = 1.0;";

    // VOID VOID VOID VOID VOID
    // let str = "void void void void void";

    // INT PLUS VOID
    // let str = "int + void";

    // let str = "int main() { celsius = 5; }";
    // let str = "int main() { celsius = 5.0f; }";

    // STRUCT IDENTIFIER IDENTIFIER EQUALS_SIGN OPENING_CURLY_BRACKET VOID COMMA VOID CLOSING_CURLY_BRACKET SEMICOLON
    // let str = "struct point p1 = { void, void };";

    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IDENTIFIER DOT IDENTIFIER EQUALS_SIGN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // let str = "void main () { p1.x = 2; }";
    // let str = "void main () { struct point p1 = { 1, 2 }; p1.x = 2; }";
    // let str = "int main () { struct point p1 = { 1, 2 }; p1.x = 2; return p1.x; }";
    // let str = "typedef struct point point_t; int main () { struct point p1 = { 1, 2 }; p1.x = 2; return p1.x; }";
    // let str = "struct point { int x; int y; }; typedef struct point point_t; int main () { struct point p1 = { 1, 2 }; p1.x = 2; point_t pp; return p1.x; }";

    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IDENTIFIER IDENTIFIER SEMICOLON CLOSING_CURLY_BRACKET
    let str = "void main () { point_t pp; }";

    //
    // Kernighan & Ritchie
    //

    // let str: String = fs::read_to_string("res/C/samples/kernighan_ritchie/page_9.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/kernighan_ritchie/page_10.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/kernighan_ritchie/page_10_scratchpad.c").expect("file cannot be read!");

    //
    // Nora Sandler
    //

    // let str: String = fs::read_to_string("res/C/samples/nora_sandler/page_26.c").expect("file cannot be read!");

    //
    // C Samples
    //

    // let str: String = fs::read_to_string("res/C/samples/c_samples/hex_numeric_0.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/expression_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/expression_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/expression_2.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/if_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/if_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/if_else_if_0.c").expect("file cannot be read!");
    
    // let str: String = fs::read_to_string("res/C/samples/c_samples/function_call_0.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/for_loop_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/for_loop_1.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/main_0.c").expect("file cannot be read!");

    // let str: String = fs::read_to_string("res/C/samples/c_samples/struct_0.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/struct_1.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/struct_2.c").expect("file cannot be read!");
    // let str: String = fs::read_to_string("res/C/samples/c_samples/struct_3.c").expect("file cannot be read!");

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

    // // init
    // let mut current_state_id: usize = 0;
    // let mut stack = Vec::<RuleElement<String>>::new();

    // init
    let mut parser: Parser<String> = Parser::<String>::new(parse_table);

    let mut step: usize = 1;

    let mut current_state_id = dfa.start_state_id;
    // let mut last_state_id = dfa.start_state_id;

    let mut current_character: char = 'x';
    let mut lookahead_character: char = 'y';
    let mut has_lookahead_character = false;

    let mut token_string_buffer = String::from("");

    // DEBUG outputting parse tree as DOT graph
    let mut string_buffer = String::from("");
    let mut debug_node_stack = Vec::<DebugNode>::new();

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
        current_state_id = consume_character(&mut dfa, current_state_id, &mut token_string_buffer, current_character, lookahead_character, &mut step, &mut parser, &grammar_state_hashmap, &mut string_buffer, &mut debug_node_stack);
    }

    // consume the lookahead from the very last cycle as a normal input. Specify dummy lookahead character.
    current_state_id = consume_character(&mut dfa, current_state_id, &mut token_string_buffer, lookahead_character, 'x', &mut step, &mut parser, &grammar_state_hashmap, &mut string_buffer, &mut debug_node_stack);

    let lexer_debug: bool = false;
    if lexer_debug {
        println!("[LEXER] Emitting '{}'. Token-Id: {}, Token-Name: {}", token_string_buffer, dfa.states[&current_state_id].token_id, dfa.states[&current_state_id].token_name);
        println!("");
    }
    
    // provide the last token to the parser
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(dfa.states[&current_state_id].token_name.clone()), &token_string_buffer, &mut string_buffer, &mut debug_node_stack);
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure, &token_string_buffer, &mut string_buffer, &mut debug_node_stack);

    println!("");
    println!("https://dreampuf.github.io/GraphvizOnline");
    println!("");
    println!("digraph {{");
    print!("{}", string_buffer);
    println!("}}");
    println!("");

    // 1. Create or overwrite the file
    let file = File::create("parse_tree.dot").expect("Create file failed!");
    
    // 2. Wrap the file in a BufWriter
    let mut writer = BufWriter::new(file);

    // 3. Write data
    write!(writer, "{}", "digraph {{");
    write!(writer, "{}", string_buffer);
    write!(writer, "{}", "}}");

    // 4. Explicitly flush the remaining data to disk
    writer.flush().expect("flush failed!");
    */
    
    println!("end");
}