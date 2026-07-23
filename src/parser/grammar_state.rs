use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;

use std::collections::HashMap;
use std::collections::BTreeMap;

use crate::parser::rule::Rule;
use crate::parser::rule::RuleElement;
use crate::Transition;

use crate::RULE_COUNTER;
use std::{
    sync::atomic::{AtomicUsize, Ordering}
};

#[derive(Clone)]
pub struct GrammarState<T: Debug> {
    pub id: usize,
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
        let debug = false;

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

        let debug = false;
        
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

            // if self.id == 22 && current_rule_id == 144 {
            //     println!("test");
            // }
            // if self.id == 22 && current_rule_id == 163 {
            //     println!("test");
            // }

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

                                if debug {
                                    println!("{:?} -{:?}-> {:?}", &current_rule.id, non_terminal.clone(), self.rules[contained_index].id);
                                }

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
                                if debug {
                                    println!("{:?} -{:?}-> {:?}", &current_rule.id, non_terminal.clone(), new_rule_id);
                                }

                                //
                                //
                            }
                        }
                    }
                }

                // if the dot points to a terminal, panic
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