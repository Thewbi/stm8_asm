// filename: main_lalr_1.rs

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

use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;

mod regex;
use crate::regex::infix_postfix_converter::InfixPostfixConverter;
use crate::regex::regex_building_block::RegexBuildingBlock;
use crate::regex::arena::Arena;
use crate::regex::arena::NodeId;
use crate::regex::arena::Node;

use crate::regex::enfa::Input;
use crate::regex::enfa::Fragment;
use crate::regex::enfa::FragmentStack;
use crate::regex::enfa::recurse_postfix_build_fragment_stack;
use crate::regex::enfa::enfa_copy;
use crate::regex::enfa::enfa_to_dfa;
use crate::regex::enfa::transition_dfa;
use crate::regex::enfa::enfa_to_dot_directed_graph;
use crate::regex::enfa::add_character_literal;

mod parser;
use crate::parser::parser::ParseTableCell;
use crate::parser::parser::Rule;
use crate::parser::parser::RuleElement;
use crate::parser::parser::Transition;
use crate::parser::parser::Parser;
use crate::parser::first::compute_first_original;

mod examplegrammars;
use crate::examplegrammars::c_full::produce_grammar_c_full;
use crate::examplegrammars::c_full_if_else::produce_grammar_c_full_if_else;
use crate::examplegrammars::c_full_if_else_2::produce_grammar_c_full_if_else_2;
use crate::examplegrammars::c_full_if_else_3::produce_grammar_c_full_if_else_3;
use crate::examplegrammars::left_recursive::produce_grammar_left_recursive;
use crate::examplegrammars::grammar_1::produce_grammar_1;
use crate::examplegrammars::grammar_2::produce_grammar_2;
use crate::examplegrammars::grammar_3::produce_grammar_3;
use crate::examplegrammars::grammar_4::produce_grammar_4;
use crate::examplegrammars::grammar_5::produce_grammar_5;
use crate::examplegrammars::grammar_6::produce_grammar_6;

#[derive(Clone)]
pub struct GrammarState<T: Debug> {
    pub id: usize,
    // pub current_rule: Rule<T>,
    pub identification_rules: Vec::<Rule<T>>,
    pub rules: Vec::<Rule<T>>,
}

impl<T: Clone + Debug + Display + std::cmp::PartialEq + Ord> GrammarState<T> {

    pub fn new(id: usize) -> Self {
        GrammarState {
            id: id,
            identification_rules: Vec::<Rule<T>>::new(),
            rules: Vec::<Rule<T>>::new(),
        }
    }

    pub fn ignore_rule(&mut self, rule_id: usize) -> bool {

        let mut current_rule = Rule::new(0);

        let mut found: bool = false;

        // search rule for id in ident and normal rules
        for ident_rule in &self.identification_rules {
            if ident_rule.id == rule_id {
                current_rule = ident_rule.clone();
                found = true;
                break;
            }
        }
        if !found {
            for rule in &self.rules {
                if rule.id == rule_id {
                    current_rule = rule.clone();
                    found = true;
                    break;
                }
            }
        }

        // early out
        if !found {
            return true;
        }

        // ignore consumed rules
        if current_rule.dot_idx >= current_rule.rhs.len() {
            return true;
        }

        // the CLOSURE() operation will not develop rules that point to terminals since they
        // do not actively add a rule to the CLOSURE() itself but transition to other states (shift).
        match &current_rule.rhs[current_rule.dot_idx] {
            RuleElement::Terminal(terminal) => {
                return true;
            }
            _ => {
                // nop
            }
        }

        false
    }

    pub fn retrieve_lookahead(&mut self, 
        rule_id: usize,
        first: &BTreeMap<RuleElement::<T>, Vec::<RuleElement::<T>>>,
        nullable: &BTreeMap::<RuleElement::<T>, bool>) -> Vec::<RuleElement<T>>
    {
        let debug = true;

        //
        // step 0 - turn rule id into rule object
        //

        let mut current_rule = Rule::new(0);

        let mut found: bool = false;

        // search rule for id in ident and normal rules
        for ident_rule in &self.identification_rules {
            if ident_rule.id == rule_id {
                current_rule = ident_rule.clone();
                found = true;
                break;
            }
        }
        if !found {
            for rule in &self.rules {
                if rule.id == rule_id {
                    current_rule = rule.clone();
                    found = true;
                    break;
                }
            }
        }

        let mut current_lookahead = Vec::<RuleElement<T>>::new();

        // early out
        if !found {
            return current_lookahead;
        }

        //
        // STEP 1 - collect all lookaheads for the RHS nonterminal
        //          Lookaheads are required for the parse table.
        //          In LALR(1) lookaheads are essential parts of a rule.
        //          The algorithm needs to build the rule plus it's lookaheads to produce valid rule items!
        //

        // DEBUG
        if debug {
            println!("[retrieve_lookahead] Determining lookahead for Rule: {}. Rule has lookahead: {:?}", current_rule, current_rule.lookahead);
        }

        let mut empty_beta = false;

        // find beta. if the dot is already at the end of the rule, then there is an empty beta
        // if there is empty beta, lookahead is the rule's own lookahead per definition
        if current_rule.dot_idx + 1 >= current_rule.rhs.len() {

            // empty beta
            if debug {
                println!("[retrieve_lookahead] empty beta {:?}", current_rule.lookahead);
            }

            current_lookahead.append(&mut current_rule.lookahead);

            // empty_beta = true;

        } else {

            // build FIRST(beta+rule.lookahead)

            // Example: S -> A C B, #
            // The developing the rule for nonterminal A, needs to build First(beta+#)
            // and beta in this case is CB instead of just C!


            // loop over each part in the beta string and add lookaheads until a rule is found
            // which is not a rule that can be empty! If an empty rule is found, stop
            // In this implementation empty information is stored in the nullable map
            for beta_idx in (current_rule.dot_idx + 1)..current_rule.rhs.len() {

                match &current_rule.rhs[beta_idx] {

                    RuleElement::NonTerminal(non_terminal) => {

                        // current_lookahead.push(grammar_rules[i].rhs[grammar_rules[i].dot_idx + 1].clone());
                        //panic!("test");

                        // // DEBUG
                        // println!("NonTerminal: {:?}, rule lookahead: {:?}", &current_rule.rhs[current_rule.dot_idx + 1], &current_rule.lookahead);

                        // TODO: retrieve FIRST(of nonterminal concat rule lookahead) and insert it into  current_lookahead
                        // TODO: what if concat rule lookahead has more than a single symbol????

                        //let temp = first.get(&current_rule.rhs[current_rule.dot_idx + 1]).expect("Compiler has no FIRST() information for NonTerminal: {}", current_rule.rhs[current_rule.dot_idx + 1]);

                        let temp_non_terminal = &current_rule.rhs[beta_idx];
                        let first_values_opt = first.get(temp_non_terminal);

                        // // DEBUG
                        // println!("First-Set for non-terminal: '{:?}' is '{:?}'", temp_non_terminal, first_values_opt);

                        if current_rule.dot_idx + 1 >= current_rule.rhs.len() {
                            empty_beta = true;
                        }

                        match first_values_opt {
                            Some(first_values) => {
                                if debug {
                                    println!("[retrieve_lookahead] first_values >> {:?}", first_values.clone());
                                    println!("[retrieve_lookahead] current_lookahead.append ++ {:?}", first_values.clone());
                                }
                                current_lookahead.append(&mut first_values.clone());
                            }
                            None => {
                                panic!("[retrieve_lookahead] Compiler has no FIRST() information for NonTerminal: {:?}! Aborting!", current_rule.rhs[beta_idx]);
                            }
                        }

                        // if current nonterminal is nullable, proceed with the next symbol
                        // if the nonterminal is not nullable or a terminal is found, then
                        // the first operation returns that first character
                        if nullable.contains_key(&temp_non_terminal) && *nullable.get(&temp_non_terminal).unwrap() == false {
                            break;
                        }
                    }

                    RuleElement::Terminal(terminal) => {

                        if debug {
                            println!("[retrieve_lookahead] current_lookahead.push + {:?}", current_rule.rhs[beta_idx].clone());
                        }

                        current_lookahead.push(current_rule.rhs[beta_idx].clone());

                        // experiment: if there is a terminal in beta, abort further lookahead search
                        break;
                    }

                    _ => { 
                        panic!("test");
                    }
                }
            }
        }

        // DEBUG
        if debug {
            println!("[unfold_grammar_state] current lookahead: {:?}", current_lookahead);
        }

        current_lookahead
    }

    pub fn unfold_grammar_state(&mut self, 
        grammar_rules: &Vec::<Rule<T>>,
        first: &BTreeMap<RuleElement::<T>, Vec::<RuleElement::<T>>>,
        nullable: &BTreeMap::<RuleElement::<T>, bool>,
        rule_channel_map: &mut HashMap::<usize, Vec::<Transition<T>>>,
    ) {

        if self.id == 50 {
            println!("Test 50");
        }

        let debug = true;
        
        // ids of rules to process
        let mut d_set = Vec::<usize>::new();

        // start with the identification rules. Add each one to the d_set
        for ident_rule in &self.identification_rules {
            d_set.push(ident_rule.id);
        }

        // while scratchpad has rules on it, loop
        let mut done: bool = d_set.is_empty();
        while !done {

            // retrieve next id by removing the first element from the queue
            let current_rule_id: usize = d_set[0];
            d_set.drain(0..1);

            // ignore consumed rules (dot marker after rule) or rules that not part of this state
            if self.ignore_rule(current_rule_id) {
                done = d_set.is_empty();
                continue;
            }


            if self.id == 22 && current_rule_id == 144 {
                println!("test");
            }
            if self.id == 22 && current_rule_id == 163 {
                println!("test");
            }

            let mut current_rule = Rule::new(0);
            let mut found: bool = false;

            // search rule for id in identification- and normal rules
            for ident_rule in &self.identification_rules {
                if ident_rule.id == current_rule_id {
                    current_rule = ident_rule.clone();
                    found = true;
                    break;
                }
            }
            if !found {
                for rule in &self.rules {
                    if rule.id == current_rule_id {
                        current_rule = rule.clone();
                        found = true;
                        break;
                    }
                }
            }

            // // DEBUG
            // println!("Current Rule: {:?}", current_rule);







            // find lookahead symbols for this rule (if beta is empty, use rule's lookahead otherwise use RHS lookhead)
            let current_lookahead = self.retrieve_lookahead(current_rule_id, first, nullable);

            // DEBUG
            // println!("Lookahead: {:?}", current_lookahead);




            



            // determine all rules that are produced by the current rule

            // for each produced rule, search if the rule is already contained in the state
            //      if not contained
            //          - insert a clone into the current state
            //          - extend cloned rule by the lookahead
            //          - add id to d_set
            //      if contained
            //          - extend contained rule by the lookahead




            // over all rules that unfold from the rule via REDUCE operations
            match &current_rule.rhs[current_rule.dot_idx] {

                // if the dot points to a non-terminal, extend the rule set
                RuleElement::NonTerminal(non_terminal) => {

                    let nt = RuleElement::<T>::NonTerminal(non_terminal.clone());

                    // // DEBUG
                    // if debug {
                    //     println!("");
                    //     println!("[unfold_grammar_state] Extending closure due to Rule: {} and NonTerminal '{}' with lookaheads '{:?}'", current_rule, non_terminal, current_lookahead);
                    //     println!("");
                    // }

                    // DEBUG
                    // println!("non_terminal {}", non_terminal);
                    
                    // find all rules that have a LHS == the non-terminal and add them into the d_set
                    for i in 0..grammar_rules.len() {

                        // if this rule starts with (has LHS equal) the expected nonterminal
                        if grammar_rules[i].lhs == nt {

                            // // DEBUG
                            // if debug {
                            //     println!("");
                            //     println!("[unfold_grammar_state] Inserting into closure! Rule: [{}] {} using lookaheads: {:?} because of source-rule-id: {}", grammar_rules[i].id, grammar_rules[i], current_lookahead, &current_rule.id);
                            //     println!("");
                            // }

                            //
                            // try to find produced rule in current state
                            // 

                            let mut contained_already = false;
                            let mut contained_index = 0;
                            for j in 0..self.rules.len() {

                                if self.rules[j] == grammar_rules[i] {

                                    contained_already = true;
                                    contained_index = j;
                                    break;
                                }
                            }

                            if contained_already {

                                // DEBUG
                                // println!("Already exists at index: {}", contained_index);

                                // - extend cloned rule by the lookahead
                                //
                                // NB: If you do not clone here, rust will empty the collection 
                                // due to the call to append!
                                let temp_clone = current_lookahead.clone();
                                for la_element in temp_clone {
                                    if !self.rules[contained_index].lookahead.contains(&la_element) {
                                        self.rules[contained_index].lookahead.push(la_element);

                                        // this rule needs to forward it's own lookaheads again
                                        if !d_set.contains(&self.rules[contained_index].id) {
                                            d_set.push(self.rules[contained_index].id);
                                        }
                                    }
                                }



                                // - insert into rule channel map

                                //
                                // Insert into rule_channel_map

                                if !rule_channel_map.contains_key(&current_rule.id) {
                                    let channel_ends = Vec::<Transition<T>>::new();
                                    rule_channel_map.insert(current_rule.id, channel_ends);
                                }
                                // retrieve the vector of first symbols for the nonterminal and extend it
                                let channel_ends = &mut rule_channel_map.get_mut(&current_rule.id).unwrap();

                                // add a transition and specify the id of the newly created rule
                                channel_ends.push(Transition(self.rules[contained_index].id, RuleElement::<T>::NonTerminal(non_terminal.clone())));

                                println!("{:?} -{:?}-> {:?}", &current_rule.id, non_terminal.clone(), self.rules[contained_index].id);

                                //
                                //

                                

                            } else {

                                // // DEBUG
                                // println!("Does not exist yet!");

                                // - insert a clone into the current state
                                let mut new_rule = grammar_rules[i].clone();
                                let new_rule_id = RULE_COUNTER.fetch_add(1, Ordering::SeqCst);
                                new_rule.id = new_rule_id;

                                // - extend cloned rule by the lookahead
                                //
                                // NB: If you do not clone here, rust will empty the collection 
                                // due to the call to append!
                                //new_rule.lookahead.append(&mut current_lookahead.clone());
                                let temp_clone = current_lookahead.clone();
                                for la_element in temp_clone {
                                    if !new_rule.lookahead.contains(&la_element) {
                                        new_rule.lookahead.push(la_element);
                                    }
                                }

                                // - add id of cloned rule to d_set
                                d_set.push(new_rule.id);

                                self.rules.push(new_rule);



                                // - insert into rule channel map

                                //
                                // Insert into rule_channel_map

                                if !rule_channel_map.contains_key(&current_rule.id) {
                                    let channel_ends = Vec::<Transition<T>>::new();
                                    rule_channel_map.insert(current_rule.id, channel_ends);
                                }
                                // retrieve the vector of first symbols for the nonterminal and extend it
                                let channel_ends = &mut rule_channel_map.get_mut(&current_rule.id).unwrap();

                                // // DEBUG
                                // if debug {
                                //     println!("{:?}, {:?}", new_rule_id, RuleElement::<T>::NonTerminal(non_terminal.clone()));
                                //     println!("");
                                // }

                                // add a transition and specify the id of the newly created rule
                                channel_ends.push(Transition(new_rule_id, RuleElement::<T>::NonTerminal(non_terminal.clone())));

                                // DEBUG
                                println!("{:?} -{:?}-> {:?}", &current_rule.id, non_terminal.clone(), new_rule_id);

                                //
                                //
                            }
                        }
                    }
                }

                // if the dot points to a terminal, ?????
                RuleElement::Terminal(terminal) => {
                    panic!("test");
                }

                _ => { 
                    panic!("test");
                }
            }

            // update loop end condition
            done = d_set.is_empty();
        }
    }

/*
    // For this function to work, insert at least one rule into the identification_rules 
    // set of the grammar set prior to calling this function!
    //
    // This function will develop all rules in the identification_rules set into the closure 
    // of all rules that the parser can potentially activate on any input symbol when it is 
    // located in the state for which this function is called.
    // 
    // All these rules are inserted into the rules-set of the grammar state.
    //
    // This function fills the channel map with channels between rules.
    //
    // The states are created prior to calling this function. The states are created in the
    // same large loop that also calls this function.
    //
    // This function does not produce new states!
    pub fn unfold_grammar_state(&mut self, 
        grammar_rules: &Vec::<Rule<T>>,
        first: &BTreeMap<RuleElement::<T>, Vec::<RuleElement::<T>>>,
        nullable: &BTreeMap::<RuleElement::<T>, bool>,
        rule_channel_map: &mut HashMap::<usize, Vec::<Transition<T>>>,
    ) {

        let debug: bool = true;

        // DEBUG
        // if debug {
            if self.id == 18 {
                println!("{:?}", self);
                println!("test");
            }
        // }

        // scratchpad of rules to process
        let mut d_set = Vec::<Rule<T>>::new();
        d_set.append(&mut self.identification_rules.clone());

        // while scratchpad has rules on it, loop
        let mut done: bool = d_set.is_empty();
        while !done {

            // let mut current_rule: Rule<T> = d_set.pop().expect("Need at least one rule!");
            let mut current_rule: Rule<T> = d_set[0].clone();
            d_set.drain(0..1);

            // DEBUG
            if debug {
                println!("[unfold_grammar_state] State-ID: {}, current_rule: {}", self.id, current_rule);
            }

            // DEBUG
            //if self.id == 18 && current_rule.lhs == RuleElement::NonTerminal(String::from("cast_expression")) {
            //if self.id == 18 && current_rule.id == 73 {
            if self.id == 18 && current_rule.id == 72 {
                println!("test {:?}", current_rule);
            }

            // ignore consumed rules
            if current_rule.dot_idx >= current_rule.rhs.len() {
                done = d_set.is_empty();
                continue;
            }

            // the CLOSURE() operation will not develop rules that point to terminals since they
            // do not actively add a rule to the CLOSURE() itself but transition to other states (shift).
            match &current_rule.rhs[current_rule.dot_idx] {
                RuleElement::Terminal(terminal) => {
                    done = d_set.is_empty();
                    continue;
                }
                _ => {
                    // nop
                }
            }

            //
            // STEP 1 - collect all lookaheads for the RHS nonterminal
            //          Lookaheads are required for the parse table.
            //          In LALR(1) lookaheads are essential parts of a rule.
            //          The algorithm needs to build the rule plus it's lookaheads to produce valid rule items!
            //

            let mut current_lookahead = Vec::<RuleElement<T>>::new();

            // DEBUG
            if debug {
                println!("[unfold_grammar_state] Determining lookahead for Rule: {}. Rule has lookahead: {:?}", current_rule, current_rule.lookahead);
            }

            let mut empty_beta = false;

            // find beta, if there is no beta, lookahead is the rule's own lookahead
            if current_rule.dot_idx + 1 >= current_rule.rhs.len() {

                // empty beta
                if debug {
                    println!("[unfold_grammar_state] empty beta {:?}", current_rule.lookahead);
                }

                current_lookahead.append(&mut current_rule.lookahead);

                empty_beta = true;

            } else {

                // build FIRST(beta+rule.lookahead)

                // // DEBUG
                // println!("found beta");

                // // DEBUG
                // println!("[unfold_grammar_state] Current Rule: {:?}", current_rule);

                // BUG: beta is more than the first non terminal !!!!!!!!

                // Example: S -> A C B, #
                // The developing the rule for nonterminal A, needs to build First(beta+#)
                // and beta in this case is CB instead of just C!

                //let beta_idx = current_rule.dot_idx + 1;

                for beta_idx in (current_rule.dot_idx + 1)..current_rule.rhs.len() {

                    match &current_rule.rhs[beta_idx] {

                        RuleElement::NonTerminal(non_terminal) => {

                            // current_lookahead.push(grammar_rules[i].rhs[grammar_rules[i].dot_idx + 1].clone());
                            //panic!("test");

                            // // DEBUG
                            // println!("NonTerminal: {:?}, rule lookahead: {:?}", &current_rule.rhs[current_rule.dot_idx + 1], &current_rule.lookahead);

                            // TODO: retrieve FIRST(of nonterminal concat rule lookahead) and insert it into  current_lookahead
                            // TODO: what if concat rule lookahead has more than a single symbol????

                            //let temp = first.get(&current_rule.rhs[current_rule.dot_idx + 1]).expect("Compiler has no FIRST() information for NonTerminal: {}", current_rule.rhs[current_rule.dot_idx + 1]);

                            let temp_non_terminal = &current_rule.rhs[beta_idx];
                            let first_values_opt = first.get(temp_non_terminal);

                            println!("First-Set for non-terminal: '{:?}' is '{:?}'", temp_non_terminal, first_values_opt);

                            if current_rule.dot_idx + 1 >= current_rule.rhs.len() {
                                empty_beta = true;
                            }

                            match first_values_opt {
                                Some(first_values) => {
                                    if debug {
                                        println!("[unfold_grammar_state] first_values >> {:?}", first_values.clone());
                                        println!("[unfold_grammar_state] current_lookahead.append ++ {:?}", first_values.clone());
                                    }
                                    current_lookahead.append(&mut first_values.clone());
                                }
                                None => {
                                    panic!("[unfold_grammar_state] Compiler has no FIRST() information for NonTerminal: {:?}! Aborting!", current_rule.rhs[beta_idx]);
                                }
                            }

                            // if current nonterminal is nullable, proceed with the next symbol
                            // if the nonterminal is not nullable or a terminal is found, then
                            // the first operation returns that first character
                            if nullable.contains_key(&temp_non_terminal) && *nullable.get(&temp_non_terminal).unwrap() == false {
                                break;
                            }
                            
                        }

                        RuleElement::Terminal(terminal) => {

                            if debug {
                                println!("[unfold_grammar_state] current_lookahead.push + {:?}", current_rule.rhs[beta_idx].clone());
                            }

                            current_lookahead.push(current_rule.rhs[beta_idx].clone());

                            // experiment: if there is a terminal in beta, abort further lookahead search
                            break;
                        }

                        _ => { 
                            panic!("test");
                        }
                    }
                }
            }

            // DEBUG
            if debug {
                println!("[unfold_grammar_state] current lookahead: {:?}", current_lookahead);
            }

            // over all rules that unfold from the rule via REDUCE operations
            match &current_rule.rhs[current_rule.dot_idx] {

                // if the dot is points to a non-terminal, extend the rule set
                RuleElement::NonTerminal(non_terminal) => {

                    // DEBUG
                    if debug {
                        println!("");
                        println!("[unfold_grammar_state] Extending closure due to Rule: {} and NonTerminal '{}' with lookaheads '{:?}'", current_rule, non_terminal, current_lookahead);
                        println!("");
                    }

                    // DEBUG
                    // println!("non_terminal {}", non_terminal);
                    
                    // find all rules that have a LHS == the non-terminal and add them into the d_set
                    for i in 0..grammar_rules.len() {

                        // if this rule starts with the same nonterminal
                        if grammar_rules[i].lhs == RuleElement::<T>::NonTerminal(non_terminal.clone()) {

                            // DEBUG
                            if debug {
                                println!("");
                                println!("[unfold_grammar_state] Inserting into closure Rule: [{}] {} using lookaheads: {:?} because of source-rule-id: {}", grammar_rules[i].id, grammar_rules[i], current_lookahead, &current_rule.id);
                                println!("");
                            }

                            // let mut empty_beta = false;
                            // if grammar_rules[i].dot_idx + 1 >= grammar_rules[i].rhs.len() {
                            //     empty_beta = true;
                            // }

                            let mut contained_already = false;
                            for j in 0..self.rules.len() {

                                if self.rules[j] == grammar_rules[i] {

                                    if empty_beta {
                                        panic!("test");
                                    }

                                    // copy all lookahead symbols over!
                                    for la in &current_lookahead {

                                        if !self.rules[j].lookahead.contains(&la) {

                                            // DEBUG
                                            if debug {
                                                println!("[unfold_grammar_state] Inserting {:?} into rule {:?}", &la, &self.rules[j]);
                                            }

                                            //
                                            // Insert into rule_channel_map

                                            if !rule_channel_map.contains_key(&current_rule.id) {
                                                let channel_ends = Vec::<Transition<T>>::new();
                                                rule_channel_map.insert(current_rule.id, channel_ends);
                                            }
                                            // retrieve the vector of first symbols for the nonterminal and extend it
                                            let channel_ends = &mut rule_channel_map.get_mut(&current_rule.id).unwrap();

                                            // DEBUG
                                            if debug {
                                                println!("{:?}, {:?}", self.rules[j].id, RuleElement::<T>::NonTerminal(non_terminal.clone()));
                                                println!("");
                                            }

                                            channel_ends.push(Transition(self.rules[j].id, RuleElement::<T>::NonTerminal(non_terminal.clone())));

                                            //
                                            //

                                            // because d_set and self.rules are independent collections, we need to update both!
                                            let mut add_back = false;
                                            if d_set.contains(&self.rules[j]) {

                                                // https://stackoverflow.com/questions/26243025/how-to-remove-an-element-from-a-vector-given-the-element
                                                let index = d_set.iter().position(|x| *x == self.rules[j]).unwrap();
                                                d_set.remove(index);

                                                add_back = true;
                                            }

                                            self.rules[j].lookahead.push(la.clone());

                                            if add_back {
                                                d_set.push(self.rules[j].clone());
                                            }
                                        }
                                    }
                                       
                                    contained_already = true;
                                }
                            }

                            if contained_already {
                                continue;
                            }

                            // add new rule to state
                            let mut rule = grammar_rules[i].clone();

                            // CHECK THIS !!!!
                            // produce new id to distinguish all rules from each other for propagation
                            rule.id = RULE_COUNTER.fetch_add(1, Ordering::SeqCst);

                            // DEBUG
                            if debug {
                                // println!("");
                                println!("[unfold_grammar_state] Inserting into closure Rule: [{}] {} using lookaheads: {:?} because of source-rule-id: {}", rule.id, rule, current_lookahead, &current_rule.id);
                                println!("[unfold_grammar_state] Source rule: {:?}", current_rule);
                            }

                            //
                            // Insert into rule_channel_map

                            if !rule_channel_map.contains_key(&current_rule.id) {
                                let channel_ends = Vec::<Transition<T>>::new();
                                rule_channel_map.insert(current_rule.id, channel_ends);
                            }
                            // retrieve the vector of first symbols for the nonterminal and extend it
                            let channel_ends = &mut rule_channel_map.get_mut(&current_rule.id).unwrap();

                            channel_ends.push(Transition(rule.id, RuleElement::<T>::NonTerminal(non_terminal.clone())));

                            //
                            //

                            rule.lookahead.append(&mut current_lookahead.clone());

                            // only if beta is empty
                            if empty_beta {
                                rule.lookahead.append(&mut current_rule.lookahead.clone());
                            }
                            self.rules.push(rule.clone());

                            d_set.insert(0, rule);
                        }
                    }
                }

                _ => {
                    // nop
                }
            }

            done = d_set.is_empty();
        }
    }
*/   
}

impl<T: Debug + Display> fmt::Debug for GrammarState<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        // state id
        write!(f, "GState {} {{\n", self.id).expect("Write failed!");

        // identifying rules
        for rule in &self.identification_rules {
            write!(f, "  [{}] {:?}\n", rule.id, rule).expect("Write failed!");
        }

        write!(f, "  ============\n").expect("Write failed!");

        // rules
        for rule in &self.rules {
            write!(f, "  [{}] {:?}\n", rule.id, rule).expect("Write failed!");
        }

        write!(f, "}}").expect("Write failed!");

        Ok(())
    }
}

fn create_rule(grammar_rules: &mut Vec::<Rule<String>>, rule_as_string: String, treat_nonterminal_lowercase: bool) {

    // split by a single whitespace!!! This is bad because the user might mess this up easily!
    // BETTER split by arbitrary whitespace!
    //let split: Vec<_> = rule_as_string.split(' ').collect();
    let split: Vec<_> = rule_as_string.split_whitespace().collect();

    // build rule instance which is complemented by the for loop
    let mut rule: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));

    // loop over each part of the rule
    for (idx, split_element) in split.iter().enumerate() {

        // println!("{}", *split_element);

        match idx {

            // first part is the left-hand side of the production rule
            0 => {
                rule.lhs = RuleElement::NonTerminal(String::from(*split_element));
            }
            // ignore the rule arrow
            1 => {
                // ignore ->
            }
            // right hand side objects follow 
            _ => {

                match *split_element {

                    "$$_EPSILON_$$" => {
                        rule.rhs.push(RuleElement::Epsilon);
                    }

                    "(" => {
                        let part_string = String::from(*split_element);
                        rule.rhs.push(RuleElement::Terminal(part_string));
                    }

                    ")" => {
                        let part_string = String::from(*split_element);
                        rule.rhs.push(RuleElement::Terminal(part_string));
                    }

                    "{" => {
                        let part_string = String::from(*split_element);
                        rule.rhs.push(RuleElement::Terminal(part_string));
                    }

                    "}" => {
                        let part_string = String::from(*split_element);
                        rule.rhs.push(RuleElement::Terminal(part_string));
                    }

                    _ => {

                        let is_all_letters_lowercase: bool = split_element
                            .chars()                       // Elements: [':', ')', ' ', '?']
                            .filter(|x| x.is_alphabetic()) // Elements: [] (no element is alphabetic, so this is an empty iterator)
                            .all(|x| x.is_lowercase());    // true (.all() is always true for empty iterators)

                        let part_string = String::from(*split_element);

                        if treat_nonterminal_lowercase {
                            if is_all_letters_lowercase {
                                rule.rhs.push(RuleElement::NonTerminal(part_string));
                            } else {
                                rule.rhs.push(RuleElement::Terminal(part_string));
                            }
                        } else {
                            if is_all_letters_lowercase {
                                rule.rhs.push(RuleElement::Terminal(part_string));
                            } else {
                                rule.rhs.push(RuleElement::NonTerminal(part_string));
                            }
                        }
                    }
                }
            }
        }
    }

    // // DEBUG
    // rule.dot_idx = std::usize::MAX;
    // println!("a{:?}", rule);
    // rule.dot_idx = 0;

    grammar_rules.push(rule);
}

// https://stackoverflow.com/questions/32935808/generate-sequential-ids-for-each-instance-of-a-struct
static RULE_COUNTER: AtomicUsize = AtomicUsize::new(0);
static STATE_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn compute_nullable_sets(grammar_rules: &mut Vec::<Rule<String>>, nullable: &mut BTreeMap::<RuleElement::<String>, bool>) {

    println!("");
    println!("Nullable start ...");

    // nullable (can produce the empty string, EPSILON)
    //
    // In Context-Free Grammars (CFG), a nonterminal that can derive the empty string \(\epsilon \) 
    // is called nullable. 
    // 
    // You can find all nullable nonterminals by using a simple iterative marking 
    // algorithm (similar to the standard method taught in computer science).
    // let mut nullable = BTreeMap::<RuleElement::<String>, bool>::new();

    let mut change_detected = true;
    while change_detected {

        // // DEBUG output nullable
        // println!("*******************************************");
        // for (key, value) in nullable.clone().into_iter() {
        //     println!("{:?} / {:?}", key, value);
        // }
        // println!("*******************************************");

        change_detected = false;

        // over all grammar rules globally
        for i in 0..grammar_rules.len() {

            // // declarator
            // println!("{:?}", &grammar_rules[i]);

            // if grammar_rules[i].lhs == RuleElement::NonTerminal(String::from("declarator")) {
            //     println!("Test");
            // }

            // if a way to an epsilon is found, never change that status
            // A nonterminal can go from non-nullable to nullable but it can never
            // go back to non-nullable once a way to an epsilon is found
            //
            // check if this rule is stored in the map with a value of true
            if nullable.contains_key(&grammar_rules[i].lhs) && *nullable.get(&grammar_rules[i].lhs).unwrap() == true {
                continue;
            }

            // if RHS is a single Epsilon, the symbol is nullable
            if grammar_rules[i].rhs.len() == 1 && grammar_rules[i].rhs[0] == RuleElement::Epsilon {
                nullable.insert(grammar_rules[i].lhs.clone(), true);
                change_detected == true;
                continue;
            }

            if nullable.contains_key(&grammar_rules[i].lhs) {
                continue;
            }

            let mut proceed_with_next_rule = false;

            // go over each element of RHS
            for j in 0..grammar_rules[i].rhs.len() {

                // a terminal in a rule makes the rule non-nullable
                match grammar_rules[i].rhs[j] {

                    RuleElement::Terminal(_) => {
                        nullable.insert(grammar_rules[i].lhs.clone(), false);
                        change_detected = true;
                        proceed_with_next_rule = true;
                        break;
                    }

                    _ => {

                        // RHS-element has no status yet, abort work on the current rule for this iteration
                        // and wait for it to have a status in the future
                        if !nullable.contains_key(&grammar_rules[i].rhs[j]) {

                            // println!("{:?}", &grammar_rules[i].rhs[j]);

                            proceed_with_next_rule = true;
                            break;

                        } else {

                            if *nullable.get(&grammar_rules[i].rhs[j]).unwrap() == false {

                                if !nullable.contains_key(&grammar_rules[i].lhs) {
                                    nullable.insert(grammar_rules[i].lhs.clone(), false);
                                    change_detected = true;
                                    proceed_with_next_rule = true;
                                    break;
                                }
                            }

                        }
                        
                    }
                }
            }

            if proceed_with_next_rule {
                continue;
            }

            // when the rule arrives here, all RHS-elements are nullable, the rule is nullable
            nullable.insert(grammar_rules[i].lhs.clone(), true);
            change_detected == true;
        }
    }

    println!("Nullable end.");

    // DEBUG output nullable
    println!("");
    println!("NULLABLE *****************************");
    for (key, value) in nullable.clone().into_iter() {
        println!("{:?} / {:?}", key, value);
    }
    println!("*******************************************");

    println!("Test");
}

pub fn validate_grammar(grammar_rules: &mut Vec::<Rule<String>>) {

    println!("");
    println!("Validating grammar start ...");

    // collect all non-terminals into a set
    // iterate over the set
    // check for each non-terminal if it appears on the left side of at least one production rule
    // if a non-terminal is found that does not satisfy this test, the grammar is invalid! Abort!

    let mut rhs_nonterminal_set: HashSet::<RuleElement<String>> = HashSet::new();
    let mut lhs_nonterminal_set: HashSet::<RuleElement<String>> = HashSet::new();

    for rule in grammar_rules.iter() {

        lhs_nonterminal_set.insert(rule.lhs.clone());

        for rule_element in rule.rhs.iter() {

            match &rule_element {
                RuleElement::NonTerminal(nt) => {
                    rhs_nonterminal_set.insert(rule_element.clone());
                }
                _ => {

                }
            }
        }
    }

    for rule_element in rhs_nonterminal_set.iter() {
        if !lhs_nonterminal_set.contains(&rule_element) {
            panic!("[Invalid Grammar] The NonTerminal '{:?}' does not appear on the LeftHandSide of 
            any production rule in the grammar although it is used as a RightHandSide element in at 
            least one production rule! The grammar is incomplete! The non-terminal '{:?}' cannot be 
            reduced! Please fix the grammar before proceeding!", &rule_element, &rule_element);
        }
    }

    println!("Validating grammar end.");
}

pub fn add_token_definition(converter: &mut InfixPostfixConverter, combined_fragment: &mut Fragment,
    alphabet: &mut HashSet::<RegexBuildingBlock>,
    regex_infix: &str, token_name: &str, token_id: usize) {
    
    converter.infix_to_postfix(regex_infix);
    let mut fragment_stack_return = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_return, alphabet);
    converter.reset();
    let mut fragment_return = fragment_stack_return.stack.pop().unwrap();
    fragment_return.enfa.states.get_mut(&fragment_return.end_id).unwrap().token_id = token_id;
    fragment_return.enfa.states.get_mut(&fragment_return.end_id).unwrap().token_name = String::from(token_name);

    let (start_id_return, end_id_return) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_return.enfa, fragment_return.end_id);

    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_return);
}

fn main() {

    println!("start");



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

    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('a'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('A'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('b'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('B'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('c'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('C'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('d'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('D'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('e'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('E'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('f'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('F'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('g'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('G'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('h'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('H'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('i'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('I'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('j'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('J'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('k'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('K'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('l'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('L'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('m'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('M'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('n'));
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
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('t'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('T'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('u'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('U'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('v'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('V'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('w'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('W'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('x'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('X'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('y'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('Y'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('z'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('Z'));

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('0'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('1'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('2'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('3'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('4'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('5'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('6'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('7'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('8'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('9'));

    // alphabet.insert(RegexBuildingBlock::CharacterLiteral(' ')); // WHITESPACE, SPACE

    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('_'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('<'));
    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('>'));
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
    //converter.infix_to_postfix("\"(a|A|b|B|c|C|d|D)+\"");
    //converter.infix_to_postfix("\"^[\",\"]+\"");
    //converter.infix_to_postfix("^a");
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
    let result = converter.infix_to_postfix("(0|1|2)*.(0|1|2)+(e|E)(\\+|\\-)?(0|1|2)+(f|F|l|L)?");
    // let result = converter.infix_to_postfix("(0|1|2)*.(0|1|2)+((e|E)(\\+|\\-)?(0|1|2)+)?(f|F|l|L)?");
    // let result = converter.infix_to_postfix("(0|1|2)*.(0|1|2)+((e|E)(+|-)?(0|1|2)+)?(f|F|l|L)?");

    println!("{:?}", result);

    let mut fragment_stack_string_literal = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_string_literal, &mut alphabet);
    converter.reset();
    let mut fragment_string_literal = fragment_stack_string_literal.stack.pop().unwrap();
    fragment_string_literal.enfa.states.get_mut(&fragment_string_literal.end_id).unwrap().token_id = 610;
    fragment_string_literal.enfa.states.get_mut(&fragment_string_literal.end_id).unwrap().token_name = String::from("STRING_LITERAL");

    // DEBUG
    enfa_to_dot_directed_graph(&mut fragment_string_literal.enfa, "string_literal_enfa_automaton.dot");

    // insert into LEXER
    let (start_id_string_literal, end_id_string_literal) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_string_literal.enfa, fragment_string_literal.end_id);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_string_literal);

    // DEBUG
    enfa_to_dot_directed_graph(&mut fragment_string_literal.enfa, "lexer_enfa_automaton.dot");

    let mut dfa = enfa_to_dfa(&mut combined_fragment.enfa, &mut alphabet);

    //enfa_to_dot_directed_graph(&mut fragment_string_literal.enfa, "dfa_automaton.dot");
    enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");

    //let str = "\"Hello World: %d\"";
    // let str = "1.0";
    // let str = "1.0e+100f";
    let str = "1.0e+100f";
    println!("Input: {}", str);
    
    let lexer_debug: bool = true;

    let mut current_state_id = dfa.start_state_id;
    let mut last_state_id = 0;

    let mut token_string_buffer = String::from("");
    for character in str.chars() {

        let mut char_consumed = false;
        while !char_consumed {

            last_state_id = current_state_id;

            if lexer_debug {
                println!("[LEXER] Input: '{}'", character);
            }

            // try to transition the large lexer DFA to produce a token for the input
            current_state_id = transition_dfa(&mut dfa, current_state_id, &RegexBuildingBlock::CharacterLiteral(character));

            if dfa.is_end_state(current_state_id) {

                // println!("STATE '{}' END STATE!", current_state_id);
                // println!("ACCEPTING '{}'! END STATE! Token-Id: {}", token_string_buffer, dfa.states[&current_state_id].token_id);

                token_string_buffer.push(character);

                char_consumed = true;

            } else if dfa.is_trap_state(current_state_id) {

                // reset the lexer's DFA back to the start state and 
                // try to accept the symbol again which was read from input already
                char_consumed = false;
                current_state_id = dfa.start_state_id;

                if lexer_debug {
                    println!("[LEXER] Emitting '{}', Token-Id: {}, Token-Name: {}", token_string_buffer, dfa.states[&last_state_id].token_id, dfa.states[&last_state_id].token_name);
                    println!("");
                }

                let terminal = RuleElement::Terminal(dfa.states[&last_state_id].token_name.clone());

                //if lexer_debug {
                    println!("[LEXER] {:?} ---> {:?}", token_string_buffer, terminal);
                //}

                // IGNORE TOKEN
                // IGNORE WHITESPACE
                //
                // WHITESPACE_TOKEN_ID is the token-id of whitespace, ignore whitespace!
                if dfa.states[&last_state_id].token_id != WHITESPACE_TOKEN_ID {
                    // provide_input(&mut parser, 
                    //     &grammar_state_hashmap, 
                    //     &mut step, 
                    //     &terminal);
                }

                token_string_buffer.clear();

            } else {
                // println!("STATE '{}' NOT END STATE!", current_state_id);

                token_string_buffer.push(character);

                char_consumed = true;
            }
        }
    }

    if dfa.is_end_state(current_state_id) {
        println!("ACCEPT!");
    } else {
        panic!("DECLINED!");
    }

    ///////////////////////////////////////////////////////////////////////////////////


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
    let g_result = produce_grammar_c_full_if_else_3(&mut grammar_rules);
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

    // DEBUG
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

    //
    // Unfold the states (CLOSURE) and build channels between the states.
    //

    let mut rule_channel_map = HashMap::<usize, Vec::<Transition<String>>>::new();

    let mut found_start_state: bool = false;
    let mut start_state_id: usize = 0;
    let mut found_final_state: bool = false;
    let mut final_state_id: usize = 0;

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

    let mut done: bool = e_set.is_empty();
    while !done {

        // // DEBUG
        // println!("e_set: {:?}", e_set);
        // println!("processed_set: {:?}", processed_set);

        // get the next state id from the e_set (= set of states to process)
        let current_grammar_state_id = e_set.pop().expect("Need at least one state!");
        processed_set.push(current_grammar_state_id);

        if current_grammar_state_id == 8 {
            println!("test");
        }

        // unfold node (retrieve state given state id, then call unfold_grammar_state())
        if let Some(grammar_state) = grammar_state_hashmap.get_mut(&current_grammar_state_id) {
            
            println!("Before: {:?}", grammar_state);

            if grammar_state.id == 18 {
                println!("test");
            }

            // unfold_grammar_state is probably the CLOSURE() operation
            // the rule_channel_map is extended with new entries, by this call
            grammar_state.unfold_grammar_state(&grammar_rules, &first, &nullable, &mut rule_channel_map);

            println!("After: {:?}", grammar_state);
            println!("");

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
                println!("next_state_id: {}", next_state_id);
                //if next_state_id == 6 as usize {
                if next_state_id == 8 as usize {
                    println!("test");
                }
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
                println!("New State Before Unfold: {:?}", new_grammar_state);

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
    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
    println!("Channels");
    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");

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
    
    for (key, value) in &rule_channel_map {
        for transition in value {
            println!("Channel: {:?}:{:?} -{:?}- {:?}:{:?}", rule_id_to_state_id_map[key], key, transition.1, rule_id_to_state_id_map[&transition.0], transition.0);
        }
    }

    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");



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
    println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
    println!("Propagation cycles start ...");
    println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++");

    // println!("{:?}", rule_id_to_state_id_map);

    // for (key, value) in &grammar_state_hashmap {
    //     println!("{:?} / {:?}", key, value);
    // }

    // // DEBUG
    // println!("rule-channel-map: {:?}", &rule_channel_map);


    /*
    let first_state: &mut GrammarState<String> = grammar_state_hashmap.get_mut(&first_state_id).unwrap();

    println!("{:?}", &first_state);

    for g in 0..first_state.rules.len() {

        let src_rule_id = first_state.rules[g].id;

        let dest_rule_transitions = rule_channel_map.get(&src_rule_id).unwrap();
        for transition in dest_rule_transitions {

            let target_state = rule_id_to_state_id_map.get(&transition.0).unwrap();

            // needs to be within same state
            if *target_state == first_state_id {

                // // push target rule id
                // if !processed_rule_ids.contains(&transition.0) {

                    println!("{} -> {}", src_rule_id, &transition.0);

                    let mut source_rule_idx = 0;
                    let mut target_rule_idx = 0;

                    // insert lookahead into rule if not contained already
                    for jj in 0..first_state.rules.len() {
                        if first_state.rules[jj].id == src_rule_id {
                            println!("source rule found!");
                            source_rule_idx = jj;
                        }
                        if first_state.rules[jj].id == transition.0 {
                            println!("target rule found!");
                            target_rule_idx = jj;
                        }
                    }

                    let mut ttttt = first_state.rules[source_rule_idx].lookahead.clone();
                    first_state.rules[target_rule_idx].lookahead.append(&mut ttttt);

                    // if !local_rule_ids.contains(&transition.0) {
                    //     local_rule_ids.push(transition.0);
                    // }
                // }
            }
        }
    }
        println!("{:?}", &first_state);
    */

    
    

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

                if *src_rule_id == 68 as usize {
                    println!("Src-RuleId: {:?}, Src-StateId: {:?} ===> Dest-RuleId: {}, Dest-StateId: {:?}",
                        src_rule_id, src_state_id, dest_rule_id.0, dest_state_id);
                    println!("test");
                }

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

/*      */          


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

                            // DEBUG
                            //if *la == RuleElement::NonTerminal(String::from(")")) {
                            if dest_state.id == 50 && *la == RuleElement::Terminal(String::from("CLOSING_BRACKET")) {
                                println!("test");
                            }

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

                                // // DEBUG
                                // if dest_state_id == 50 && *la == RuleElement::Terminal(String::from("ELSE")) {
                                //     println!("({} -> {}) Dirty: {} because '{:?}' has been pushed into one of it's identifying rules", src_state_id, dest_state_id, dest_state_id, la.clone());
                                //     println!("test");
                                // }

                                // if dest_state_id == 31 && *la == RuleElement::Terminal(String::from("ELSE")) {
                                //     println!("({} -> {}) Dirty: {} because '{:?}' has been pushed into one of it's identifying rules", src_state_id, dest_state_id, dest_state_id, la.clone());
                                //     println!("test");
                                // }

                                // if dest_state_id == 22 && *la == RuleElement::Terminal(String::from("ELSE")) {
                                //     println!("({} -> {}) Dest-Rule: {}, Dirty: {} because '{:?}' has been pushed into one of it's identifying rules", src_state_id, dest_state_id, dest_rule_id.0, dest_state_id, la.clone());
                                //     println!("test");
                                // }

                                // if dest_state_id == 56 && *la == RuleElement::Terminal(String::from("ELSE")) {
                                //     println!("({} -> {}) Dirty: {} because '{:?}' has been pushed into one of it's identifying rules", src_state_id, dest_state_id, dest_state_id, la.clone());
                                //     println!("test");
                                // }

                                // if dest_state_id == 48 && *la == RuleElement::Terminal(String::from("ELSE")) {
                                //     println!("({} -> {}) Dirty: {} because '{:?}' has been pushed into one of it's identifying rules", src_state_id, dest_state_id, dest_state_id, la.clone());
                                //     println!("test");
                                // }

                                // if dest_state_id == 27 && *la == RuleElement::Terminal(String::from("ELSE")) {
                                //     println!("({} -> {}) Dirty: {} because '{:?}' has been pushed into one of it's identifying rules", src_state_id, dest_state_id, dest_state_id, la.clone());
                                //     println!("test");
                                // }

                                // if dest_state_id == 21 && *la == RuleElement::Terminal(String::from("ELSE")) {
                                //     println!("({} -> {}) Dirty: {} because '{:?}' has been pushed into one of it's identifying rules", src_state_id, dest_state_id, dest_state_id, la.clone());
                                //     println!("test");
                                // }

                                change_detected = true;
                            }
                        }
                    }
                }

                

/*                
                //
                // Step 3 - check if the channel points to a normal rule
                //

                //
                // This is the same code as for identifying rules above.
                // Try to find 
                //

                println!("{:?}", dest_state);

                if dest_rule_id.0 == 33 {
                    println!("test");
                }

                for i in 0..dest_state.rules.len() {

                    if dest_state.rules[i].id == dest_rule_id.0 {

                        // copy lookaheads into dest rule
                        let temp_rule = src_rule.first().unwrap();
                        println!("{}", temp_rule);
                        for la in &temp_rule.lookahead {

                            if *la == RuleElement::NonTerminal(String::from(")")) {
                                println!("test");
                            }

                            // do not forward the end symbol within the same state, only inter states
                            //if src_state_id == dest_state_id {
                            //if *src_state_id == 0 as usize {
                                // if *la == RuleElement::Closure {
                                //     continue;
                                // }
                            //}

                            // do not forward in start state
                            // if src_state_id == dest_state_id && *src_state_id == 0 as usize {
                            //     continue;
                            // }

                            // within the same state only propagate if state currently has the dirty flag
                            if *src_state_id == dest_state_id && !dirty_state_ids.contains(&dest_state_id) {
                                // println!("no change to {}", dest_state_id);
                                // println!("");
                                continue;
                            }
                            
                            println!("Updating dirty state: {} {:?}", dest_state_id, la.clone());
                            if !dest_state.rules[i].lookahead.contains(&la) {
                                dest_state.rules[i].lookahead.push(la.clone());
                                change_detected = true;
                            }
                        }
                    }
                }

                // println!("Test");
*/
            } // over all destination rules



        } // over all rules




        println!("DirtySet: {:?}", dirty_state_ids);

        // for each state in the dirty set, perform inner propagation of the newly pushed symbol!
        for state_id in dirty_state_ids {

            let state = grammar_state_hashmap.get_mut(&state_id).unwrap();

            if state.id == 52 {
                println!("{:?}", state);
            }

            let mut local_rule_ids = Vec::<usize>::new();
            let mut processed_rule_ids = Vec::<usize>::new();

            // collect all rules which the identification_rules point to within the same state
            for i in 0..state.identification_rules.len() {

                let identification_rule_id = state.identification_rules[i].id;
                // println!("Rule id:{:?}", identification_rule_id);

                processed_rule_ids.push(identification_rule_id);

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
                        local_rule_ids.push(transition.0);

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
                
                processed_rule_ids.push(src_rule_id);

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
                                local_rule_ids.push(transition.0);
                            }
                        }
                    }

                //}
                

                /*
                // collect all rules which the normal rule points to within the same state
                // retrieve all channels for the current rule
                let dest_rule_transitions = rule_channel_map.get(&src_rule_id).unwrap();
                for transition in dest_rule_transitions {

                    let target_state = rule_id_to_state_id_map.get(&transition.0).unwrap();

                    // needs to be within same state
                    if *target_state == state_id {

                        // // push target rule id
                        // if !processed_rule_ids.contains(&transition.0) {

                            println!("{} -> {}", src_rule_id, &transition.0);

                            let mut source_rule_idx = 0;
                            let mut target_rule_idx = 0;

                            // insert lookahead into rule if not contained already
                            for jj in 0..state.rules.len() {
                                if state.rules[jj].id == src_rule_id {
                                    println!("source rule found!");
                                    source_rule_idx = jj;
                                }
                                if state.rules[jj].id == transition.0 {
                                    println!("target rule found!");
                                    target_rule_idx = jj;
                                }
                            }

                            let mut ttttt = state.rules[source_rule_idx].lookahead.clone();
                            state.rules[target_rule_idx].lookahead.append(&mut ttttt);

                            if !local_rule_ids.contains(&transition.0) {
                                local_rule_ids.push(transition.0);
                            }
                        // }
                    }
                }*/
            
                done = local_rule_ids.len() == 0;
            }
                
            if state.id == 50 {
                println!("{:?}", state);
            }

            // println!("done");
        }

        iteration = iteration + 1;

        // change_detected = false;
    }

    println!("Propagation cycles end after {} iterations.", iteration);

    println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++");

    

    // DEBUG
    // rust iterate over hashmap
    // https://stackoverflow.com/questions/45724517/how-to-iterate-through-a-hashmap-print-the-key-value-and-remove-the-value-in-ru
    println!("");
    println!("***************************************************************");
    println!("RESULT - FINISHED - READY - RESULT - FINISHED - READY - RESULT - FINISHED - READY - ");
    println!("***************************************************************");
    for (key, value) in &grammar_state_hashmap {
        println!("");
        println!("{} / {:?}", key, value);
        println!("");
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
    println!("***************************************************************");
    println!("Building the Parse Table from the LALR(1) DFA                  ");
    println!("***************************************************************");

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

    let mut parse_table = HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>::new();

    let rust_is_a_kek = grammar_state_hashmap.clone();

    let mut process_list = Vec::<usize>::new();
    let mut visited_list = Vec::<usize>::new();

    process_list.push(0);

    while process_list.len() > 0 {

        let current_state_id = process_list.remove(0);

        // DEBUG
        println!("");
        println!("Visiting State-ID: {}", current_state_id);

        if current_state_id == 9 {
            println!("test");
        }

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

            println!("  rule: {:?}", current_rule);

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
                
                if current_rule.lhs == augmented_start_symbol {
                    println!("    ACCEPT 1");
                    parse_table_row.insert(RuleElement::Closure, ParseTableCell::<usize>::Accept);

                    found_final_state = true;
                    final_state_id = current_state_id;
                }
                else {
                    println!("    REDUCE: {:?}", rule_id);

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

                println!("  Target State: {} over symbol: {:?}", target_state_id, &target_rule_id.1);

                if current_rule.dot_idx >= current_rule.rhs.len() {

                    // panic!("    leaf node");
                    
                } else {

                    // println!("    inner node");

                    match &target_rule_id.1 {
                        RuleElement::NonTerminal(_) => {
                            println!("    GOTO: {:?} -{:?}-> {:?}", state.id, &target_rule_id.1, target_state_id);

                            parse_table_row.insert(target_rule_id.1.clone(), ParseTableCell::<usize>::Goto(*target_state_id));
                        }
                        RuleElement::Terminal(_) => {
                            println!("    SHIFT: {:?} -{:?}-> {:?}", state.id, &target_rule_id.1, target_state_id);

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

            println!("  rule: {:?}", current_rule);

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
                
                if current_rule.lhs == augmented_start_symbol {
                    println!("    ACCEPT 2");
                    parse_table_row.insert(RuleElement::Closure, ParseTableCell::<usize>::Accept);

                    found_final_state = true;
                    final_state_id = current_state_id;
                }
                else {
                    println!("    REDUCE: {:?}", rule_id);
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

                println!("  Target State: {} over symbol: {:?}", target_state_id, &target_rule_id.1);

                if current_rule.dot_idx >= current_rule.rhs.len() {

                    // panic!("    leaf");
                    
                } else {

                    // println!("    inner");
                    match &target_rule_id.1 {

                        RuleElement::NonTerminal(_) => {
                            println!("    GOTO: {:?} -{:?}-> {:?}", state.id, &target_rule_id.1, target_state_id);

                            parse_table_row.insert(target_rule_id.1.clone(), ParseTableCell::<usize>::Goto(*target_state_id));
                        }

                        RuleElement::Terminal(_) => {
                            println!("    SHIFT: {:?} -{:?}-> {:?}", state.id, &target_rule_id.1, target_state_id);

                            parse_table_row.insert(target_rule_id.1.clone(), ParseTableCell::<usize>::Shift(*target_state_id));
                        }

                        RuleElement::Epsilon => {
                            // panic!("EPSILON!");

                            // TODO

                            println!("    SHIFT: {:?} -{:?}-> {:?}", state.id, &target_rule_id.1, target_state_id);

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




    if !found_final_state {
        panic!("DFA no final state detected!");
    } else {
        println!("Final state: {}", final_state_id);
    }





    //
    // DEBUG: Print the parse table
    //

    println!("");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("ParseTable");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    for i in 0..parse_table.len() {
        println!("{}) {:?}", i, parse_table[&i]);
    }
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");



    

    // let mut parse_table = HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>::new();

    // // state-id 0
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("a")), ParseTableCell::<usize>::Shift(2));
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("z")), ParseTableCell::<usize>::Shift(4));
    // parse_table_row.insert(RuleElement::Terminal(String::from("S")), ParseTableCell::<usize>::Goto(1));
    // parse_table_row.insert(RuleElement::Terminal(String::from("B")), ParseTableCell::<usize>::Goto(3));
    // parse_table.insert(0, parse_table_row);

    // // state-id 1
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("#")), ParseTableCell::<usize>::Accept);
    // parse_table.insert(1, parse_table_row);

    // // state-id 2
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("z")), ParseTableCell::<usize>::Shift(7));
    // parse_table_row.insert(RuleElement::Terminal(String::from("A")), ParseTableCell::<usize>::Goto(5));
    // parse_table_row.insert(RuleElement::Terminal(String::from("B")), ParseTableCell::<usize>::Goto(6));
    // parse_table.insert(2, parse_table_row);

    // // state-id 3
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("c")), ParseTableCell::<usize>::Shift(8));
    // parse_table.insert(3, parse_table_row);

    // // state-id 4
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("c")), ParseTableCell::<usize>::Reduce(14));
    // parse_table.insert(4, parse_table_row);

    // // state-id 5
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("c")), ParseTableCell::<usize>::Shift(9));
    // parse_table.insert(5, parse_table_row);

    // // state-id 6
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("d")), ParseTableCell::<usize>::Shift(10));
    // parse_table.insert(6, parse_table_row);

    // // state-id 7
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("c")), ParseTableCell::<usize>::Reduce(20));
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("d")), ParseTableCell::<usize>::Reduce(19));
    // parse_table.insert(7, parse_table_row);

    // // state-id 8
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("#")), ParseTableCell::<usize>::Reduce(21));
    // parse_table.insert(8, parse_table_row);

    // // state-id 9
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("#")), ParseTableCell::<usize>::Reduce(22));
    // parse_table.insert(9, parse_table_row);

    // // state-id 10
    // let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
    // parse_table_row.insert(RuleElement::NonTerminal(String::from("#")), ParseTableCell::<usize>::Reduce(23));
    // parse_table.insert(10, parse_table_row);

    // let parse_table_row = parse_table.get(&0);
    // let parser_step = parse_table_row.expect("Parse Table is broken!").get(&RuleElement::Terminal(String::from("S"))).unwrap();

    //
    // Driving the parser against input
    //

    println!("");
    println!("***************************************************************");
    println!("Driving the parser against input                               ");
    println!("***************************************************************");

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

/*
    // void main() {}
    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET CLOSING_CURLY_BRACKET
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACKET")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACKET")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACKET")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACKET")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

    // // void main() { EXPRESSION_STOP; }
    // // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET EXPRESSION_STOP SEMICOLON CLOSING_CURLY_BRACKET
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("EXPRESSION_STOP")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SEMICOLON")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);

    // void main() { EXPRESSION_STOP; }
    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET EXPRESSION_STOP SEMICOLON CLOSING_CURLY_BRACKET
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CAST_STOP")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SEMICOLON")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);

    // void main() { SIZEOF ( VOID ); }
    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET SIZEOF OPENING_BRACKET VOID CLOSING_BRACKET SEMICOLON CLOSING_CURLY_BRACKET
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SIZEOF")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SEMICOLON")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);

    // // void main() { IDENTIFIER = IDENTIFIER; }
    // // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET IDENTIFIER EQUALS_SIGN IDENTIFIER SEMICOLON CLOSING_CURLY_BRACKET
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("EQUALS_SIGN")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SEMICOLON")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);

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
    fragment_identifier.enfa.states.get_mut(&fragment_identifier.end_id).unwrap().token_id = 500;
    fragment_identifier.enfa.states.get_mut(&fragment_identifier.end_id).unwrap().token_name = String::from("IDENTIFIER");
    // insert into LEXER
    let (start_id_identifier, end_id_identifier) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_identifier.enfa, fragment_identifier.end_id);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_identifier);
    
    // DEBUG dump the graph to .dot format for viewing using https://dreampuf.github.io/GraphvizOnline
    //enfa_to_dot_directed_graph(&mut fragment_identifier.enfa, "fragment_identifier_automaton.dot");

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
    // Float Numeric - //{D}*"."{D}+({E})?{FS}?
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "(0|1|2|3|4|5|6|7|8|9)*.(0|1|2|3|4|5|6|7|8|9)+", "FLOAT_NUMERIC", 601);

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
    // DEBUG
    enfa_to_dot_directed_graph(&mut fragment_string_literal.enfa, "string_literal_enfa_automaton.dot");
    // insert into LEXER
    let (start_id_string_literal, end_id_string_literal) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_string_literal.enfa, fragment_string_literal.end_id);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_string_literal);

/*
    // //
    // // if (token-id: 110)
    // converter.infix_to_postfix("if");
    // let mut fragment_stack_if = FragmentStack::new();
    // recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_if, &mut alphabet);
    // converter.reset();
    // let mut fragment_if = fragment_stack_if.stack.pop().unwrap();
    // fragment_if.enfa.states.get_mut(&fragment_if.end_id).unwrap().token_id = 110;
    // fragment_if.enfa.states.get_mut(&fragment_if.end_id).unwrap().token_name = String::from("IF");

    // //
    // // VOID (token-id: 200)
    // converter.infix_to_postfix("void");
    // let mut fragment_stack_void = FragmentStack::new();
    // recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_void, &mut alphabet);
    // converter.reset();
    // let mut fragment_void = fragment_stack_void.stack.pop().unwrap();
    // fragment_void.enfa.states.get_mut(&fragment_void.end_id).unwrap().token_id = 200;
    // fragment_void.enfa.states.get_mut(&fragment_void.end_id).unwrap().token_name = String::from("VOID");

    // //
    // // INT (token-id: 210)
    // converter.infix_to_postfix("int");
    // let mut fragment_stack_int = FragmentStack::new();
    // recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_int, &mut alphabet);
    // converter.reset();
    // let mut fragment_int = fragment_stack_int.stack.pop().unwrap();
    // fragment_int.enfa.states.get_mut(&fragment_int.end_id).unwrap().token_id = 210;
    // fragment_int.enfa.states.get_mut(&fragment_int.end_id).unwrap().token_name = String::from("INT");

    //
    // Whitespace
    // ' ' (toke-id: 15)
    let mut fragment_stack_whitespace = FragmentStack::new();
    add_character_literal(&mut fragment_stack_whitespace, RegexBuildingBlock::CharacterLiteral(' '), &mut alphabet);
    // the top fragment on the fragment stack contains the root of the eNFA
    let mut fragment_whitespace = fragment_stack_whitespace.stack.pop().unwrap();
    fragment_whitespace.enfa.states.get_mut(&fragment_whitespace.end_id).unwrap().token_id = 15;
    fragment_whitespace.enfa.states.get_mut(&fragment_whitespace.end_id).unwrap().token_name = String::from("WHITESPACE");

    //
    // OPENING_BRACKET (token-id: 20)
    converter.infix_to_postfix("\\(");
    let mut fragment_stack_opening_bracket = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_opening_bracket, &mut alphabet);
    converter.reset();
    let mut fragment_opening_bracket = fragment_stack_opening_bracket.stack.pop().unwrap();
    fragment_opening_bracket.enfa.states.get_mut(&fragment_opening_bracket.end_id).unwrap().token_id = 20;
    fragment_opening_bracket.enfa.states.get_mut(&fragment_opening_bracket.end_id).unwrap().token_name = String::from("OPENING_BRACKET");

    //
    // CLOSING_BRACKET (token-id: 25)
    converter.infix_to_postfix("\\)");
    let mut fragment_stack_closing_bracket = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_closing_bracket, &mut alphabet);
    converter.reset();
    let mut fragment_closing_bracket = fragment_stack_closing_bracket.stack.pop().unwrap();
    fragment_closing_bracket.enfa.states.get_mut(&fragment_closing_bracket.end_id).unwrap().token_id = 25;
    fragment_closing_bracket.enfa.states.get_mut(&fragment_closing_bracket.end_id).unwrap().token_name = String::from("CLOSING_BRACKET");

    //
    // OPENING_SQUIGGLY_BRACKET (token-id: 30)
    converter.infix_to_postfix("\\{");
    let mut fragment_stack_opening_squiggly_bracket = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_opening_squiggly_bracket, &mut alphabet);
    converter.reset();
    let mut fragment_opening_squiggly_bracket = fragment_stack_opening_squiggly_bracket.stack.pop().unwrap();
    fragment_opening_squiggly_bracket.enfa.states.get_mut(&fragment_opening_squiggly_bracket.end_id).unwrap().token_id = 30;
    fragment_opening_squiggly_bracket.enfa.states.get_mut(&fragment_opening_squiggly_bracket.end_id).unwrap().token_name = String::from("OPENING_SQUIGGLY_BRACKET");

    //
    // CLOSING_SQUIGGLY_BRACKET (token-id: 35)
    converter.infix_to_postfix("\\}");
    let mut fragment_stack_closing_squiggly_bracket = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_closing_squiggly_bracket, &mut alphabet);
    converter.reset();
    let mut fragment_closing_squiggly_bracket = fragment_stack_closing_squiggly_bracket.stack.pop().unwrap();
    fragment_closing_squiggly_bracket.enfa.states.get_mut(&fragment_closing_squiggly_bracket.end_id).unwrap().token_id = 35;
    fragment_closing_squiggly_bracket.enfa.states.get_mut(&fragment_closing_squiggly_bracket.end_id).unwrap().token_name = String::from("CLOSING_SQUIGGLY_BRACKET");

    //
    // OPENING_ANGULAR_BRACKET (token-id: 40)
    converter.infix_to_postfix("\\[");
    let mut fragment_stack_opening_angular_bracket = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_opening_angular_bracket, &mut alphabet);
    converter.reset();
    let mut fragment_opening_angular_bracket = fragment_stack_opening_angular_bracket.stack.pop().unwrap();
    fragment_opening_angular_bracket.enfa.states.get_mut(&fragment_opening_angular_bracket.end_id).unwrap().token_id = 40;
    fragment_opening_angular_bracket.enfa.states.get_mut(&fragment_opening_angular_bracket.end_id).unwrap().token_name = String::from("OPENING_ANGULAR_BRACKET");

    //
    // CLOSING_ANGULAR_BRACKET (token-id: 45)
    converter.infix_to_postfix("\\]");
    let mut fragment_stack_closing_angular_bracket = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_closing_angular_bracket, &mut alphabet);
    converter.reset();
    let mut fragment_closing_angular_bracket = fragment_stack_closing_angular_bracket.stack.pop().unwrap();
    fragment_closing_angular_bracket.enfa.states.get_mut(&fragment_closing_angular_bracket.end_id).unwrap().token_id = 45;
    fragment_closing_angular_bracket.enfa.states.get_mut(&fragment_closing_angular_bracket.end_id).unwrap().token_name = String::from("CLOSING_ANGULAR_BRACKET");

    //
    // Semicolon (token-id: 50)
    converter.infix_to_postfix(";");
    let mut fragment_stack_semicolon = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_semicolon, &mut alphabet);
    converter.reset();
    let mut fragment_semicolon = fragment_stack_semicolon.stack.pop().unwrap();
    fragment_semicolon.enfa.states.get_mut(&fragment_semicolon.end_id).unwrap().token_id = 50;
    fragment_semicolon.enfa.states.get_mut(&fragment_semicolon.end_id).unwrap().token_name = String::from("SEMICOLON");

    //
    // Colon (token-id: 51)
    converter.infix_to_postfix(":");
    let mut fragment_stack_colon = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_colon, &mut alphabet);
    converter.reset();
    let mut fragment_colon = fragment_stack_colon.stack.pop().unwrap();
    fragment_colon.enfa.states.get_mut(&fragment_colon.end_id).unwrap().token_id = 51;
    fragment_colon.enfa.states.get_mut(&fragment_colon.end_id).unwrap().token_name = String::from("COLON");

    // //
    // // QUESTION_MARK (token-id: 52)
    // converter.infix_to_postfix("?");
    // let mut fragment_stack_question_mark = FragmentStack::new();
    // recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_question_mark, &mut alphabet);
    // converter.reset();
    // let mut fragment_question_mark = fragment_stack_question_mark.stack.pop().unwrap();
    // fragment_question_mark.enfa.states.get_mut(&fragment_question_mark.end_id).unwrap().token_id = 52;
    // fragment_question_mark.enfa.states.get_mut(&fragment_question_mark.end_id).unwrap().token_name = String::from("QUESTION_MARK");

    //
    // COMMA (token-id: 53)
    converter.infix_to_postfix(",");
    let mut fragment_stack_comma = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_comma, &mut alphabet);
    converter.reset();
    let mut fragment_comma = fragment_stack_comma.stack.pop().unwrap();
    fragment_comma.enfa.states.get_mut(&fragment_comma.end_id).unwrap().token_id = 53;
    fragment_comma.enfa.states.get_mut(&fragment_comma.end_id).unwrap().token_name = String::from("COMMA");

    //
    // EQUALS_SIGN (token-id: 54)
    converter.infix_to_postfix("=");
    let mut fragment_stack_equals_sign = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_equals_sign, &mut alphabet);
    converter.reset();
    let mut fragment_equals_sign = fragment_stack_equals_sign.stack.pop().unwrap();
    fragment_equals_sign.enfa.states.get_mut(&fragment_equals_sign.end_id).unwrap().token_id = 54;
    fragment_equals_sign.enfa.states.get_mut(&fragment_equals_sign.end_id).unwrap().token_name = String::from("EQUALS_SIGN");

    //
    // LessThan LT (token-id: 55)
    converter.infix_to_postfix("<");
    let mut fragment_stack_less_than = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_less_than, &mut alphabet);
    converter.reset();
    let mut fragment_less_than = fragment_stack_less_than.stack.pop().unwrap();
    fragment_less_than.enfa.states.get_mut(&fragment_less_than.end_id).unwrap().token_id = 55;
    fragment_less_than.enfa.states.get_mut(&fragment_less_than.end_id).unwrap().token_name = String::from("LT");

    //
    // GreaterThan GT (token-id: 60)
    converter.infix_to_postfix(">");
    let mut fragment_stack_greater_than = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_greater_than, &mut alphabet);
    converter.reset();
    let mut fragment_greater_than = fragment_stack_greater_than.stack.pop().unwrap();
    fragment_greater_than.enfa.states.get_mut(&fragment_greater_than.end_id).unwrap().token_id = 60;
    fragment_greater_than.enfa.states.get_mut(&fragment_greater_than.end_id).unwrap().token_name = String::from("GT");

    //
    // PLUS (token-id: 65)
    converter.infix_to_postfix("\\+");
    let mut fragment_stack_plus = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_plus, &mut alphabet);
    converter.reset();
    let mut fragment_plus = fragment_stack_plus.stack.pop().unwrap();
    fragment_plus.enfa.states.get_mut(&fragment_plus.end_id).unwrap().token_id = 65;
    fragment_plus.enfa.states.get_mut(&fragment_plus.end_id).unwrap().token_name = String::from("PLUS");

    //
    // MINUS (token-id: 66)
    converter.infix_to_postfix("\\-");
    let mut fragment_stack_minus = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_minus, &mut alphabet);
    converter.reset();
    let mut fragment_minus = fragment_stack_minus.stack.pop().unwrap();
    fragment_minus.enfa.states.get_mut(&fragment_minus.end_id).unwrap().token_id = 66;
    fragment_minus.enfa.states.get_mut(&fragment_minus.end_id).unwrap().token_name = String::from("MINUS");

    //
    // PERCENT (token-id: 67)
    converter.infix_to_postfix("%");
    let mut fragment_stack_percent = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_percent, &mut alphabet);
    converter.reset();
    let mut fragment_percent = fragment_stack_percent.stack.pop().unwrap();
    fragment_percent.enfa.states.get_mut(&fragment_percent.end_id).unwrap().token_id = 67;
    fragment_percent.enfa.states.get_mut(&fragment_percent.end_id).unwrap().token_name = String::from("PERCENT");

    //
    // INC_OP (token-id: 68)
    converter.infix_to_postfix("\\+\\+");
    let mut fragment_stack_inc_op = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_inc_op, &mut alphabet);
    converter.reset();
    let mut fragment_inc_op = fragment_stack_inc_op.stack.pop().unwrap();
    fragment_inc_op.enfa.states.get_mut(&fragment_inc_op.end_id).unwrap().token_id = 68;
    fragment_inc_op.enfa.states.get_mut(&fragment_inc_op.end_id).unwrap().token_name = String::from("INC_OP");

    // // DEBUG
    // enfa_to_dot_directed_graph(&mut fragment_fragment_whitespace.enfa, "fragment_hitespace_automaton.dot");

    //
    // Phase 2 - Combine all eNFA into a large eNFA
    //

    // // copy first keyword over (hello)
    // let (start_id_1, end_id_1) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_1.enfa, fragment_1.end_id);
    // // copy second keyword over (world)
    // let (start_id_2, end_id_2) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_2.enfa, fragment_2.end_id);
    // // copy third keyword over (int)
    // let (start_id_3, end_id_3) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_3.enfa, fragment_3.end_id);
    // // copy fourth keyword over (interop)
    // let (start_id_4, end_id_4) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_4.enfa, fragment_4.end_id);
    // // // copy 5th keyword over (ab)
    // // let (start_id_5, end_id_5) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_5.enfa, fragment_5.end_id);
    // // copy 6th keyword over (identifier)
    // let (start_id_6, end_id_6) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_6.enfa, fragment_6.end_id);
    // let (start_id_7, end_id_7) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_7.enfa, fragment_7.end_id);

    let (start_id_identifier, end_id_identifier)                                = enfa_copy(&mut combined_fragment.enfa, &mut fragment_identifier.enfa, fragment_identifier.end_id);
    let (start_id_numeric, end_id_numeric)                                      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_numeric.enfa, fragment_numeric.end_id);
    //let (start_id_return, end_id_return)                                        = enfa_copy(&mut combined_fragment.enfa, &mut fragment_return.enfa, fragment_return.end_id);
    // let (start_id_if, end_id_if)                                                = enfa_copy(&mut combined_fragment.enfa, &mut fragment_if.enfa, fragment_if.end_id);
    // let (start_id_void, end_id_void)                                            = enfa_copy(&mut combined_fragment.enfa, &mut fragment_void.enfa, fragment_void.end_id);
    // let (start_id_int, end_id_int)                                              = enfa_copy(&mut combined_fragment.enfa, &mut fragment_int.enfa, fragment_int.end_id);
    let (start_id_whitespace, end_id_whitespace)                                = enfa_copy(&mut combined_fragment.enfa, &mut fragment_whitespace.enfa, fragment_whitespace.end_id);
    let (start_id_opening_bracket, end_id_opening_bracket)                      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_opening_bracket.enfa, fragment_opening_bracket.end_id);
    let (start_id_closing_bracket, end_id_closing_bracket)                      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_closing_bracket.enfa, fragment_closing_bracket.end_id);
    let (start_id_opening_squiggly_bracket, end_id_opening_squiggly_bracket)    = enfa_copy(&mut combined_fragment.enfa, &mut fragment_opening_squiggly_bracket.enfa, fragment_opening_squiggly_bracket.end_id);
    let (start_id_closing_squiggly_bracket, end_id_closing_squiggly_bracket)    = enfa_copy(&mut combined_fragment.enfa, &mut fragment_closing_squiggly_bracket.enfa, fragment_closing_squiggly_bracket.end_id);
    let (start_id_opening_angular_bracket, end_id_opening_angular_bracket)      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_opening_angular_bracket.enfa, fragment_opening_angular_bracket.end_id);
    let (start_id_closing_angular_bracket, end_id_closing_angular_bracket)      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_closing_angular_bracket.enfa, fragment_closing_angular_bracket.end_id);
    let (start_id_semicolon, end_id_semicolon)                                  = enfa_copy(&mut combined_fragment.enfa, &mut fragment_semicolon.enfa, fragment_semicolon.end_id);
    let (start_id_colon, end_id_colon)                                          = enfa_copy(&mut combined_fragment.enfa, &mut fragment_colon.enfa, fragment_colon.end_id);
    // let (start_id_question_mark, end_id_question_mark)                          = enfa_copy(&mut combined_fragment.enfa, &mut fragment_question_mark.enfa, fragment_question_mark.end_id);
    let (start_id_comma, end_id_comma)                                          = enfa_copy(&mut combined_fragment.enfa, &mut fragment_comma.enfa, fragment_comma.end_id);
    let (start_id_equals_sign, end_id_equals_sign)                              = enfa_copy(&mut combined_fragment.enfa, &mut fragment_equals_sign.enfa, fragment_equals_sign.end_id);
    let (start_id_less_than, end_id_less_than)                                  = enfa_copy(&mut combined_fragment.enfa, &mut fragment_less_than.enfa, fragment_less_than.end_id);
    let (start_id_greater_than, end_id_greater_than)                            = enfa_copy(&mut combined_fragment.enfa, &mut fragment_greater_than.enfa, fragment_greater_than.end_id);
    let (start_id_plus, end_id_plus)                                            = enfa_copy(&mut combined_fragment.enfa, &mut fragment_plus.enfa, fragment_plus.end_id);
    let (start_id_minus, end_id_minus)                                          = enfa_copy(&mut combined_fragment.enfa, &mut fragment_minus.enfa, fragment_minus.end_id);
    let (start_id_percent, end_id_percent)                                      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_percent.enfa, fragment_percent.end_id);
    let (start_id_inc_op, end_id_inc_op)                                        = enfa_copy(&mut combined_fragment.enfa, &mut fragment_inc_op.enfa, fragment_inc_op.end_id);

    // add epsilon transitions to all the individual keyword eNFAs
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_identifier);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_numeric);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_whitespace);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_return);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_if);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_void);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_int);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_opening_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_closing_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_opening_squiggly_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_closing_squiggly_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_opening_angular_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_closing_angular_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_semicolon);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_colon);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_question_mark);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_comma);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_equals_sign);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_less_than);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_greater_than);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_plus);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_minus);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_percent);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_inc_op);
*/
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
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\{", "OPENING_SQUIGGLY_BRACKET", 23);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\}", "CLOSING_SQUIGGLY_BRACKET", 24);
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

    const WHITESPACE_TOKEN_ID: usize = 46;
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, " ", "WHITESPACE", WHITESPACE_TOKEN_ID);

    const NEWLINE_TOKEN_ID: usize = 47;
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

    let mut current_state_id = dfa.start_state_id;
    let mut last_state_id = dfa.start_state_id;

    // NUMERIC PLUS NUMERIC
    // let str = "2 + 2";

    // NUMERIC PLUS NUMERIC SEMICOLON
    // let str = "2 + 2;";

    // IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET RETURN NUMERIC SEMICOLON
    // let str = "if (1 < 2) return 2;";

    // IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET RETURN NUMERIC PLUS NUMERIC SEMICOLON
    // let str = "if (1 < 2) return 2 + 2;";

    // IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET RETURN NUMERIC PLUS NUMERIC SEMICOLON CLOSING_SQUIGGLY_BRACKET
    // let str = "if (1 < 2) { }";

    // IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET RETURN SEMICOLON CLOSING_SQUIGGLY_BRACKET
    // let str = "if (1 < 2) { return; }";

    // let str = "if (1 < 2) { return 0; }";

    // let str = "if (1 < 2) { return 2 + 2; }";

    // let str = "if (1 < 2) { return 2 + 2; } if (1 < 2) { return 2 + 2; }";

    // let str = "{ if (1 < 2) { return 2 + 2; } if (1 < 2) { return 2 + 2; } }";

    // https://github.com/nlsandler/writing-a-c-compiler-tests/blob/main/tests/chapter_1/valid/return_0.c
    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_SQUIGGLY_BRACKET
    // let str = "int main() { return 2; }";
    // let str = "int main() { return void; }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET INT IDENTIFIER SEMICOLON CLOSING_SQUIGGLY_BRACKET
    // let str = "int main() { int abc; }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET INT IDENTIFIER SEMICOLON RETURN NUMERIC SEMICOLON CLOSING_SQUIGGLY_BRACKET
    // let str = "int main() { int abc; return 2; }";

    // let str = "int main() { if (1 < 2) return 2; }";
    // let str = "int main() { if (1 < 2) { return 2; } }";
    
    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_SQUIGGLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_SQUIGGLY_BRACKET
    // let str = "int main() { if (1 < 2) { return 2; } return 0; }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_SQUIGGLY_BRACKET ELSE OPENING_SQUIGGLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_SQUIGGLY_BRACKET CLOSING_SQUIGGLY_BRACKET
    // let str = "int main() { if (1 < 2) { return 2; } else { return 3; } }";
    // let str = "int main() { if (1 < 2) { return; } else { return; } }";
    // let str = "int main() { if (1 < 2) {} else {} }";
 
    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET IF OPENING_BRACKET VOID CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET CLOSING_SQUIGGLY_BRACKET ELSE OPENING_SQUIGGLY_BRACKET CLOSING_SQUIGGLY_BRACKET CLOSING_SQUIGGLY_BRACKET
    // let str = "void main() { if (void) {} else {} }";
    // let str = "void main() { if (1) {} else {} }";
    // let str = "void main() { if (1) { return; } else {} }";
    // let str = "void main() { if (1) { return; } else {return 0; } }";
    // let str = "void main() { if (1 < 2) { return; } else {return 0; } }";
    // let str = "void main() { if (1 < 2) { return 2 + 2; } else {return 0; } }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET IF OPENING_BRACKET EXPRESSION_STOP CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET STATEMENT_STOP CLOSING_SQUIGGLY_BRACKET ELSE OPENING_SQUIGGLY_BRACKET STATEMENT_STOP CLOSING_SQUIGGLY_BRACKET CLOSING_SQUIGGLY_BRACKET
    // let str = "int main() { if (EXPRESSION_STOP) { STATEMENT_STOP } else { STATEMENT_STOP } }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET IF OPENING_BRACKET EXPRESSION_STOP CLOSING_BRACKET STATEMENT_STOP ELSE STATEMENT_STOP CLOSING_SQUIGGLY_BRACKET
    // let str = "int main() { if (EXPRESSION_STOP) STATEMENT_STOP else STATEMENT_STOP }";

    // IF OPENING_BRACKET EXPRESSION_STOP CLOSING_BRACKET STATEMENT_STOP ELSE STATEMENT_STOP
    // let str = "if ( EXPRESSION_STOP ) STATEMENT_STOP else STATEMENT_STOP";
    // let str = "if ( void ) void else void";
    // let str = "if ( void ) return; else return;";
    // let str = "if ( void ) return; else if ( void ) return;";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET RETURN NUMERIC PLUS NUMERIC SEMICOLON CLOSING_SQUIGGLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_SQUIGGLY_BRACKET
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

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET INT IDENTIFIER EQUALS_SIGN NUMERIC SEMICOLON CLOSING_SQUIGGLY_BRACKET
    // let str = "int main() { int a = 0; }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET INT IDENTIFIER IDENTIFIER EQUALS_SIGN NUMERIC SEMICOLON CLOSING_SQUIGGLY_BRACKET
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

    // let str = "int myFunction(int x, int y) { return x + y; }";

    // let str = "\"aaa\"";
    // let str = "int main() { char *message = \"aaa\"; }";
    //let str = "int main() { char *message = \"This is a string literal.\"; }";
    // let str = "int main() { char *message = \"This is a string literal.\"; }";
    // let str = "int main() { printf(\"This is a string literal: %d.\", 199); }";

    // let str = "int main(int argc, char **argv) { int (*say)(const char *); }";
    // let str = "int main(int argc, char **argv) { int (*say)(const char *); say = puts; }";
    // let str = "int main(int argc, char **argv) { int (*say)(const char *); say = puts; say(\"hello world\"); }";
    //let str = "int main(int argc, char **argv) { int (*say)(const char *); say = puts; say(\"hello world\"); return 0; }";

    // INT IDENTIFIER SEMICOLON
    // let str = "int abc;";

    // VOID VOID VOID VOID VOID
    //let str = "void void void void void";

    // INT PLUS VOID
    // let str = "int + void";

    //let str: String = fs::read_to_string("res/C/samples/kernighan_ritchie/page_9.c").expect("file cannot be read!");
    let str: String = fs::read_to_string("res/C/samples/kernighan_ritchie/page_10.c").expect("file cannot be read!");

    println!("Input: {}", str);
    
    let lexer_debug: bool = true;

    let mut token_string_buffer = String::from("");
    for character in str.chars() {

        let mut char_consumed = false;
        while !char_consumed {

            last_state_id = current_state_id;

            if lexer_debug {
                println!("[LEXER] Input: '{}'", character);
            }

            // try to transition the large lexer DFA to produce a token for the input
            current_state_id = transition_dfa(&mut dfa, current_state_id, &RegexBuildingBlock::CharacterLiteral(character));

            if dfa.is_end_state(current_state_id) {

                // println!("STATE '{}' END STATE!", current_state_id);
                // println!("ACCEPTING '{}'! END STATE! Token-Id: {}", token_string_buffer, dfa.states[&current_state_id].token_id);

                token_string_buffer.push(character);

                char_consumed = true;

            } else if dfa.is_trap_state(current_state_id) {

                // reset the lexer's DFA back to the start state and 
                // try to accept the symbol again which was read from input already
                char_consumed = false;
                current_state_id = dfa.start_state_id;

                if lexer_debug {
                    println!("[LEXER] Emitting '{}', Token-Id: {}, Token-Name: {}", token_string_buffer, dfa.states[&last_state_id].token_id, dfa.states[&last_state_id].token_name);
                    println!("");
                }

                let terminal = RuleElement::Terminal(dfa.states[&last_state_id].token_name.clone());

                //if lexer_debug {
                    println!("[LEXER] {:?} ---> {:?}", token_string_buffer, terminal);
                //}

                // // IGNORE TOKEN
                // // IGNORE WHITESPACE
                // //
                // // WHITESPACE_TOKEN_ID is the token-id of whitespace, ignore whitespace!
                // if dfa.states[&last_state_id].token_id != WHITESPACE_TOKEN_ID {
                //     provide_input(&mut parser, 
                //         &grammar_state_hashmap, 
                //         &mut step, 
                //         &terminal);
                // }

                match dfa.states[&last_state_id].token_id {
                    NEWLINE_TOKEN_ID | WHITESPACE_TOKEN_ID => {
                        // nop
                    }
                    _ => {
                        provide_input(&mut parser, 
                        &grammar_state_hashmap, 
                        &mut step, 
                        &terminal);
                    }
                }

                token_string_buffer.clear();

            } else {
                // println!("STATE '{}' NOT END STATE!", current_state_id);

                token_string_buffer.push(character);

                char_consumed = true;
            }
        }
    }

    if lexer_debug {
        println!("[LEXER] Emitting '{}'. Token-Id: {}, Token-Name: {}", token_string_buffer, dfa.states[&current_state_id].token_id, dfa.states[&current_state_id].token_name);
        println!("");
    }
    
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(dfa.states[&current_state_id].token_name.clone()));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);

    // // void main() { RETURN 0; }
    // // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_CURLY_BRACKET
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("RETURN")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("NUMERIC")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SEMICOLON")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACKET")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
/*
    let mut consumed = false;

    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        consumed = parser.consume(RuleElement::Terminal(String::from("a")), &grammar_state_hashmap);
        step = step + 1;
    }

    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        consumed = parser.consume(RuleElement::Terminal(String::from("z")), &grammar_state_hashmap);
        step = step + 1;
    }

    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        consumed = parser.consume(RuleElement::Terminal(String::from("c")), &grammar_state_hashmap);
        step = step + 1;
    }

    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        //consumed = parser.consume(RuleElement::Terminal(String::from("#")), &grammar_state_hashmap);
        consumed = parser.consume(RuleElement::Closure, &grammar_state_hashmap);
        step = step + 1;
    }
*/

/*
    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        consumed = parser.consume(RuleElement::Terminal(String::from("a")), &grammar_state_hashmap);
        step = step + 1;
    }

    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        consumed = parser.consume(RuleElement::Terminal(String::from("c")), &grammar_state_hashmap);
        step = step + 1;
    }

    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        consumed = parser.consume(RuleElement::Terminal(String::from("b")), &grammar_state_hashmap);
        step = step + 1;
    }

    consumed = false;
    while !consumed {
        println!("");
        println!("Step {}", step);
        //consumed = parser.consume(RuleElement::Terminal(String::from("#")), &grammar_state_hashmap);
        consumed = parser.consume(RuleElement::Closure, &grammar_state_hashmap);
        step = step + 1;
    }
*/

/*
    // IF ( EXPRESSION ) TEST_STMT ELSE TEST_STMT
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IF")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("(")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("EXPRESSION")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from(")")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("TEST_STMT")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("ELSE")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("TEST_STMT")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

/*
    // IF ( EXPRESSION ) TEST_STMT ELSE IF ( EXPRESSION ) TEST_STMT ELSE TEST_STMT
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IF")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("(")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("EXPRESSION")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from(")")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("TEST_STMT")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("ELSE")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IF")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("(")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("EXPRESSION")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from(")")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("TEST_STMT")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("ELSE")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("TEST_STMT")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

/*
    // * id = id
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("*")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("id")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("=")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("id")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

/*
    //  n - ( n )
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("n")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("-")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("(")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("n")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from(")")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

/*
    // a b a b
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("a")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("b")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("a")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("b")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

/*
    // a z d #
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("a")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("z")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("d")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

/*
    // a c b #
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("a")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("c")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("b")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

/*
    // d a h g #
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("d")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("a")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("h")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("g")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

    println!("end");
}

fn provide_input(parser: &mut Parser<String>, grammar_state_hashmap: &BTreeMap<usize, GrammarState<String>>, step: &mut usize, rule_element: &RuleElement<String>) -> usize {

    let mut consumed = false;
    while !consumed {
        println!("");
        println!("[provide_input] Step {}", *step);
        consumed = parser.consume(rule_element.clone(), &grammar_state_hashmap);
        *step = *step + 1;
    }

    *step
}