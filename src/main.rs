// filename: main_driver.rs

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
// use std::borrow::BorrowMut; // DO NOT IMPORT THIS! https://www.reddit.com/r/rust/comments/1cbsbdu/how_to_get_value_out_of_an_rcrefcell/

use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;

use std::rc::Rc;
use std::cell::RefCell;

use crate::parser::parser::AST_NODE_ID_COUNTER;
use crate::parser::rule::read_rule_map;
use crate::parser::rule::serialize_rules;

mod common;
use crate::common::variable_naming_source::VariableNamingSource;
use crate::common::symbol_table::SymbolTable;
use crate::common::file_handling::write_string_to_file;

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
use crate::regex::enfa::enfa_serialize;
use crate::regex::enfa::enfa_deserialize;

mod parser;
use crate::parser::parser::ParseTableCell;
use crate::parser::parser::Transition;
use crate::parser::parser::Parser;
use crate::parser::parser::DebugNode;
use crate::parser::rule::Rule;
use crate::parser::rule::RuleElement;
use crate::parser::propagation::perform_propagation;
use crate::parser::first::compute_first_original;
use crate::parser::build_parse_table::build_parse_table;
use crate::parser::parser::output_parse_table_to_csv;
use crate::parser::parser::read_parse_table_from_csv;
use crate::parser::perform_lalr_1::perform_lalr_1;
use crate::parser::nullable_sets::compute_nullable_sets;
use crate::parser::validate_grammar::validate_grammar;
use crate::parser::print_rules::print_rules;

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

mod c_ast;
use crate::c_ast::ast_node::AstNode;
use crate::c_ast::ast_node::AstNodeType;
// use crate::c_ast::identifier_resolution_visitor::IdentifierResolutionVisitor;
// use crate::c_ast::type_checking_visitor::TypeCheckingVisitor;

mod tacky;
use crate::tacky::tacky::Instruction;
// use crate::tacky::tacky_visitor::TackyVisitor;
use crate::tacky::tacky::Program;
use crate::tacky::tacky::print_tacky_program;

mod asm_ast;
use crate::asm_ast::asm_ast::AsmAstProgram;
use crate::asm_ast::asm_ast_conversion_visitor::AsmAstConversionVisitor;
use crate::asm_ast::asm_ast_fixup_visitor::AsmAstFixupVisitor;
use crate::asm_ast::asm_ast_emitter_visitor::AsmAstASEmitterVisitor;
use crate::asm_ast::asm_ast_masm_emitter_visitor::AsmAstMasmEmitterVisitor;
use crate::asm_ast::asm_ast::print_asm_ast_program;

// const BASE_PATH: &'static str = "C:\\Users\\U5353\\source\\repos\\x64_test\\";
const BASE_PATH: &'static str = "";

// https://stackoverflow.com/questions/32935808/generate-sequential-ids-for-each-instance-of-a-struct
static RULE_COUNTER: AtomicUsize = AtomicUsize::new(0);
static STATE_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn print_ast(program_ast_node_id: &usize, node_map: &Box<HashMap::<usize, AstNode>>) {

    let debug = false;

    // println!("");
    // println!("---------------------------------------------------------------------------------");
    // println!("{{");
    // println!("\"function_definitions\": [");
    // program_ast_node.pretty_print_ast_json();
    // println!("]");
    // println!("}}");
    // println!("---------------------------------------------------------------------------------");

    if debug {
        println!("");
        println!("---------------------------------------------------------------------------------");
    }

    let mut ast_string_buffer = String::from("");

    // serialize the AST into .dot graphviz format
    ast_string_buffer.push_str("digraph {\n");
    if let Some(body_ast_node) = node_map.get(&program_ast_node_id) {
        body_ast_node.pretty_print_ast_dot(&mut ast_string_buffer, &node_map);
    }
    ast_string_buffer.push_str("}");

    // DEBUG - print AST dot to console
    // let output_ast_as_dot_to_console: bool = true;
    let output_ast_as_dot_to_console: bool = false;
    if output_ast_as_dot_to_console {
        println!("{}", ast_string_buffer);
    }

    // DEBUG - print AST dot to dot file (before variable replacement)
    let output_abstract_syntax_tree_as_dot_to_file: bool = true;
    // let output_abstract_syntax_tree_as_dot_to_file: bool = false;
    if output_abstract_syntax_tree_as_dot_to_file {

        // https://dreampuf.github.io/GraphvizOnline

        // 1. Create or overwrite the file
        let file = File::create("abstract_syntax_tree.dot").expect("Create file failed!");

        // 2. Wrap the file in a BufWriter
        let mut writer = BufWriter::new(file);

        // 3. Write data
        write!(writer, "{}", ast_string_buffer);

        // 4. Explicitly flush the remaining data to disk
        writer.flush().expect("flush failed!");
    }

    if debug {
        println!("---------------------------------------------------------------------------------");
    }
}

fn main() {

    let debug = false;

    println!("start");

    let mut parse_table = HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>::new();

    //
    // You only need to generate the LALR_1 parser if you have not done so before
    // or if you have changed the grammar. Otherwise, the application will read the generated
    // parse table and rules from the files parse_table.txt and rule_table.txt
    //
    // https://jsmachines.sourceforge.net/machines/lalr1.html
    //

    // let generate_lalr_1 = true;
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

        if debug {
            println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
            println!("LALR(1) generation channel algorithm starts                                      ");
            println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        }

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

        if debug {
            println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
        }

        //
        // Propagation Cycles
        //

        if debug {
            println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        }
        perform_propagation(&rule_ids, &rule_id_to_state_id_map, &rule_channel_map, &mut grammar_state_hashmap);
        if debug {
            println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
        }

        if debug {
            println!("");
            println!("*********************************************************************************");
            println!("RESULT - FINISHED - READY - RESULT - FINISHED - READY - RESULT - FINISHED - READY");
            println!("*********************************************************************************");
        }

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

        if debug {
            println!("");
            println!("*********************************************************************************");
            println!("Building the Parse Table from the LALR(1) DFA                                    ");
            println!("*********************************************************************************");
        }

        build_parse_table(&mut parse_table,
            &mut grammar_state_hashmap,
            &rule_channel_map,
            &augmented_start_symbol,
            &rule_id_to_state_id_map);

        if debug {
            println!("*********************************************************************************");
        }

        //
        // Output Parse Table into CSV file
        //

        let mut rule_map = BTreeMap::<usize, Rule<String>>::new();
        let mut parse_table_string_buffer = String::new();
        let mut rules = Vec::<Rule::<String>>::new();
        let mut rule_ids = Vec::<usize>::new();
        output_parse_table_to_csv(&mut parse_table_string_buffer,
            &parse_table, &grammar_state_hashmap, &mut rules, &mut rule_ids, &mut rule_map);
        write_string_to_file("parse_table.txt", &parse_table_string_buffer);

        //
        // Output Rule Table into file
        //

        let mut rules_string_buffer = String::new();
        serialize_rules(&rules, &mut rules_string_buffer);
        write_string_to_file("rule_table.txt", &rules_string_buffer);
    }

    //
    // reading back the rule_map (!= Parse Table) from file.
    //
    // The rule map contains all rules from the grammar including the rule's id.
    // The rule map is generated by the LALR(1) generator from the original grammar definition.
    //
    // If the user decides to not run the LALR(1) generator but if they load
    // a pre-generated parse-table from a earlier LALR(1) generator-run, which
    // has been persisted to a file, then there is no parse table!
    //
    // This is the reason why in addition to the parse-table itself, the rule-map
    // needs to be also serialized and deserialized when skipping LALR(1) generation!
    //

    if debug {
        println!("");
        println!("*********************************************************************************");
        println!("Reading back the rule_map from file                                              ");
        println!("*********************************************************************************");
    }

    let mut rule_map = BTreeMap::<usize, Rule<String>>::new();
    let rule_map_filename = "rule_table.txt";
    read_rule_map(rule_map_filename, &mut rule_map);

    if debug {
        println!("*********************************************************************************");
    }

    //
    // reading back pregenerated the parse table from file
    //

    if debug {
        println!("");
        println!("*********************************************************************************");
        println!("Reading back the parse table from file                                           ");
        println!("*********************************************************************************");
    }

    parse_table.clear();
    read_parse_table_from_csv("parse_table.txt", &mut parse_table);

    if debug {
        println!("*********************************************************************************");
    }

    //
    // Build the Lexer
    //

    if debug {
        println!("");
        println!("*********************************************************************************");
        println!("Building the Lexer (This may take some time ...) or load from file.              ");
        println!("*********************************************************************************");
    }

    let temp_q0 = State::new(0);
    let mut dfa = EpsilonNfa::new(temp_q0);

    // let generate_lexer = true;
    let generate_lexer = false;
    if generate_lexer {
        dfa = produce_c_lexer();
        // store into file
        enfa_serialize(&mut dfa, "enfa.txt");
    } else {
        // load from file
        enfa_deserialize(&mut dfa, "enfa.txt");
    }

    if debug {
        println!("*********************************************************************************");
    }

    //
    // Process some input
    //

    // ( str, filename )
    let input_tuple = provide_sourcode_input();

    // DEBUG
    if debug {
        println!("");
        println!("");
        println!("Input:\n{}", input_tuple.0); // text data
        println!("Filename:\n{}", input_tuple.1); // filename
    }

    //
    // Driving the parser against input
    //

    if debug {
        println!("");
        println!("*********************************************************************************");
        println!("Driving the parser against input                                                 ");
        println!("*********************************************************************************");
    }

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

    let lexer_debug: bool = false;
    let lexer_token_debug: bool = false;
    let mut lexer: Lexer = Lexer::new(dfa, lexer_debug, lexer_token_debug);

    let mut step: usize = 1;

    let mut current_character: char = 'x';
    let mut lookahead_character: char = 'y';
    let mut has_lookahead_character = false;

    let mut node_map = Box::new(HashMap::<usize, AstNode>::new());

    //
    // DOT Graph
    //
    // DEBUG outputting parse tree as DOT graph
    //

    let mut debug_node_string_buffer = String::from("");
    let mut debug_node_stack = Vec::<DebugNode>::new();

    // build the root node which is of type program
    let program_ast_node_id = AST_NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    let mut program_ast_node: AstNode = AstNode::new(program_ast_node_id);

    //
    // Feed the input source code to the lexer
    //

    let mut line_number: usize = 1;

    for character in input_tuple.0.chars() {

        current_character = lookahead_character;
        lookahead_character = character;

        // the very first iteration is here to load the lookahead character
        if !has_lookahead_character {
            has_lookahead_character = true;
            continue;
        }

        // TODO: the lookahead character is not used at all!
        // Remove it! It makes the parser loop more complicated
        lexer.consume_character(
            current_character,
            lookahead_character,
            &mut step,
            &mut parser,
            &rule_map,
            &mut debug_node_string_buffer,
            &mut debug_node_stack,
            &input_tuple.1,
            line_number,
            &mut node_map
        );

        if character == '\n' {
            line_number = line_number + 1;
        }
    }

    // consume the lookahead from the very last cycle as a normal input. Specify dummy lookahead character.
    lexer.consume_character(
        lookahead_character,
        'x',
        &mut step,
        &mut parser,
        &mut rule_map,
        &mut debug_node_string_buffer,
        &mut debug_node_stack,
        &input_tuple.1,
        line_number,
        &mut node_map
    );

    // // DEBUG
    // let lexer_debug: bool = false;
    // if lexer_debug {
    //     println!("[LEXER] Emitting '{}'. Token-Id: {}, Token-Name: {}", token_string_buffer, dfa.states[&current_state_id].token_id, dfa.states[&current_state_id].token_name);
    //     println!("");
    // }

    // TODO: write line and file into the token before passing it to the parser so that the parser has line and file information

    // DEBUG - this outputs the string and the token generated from the string
    // This is a good starting point for debugging
    if lexer_token_debug {
        println!("[LEXER.TRAP_STATE] {:?} ---> {:?} | File: {:?}, Line: {:?}",
            lookahead_character,
            RuleElement::Terminal(lexer.dfa.states[&lexer.current_state_id].token_name.clone()),
            &input_tuple.1,
            line_number
        );
    }

    // provide the last token to the parser
    lexer.parser_provide_input(
        &mut parser,
        &mut step,
        &rule_map,
        &RuleElement::Terminal(lexer.dfa.states[&lexer.current_state_id].token_name.clone()),
        &mut debug_node_string_buffer,
        &mut debug_node_stack,
        &mut node_map
    );

    // provide EOI (End of Input) to the parser
    lexer.parser_provide_input(
        &mut parser,
        &mut step,
        &rule_map,
        &RuleElement::Closure,
        &mut debug_node_string_buffer,
        &mut debug_node_stack,
        &mut node_map
    );

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
    // let output_parse_tree_as_dot_to_file: bool = false;
    if output_parse_tree_as_dot_to_file {

        // 1. Create or overwrite the file
        let file = File::create("parse_tree.dot").expect("Create file failed!");

        // 2. Wrap the file in a BufWriter
        let mut writer = BufWriter::new(file);

        // 3. Write data
        write!(writer, "{}", "digraph {");
        write!(writer, "{}", debug_node_string_buffer);
        write!(writer, "{}", "}");

        // 4. Explicitly flush the remaining data to disk
        writer.flush().expect("flush failed!");
    }

    //
    // Finalize AST
    //

    if parser.construct_ast {

        // build the root node which is of type program
        program_ast_node.node_type = AstNodeType::Program;

        // insert all nodes into program node
        let mut done = false;
        while !done {

            let body_ast_node_id = parser.ast_stack.pop().unwrap();
            // let body_ast_node = node_map.get(&body_ast_node_id);
            program_ast_node.block_items.push(body_ast_node_id);

            done = parser.ast_stack.len() == 0;
        }

        // place the root-program node onto the stack
        parser.ast_stack.push(program_ast_node_id);

        node_map.insert(program_ast_node.id, program_ast_node);

        //
        // pretty print AST (pre semantic visitor)
        //

        let ast_stack_root_option = parser.ast_stack.pop();
        if let Some(ref program_ast_node_id) = ast_stack_root_option {
            print_ast(program_ast_node_id, &node_map);
        }
/*
        //
        // Semantic Analysis Stage, page 103, 174ff
        //

        if let Some(mut program_ast_node) = ast_stack_root_option {

            // rc - refcell VariableNamingSource so that the IdentifierResolutionVisitor
            // and the TackyVisitor can both use the same object
            let variable_naming_source = VariableNamingSource::new();

            // duplicate the reference counting smart pointer so it can distributed to all users
            let variable_naming_source_rc_1 = Rc::new(RefCell::new(variable_naming_source));
            let variable_naming_source_rc_2 = variable_naming_source_rc_1.clone();
            let variable_naming_source_rc_3 = variable_naming_source_rc_1.clone();

            //
            // add initial scope for global namespace
            //

            // variable_naming_source_rc_3.enter_scope();
            variable_naming_source_rc_3.borrow_mut().enter_scope();

            //
            // 1. Identifier Resolution Phase
            //
            // IdentifierResolutionVisitor
            //
            // has a reference to the VariableNamingSource which is used to
            // output unique variable names and maintains a map from user choosen
            // varible name to unique variable name.
            //
            // The VariableNamingSource also maintains a STACK of mappings
            // between user choosen varible name to unique variable name.
            // This stack of mappings is used to implement block scopes in which
            // variables can be defined and are valid only within the scope they
            // are defined in.
            //

            let mut identifier_resolution_visitor = IdentifierResolutionVisitor::new(variable_naming_source_rc_1);
            identifier_resolution_visitor.visit(&mut program_ast_node);

            //
            // 2. Type Checking Phase - Nora Sandler, page 178ff
            //

            let symbol_table = SymbolTable::new();
            let symbol_table_rc_1 = Rc::new(RefCell::new(symbol_table));

            let mut type_checking_visitor = TypeCheckingVisitor::new(symbol_table_rc_1);
            type_checking_visitor.visit(&mut program_ast_node);

            type_checking_visitor.print_symbol_table();

            //
            // 3. Loop Labeling Phase
            //

            //
            // remove initial scope for global namespace
            //

            variable_naming_source_rc_3.borrow_mut().exit_scope();

            //
            // Output AST post IdentifierResolutionVisitor (replaces variables)
            //

            let mut ast_string_buffer = String::from("");

            ast_string_buffer.push_str("digraph {\n");
            program_ast_node.pretty_print_ast_dot(&mut ast_string_buffer);
            ast_string_buffer.push_str("}");

            // DEBUG - print AST dot to console
            // let output_ast_as_dot_to_console: bool = true;
            let output_ast_as_dot_to_console: bool = false;
            if output_ast_as_dot_to_console {
                println!("{}", ast_string_buffer);
            }

            // DEBUG - print AST dot to dot file
            let output_abstract_syntax_tree_as_dot_to_file: bool = true;
            // let output_abstract_syntax_tree_as_dot_to_file: bool = false;
            if output_abstract_syntax_tree_as_dot_to_file {

                // https://dreampuf.github.io/GraphvizOnline

                // 1. Create or overwrite the file
                let file = File::create("abstract_syntax_tree_post_semantic.dot").expect("Create file failed!");

                // 2. Wrap the file in a BufWriter
                let mut writer = BufWriter::new(file);

                // 3. Write data
                write!(writer, "{}", ast_string_buffer);

                // 4. Explicitly flush the remaining data to disk
                writer.flush().expect("flush failed!");
            }

            //
            // Generate TACKY (from AST)
            //

            let mut tacky_visitor = TackyVisitor::new(variable_naming_source_rc_2);

            tacky_visitor.program.name = String::from(input_tuple.1);
            let mut br_cnt = 0;
            tacky_visitor.visit(&mut program_ast_node, &String::from(""), &mut br_cnt);

            //
            // DEBUG print TACKY statements to file
            //

            let mut string_buffer = String::from("");
            let indent = 0usize;

            print_tacky_program(&tacky_visitor.program, &mut string_buffer, indent);

            // 1. Create or overwrite the file
            let file = File::create("tacky.tky").expect("Create file failed!");

            // 2. Wrap the file in a BufWriter
            let mut writer = BufWriter::new(file);

            // 3. Write data
            write!(writer, "{}", string_buffer);

            // 4. Explicitly flush the remaining data to disk
            writer.flush().expect("flush failed!");

            //
            // Generate Assembler AST (from TACKY)
            //

            let mut asm_ast_conversion_visitor = AsmAstConversionVisitor::new();
            asm_ast_conversion_visitor.visit_tacky_program(&tacky_visitor.program);

            //
            // DEBUG: output intermedate assembler code to file
            //

            let mut string_buffer = String::from("");
            let indent = 0usize;

            print_asm_ast_program(&asm_ast_conversion_visitor.asm_ast_program, &mut string_buffer, indent);

            // 1. Create or overwrite the file
            // extension intasm == intermediate assembler code
            let file = File::create("asm_ast.intasm").expect("Create file failed!");

            // 2. Wrap the file in a BufWriter
            let mut writer = BufWriter::new(file);

            // 3. Write data
            write!(writer, "{}", string_buffer);

            // 4. Explicitly flush the remaining data to disk
            writer.flush().expect("flush failed!");

            //
            // Fixup
            //

            // DEBUG
            // println!("------------------------- Fix up Pseudo Variable -------------------------------");

            let mut asm_ast_fixup_visitor = AsmAstFixupVisitor::new();

            // replace pseudo variables (from TACKY) by addresses on the stack
            // replace illegal MOV (mem2mem) by a combination of mem2reg reg2mem
            asm_ast_fixup_visitor.replace_pseudo = true;
            asm_ast_fixup_visitor.visit_asm_ast_program(&mut asm_ast_conversion_visitor.asm_ast_program);

            // output all statements
            asm_ast_fixup_visitor.replace_pseudo = false;
            asm_ast_fixup_visitor.visit_asm_ast_program(&mut asm_ast_conversion_visitor.asm_ast_program);

            // DEBUG
            // println!("---------------------------------------------------------------------------------");

            //
            // emit assembler instructions
            //

            let emit_gcc = false;
            if emit_gcc {
                println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
                let mut asm_ast_emitter_visitor = AsmAstASEmitterVisitor::new();
                asm_ast_emitter_visitor.visit_asm_ast_program(&mut asm_ast_conversion_visitor.asm_ast_program);
                println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
                println!("gcc -c temp.S -o temp.o");
                println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
            }

            let emit_masm_visual_studio = true;
            // let emit_masm_visual_studio = false;
            if emit_masm_visual_studio {

                // println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
                let mut asm_ast_emitter_visitor = AsmAstMasmEmitterVisitor::new();
                asm_ast_emitter_visitor.visit_asm_ast_program(&mut asm_ast_conversion_visitor.asm_ast_program);
                // println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
                // println!("Use MASM from within Visual Studio (Community Edition)");
                // println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

                // 1. Create or overwrite the file
                // let file = File::create("main.asm").expect("Create file failed!");
                //let file = File::create("C:\\Users\\U5353\\source\\repos\\test_1\\test_1\\main.asm").expect("Create file failed!");
                //let file = File::create("C:\\Users\\U5353\\source\\repos\\x64_test\\main.asm").expect("Create file failed!");
                let file = File::create(BASE_PATH.to_owned() + "main.asm").expect("Create file failed!");

                // 2. Wrap the file in a BufWriter
                let mut writer = BufWriter::new(file);

                // 3. Write data
                write!(writer, "{}", asm_ast_emitter_visitor.string_buffer);

                // 4. Explicitly flush the remaining data to disk
                writer.flush().expect("flush failed!");
            }
        }
*/
    } else {
        println!("AST is empty!");
    }

    println!("end");
}