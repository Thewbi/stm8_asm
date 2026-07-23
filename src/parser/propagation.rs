use std::collections::HashMap;
use std::collections::BTreeMap;

use crate::parser::rule::Rule;
use crate::parser::rule::RuleElement;
use crate::Transition;

use crate::parser::grammar_state::GrammarState;

pub fn perform_propagation(rule_ids: &Vec::<usize>, 
    rule_id_to_state_id_map: &HashMap::<usize, usize>,
    rule_channel_map: &HashMap::<usize, Vec::<Transition<String>>>,
    grammar_state_hashmap: &mut BTreeMap<usize, GrammarState<String>>) {

    let debug: bool = false;

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
        for rule_id in rule_ids {

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

                                // asdf
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
}