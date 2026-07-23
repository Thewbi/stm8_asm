use std::collections::HashMap;
use std::collections::BTreeMap;
use std::{
    sync::atomic::{AtomicUsize, Ordering}
};

use crate::Transition;
use crate::STATE_COUNTER;
use crate::RULE_COUNTER;

use crate::parser::rule::Rule;
use crate::parser::rule::RuleElement;

use crate::parser::grammar_state::GrammarState;

pub fn perform_lalr_1(rule_1: &Rule<String>, 
    rule_ids: &mut Vec::<usize>,
    rule_id_to_state_id_map: &mut HashMap::<usize, usize>,
    rule_channel_map: &mut HashMap::<usize, Vec::<Transition<String>>>,
    grammar_rules: &Vec::<Rule<String>>, 
    first: &BTreeMap<RuleElement::<String>, Vec::<RuleElement::<String>>>,
    nullable: &BTreeMap::<RuleElement::<String>, bool>) -> BTreeMap<usize, GrammarState<String>> {

    //
    // Unfold the states (CLOSURE) and build channels between the states.
    //

    let mut found_start_state: bool = false;
    let mut start_state_id: usize = 0;

    // build a state for the first rule
    let first_state_id = STATE_COUNTER.fetch_add(1, Ordering::SeqCst);
    let mut grammar_state: GrammarState<String> = GrammarState::new(first_state_id);
    grammar_state.identification_rules.push(rule_1.clone());

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
            grammar_state.unfold_grammar_state(&grammar_rules, &first, &nullable, rule_channel_map);

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
        for (key, value) in rule_channel_map {
            for transition in value {
                println!("Channel: {:?}:{:?} -{:?}- {:?}:{:?}", rule_id_to_state_id_map[&key], key, transition.1, rule_id_to_state_id_map[&transition.0], transition.0);
            }
        }
    }

    if !found_start_state {
        panic!("DFA no start state detected!");
    } else {
        println!("Start state: {}", start_state_id);
    }

    grammar_state_hashmap
}