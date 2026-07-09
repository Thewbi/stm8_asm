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
use crate::examplegrammars::left_recursive::produce_grammar_left_recursive;
use crate::examplegrammars::grammar_1::produce_grammar_1;
use crate::examplegrammars::grammar_2::produce_grammar_2;
use crate::examplegrammars::grammar_3::produce_grammar_3;

#[derive(Clone)]
pub struct GrammarState<T: Debug> {
    pub id: usize,
    pub current_rule: Rule<T>,
    pub identification_rules: Vec::<Rule<T>>,
    pub rules: Vec::<Rule<T>>,
}

impl<T: Clone + Debug + Display + std::cmp::PartialEq + Ord> GrammarState<T> {

    pub fn new(id: usize) -> Self {
        GrammarState {
            id: id,
            current_rule: Rule::new(id),
            identification_rules: Vec::<Rule<T>>::new(),
            rules: Vec::<Rule<T>>::new(),
        }
    }

    // For this function to work, insert at least one rule into the identification_rules 
    // set of the grammar set prior to calling this function!
    //
    // This function will develop all rules in the identification_rules set into the closure 
    // of all rules that the parser can potentially activate on any input symbol when it is 
    // located in the state for which this function is called.
    // 
    // All these rules are inserted into the rules-set of the grammar state.
    //
    // The states are created prior to calling this function. The states are created in the
    // same large loop that also calls this function
    pub fn unfold_grammar_state(&mut self, grammar_rules: &Vec::<Rule<T>>,
        first: &BTreeMap<RuleElement::<T>, Vec::<RuleElement::<T>>>,
        nullable: &BTreeMap::<RuleElement::<T>, bool>,
        rule_channel_map: &mut HashMap::<usize, Vec::<Transition<T>>>,
    ) {

        let debug: bool = true;

        let apply_lookahead = true;

        // if apply_lookahead {
        //     // append symbol into lookahead of each identification rule
        //     //for &mut ident_rule in self.identification_rules {
        //     for i in 0..self.identification_rules.len() {
        //         if !self.identification_rules[i].lookahead.contains(&RuleElement::Closure) {
        //             self.identification_rules[i].lookahead.push(RuleElement::Closure);
        //         }
        //     }
        // }

        // DEBUG
        if debug {
            if self.id == 9 {
                println!("{:?}", self);
                println!("test");
            }
        }

        // scratchpad of rules to process
        let mut d_set = Vec::<Rule<T>>::new();
        d_set.append(&mut self.identification_rules.clone());

        // while scratchpad has rules on it, loop
        let mut done: bool = d_set.is_empty();
        while !done {

            let mut current_rule: Rule<T> = d_set.pop().expect("Need at least one rule!");

            // DEBUG
            if debug {
                println!("[unfold_grammar_state] State-ID: {}, current_rule: {}", self.id, current_rule);
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
            // STEP 1 - collect all lookaheads for the LHS nonterminal
            //          Lookaheads are required for the parse table.
            //          In LALR(1) lookaheads are essential parts of a rule.
            //          The algorithm needs to build the rule plus it's lookaheads to produce valid rule items!
            //

            let mut current_lookahead = Vec::<RuleElement<T>>::new();

            // DEBUG
            if debug {
                println!("[unfold_grammar_state] Determining lookahead for Rule: {}. Rule has lookahead: {:?}", current_rule, current_rule.lookahead);
            }

            // find beta, if there is no beta, lookahead is the rule's own lookahead
            if current_rule.dot_idx + 1 >= current_rule.rhs.len() {

                // empty beta
                if debug {
                    println!("[unfold_grammar_state] empty beta {:?}", current_rule.lookahead);
                }

                current_lookahead.append(&mut current_rule.lookahead);

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

                            let first_values_opt = first.get(&current_rule.rhs[beta_idx]);
                            match first_values_opt {
                                Some(first_values) => {
                                    if debug {
                                        println!("[unfold_grammar_state] first_values >> {:?}", first_values.clone());
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
                            if !nullable.contains_key(&current_rule.rhs[beta_idx]) {
                                break;
                            }
                            
                            // panic!("test");
                        }

                        RuleElement::Terminal(terminal) => {
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
                        println!("[unfold_grammar_state] Will extend closure due to Rule: {} and NonTerminal '{}' with lookaheads '{:?}'", current_rule, non_terminal, current_lookahead);
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
                                println!("[unfold_grammar_state] Inserting into closure Rule: [{}] {} using lookaheads: {:?} because of source-rule-id: {}", grammar_rules[i].id, grammar_rules[i], current_lookahead, &current_rule.id);
                            }

                            let mut contained_already = false;
                            for j in 0..self.rules.len() {

                                if self.rules[j] == grammar_rules[i] {

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
                            self.rules.push(rule.clone());

                            d_set.insert(0, rule);
                        }
                    }
                }

                _ => {
                    // nop
                }
            }

            // // over all rules that unfold from the rules via SHIFT operations of terminals and nonterminals
            // match &current_rule.rhs[current_rule.dot_idx] {

            //     // if the dot is points to a non-terminal, extend the rule set
            //     RuleElement::NonTerminal(non_terminal) => {
            //         // DEBUG
            //         if debug {
            //             println!("[unfold_grammar_state] SHIFT Will extend closure due to Rule: {} and NonTerminal '{}' with lookaheads '{:?}'", current_rule, non_terminal, current_lookahead);
            //             println!("");
            //         }
            //     }

            //     RuleElement::NonTerminal(terminal) => {
            //         // DEBUG
            //         if debug {
            //             println!("[unfold_grammar_state] SHIFT Will extend closure due to Rule: {} and Terminal '{}' with lookaheads '{:?}'", current_rule, terminal, current_lookahead);
            //             println!("");
            //         }
            //     }

            //     _ => todo!()

            // }

            done = d_set.is_empty();
        }
    }
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
    // is called nullable. You can find all nullable nonterminals by using a simple iterative marking 
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

        for i in 0..grammar_rules.len() {

            // println!("{:?}", &grammar_rules[i]);

            // if a way to an epsilon is found, never change that status
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

                // a nonterminal in a rule makes the rule non-nullable
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

    // println!("Test");
}

pub fn validate_grammar(grammar_rules: &mut Vec::<Rule<String>>) {

    println!("");
    println!("Validating grammar start ...");

    // collect all non-terminals into a set
    // iterate over the set
    // check for each non-terminal if it appears on the left side of at least one production rule
    // if a non-terminal is found that does not satisfy this test, the grammar is invalid! Abort!

    //let mut nonterminal_set: HashSet<&Rule<String>> = HashSet::new();
    let mut rhs_nonterminal_set: HashSet::<RuleElement<String>> = HashSet::new();
    let mut lhs_nonterminal_set: HashSet::<RuleElement<String>> = HashSet::new();

    for rule in grammar_rules.iter() {

        //println!("{:?}", rule);

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
            panic!("[Invalid Grammar] The NonTerminal '{:?}' does not appear on the LeftHandSide of any production rule in the grammar although it is used as a RightHandSide element in at least one production rule! The grammar is incomplete! The non-terminal '{:?}' cannot be reduced! Please fix the grammar before proceeding!", &rule_element, &rule_element);
        }
    }

    println!("Validating grammar end.");
}

fn main() {

    println!("start");

    let mut grammar_rules = Vec::<Rule<String>>::new();

    //
    // Select one of the grammars
    //
    
    let g_result = produce_grammar_c_full(&mut grammar_rules);
    // let g_result = produce_grammar_left_recursive(&mut grammar_rules);
    // let g_result = produce_grammar_1(&mut grammar_rules); // has epsilon rules (wont work)
    // let g_result = produce_grammar_2(&mut grammar_rules);
    // let g_result = produce_grammar_3(&mut grammar_rules); // shows # is not propagated

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
    let mut grammar_state: GrammarState<String> = GrammarState::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));
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

        let current_grammar_state_id = e_set.pop().expect("Need at least one state!");
        processed_set.push(current_grammar_state_id);

        // unfold node
        if let Some(grammar_state) = grammar_state_hashmap.get_mut(&current_grammar_state_id) {

            // unfold_grammar_state is probably the CLOSURE() operation
            // the rule_channel_map is extended with new entries, by this call
            grammar_state.unfold_grammar_state(&grammar_rules, &first, &nullable, &mut rule_channel_map);

            // // DEBUG
            // println!("\n");
            // println!("----------------------------------------");
            // println!("{:?}", grammar_state);
            // println!("========================================");
        }

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

            let mut state_contained_already = false;
            let mut state_id: usize = 0;

            for (loop_state_id, loop_state) in &grammar_state_hashmap {

                // a state is identified via the (all rules in) identification rules set
                if loop_state.identification_rules == rules_for_symbol_copy {

                    // state found
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

                if current_grammar_state_id == state_id {
                    // // DEBUG
                    // println!("[CHANNELS] STATE-TRANSITION-EXISTING-SELF: {:?} -{:?}-> {:?}", &current_grammar_state_id, &current_symbol, &state_id);
                } else {
                    // // DEBUG output transition (to already existing state)
                    // println!("[CHANNELS] STATE-TRANSITION-EXISTING: {:?} -{:?}-> {:?}", &current_grammar_state_id, &current_symbol, &state_id);
                }

            } else {

                // state not contained, build state, insert into e_set, insert transition

                let mut new_grammar_state: GrammarState<String> = GrammarState::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));

                // // DEBUG - output rule channels
                // for rule_copy in &mut rules_for_symbol_copy {
                //     println!("RULE-RULE-CHANNEL: ? -> {}", rule_copy.id);
                // }

                // DEBUG
                // println!("Created new state: {:?}", new_grammar_state.id);

                //new_grammar_state.identification_rules.append(&mut rules_for_symbol);
                //new_grammar_state.identification_rules.append(&mut rules_for_symbol_copy);

                let mut iter_index: usize = 0;
                for rule_copy in rules_for_symbol_copy {

                    let rule_copy_id = rule_copy.id;
                    new_grammar_state.identification_rules.push(rule_copy);

                    // // DEBUG
                    // println!("[CHANNELS] RULE-RULE-CHANNEL: {} -> {}", src_rule_id[iter_index], rule_copy_id);

                    if !rule_channel_map.contains_key(&src_rule_id[iter_index]) {

                        let channel_ends = Vec::<Transition<String>>::new();

                        // TODO (usize, RuleElement)
                        rule_channel_map.insert(src_rule_id[iter_index], channel_ends);
                    }

                    // retrieve the vector of first symbols for the nonterminal and extend it
                    let channel_ends = &mut rule_channel_map.get_mut(&src_rule_id[iter_index]).unwrap();

                    // TODO: (target-id, RuleElement)
                    channel_ends.push(Transition(rule_copy_id, current_symbol.clone()));

                    iter_index = iter_index + 1;
                }

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

    // // provide fast access, map from rule-id to rule
    // let id_to_rule_map = HashMap::<usize, Rule<String>>::new();

    let mut rule_id_to_state_id_map = HashMap::<usize, usize>::new();
    let mut rule_ids = Vec::<usize>::new();

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

    let mut change_detected = true;
    let mut iteration = 0;
    while change_detected {

        change_detected = false;

        println!("[Propagation] Iteration: {:?}", iteration);

        let mut dirty_state_ids = Vec::<usize>::new();
        // println!("new");

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

            // over all of the channels to target rules that this rule has
            let dest_rule_ids = rule_channel_map.get(src_rule_id).unwrap();

            for dest_rule_id in dest_rule_ids {

                let dest_state_id = rule_id_to_state_id_map.get(&dest_rule_id.0).unwrap();

                // if *src_rule_id == 9 as usize {
                //     // println!("Src-RuleId: {:?}, Src-StateId: {:?} ===> Dest-RuleId: {:?}, Dest-StateId: {:?}",
                //     //     src_rule_id, src_state_id, dest_rule_id, dest_state_id);
                //     println!("test");
                // }

                // push lookaheads

                // retrieve src-rule from src-state (src-state is read-only, non-mutable clone!)
                let src_state = grammar_state_hashmap.get(src_state_id).unwrap().clone();

                let mut src_rule = src_state.identification_rules.iter().filter(|r| r.id == *src_rule_id).collect::<Vec<_>>();
                if src_rule.len() == 0 {
                    src_rule = src_state.rules.iter().filter(|r| r.id == *src_rule_id).collect::<Vec<_>>();
                }
                // println!("{:?}", &src_rule.first());

                // retrieve dest-rule from dest-state
                // let dest_state = grammar_state_hashmap.get(dest_state_id).unwrap();
                let dest_state = grammar_state_hashmap.get_mut(dest_state_id).unwrap();
                // println!("{:?}", dest_state);

                // let mut dest_rule = dest_state.identification_rules.iter().filter(|r| r.id == *dest_rule_id).collect::<Vec<_>>();
                // if dest_rule.len() == 0 {
                //     dest_rule = dest_state.rules.iter().filter(|r| r.id == *dest_rule_id).collect::<Vec<_>>();
                // }
                // println!("{:?}", &dest_rule.first());

                for i in 0..dest_state.identification_rules.len() {

                    if dest_state.identification_rules[i].id == dest_rule_id.0 {

                        // copy lookaheads into dest rule
                        for la in &src_rule.first().unwrap().lookahead {

                            // do not forward the end symbol within the same state, only inter states
                            //if src_state_id == dest_state_id {
                            //if *src_state_id == 0 as usize {
                                // if *la == RuleElement::Closure {
                                //     continue;
                                // }
                            //}

                            // // do not forward in start state
                            // if src_state_id == dest_state_id && *src_state_id == 0 as usize {
                            //     continue;
                            // }

                            // if a lookahead is inserted into the identification rules where it 
                            // has not been contained already, the state becomes dirty
                            if !dest_state.identification_rules[i].lookahead.contains(&la) {

                                // insert lookahead
                                dest_state.identification_rules[i].lookahead.push(la.clone());

                                println!("{}", dest_state.identification_rules[i].id);

                                // identification rules have been changed, a new lookahead was added.
                                // The state becomes dirty and a progation inside that state and
                                // across that state to other states needs to take place
                                dirty_state_ids.push(*dest_state_id);

                                println!("Dirty: {} {:?}", dest_state_id, la.clone());
                                println!("");

                                change_detected = true;
                            }
                        }
                    }
                }

                for i in 0..dest_state.rules.len() {

                    if dest_state.rules[i].id == dest_rule_id.0 {

                        // copy lookaheads into dest rule
                        let temp_rule = src_rule.first().unwrap();
                        println!("{}", temp_rule);
                        for la in &temp_rule.lookahead {

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
                            if src_state_id == dest_state_id && !dirty_state_ids.contains(dest_state_id) {
                                // println!("no change to {}", dest_state_id);
                                // println!("");
                                continue;
                            }
                            
                            println!("Changing {} {:?}", dest_state_id, la.clone());
                            if !dest_state.rules[i].lookahead.contains(&la) {

                                dest_state.rules[i].lookahead.push(la.clone());

                                change_detected = true;
                            }
                        }
                    }
                }

                // println!("Test");
            }
        }

        iteration = iteration + 1;
    }

    println!("Propagation cycles end after {} iterations.", iteration);

    println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++");

    

    // DEBUG
    // rust iterate over hashmap
    // https://stackoverflow.com/questions/45724517/how-to-iterate-through-a-hashmap-print-the-key-value-and-remove-the-value-in-ru
    println!("");
    println!("***************************************************************");
    println!("RESULT - RESULT - RESULT - RESULT - RESULT - RESULT - RESULT - ");
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
        // convert DFA state into parser table row
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
    // DEBUG: PRINT PARSE TABLE
    //

    println!("");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    // println!("ParseTable: {:?}", parse_table);
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

    //
    // Pre-Build alphabet
    //

    // complete alphabet has to be known in advance
    let mut alphabet = HashSet::<RegexBuildingBlock>::new();

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('a'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('b'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('c'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('d'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('e'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('f'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('g'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('h'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('i'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('j'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('k'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('l'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('m'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('n'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('o'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('p'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('q'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('r'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('s'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('t'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('u'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('v'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('w'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('x'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('y'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('z'));

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

    alphabet.insert(RegexBuildingBlock::CharacterLiteral(' '));

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('_'));

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('<'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('>'));
    
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('{'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('}'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('('));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral(')'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('['));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral(']'));

    alphabet.insert(RegexBuildingBlock::CharacterLiteral(';'));

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('+'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('-'));

    // alphabet.insert(RegexBuildingBlock::CharacterLiteral('\n'));

    //
    // Phase 1 - build all regexes
    //

    //
    // identifier

    // provide a regex in infix notation and let the converter produce a postfix notation
    // The result is stored within the state of the converter instance, this is why the converter can be reset
    let mut converter = InfixPostfixConverter::new();
    //converter.infix_to_postfix("(_|a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z)(_|a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z)+");
    converter.infix_to_postfix("(_|a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z)|(_|a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z)(_|a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z|0|1|2|3|4|5|6|7|8|9)+");
    
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

    // auto        break       case        char 
    // const       continue    default     do 
    // double      else        enum        extern 
    // float       for         goto        if 
    // int         long        register    return 
    // short       signed      sizeof      static 
    // struct      switch      typedef     union 
    // unsigned    void        volatile    while

    //
    // RETURN (token-id: 100)
    converter.infix_to_postfix("return");
    let mut fragment_stack_return = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_return, &mut alphabet);
    converter.reset();
    let mut fragment_return = fragment_stack_return.stack.pop().unwrap();
    fragment_return.enfa.states.get_mut(&fragment_return.end_id).unwrap().token_id = 100;
    fragment_return.enfa.states.get_mut(&fragment_return.end_id).unwrap().token_name = String::from("RETURN");

    //
    // if (token-id: 110)
    converter.infix_to_postfix("if");
    let mut fragment_stack_if = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_if, &mut alphabet);
    converter.reset();
    let mut fragment_if = fragment_stack_if.stack.pop().unwrap();
    fragment_if.enfa.states.get_mut(&fragment_if.end_id).unwrap().token_id = 110;
    fragment_if.enfa.states.get_mut(&fragment_if.end_id).unwrap().token_name = String::from("IF");

    //
    // VOID (token-id: 200)
    converter.infix_to_postfix("void");
    let mut fragment_stack_void = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_void, &mut alphabet);
    converter.reset();
    let mut fragment_void = fragment_stack_void.stack.pop().unwrap();
    fragment_void.enfa.states.get_mut(&fragment_void.end_id).unwrap().token_id = 200;
    fragment_void.enfa.states.get_mut(&fragment_void.end_id).unwrap().token_name = String::from("VOID");

    //
    // INT (token-id: 210)
    converter.infix_to_postfix("int");
    let mut fragment_stack_int = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_int, &mut alphabet);
    converter.reset();
    let mut fragment_int = fragment_stack_int.stack.pop().unwrap();
    fragment_int.enfa.states.get_mut(&fragment_int.end_id).unwrap().token_id = 210;
    fragment_int.enfa.states.get_mut(&fragment_int.end_id).unwrap().token_name = String::from("INT");

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


    // // DEBUG
    // enfa_to_dot_directed_graph(&mut fragment_fragment_whitespace.enfa, "fragment_hitespace_automaton.dot");

    //
    // Phase 2 - Combine all eNFA into a large eNFA
    //

    let mut combined_fragment = Fragment::new(RegexBuildingBlock::Or);

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
    let (start_id_return, end_id_return)                                        = enfa_copy(&mut combined_fragment.enfa, &mut fragment_return.enfa, fragment_return.end_id);
    let (start_id_if, end_id_if)                                                = enfa_copy(&mut combined_fragment.enfa, &mut fragment_if.enfa, fragment_if.end_id);
    let (start_id_void, end_id_void)                                            = enfa_copy(&mut combined_fragment.enfa, &mut fragment_void.enfa, fragment_void.end_id);
    let (start_id_int, end_id_int)                                              = enfa_copy(&mut combined_fragment.enfa, &mut fragment_int.enfa, fragment_int.end_id);
    let (start_id_whitespace, end_id_whitespace)                                = enfa_copy(&mut combined_fragment.enfa, &mut fragment_whitespace.enfa, fragment_whitespace.end_id);
    let (start_id_opening_bracket, end_id_opening_bracket)                      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_opening_bracket.enfa, fragment_opening_bracket.end_id);
    let (start_id_closing_bracket, end_id_closing_bracket)                      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_closing_bracket.enfa, fragment_closing_bracket.end_id);
    let (start_id_opening_squiggly_bracket, end_id_opening_squiggly_bracket)    = enfa_copy(&mut combined_fragment.enfa, &mut fragment_opening_squiggly_bracket.enfa, fragment_opening_squiggly_bracket.end_id);
    let (start_id_closing_squiggly_bracket, end_id_closing_squiggly_bracket)    = enfa_copy(&mut combined_fragment.enfa, &mut fragment_closing_squiggly_bracket.enfa, fragment_closing_squiggly_bracket.end_id);
    let (start_id_opening_angular_bracket, end_id_opening_angular_bracket)      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_opening_angular_bracket.enfa, fragment_opening_angular_bracket.end_id);
    let (start_id_closing_angular_bracket, end_id_closing_angular_bracket)      = enfa_copy(&mut combined_fragment.enfa, &mut fragment_closing_angular_bracket.enfa, fragment_closing_angular_bracket.end_id);
    let (start_id_semicolon, end_id_semicolon)                                  = enfa_copy(&mut combined_fragment.enfa, &mut fragment_semicolon.enfa, fragment_semicolon.end_id);
    let (start_id_greater_than, end_id_greater_than)                            = enfa_copy(&mut combined_fragment.enfa, &mut fragment_greater_than.enfa, fragment_greater_than.end_id);
    let (start_id_less_than, end_id_less_than)                                  = enfa_copy(&mut combined_fragment.enfa, &mut fragment_less_than.enfa, fragment_less_than.end_id);
    let (start_id_plus, end_id_plus)                                            = enfa_copy(&mut combined_fragment.enfa, &mut fragment_plus.enfa, fragment_plus.end_id);
    let (start_id_minus, end_id_minus)                                          = enfa_copy(&mut combined_fragment.enfa, &mut fragment_minus.enfa, fragment_minus.end_id);

    // add epsilon transitions to all the individual keyword eNFAs
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_identifier);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_numeric);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_whitespace);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_return);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_if);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_void);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_int);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_opening_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_closing_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_opening_squiggly_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_closing_squiggly_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_opening_angular_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_closing_angular_bracket);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_semicolon);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_less_than);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_greater_than);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_plus);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_minus);

    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_2);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_3);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_4);
    // // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_5);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_6);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_7);

    // DEBUG - print to dot file format for debugging with https://dreampuf.github.io/GraphvizOnline
    //enfa_to_dot_directed_graph(&mut combined_fragment.enfa, "enfa_automaton.dot");

    //
    // Phase 3 - Convert eNFA to DFA
    //

    let mut dfa = enfa_to_dfa(&mut combined_fragment.enfa, &mut alphabet);

    // DEBUG - print to dot file format for debugging with https://dreampuf.github.io/GraphvizOnline
    //enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");

    //
    // Phase 4 - lex some input
    //

    let mut current_state_id = dfa.start_state_id;
    let mut last_state_id = dfa.start_state_id;

    // https://github.com/nlsandler/writing-a-c-compiler-tests/blob/main/tests/chapter_1/valid/return_0.c
    //let str = "void main() { return 100; }";
    //let str = "int main() { return 2; }";
    
    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_SQUIGGLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_SQUIGGLY_BRACKET
    //let str = "int main() { if (1 < 2) { return 2; } return 0; }";

    // INT IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET IF OPENING_BRACKET NUMERIC LT NUMERIC CLOSING_BRACKET OPENING_SQUIGGLY_BRACKET RETURN NUMERIC PLUS NUMERIC SEMICOLON CLOSING_SQUIGGLY_BRACKET RETURN NUMERIC SEMICOLON CLOSING_SQUIGGLY_BRACKET
    // let str = "int main() { if (1 < 2) { return 2 + 2; } return 0; }";

    // NUMERIC PLUS NUMERIC
    //let str = "2 + 2";

    // NUMERIC PLUS NUMERIC SEMICOLON
    //let str = "2 + 2;";

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

    // VOID VOID VOID VOID VOID
    //let str = "void void void void void";

    println!("Input: {}", str);
    
    let lexer_debug: bool = false;

    let mut token_string_buffer = String::from("");
    for character in str.chars() {

        let mut char_consumed = false;
        while !char_consumed {

            last_state_id = current_state_id;

            // println!("Input: {}", character);
            //token_string_buffer.push(character);

            // try to transition
            current_state_id = transition_dfa(&mut dfa, current_state_id, &RegexBuildingBlock::CharacterLiteral(character));

            if dfa.is_end_state(current_state_id) {

                // println!("STATE '{}' END STATE!", current_state_id);
                // println!("ACCEPTING '{}'! END STATE! Token-Id: {}", token_string_buffer, dfa.states[&current_state_id].token_id);

                token_string_buffer.push(character);

                char_consumed = true;

            } else if dfa.is_trap_state(current_state_id) {

                // reset the dfa to the start state and try to accept the symbol again
                char_consumed = false;
                current_state_id = dfa.start_state_id;

                if lexer_debug {
                    println!("[LEXER] Emitting '{}', Token-Id: {}, Token-Name: {}", token_string_buffer, dfa.states[&last_state_id].token_id, dfa.states[&last_state_id].token_name);
                    println!("");
                }

                // 15 is the token-id of whitespace, ignore whitespace!
                if dfa.states[&last_state_id].token_id != 15 {
                    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(dfa.states[&last_state_id].token_name.clone()));
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