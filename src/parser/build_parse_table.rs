use std::collections::HashMap;
use std::collections::BTreeMap;

use crate::parser::rule::Rule;
use crate::parser::rule::RuleElement;
use crate::ParseTableCell;
use crate::Transition;

use crate::parser::grammar_state::GrammarState;

pub fn build_parse_table(parse_table: &mut HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>,
    grammar_state_hashmap: &mut BTreeMap<usize, GrammarState<String>>,
    rule_channel_map: &HashMap::<usize, Vec::<Transition<String>>>,
    augmented_start_symbol: &RuleElement<String>,
    rule_id_to_state_id_map: &HashMap::<usize, usize>) {

    // The parse table contains a row per state.
    // Column-wise, the parse table has two general parts which are ACTION and GOTO.
    //
    // The ACTION part contains columns for each NonTerminal and the EOI (#) symbol
    // The cells contain either nothing or one or more of the actions { shift, reduce }
    // 
    // A shift-action also contains the id of the next state to transition to after executing the action.
    // A reduce-action also contains the id of the rule to reduce. The DFA remains in the same state after reduce.
    //
    // For leaf-states in the DFA, enter reduce-actions based on the lookaheads. (A leaf has the dot-marker after the rhs-symbols)
    // For inner-state in the DFA, enter shift-actions to target states based on the symbol attached to the edges of the graph (transitions)
    // 
    // Conflicts:
    // If a cell in the ACTIONS part contains shift and reduce at the same time, this is a shift reduce conflict.
    // If a cell in the ACTIONS part contains more than one reduce for non-terminals, then this is a reduce reduce conflict.
    // Any conflict means that the grammar needs to be reworked or that some priority tricks need to be applied.
    //
    // The GOTO section contains columns for NonTerminals. 
    // The cells contain the state ids to transtition to when the NonTerminal is detected.

    // SideNote:
    // When code generation is used, large nested switches can be generated to implement the table.
    // When the parse table is generated at runtime, each row will become a map from.

    // https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html (do not add augmented start rule)
    // https://jsmachines.sourceforge.net/machines/lalr1.html (add augmented start rule)

    // visiting all states because each state causes one row in the parse_table
    // put start state id into process_list
    // visited_list is empty
    // while process_list is not empty
    //  - take state_id from process_list
    //  - if state_id is in visited_list, skip it
    //  - else, place state_id into visited_list
    //  - put all reachable state id's into process_list
    //  - construct a row from parse_table

    let rust_is_a_kek = grammar_state_hashmap.clone();

    let mut process_list = Vec::<usize>::new();
    let mut visited_list = Vec::<usize>::new();

    process_list.push(0);

    let parse_table_debug = false;
    let mut found_final_state: bool = false;
    let mut final_state_id: usize = 0;

    while process_list.len() > 0 {

        let current_state_id = process_list.remove(0);

        // DEBUG
        if parse_table_debug {
            println!("");
            println!("Visiting State-ID: {}", current_state_id);
        }

        // if current_state_id == 9 {
        //     println!("test");
        // }

        visited_list.push(current_state_id);

        let state = rust_is_a_kek.get(&current_state_id).unwrap();
        // println!("{:?}", state);

        //
        // convert DFA state into parse table row
        //
        
        let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();

        // over the identification rules of the state
        for i in 0..state.identification_rules.len() {

            let current_rule = &state.identification_rules[i];

            if parse_table_debug {
                println!("  rule: {:?}", current_rule);
            }

            // if current_rule.dot_idx >= current_rule.rhs.len() {
            //     println!("leaf");
            // } else {
            //     println!("inner");
            //     println("GOTO: {:?} -{:?}-> {:?}", state.id, current_rule)
            // }

            // retrieve id of identification rule
            let rule_id = current_rule.id;
            // println!("rule-id: {}", rule_id);

            // resolve target rules that the current rule points to
            let target_rule_ids_option = rule_channel_map.get(&rule_id);

            // the rule that leads to the accept state does not point to a state! 
            // The acceptance situation is not implemented as an explicit state.
            // Also leaf-nodes contain rules that are used for reduce only and point to no other states.
            if target_rule_ids_option.is_none() {

                // let last_symbol = &current_rule.rhs[current_rule.rhs.len() - 1];
                // if *last_symbol == start_symbol {
                //     println!("    ACCEPT 1");
                //     parse_table_row.insert(RuleElement::Closure, ParseTableCell::<usize>::Accept);
                // }
                
                if current_rule.lhs == *augmented_start_symbol {
                    if parse_table_debug {
                        println!("    ACCEPT 1");
                    }
                    parse_table_row.insert(RuleElement::Closure, ParseTableCell::<usize>::Accept);

                    found_final_state = true;
                    final_state_id = current_state_id;
                }
                else {
                    if parse_table_debug {
                        println!("    REDUCE: {:?}", rule_id);
                    }

                    for lookahead_index in 0..current_rule.lookahead.len() {
                        parse_table_row.insert(current_rule.lookahead[lookahead_index].clone(), ParseTableCell::<usize>::Reduce(rule_id));
                    }
                }

                continue;
            }

            let target_rule_ids = target_rule_ids_option.unwrap();
            // println!("target_rule_ids: {:?}", target_rule_ids);

            // for all target rules which point to connected states
            for target_rule_id in target_rule_ids {

                // resolve target rule into the target state (= state which contains the target rule)
                let target_state_id = rule_id_to_state_id_map.get(&target_rule_id.0).unwrap();

                if parse_table_debug {
                    println!("  Target State: {} over symbol: {:?}", target_state_id, &target_rule_id.1);
                }

                if current_rule.dot_idx >= current_rule.rhs.len() {

                    // panic!("    leaf node");
                    
                } else {

                    // println!("    inner node");

                    match &target_rule_id.1 {
                        RuleElement::NonTerminal(_) => {
                            if parse_table_debug {
                                println!("    GOTO: {:?} -{:?}-> {:?}", state.id, &target_rule_id.1, target_state_id);
                            }

                            parse_table_row.insert(target_rule_id.1.clone(), ParseTableCell::<usize>::Goto(*target_state_id));
                        }
                        RuleElement::Terminal(_) => {
                            if parse_table_debug {
                                println!("    SHIFT: {:?} -{:?}-> {:?}", state.id, &target_rule_id.1, target_state_id);
                            }

                            parse_table_row.insert(target_rule_id.1.clone(), ParseTableCell::<usize>::Shift(*target_state_id));
                        }
                        _ => {
                            panic!("    Panic: {:?}", &target_rule_id.1);
                        }
                    }
                }

                // check if state is already visited or already on the process list
                if visited_list.contains(target_state_id) {
                    // println!("continue");
                    continue;
                }
                if process_list.contains(target_state_id) {
                    // println!("continue");
                    continue;
                }

                // add to list for further processing
                process_list.push(*target_state_id);
            }
        }

        // over the normal rules of the state
        for i in 0..state.rules.len() {

            let current_rule = &state.rules[i];

            if parse_table_debug {
                println!("  rule: {:?}", current_rule);
            }

            // retrieve id of rule
            let rule_id = current_rule.id;

            // resolve target rules that the current rule points to
            let target_rule_ids_option = rule_channel_map.get(&rule_id);

            // the rule that leads to the accept state does not point to a state! 
            // The acceptance situation is not implemented as an explicit state.
            // Also leaf-nodes contain rules that are used for reduce only and point to no other states.
            if target_rule_ids_option.is_none() {

                let last_symbol = &current_rule.rhs[current_rule.rhs.len() - 1];

                // if *last_symbol == start_symbol {
                //     println!("    ACCEPT 2");
                //     parse_table_row.insert(RuleElement::Closure, ParseTableCell::<usize>::Accept);
                // } 
                
                if current_rule.lhs == *augmented_start_symbol {
                    if parse_table_debug {
                        println!("    ACCEPT 2");
                    }
                    parse_table_row.insert(RuleElement::Closure, ParseTableCell::<usize>::Accept);

                    found_final_state = true;
                    final_state_id = current_state_id;
                }
                else {
                    if parse_table_debug {
                        println!("    REDUCE: {:?}", rule_id);
                    }
                    for lookahead_index in 0..current_rule.lookahead.len() {
                        parse_table_row.insert(current_rule.lookahead[lookahead_index].clone(), ParseTableCell::<usize>::Reduce(rule_id));
                    }
                }

                continue;
            }

            let target_rule_ids = target_rule_ids_option.unwrap();
            // println!("target_rule_ids: {:?}", target_rule_ids);

            // for all target rules which point to connected states
            for target_rule_id in target_rule_ids {

                // resolve target rule into the target state (= state which contains the target rule)
                let target_state_id = rule_id_to_state_id_map.get(&target_rule_id.0).unwrap();

                if parse_table_debug {
                    println!("  Target State: {} over symbol: {:?}", target_state_id, &target_rule_id.1);
                }

                if current_rule.dot_idx >= current_rule.rhs.len() {

                    // panic!("    leaf");
                    
                } else {

                    // println!("    inner");
                    match &target_rule_id.1 {

                        RuleElement::NonTerminal(_) => {
                            if parse_table_debug {
                                println!("    GOTO: {:?} -{:?}-> {:?}", state.id, &target_rule_id.1, target_state_id);
                            }

                            parse_table_row.insert(target_rule_id.1.clone(), ParseTableCell::<usize>::Goto(*target_state_id));
                        }

                        RuleElement::Terminal(_) => {
                            if parse_table_debug {
                                println!("    SHIFT: {:?} -{:?}-> {:?}", state.id, &target_rule_id.1, target_state_id);
                            }

                            parse_table_row.insert(target_rule_id.1.clone(), ParseTableCell::<usize>::Shift(*target_state_id));
                        }

                        RuleElement::Epsilon => {
                            if parse_table_debug {
                                println!("    SHIFT: {:?} -{:?}-> {:?}", state.id, &target_rule_id.1, target_state_id);
                            }

                            parse_table_row.insert(target_rule_id.1.clone(), ParseTableCell::<usize>::Shift(*target_state_id));
                        }

                        _ => {
                            panic!("    Panic: {:?}", &target_rule_id.1);
                        }
                    }
                }

                // check if state is already visited or already on the process list
                if visited_list.contains(target_state_id) {
                    // println!("continue");
                    continue;
                }
                if process_list.contains(target_state_id) {
                    // println!("continue");
                    continue;
                }

                // add to list for further processing
                process_list.push(*target_state_id);
            }
        }

        parse_table.insert(current_state_id, parse_table_row);
    }

    //
    // Validate
    //

    if !found_final_state {
        panic!("DFA no final state detected!");
    } else {
        println!("Final state: {}", final_state_id);
    }

    //
    // DEBUG: Print the parse table
    //

    println!("");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("ParseTable");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    // let debug_print_parse_table = true;
    let debug_print_parse_table = false;

    if debug_print_parse_table {
        for i in 0..parse_table.len() {
            println!("{}) {:?}", i, parse_table[&i]);
        }
    }
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
}