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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleElement<T> {
    NonTerminal(T),
    Terminal(T),
    Epsilon,
    Dot,
    Unknown,
    AcceptingStateTransition,
    Closure,
    Unused
}

impl<T: Ord> Ord for RuleElement<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        
        match &self {
            RuleElement::<T>::NonTerminal(lhs_val) => {
                match &other {
                    RuleElement::<T>::NonTerminal(rhs_val) => {
                        lhs_val.cmp(rhs_val)
                    }
                    _ => {
                        panic!("test");
                    }
                }
            }
            _ => {
                panic!("test");
            }
        }
    }
}

impl<T: PartialOrd> PartialOrd for RuleElement<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        panic!("test");
    }
}

impl<T: Display> fmt::Debug for RuleElement<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        // LHS
        match &self {
            RuleElement::NonTerminal(str_val) => {
                write!(f, "{}", str_val).expect("Write failed!");
            }
            RuleElement::Terminal(str_val) => {
                write!(f, "{}", str_val).expect("Write failed!");
            }
            RuleElement::Epsilon => {
                write!(f, "ϵ").expect("Write failed!");
            }
            RuleElement::Dot => {
                write!(f, ".").expect("Write failed!");
            }
            RuleElement::AcceptingStateTransition => {
                write!(f, "$").expect("Write failed!");
            }
            RuleElement::Closure => {
                write!(f, "#").expect("Write failed!");
            }
            RuleElement::Unknown => {
                write!(f, "UNKNOWN").expect("Write failed!");
            }
            RuleElement::Unused => {
                // nop, do not display unused
            }
        }

        Ok(())
    }
}

#[derive(Clone, Eq, Hash)]
pub struct Rule<T> {
    id: usize,
    dot_idx: usize,
    lhs: RuleElement<T>,
    rhs: Vec::<RuleElement<T>>,
    lookahead: Vec::<RuleElement<T>>,
    channels: Vec::<usize>,
}

impl<T: Debug> Rule<T> {
    pub fn new(id: usize) -> Self {
        Rule {
            id: id,
            dot_idx: 0,
            lhs: RuleElement::<T>::Unknown,
            rhs: Vec::<RuleElement<T>>::new(),
            lookahead: Vec::<RuleElement<T>>::new(),
            channels: Vec::<usize>::new(),
        }
    }
}

impl<T: std::cmp::PartialEq> PartialEq<Rule<T>> for Rule<T> {

    // Rule equality is defined over 
    // - LHS 
    // - RHS, same amount, same order of elements
    // - dot marker, located at same index
    //
    // Not defined over id!
    fn eq(&self, other: &Rule<T>) -> bool {
        // https://stackoverflow.com/questions/29504514/whats-the-best-way-to-compare-2-vectors-or-strings-element-by-element

        // first zip to compare element by element, the result is the amount of matching elements
        let matching = self.rhs.iter().zip(&other.rhs).filter(|&(a, b)| a == b).count();

        // if lhs matches and all elements in rhs match and the dot idx is at the same spot, the rules are equal
        self.lhs == other.lhs && matching == self.rhs.len() && self.dot_idx == other.dot_idx
    }
}

impl<T: Display> fmt::Debug for Rule<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        // LHS
        write!(f, "{:?}", &self.lhs).expect("Write failed!");

        write!(f, " -> ").expect("Write failed!");

        let mut index: usize = 0;
        for symbol in &self.rhs {

            if index == self.dot_idx {
                write!(f, ".");
            }

            match &symbol {
                RuleElement::NonTerminal(str_val) => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "{}", str_val).expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Terminal(str_val) => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "{}", str_val).expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Epsilon => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "ϵ").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Dot => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, ".").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::AcceptingStateTransition => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "$").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Closure => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "#").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Unknown => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "UNKNOWN").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Unused => {
                    // nop, do not display unused
                }
            }
        }

        if index == self.dot_idx {
            write!(f, ".");
        }

        // lookahead
        write!(f, "     ");
        for symbol in &self.lookahead {
            write!(f, " / {:?}", &symbol).expect("Write failed!");
        }

        // channels
        if self.channels.len() > 0 {
            write!(f, "    channels: {:?}", &self.channels).expect("Write failed!");
        }

        Ok(())
    }
}

impl<T: Display> fmt::Display for Rule<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        // LHS
        write!(f, "{:?}", &self.lhs).expect("Write failed!");

        write!(f, " -> ").expect("Write failed!");

        let mut index: usize = 0;
        for symbol in &self.rhs {

            if index == self.dot_idx {
                write!(f, ".");
            }

            match &symbol {
                RuleElement::NonTerminal(str_val) => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "{}", str_val).expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Terminal(str_val) => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "{}", str_val).expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Epsilon => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "ϵ").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Dot => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, ".").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::AcceptingStateTransition => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "$").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Closure => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "#").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Unknown => {
                    if index > 0 {
                        write!(f, " ");
                    }
                    write!(f, "UNKNOWN").expect("Write failed!");
                    index = index + 1;
                }
                RuleElement::Unused => {
                    // nop, do not display unused
                }
            }
        }

        if index == self.dot_idx {
            write!(f, ".");
        }

        // lookahead
        write!(f, "     ");
        for symbol in &self.lookahead {
            write!(f, " / {:?}", &symbol).expect("Write failed!");
        }

        // channels
        if self.channels.len() > 0 {
            write!(f, "    channels: {:?}", &self.channels).expect("Write failed!");
        }

        Ok(())
    }
}

pub struct Transition<T>(usize, RuleElement<T>);

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
    pub fn unfold_grammar_state(&mut self, grammar_rules: &Vec::<Rule<T>>,
        first: &BTreeMap<RuleElement::<T>, Vec::<RuleElement::<T>>>,
        nullable: &BTreeMap::<RuleElement::<T>, bool>,
        rule_channel_map: &mut HashMap::<usize, Vec::<Transition<T>>>,
    ) {

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

        // scratchpad of rules to process
        let mut d_set = Vec::<Rule<T>>::new();
        d_set.append(&mut self.identification_rules.clone());

        // while scratchpad has rules on it, loop
        let mut done: bool = d_set.is_empty();
        while !done {

            let mut current_rule: Rule<T> = d_set.pop().expect("Need at least one rule!");

            // DEBUG
            println!("current_rule: {}", current_rule);

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

            // // DEBUG
            // println!("Determining lookahead for Rule: {}. Rule has lookahead: {:?}", current_rule, current_rule.lookahead);

            // find beta, if there is no beta, lookahead is the rule's own lookahead
            if current_rule.dot_idx + 1 >= current_rule.rhs.len() {

                // // empty beta
                // println!("empty beta");

                current_lookahead.append(&mut current_rule.lookahead);

            } else {

                // build FIRST(beta+rule.lookahead)

                // // DEBUG
                // println!("found beta");

                println!("Current Rule: {:?}", current_rule);

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
                                    current_lookahead.append(&mut first_values.clone());
                                }
                                None => {
                                    panic!("Compiler has no FIRST() information for NonTerminal: {:?}! Aborting!", current_rule.rhs[beta_idx]);
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
                        }

                        _ => { 
                            panic!("test");
                        }
                    }
                }
            }

            // // DEBUG
            // println!("current lookahead: {:?}", current_lookahead);

            // over all rules that unfold from the rule (Closure)
            match &current_rule.rhs[current_rule.dot_idx] {

                // if the dot is points to a non-terminal, extend the rule set
                RuleElement::NonTerminal(non_terminal) => {

                    // // DEBUG
                    // println!("Will extend closure due to Rule: {} and NonTerminal {} with lookaheads {:?}", current_rule, non_terminal, current_lookahead);

                    // DEBUG
                    // println!("non_terminal {}", non_terminal);
                    
                    // find all rules that have a LHS == the non-terminal and add them into the d_set
                    for i in 0..grammar_rules.len() {

                        // if this rule starts with the same nonterminal
                        if grammar_rules[i].lhs == RuleElement::<T>::NonTerminal(non_terminal.clone()) {

                            // // DEBUG
                            // println!("Inserting into closure Rule: [{}] {} using lookaheads: {:?} because of source-rule-id: {}", grammar_rules[i].id, grammar_rules[i], current_lookahead, &current_rule.id);

                            let mut contained_already = false;
                            for j in 0..self.rules.len() {

                                if self.rules[j] == grammar_rules[i] {

                                    // copy all lookahead symbols over!
                                    for la in &current_lookahead {

                                        if !self.rules[j].lookahead.contains(&la) {

                                            // DEBUG
                                            println!("Inserting {:?} into rule {:?}", &la, &self.rules[j]);

                                            //
                                            // Insert into rule_channel_map

                                            if !rule_channel_map.contains_key(&current_rule.id) {
                                                let channel_ends = Vec::<Transition<T>>::new();
                                                rule_channel_map.insert(current_rule.id, channel_ends);
                                            }
                                            // retrieve the vector of first symbols for the nonterminal and extend it
                                            let channel_ends = &mut rule_channel_map.get_mut(&current_rule.id).unwrap();

                                            // TODO (usize, RuleElement)
                                            // channel_ends.push(self.rules[j].id);
                                            //channel_ends.push(Transition(self.rules[j].id, RuleElement::NonTerminal(String::from("a"))));
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
                            println!("Inserting into closure Rule: [{}] {} using lookaheads: {:?} because of source-rule-id: {}", rule.id, rule, current_lookahead, &current_rule.id);

                            //
                            // Insert into rule_channel_map

                            if !rule_channel_map.contains_key(&current_rule.id) {
                                let channel_ends = Vec::<Transition<T>>::new();
                                rule_channel_map.insert(current_rule.id, channel_ends);
                            }
                            // retrieve the vector of first symbols for the nonterminal and extend it
                            let channel_ends = &mut rule_channel_map.get_mut(&current_rule.id).unwrap();

                            // TODO (usize, RuleElement)
                            //channel_ends.push(rule.id);
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

    // DEBUG
    rule.dot_idx = std::usize::MAX;
    println!("{:?}", rule);
    rule.dot_idx = 0;

    grammar_rules.push(rule);
}

// https://stackoverflow.com/questions/32935808/generate-sequential-ids-for-each-instance-of-a-struct
static RULE_COUNTER: AtomicUsize = AtomicUsize::new(0);
static STATE_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ParseTableCell<T> {

    // ACTION-Part
    Shift(T),
    Reduce(T),
    Accept,

    // GOTO-Part
    Goto(T),
}

#[derive(Clone, Debug)]
pub enum ParseStackElement<T: std::fmt::Display> {
    RuleElement(RuleElement<T>),
    StateId(usize),
}

pub struct Parser<T> {
    // pub current_state_id: usize,
    pub parse_table: HashMap::<usize, HashMap::<RuleElement<T>, ParseTableCell<usize>>>,
    // pub stack: Vec::<RuleElement<T>>,
    // pub state_stack: Vec::<usize>,
    pub stack: Vec::<ParseStackElement<String>>,
}

//impl<T: Clone + Debug + Display + std::cmp::PartialEq + Ord> Parser<String> {
impl Parser<String> {

    pub fn new(parse_table_param: HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>) -> Self {

        let mut p = Parser {
            // parse_table: HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>::new(),
            parse_table: parse_table_param,
            //stack: Vec::<RuleElement<String>>::new(),
            //state_stack: Vec::<usize>::new(),
            stack: Vec::<ParseStackElement<String>>::new(),
        };

        let pse = ParseStackElement::<String>::StateId(0);
        // pse.state_id = 0;
        p.stack.push(pse);

        p
    }

    //pub fn consume(&mut self, input: RuleElement<String>, grammar_rules: &Vec::<Rule<String>>) -> bool {
    // let mut grammar_state_hashmap = BTreeMap::new();
    pub fn consume(&mut self, input: RuleElement<String>, grammar_state_hashmap: &BTreeMap<usize, GrammarState<String>>) -> bool {

        println!("stack {:?}", self.stack);

        let parse_stack_element = self.stack.pop().unwrap();
        self.stack.push(parse_stack_element.clone());

        match &parse_stack_element {

            ParseStackElement::RuleElement(rule_element) => {
                println!("RuleElement {:?}", rule_element);

                let stack_len = self.stack.len();
                println!("stack_len {:?}", stack_len);

                let stack_content = &self.stack[stack_len - 2];

                match &stack_content {

                    ParseStackElement::StateId(current_state_id) => {
                        println!("StateId: {}", current_state_id);

                        let parse_table_row = self.parse_table.get(&current_state_id).unwrap();

                        // DEBUG
                        // let contains_key: bool = parse_table_row.contains_key(&rule_element);
                        // println!("State: {}, parse_table_row: {:?}, input: {:?}, contains_key: {:?}", current_state_id, parse_table_row, input, contains_key);

                        let idk = parse_table_row.get(&rule_element).unwrap();
                        match idk {

                            ParseTableCell::Goto(next_state_id) => {
                                println!("GOTO: {:?}", *next_state_id);
                                let pse = ParseStackElement::<String>::StateId(*next_state_id);
                                self.stack.push(pse);
                            }

                            _ => {
                                panic!("test");
                            }
                        }
                    }

                    ParseStackElement::RuleElement(rule_element) => {
                        panic!("RuleElement");
                    }
                }

                false
            }

            ParseStackElement::StateId(current_state_id) => {
                println!("StateId: {}", current_state_id);

                let parse_table_row = self.parse_table.get(&current_state_id).unwrap();

                // DEBUG
                let contains_key: bool = parse_table_row.contains_key(&input);
                println!("State: {}, parse_table_row: {:?}, input: {:?}, contains_key: {:?}", current_state_id, parse_table_row, input, contains_key);

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

                    let t = self.stack.pop().unwrap();
                    self.stack.push(t.clone());

                    //println!("Test: {:?}", t);

                    match &t {

                        ParseStackElement::StateId(current_state_id) => {
                            panic!("test");
                        }

                        ParseStackElement::RuleElement(rule_element) => {
                            println!("RuleElement");

                            let idk = parse_table_row.get(&rule_element).unwrap();

                            match idk {

                                ParseTableCell::Goto(state_id) => {
                                    println!("GOTO: {:?}", *state_id);
                                    let pse = ParseStackElement::<String>::StateId(*state_id);
                                    self.stack.push(pse);

                                    false
                                }

                                ParseTableCell::Shift(state_id) => {
                                    println!("SHIFT: {:?}", *state_id);
                                    let pse = ParseStackElement::<String>::StateId(*state_id);
                                    self.stack.push(pse);

                                    false
                                }

                                _ => {
                                    panic!("test {:?}", idk);
                                }
                            }

                            

                        }
                    }

                } else {

                    
                    let parser_step = parse_table_row.get(&input).expect("Parse Table is broken!");
                    match parser_step {

                        ParseTableCell::Shift(next_state_id) => {
                            println!("shift {}", next_state_id);
                            self.stack.push(ParseStackElement::<String>::RuleElement(input));
                            self.stack.push(ParseStackElement::<String>::StateId(*next_state_id));

                            true
                        }

                        ParseTableCell::Reduce(rule_id) => {
                            println!("reduce: {}", rule_id);

                            let state = grammar_state_hashmap.get(&current_state_id).unwrap();

                            println!("reduce: {:?}, rule_id: {:?}", state, rule_id);

                            // TODO: only using rules (and not identification_rules) right now
                            //let rule = state.unwrap().rules.into_iter().filter(|r| r.id == *rule_id).collect::<Vec<_>>().first();

                            let mut found_rule = Rule::<String>::new(0);

                            let mut found = false;
                            for i in 0..state.identification_rules.len() {
                                if state.identification_rules[i].id == *rule_id {
                                    println!("rule: {:?}", state.identification_rules[i]);

                                    found_rule = state.identification_rules[i].clone();

                                    found = true;
                                }
                            }

                            if !found {
                                for i in 0..state.rules.len() {
                                    if state.rules[i].id == *rule_id {
                                        println!("rule: {:?}", state.rules[i]);

                                        found_rule = state.rules[i].clone();

                                        found = true;
                                    }
                                }
                            }

                            if !found {
                                panic!("Rule not found!");
                            } else {
                                println!("rule: {:?}", found_rule);

                                for rhs in found_rule.rhs {
                                    self.stack.pop();
                                    self.stack.pop();
                                }

                                self.stack.push(ParseStackElement::<String>::RuleElement(found_rule.lhs));
                            }

                            false
                        }

                        ParseTableCell::Accept => {
                            println!("ACCEPT !!!!");

                            true
                        }

                        _ => {
                            panic!("NIY!");
                        }

                    }
                }
            }
        }

/*
        
             */
    }
}

fn main() {

    println!("start");

    println!("");
    println!("All Rules:");

    let mut grammar_rules = Vec::<Rule<String>>::new();

    // <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

/*
    // https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html
    // https://jsmachines.sourceforge.net/machines/lalr1.html

    // DragonBook 2nd Edition, page 255, Example 4.48. Figure 4.39
    // Reproduced on page 271
    // https://cyberzhg.github.io/toolbox/lr0

    // S' -> S
    // S -> L = R | R
    // L -> *R | id
    // R -> L

    // S' -> S
    // S -> L = R
    // S -> R
    // L -> * R 
    // L -> id
    // R -> L

    // VALID-INPUT
    // * id = id

    // has to be the start symbol of the non-augmented (= original) grammar!
    let start_symbol = RuleElement::NonTerminal(String::from("S"));

    let treat_nonterminal_lowercase = false;
    create_rule(&mut grammar_rules, String::from("S' -> S"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> L = R"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> R"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("L -> * R"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("L -> id"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("R -> L"), treat_nonterminal_lowercase);

    // the first rule per definition has the closure symbol as a spontaneous symbol
    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);
*/






/*
    // https://softwareengineering.stackexchange.com/questions/177872/how-lookaheads-are-propagated-in-channel-method-of-building-lalr-parser

    // S -> E
    // E -> E - T
    // E -> T
    // T -> ( E )
    // T -> n

    // S' -> S
    // S -> E
    // E -> E - T
    // E -> T
    // T -> ( E )
    // T -> n

    // VALID-INPUT
    // n - ( n )

    // This must be the start symbol of the original grammar
    let start_symbol = RuleElement::NonTerminal(String::from("S"));

    let treat_nonterminal_lowercase = false;
    create_rule(&mut grammar_rules, String::from("S' -> S"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> E"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("E -> E - T"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("E -> T"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("T -> n"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("T -> ( E )"), treat_nonterminal_lowercase);

    // the first rule per definition has the closure symbol as a spontaneous symbol
    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);
*/






/*
    // S' -> S
    // S -> A A
    // A -> a A
    // A -> b

    // Valid input: a b a b

    // this must be the start symbol of the original grammar
    let start_symbol = RuleElement::NonTerminal(String::from("S"));

    let treat_nonterminal_lowercase = false;
    create_rule(&mut grammar_rules, String::from("S' -> S"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> A A"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("A -> a A"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("A -> b"), treat_nonterminal_lowercase);

    // the first rule per definition has the closure symbol as a spontaneous symbol
    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);

    // let terminal_a = RuleElement::<String>::Terminal(String::from("a"));
    // let terminal_b = RuleElement::<String>::Terminal(String::from("b"));

    // let mut first_A = Vec::<RuleElement::<String>>::new();
    // first_A.push(terminal_a);
    // first_A.push(terminal_b);

    // let non_terminal_A = RuleElement::<String>::NonTerminal(String::from("A"));

    // let mut first = BTreeMap::new();
    // first.insert(non_terminal_A.clone(), first_A);
*/





/*
    // https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html#eyJncmFtbWFyIjoiUyAtPiBhIEEgY1xuUyAtPiBhIEIgZFxuUyAtPiBCIGNcbkEgLT4gelxuQiAtPiB6IiwiaW5wdXQiOiIifQ==

    // S' -> S
    // S -> a A c
    // S -> a B d
    // S -> B c
    // A -> z
    // B -> z

    // VALID-INPUT
    // a z d #

    // this must be the start symbol of the original grammar
    let start_symbol = RuleElement::NonTerminal(String::from("S"));

    let treat_nonterminal_lowercase = false;
    create_rule(&mut grammar_rules, String::from("S' -> S"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> a A c"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> a B d"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> B c"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("A -> z"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("B -> z"), treat_nonterminal_lowercase);

    // the first rule per definition has the closure symbol as a spontaneous symbol
    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);
*/
    


    

/*
    //
    // This is an example grammar for a grammar that needs lookahead propagation iteration!
    //
    // https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html (do not add augmented start rule)
    // https://jsmachines.sourceforge.net/machines/lalr1.html (add augmented start rule)
    //
 
    // https://stackoverflow.com/questions/77577494/is-this-grammar-lalr1

    // VALID-INPUT: a c b #

    // this must be the start symbol of the original grammar
    let start_symbol = RuleElement::NonTerminal(String::from("S"));

    let treat_nonterminal_lowercase = false;
    create_rule(&mut grammar_rules, String::from("S' -> S"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> a S b"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> c"), treat_nonterminal_lowercase);

    // the first rule per definition has the closure symbol as a spontaneous symbol
    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);
*/





/*
    // https://stackoverflow.com/questions/77577494/is-this-grammar-lalr1
    // https://en.wikipedia.org/wiki/Dangling_else

    // https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html
    // https://jsmachines.sourceforge.net/machines/lalr1.html

    // statement' -> statement
    // statement -> open_statement
    // statement -> closed_statement
    // open_statement -> IF ( expression ) statement
    // open_statement -> IF ( expression ) closed_statement ELSE open_statement
    // closed_statement -> non_if_statement
    // closed_statement -> IF ( expression ) closed_statement ELSE closed_statement
    // non_if_statement -> TEST_STMT

    // Valid Test input:
    // TEST_STMT
    // IF ( EXPRESSION ) TEST_STMT
    // IF ( EXPRESSION ) TEST_STMT ELSE TEST_STMT
    // IF ( EXPRESSION ) IF ( EXPRESSION ) TEST_STMT ELSE TEST_STMT
    // IF ( EXPRESSION ) TEST_STMT ELSE IF ( EXPRESSION ) TEST_STMT
    // IF ( EXPRESSION ) TEST_STMT ELSE IF ( EXPRESSION ) TEST_STMT ELSE TEST_STMT

    // Invalid Input:
    // IF ( EXPRESSION ) TEST_STMT TEST_STMT

    // this must be the start symbol of the original grammar
    let start_symbol = RuleElement::NonTerminal(String::from("statement"));

    let treat_nonterminal_lowercase = true;
    create_rule(&mut grammar_rules, String::from("statement' -> statement"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("statement -> open_statement"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("statement -> closed_statement"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("open_statement -> IF ( EXPRESSION ) statement"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("open_statement -> IF ( EXPRESSION ) closed_statement ELSE open_statement"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("closed_statement -> non_if_statement"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("closed_statement -> IF ( EXPRESSION ) closed_statement ELSE closed_statement"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("non_if_statement -> TEST_STMT"), treat_nonterminal_lowercase);

    // the first rule per definition has the closure symbol as a spontaneous symbol
    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);

    // let terminal_a = RuleElement::<String>::Terminal(String::from("a"));
    // let terminal_b = RuleElement::<String>::Terminal(String::from("b"));

    // let mut first_A = Vec::<RuleElement::<String>>::new();
    // first_A.push(terminal_a);
    // first_A.push(terminal_b);

    // let non_terminal_A = RuleElement::<String>::NonTerminal(String::from("A"));

    //let first = BTreeMap::new();
    // first.insert(non_terminal_A.clone(), first_A);
*/




/*
    // https://en.wikipedia.org/wiki/Dangling_else

    // https://cyberzhg.github.io/toolbox/lr0
    // https://jsmachines.sourceforge.net/machines/lalr1.html

    // statement' -> statement
    // statement -> selection-statement
    // statement -> TEST_STMT
    // selection-statement -> IF ( EXPRESSION ) statement
    // selection-statement -> IF ( EXPRESSION ) statement ELSE statement
*/





/*
    // https://www.geeksforgeeks.org/compiler-design/first-set-in-syntax-analysis/

    // Production Rules of Grammar
    // S -> ACB | Cbb | Ba
    // A -> da | BC
    // B -> g | ε
    // C -> h | ε

    // FIRST sets
    // FIRST(S) = FIRST(ACB) U FIRST(Cbb) U FIRST(Ba) = { d, g, h, b, a,  ε}
    // FIRST(A) = { d } U FIRST(BC) = { d, g, h,  ε }
    // FIRST(B) = { g ,  ε }
    // FIRST(C) = { h ,  ε }

    // https://cyberzhg.github.io/toolbox/lr0
    // https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html (Do not add a rule S' -> S into the webapp)
    // https://jsmachines.sourceforge.net/machines/lalr1.html (add a rule S' -> S into the webapp)

    // VALID-INPUT:
    // d a h g #

    // this has to be the start symbol of the original unaugmented grammar
    let start_symbol = RuleElement::NonTerminal(String::from("S"));

    let treat_nonterminal_lowercase: bool = false;

    // add augmentation start rule
    create_rule(&mut grammar_rules, String::from("S' -> S"), treat_nonterminal_lowercase);

    // S -> ACB | Cbb | Ba
    create_rule(&mut grammar_rules, String::from("S -> A C B"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> C b b"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> B a"), treat_nonterminal_lowercase);

    // A -> da | BC
    create_rule(&mut grammar_rules, String::from("A -> d a"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("A -> B C"), treat_nonterminal_lowercase);

    // B -> g | ε
    create_rule(&mut grammar_rules, String::from("B -> g"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("B -> $$_EPSILON_$$"), treat_nonterminal_lowercase);

    // C -> h | ε
    create_rule(&mut grammar_rules, String::from("C -> h"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("C -> $$_EPSILON_$$"), treat_nonterminal_lowercase);

    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);
*/



    
/**/
    // https://cyberzhg.github.io/toolbox/lr0
    // https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html (Do not add augmented start rule!)
    // https://jsmachines.sourceforge.net/machines/lalr1.html (Add augmented start rule!)

    // https://www.lysator.liu.se/c/ANSI-C-grammar-y.html

    // VALID INPUT:
    
    // void main () {}
    // VOID IDENTIFIER OPENING_BRACES CLOSING_BRACES OPENING_CURLY_BRACES CLOSING_CURLY_BRACES

    // void main ( EXPRESSION_STOP SEMICOLON )
    // VOID IDENTIFIER OPENING_BRACES CLOSING_BRACES OPENING_CURLY_BRACES EXPRESSION_STOP SEMICOLON CLOSING_CURLY_BRACES

    // this has to be the start symbol of the original unaugmented grammar
    let start_symbol = RuleElement::NonTerminal(String::from("translation_unit"));

    let treat_nonterminal_lowercase: bool = true;

    // add augmentation start rule
    create_rule(&mut grammar_rules, String::from("translation_unit' -> translation_unit"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("translation_unit -> function_definition"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("function_definition -> declaration_specifiers declarator compound_statement"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("declaration_specifiers -> type_specifier declaration_specifiers"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("declaration_specifiers -> type_specifier"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("declaration_specifiers -> type_qualifier declaration_specifiers"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("declaration_specifiers -> type_qualifier"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("type_specifier -> VOID"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("type_specifier -> INT"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("type_qualifier -> CONST"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("type_qualifier -> VOLATILE"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("declarator -> direct_declarator"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("direct_declarator -> IDENTIFIER direct_declarator"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("direct_declarator -> IDENTIFIER"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("direct_declarator -> OPENING_BRACES declarator CLOSING_BRACES"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("direct_declarator -> OPENING_BRACES CLOSING_BRACES"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("compound_statement -> OPENING_CURLY_BRACES declaration_or_statement_list CLOSING_CURLY_BRACES"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("compound_statement -> OPENING_CURLY_BRACES CLOSING_CURLY_BRACES"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("declaration_or_statement_list -> declaration declaration_or_statement_list"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("declaration_or_statement_list -> declaration"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("declaration_or_statement_list -> statement declaration_or_statement_list"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("declaration_or_statement_list -> statement"), treat_nonterminal_lowercase);

    // DEBUG
    create_rule(&mut grammar_rules, String::from("declaration -> DECLARATION_STOP"), treat_nonterminal_lowercase);
    // create_rule(&mut grammar_rules, String::from("statement -> STATEMENT_STOP"), treat_nonterminal_lowercase);
/*
    create_rule(&mut grammar_rules, String::from("declaration -> declaration_specifiers init_declarator_list SEMICOLON"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("declaration -> declaration_specifiers SEMICOLON"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("init_declarator_list -> init_declarator COMMA init_declarator_list"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("init_declarator_list -> init_declarator"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("init_declarator -> declarator EQUALS_SIGN initializer"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("init_declarator -> declarator"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("initializer -> assignment_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("initializer -> OPENING_CURLY_BRACES initializer_list CLOSING_CURLY_BRACES"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("initializer -> OPENING_CURLY_BRACES initializer_list COMMA CLOSING_CURLY_BRACES"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("initializer_list -> initializer"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("initializer_list -> initializer COMMA initializer_list"), treat_nonterminal_lowercase);
*/
    create_rule(&mut grammar_rules, String::from("statement -> expression_statement"), treat_nonterminal_lowercase);
    // DEBUG
    //create_rule(&mut grammar_rules, String::from("statement -> STOP_1"), treat_nonterminal_lowercase);
    
    create_rule(&mut grammar_rules, String::from("expression_statement -> expression SEMICOLON"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("expression_statement -> SEMICOLON"), treat_nonterminal_lowercase);

    // ORIG 
    create_rule(&mut grammar_rules, String::from("expression -> assignment_expression"), treat_nonterminal_lowercase);
    // DEBUG
    //create_rule(&mut grammar_rules, String::from("expression -> EXPRESSION_STOP"), treat_nonterminal_lowercase);

    // ORIG - this rule causes deep-dive with loop
    create_rule(&mut grammar_rules, String::from("assignment_expression -> unary_expression assignment_operator assignment_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("assignment_expression -> conditional_expression"), treat_nonterminal_lowercase);
    // 
    //DEBUG  
    //
    //create_rule(&mut grammar_rules, String::from("assignment_expression -> ASSIGN_STOP"), treat_nonterminal_lowercase);
/**/
    create_rule(&mut grammar_rules, String::from("unary_expression -> postfix_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_expression -> INC_OP unary_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_expression -> DEC_OP unary_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_expression -> unary_operator cast_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_expression -> SIZEOF unary_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_expression -> SIZEOF OPENING_BRACES type_name CLOSING_BRACES"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("unary_operator -> AMPERSAND"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_operator -> ASTERISK"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_operator -> PLUS"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_operator -> MINUS"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_operator -> TILDE"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_operator -> EXCLAMATION_MARK"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("type_name -> specifier_qualifier_list"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("specifier_qualifier_list -> type_specifier"), treat_nonterminal_lowercase);
    // ORIG
    create_rule(&mut grammar_rules, String::from("cast_expression -> OPENING_BRACES type_name CLOSING_BRACES cast_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("cast_expression -> unary_expression"), treat_nonterminal_lowercase);
    // // DEBUG
    // create_rule(&mut grammar_rules, String::from("cast_expression -> CAST_STOP"), treat_nonterminal_lowercase);

    // DEBUG
    //create_rule(&mut grammar_rules, String::from("postfix_expression -> END_POSTFIX"), treat_nonterminal_lowercase);
/**/
    create_rule(&mut grammar_rules, String::from("postfix_expression -> primary_expression postfix_expression_list"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("postfix_expression -> primary_expression"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("postfix_expression_list -> INC_OP postfix_expression_list"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("postfix_expression_list -> INC_OP"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("primary_expression -> IDENTIFIER"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("primary_expression -> HEX_NUMERIC"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("primary_expression -> NUMERIC"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("assignment_operator -> EQUALS_SIGN"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("assignment_operator -> MUL_ASSIGN"), treat_nonterminal_lowercase);

    // -----------------------------------------------------------

    create_rule(&mut grammar_rules, String::from("conditional_expression -> logical_or_expression QUESTION_MARK expression COLON conditional_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("conditional_expression -> logical_or_expression"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("logical_or_expression -> logical_and_expression OR_OP logical_or_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("logical_or_expression -> logical_and_expression"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("logical_and_expression -> inclusive_or_expression AND_OP logical_and_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("logical_and_expression -> inclusive_or_expression"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("inclusive_or_expression -> exclusive_or_expression BIN_OR_OP inclusive_or_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("inclusive_or_expression -> exclusive_or_expression"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("exclusive_or_expression -> and_expression CIRCUMFLEX exclusive_or_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("exclusive_or_expression -> and_expression"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("and_expression -> equality_expression AMPERSAND and_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("and_expression -> equality_expression"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("equality_expression -> relational_expression EQ_OP equality_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("equality_expression -> relational_expression NE_OP equality_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("equality_expression -> relational_expression"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("relational_expression -> shift_expression LT relational_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("relational_expression -> shift_expression GT relational_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("relational_expression -> shift_expression LTE relational_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("relational_expression -> shift_expression GTE relational_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("relational_expression -> shift_expression"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("shift_expression -> additive_expression LEFT_OP shift_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("shift_expression -> additive_expression RIGHT_OP shift_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("shift_expression -> additive_expression"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("additive_expression -> multiplicative_expression PLUS additive_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("additive_expression -> multiplicative_expression MINUS additive_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("additive_expression -> multiplicative_expression"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("multiplicative_expression -> cast_expression ASTERISK multiplicative_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("multiplicative_expression -> cast_expression SLASH multiplicative_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("multiplicative_expression -> cast_expression PERCENT multiplicative_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("multiplicative_expression -> cast_expression"), treat_nonterminal_lowercase);
/**/

    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);









    //
    // Validating the Grammar
    //

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



    //
    // Nullable
    //

    println!("");
    println!("Nullable start ...");

    // nullable (can produce the empty string, EPSILON)
    //
    // In Context-Free Grammars (CFG), a nonterminal that can derive the empty string \(\epsilon \) 
    // is called nullable. You can find all nullable nonterminals by using a simple iterative marking 
    // algorithm (similar to the standard method taught in computer science).
    let mut nullable = BTreeMap::<RuleElement::<String>, bool>::new();

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




    //
    // First Set
    //

    // How to compute the first map:
    //
    // Rule 1: For a Terminal symbol a: The set FIRST(a) is simply the terminal itself: FIRST(a) = {a}
    // Rule 2: For an Epsilon (empty string): If a production derives Epsilon (the empty string) then 
    //         Epsilon is included in the set: FIRST(Epsilon) = { Epsilon }.
    // Rule 3: For a Non-terminal X: The set FIRST(X) is the union of the FIRST(X) sets of the right-hand 
    //         side of all its production rules.
    //         E.g. S -> ACB | Cbb | Ba then FIRST(S) = FIRST(ACB) U FIRST(Cbb) U FIRST(Ba)
    //         HINT: This rule is not relevant if the | rules have been split up into one production per rule.
    // Rule 4: For a sequence of symbols Y_1 Y_2 Y_3...: You start by adding \(\text{FIRST}(Y_1)\). 
    //         If Y_1 can derive Epsilon, you also add the FIRST(Y_2) (excluding (Epsilon)), 
    //         and so on, until you reach a symbol that does not derive Epsilon.
    //         If all symbols can derive epsilon, then add epsilon.

/**/
    let mut first = BTreeMap::<RuleElement::<String>, Vec::<RuleElement::<String>>>::new();

    let mut change_detected = true;
    while change_detected {

        // println!("Change detected == false");
        change_detected = false;

        for rule in grammar_rules.iter() {

            println!("{:?}", rule);

            for r in rule.rhs.iter() {

                match &r {

                    RuleElement::Terminal(t) => {
                        //println!("Terminal in first position found: {:?}", t);

                        // if rule's lhs is not part of the map yet, insert a vector
                        if !first.contains_key(&rule.lhs) {
                            // println!("Not contained yet!");

                            let first_terminals = Vec::<RuleElement::<String>>::new();
                            first.insert(rule.lhs.clone(), first_terminals);
                        }

                        // retrieve the vector of first symbols for the nonterminal and extend it
                        let first_terminals = &mut first.get_mut(&rule.lhs).unwrap();

                        // add the rhs[0] into the first vector
                        if !first_terminals.contains(&r) {

                            // println!("(1) Adding {:?} into First({:?})", r.clone(), &rule.lhs);

                            first_terminals.push(r.clone());

                            // println!("Change detected == true");
                            change_detected = true;
                        }

                        // stop iterating over RHS once the first terminal is found since
                        // after the first terminal, no other terminal can be in the first set!
                        break;
                    }

                    RuleElement::NonTerminal(nt) => {

                        // add first set for the non-terminal to the first set.
                        // If the non-terminal is nullable, proceed with the next non terminal.

                        // if rule's lhs is not part of the map yet, 
                        if !first.contains_key(&r) {

                            // wait for information about the NT appear due to other iterations
                            change_detected = true;
                            break;

                        } else {

                            // retrieve the vector of first symbols for the RHS nonterminal
                            let first_terminals_rhs = first.get(&r).unwrap().clone();

                            // if rule's lhs is not part of the map yet, insert a vector
                            if !first.contains_key(&rule.lhs) {
                                let first_terminals = Vec::<RuleElement::<String>>::new();
                                first.insert(rule.lhs.clone(), first_terminals);
                            }

                            // retrieve the vector of first symbols for the LHS nonterminal and extend it
                            let first_terminals_lhs = &mut first.get_mut(&rule.lhs).unwrap();

                            // println!("(2) Adding {:?} into First({:?})", first_terminals.clone(), &rule.lhs);

                            // add the rhs[0] into the first vector
                            for ch in first_terminals_rhs {
                                if !first_terminals_lhs.contains(&ch) {
                                    first_terminals_lhs.push(ch);

                                    change_detected = true;
                                }
                            }

                            // If the non-terminal is nullable, proceed with the next symbol, 
                            // otherwise abort if not nullable
                            if *nullable.get(&r).unwrap() == false {
                                break;
                            }
                        }
                    }

                    _ => {
                        
                    }
                }
            }
        }
    }

    // // DEBUG output FIRST()
    // println!("");
    // println!("FIRST() *****************************");
    // for (key, value) in first.clone().into_iter() {
    //     println!("{:?} / {:?}", key, value);
    // }
    // println!("*******************************************");

    // Production Rules of Grammar
    // S -> ACB | Cbb | Ba
    // A -> da | BC
    // B -> g | ε
    // C -> h | ε

    // FIRST sets
    // FIRST(S) = FIRST(ACB) U FIRST(Cbb) U FIRST(Ba) = { d, g, h, b, a,  ε}
    // FIRST(A) = { d } U FIRST(BC) = { d, g, h,  ε }
    // FIRST(B) = { g ,  ε }
    // FIRST(C) = { h ,  ε }

    // println!("Test");

    // let first = BTreeMap::new();

    // let terminal_a = RuleElement::<String>::Terminal(String::from("a"));
    // let terminal_b = RuleElement::<String>::Terminal(String::from("b"));

    // let mut first_A = Vec::<RuleElement::<String>>::new();
    // first_A.push(terminal_a);
    // first_A.push(terminal_b);

    // let non_terminal_A = RuleElement::<String>::NonTerminal(String::from("A"));

    // let first = BTreeMap::new();
    // first.insert(non_terminal_A.clone(), first_A);




    // >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

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

            // DEBUG
            println!("\n");
            println!("----------------------------------------");
            println!("{:?}", grammar_state);
            println!("========================================");
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

            // check for the end state.
            // The end state has the dot marker after the start symbol
            if consumed_rules.len() == 1 && let Some(last_symbol) = consumed_rules[0].rhs.last() {

                if *last_symbol == start_symbol {

                    if found_final_state {
                        eprintln!("DFA cannot have two end states! First final state: {}, This state: {}", final_state_id, current_grammar_state_id);
                    }

                    found_final_state = true;
                    final_state_id = current_grammar_state_id;

                    // TODO output transition
                    println!("STATE-TRANSITION-ENDSTATE: {:?} -{:?}-> {:?}", &current_grammar_state_id, RuleElement::<String>::AcceptingStateTransition, std::usize::MAX);
                }
            }

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

            // this is an example for the global map
            // TODO: we do not want this huge for loop over all global states! This is trash! 
            // Needs to be constant time and not O(n)
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
                                println!("RULE-RULE-CHANNEL: {} -> {}", src_rule_id[iter_index], rrule.id);

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

                                // TODO (usize, RuleElement)
                                // channel_ends.push(rrule.id);
                                //channel_ends.push(Transition(rrule.id, RuleElement::<String>::Unknown));
                                //channel_ends.push(Transition(rrule.id, RuleElement::<String>::Terminal(String::from("IMA"))));
                                
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
                    println!("STATE-TRANSITION-EXISTING-SELF: {:?} -{:?}-> {:?}", &current_grammar_state_id, &current_symbol, &state_id);
                } else {
                    // TODO output transition (to already existing state)
                    println!("STATE-TRANSITION-EXISTING: {:?} -{:?}-> {:?}", &current_grammar_state_id, &current_symbol, &state_id);
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

                    // println!("RULE-RULE-CHANNEL: ? -> {}", rule_copy.id);
                    println!("RULE-RULE-CHANNEL: {} -> {}", src_rule_id[iter_index], rule_copy_id);

                    if !rule_channel_map.contains_key(&src_rule_id[iter_index]) {

                        let channel_ends = Vec::<Transition<String>>::new();

                        // TODO (usize, RuleElement)
                        rule_channel_map.insert(src_rule_id[iter_index], channel_ends);
                    }

                    // retrieve the vector of first symbols for the nonterminal and extend it
                    let channel_ends = &mut rule_channel_map.get_mut(&src_rule_id[iter_index]).unwrap();

                    // TODO: (target-id, RuleElement)
                    // channel_ends.push(rule_copy_id);
                    // channel_ends.push(Transition(rule_copy_id, RuleElement::<String>::Unknown));
                    channel_ends.push(Transition(rule_copy_id, current_symbol.clone()));

                    iter_index = iter_index + 1;
                }

                e_set.insert(0, new_grammar_state.id);

                // TODO output transition (to new state)
                println!("STATE-TRANSITION-NEW_STATE: {:?} -{:?}-> {:?}", &current_grammar_state_id, &current_symbol, &new_grammar_state.id);

                grammar_state_hashmap.insert(new_grammar_state.id, new_grammar_state);
            }
        }

        done = e_set.is_empty();
    }



/**/
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

    // DEBUG
    // println!("rule-channel-map: {:?}", &rule_channel_map);

    println!("Propagation cycles start ...");

    // provide fast access, map from rule-id to rule
    let id_to_rule_map = HashMap::<usize, Rule<String>>::new();
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

        println!("Iteration: {:?}", iteration);

        let rust_is_a_kek = grammar_state_hashmap.clone();

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

                // println!("Src-RuleId: {:?}, Src-StateId: {:?} ===> Dest-RuleId: {:?}, Dest-StateId: {:?}",
                //     src_rule_id, src_state_id, dest_rule_id, dest_state_id);

                // push lookaheads

                // retrieve src-rule from src-state
                // let mut src_state = grammar_state_hashmap.get_mut(src_state_id).unwrap();
                let src_state = rust_is_a_kek.get(src_state_id).unwrap();
                // println!("{:?}", src_state);

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

                            // // do not forward the end symbol
                            // if *la == RuleElement::Closure {
                            //     continue;
                            // }

                            if !dest_state.identification_rules[i].lookahead.contains(&la) {
                                dest_state.identification_rules[i].lookahead.push(la.clone());

                                change_detected = true;
                            }
                        }
                    }
                }
                for i in 0..dest_state.rules.len() {
                    if dest_state.rules[i].id == dest_rule_id.0 {
                        // copy lookaheads into dest rule
                        for la in &src_rule.first().unwrap().lookahead {

                            // // do not forward the end symbol
                            // if *la == RuleElement::Closure {
                            //     continue;
                            // }

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


    

    // DEBUG
    // rust iterate over hashmap
    // https://stackoverflow.com/questions/45724517/how-to-iterate-through-a-hashmap-print-the-key-value-and-remove-the-value-in-ru
    println!("");
    println!("***************************************************************");
    println!("RESULT - RESULT - RESULT - RESULT - RESULT - RESULT - RESULT - ");
    println!("***************************************************************");
    for (key, value) in &grammar_state_hashmap {
        println!("{} / {:?}", key, value);
    }
    
    if !found_start_state {
        panic!("DFA no start state detected!");
    } else {
        println!("Start state: {}", start_state_id);
    }

    if !found_final_state {
        panic!("DFA no final state detected!");
    } else {
        println!("Final state: {}", final_state_id);
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
    // The ACTION part contains columns for each nonterminal and the EOI (#) symbol
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
    // The GOTO section contains columns for nonterminals. The cells contain the state ids to transtition to when the 
    // Nonterminal is detected.

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
        println!("Visiting State: {}", current_state_id);

        visited_list.push(current_state_id);

        let state = rust_is_a_kek.get(&current_state_id).unwrap();
        // println!("{:?}", state);





        // convert DFA state into parser table row
        
        let mut parse_table_row = HashMap::<RuleElement<String>, ParseTableCell<usize>>::new();
        // parse_table_row.insert(RuleElement::NonTerminal(String::from("a")), ParseTableCell::<usize>::Shift(2));
        // parse_table_row.insert(RuleElement::NonTerminal(String::from("z")), ParseTableCell::<usize>::Shift(4));
        // parse_table_row.insert(RuleElement::Terminal(String::from("S")), ParseTableCell::<usize>::Goto(1));
        // parse_table_row.insert(RuleElement::Terminal(String::from("B")), ParseTableCell::<usize>::Goto(3));
        // parse_table.insert(current_state_id, parse_table_row);



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

                let last_symbol = &current_rule.rhs[current_rule.rhs.len() - 1];
                if *last_symbol == start_symbol {
                    println!("    ACCEPT");
                    //parse_table_row.insert(current_rule.lookahead[0].clone(), ParseTableCell::<usize>::Accept);
                    //parse_table_row.insert(RuleElement::Terminal(String::from("#")), ParseTableCell::<usize>::Accept);
                    parse_table_row.insert(RuleElement::Closure, ParseTableCell::<usize>::Accept);
                } else {
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

                    panic!("    leaf");
                    
                } else {

                    println!("    inner");
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
                    println!("continue");
                    continue;
                }
                if process_list.contains(target_state_id) {
                    println!("continue");
                    continue;
                }

                // add to list for further processing
                process_list.push(*target_state_id);
            }
        }

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

                if *last_symbol == start_symbol {
                    println!("    ACCEPT");
                    //parse_table_row.insert(current_rule.lookahead[0].clone(), ParseTableCell::<usize>::Accept);
                    // parse_table_row.insert(RuleElement::Terminal(String::from("#")), ParseTableCell::<usize>::Accept);
                    parse_table_row.insert(RuleElement::Closure, ParseTableCell::<usize>::Accept);
                } else {
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

                    panic!("    leaf");
                    
                } else {

                    println!("    inner");
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
                    println!("continue");
                    continue;
                }
                if process_list.contains(target_state_id) {
                    println!("continue");
                    continue;
                }

                // add to list for further processing
                process_list.push(*target_state_id);
            }
        }

        parse_table.insert(current_state_id, parse_table_row);
    }

    println!("ParseTable: {:?}", parse_table);



    

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



    // https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html (Do not add a rule S' -> S into the webapp)
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
    // VOID IDENTIFIER OPENING_BRACES CLOSING_BRACES OPENING_CURLY_BRACES CLOSING_CURLY_BRACES
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACES")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACES")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACES")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACES")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
*/

    // // void main() { EXPRESSION_STOP; }
    // // VOID IDENTIFIER OPENING_BRACES CLOSING_BRACES OPENING_CURLY_BRACES EXPRESSION_STOP SEMICOLON CLOSING_CURLY_BRACES
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACES")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACES")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACES")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("EXPRESSION_STOP")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SEMICOLON")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACES")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);

    // void main() { EXPRESSION_STOP; }
    // VOID IDENTIFIER OPENING_BRACES CLOSING_BRACES OPENING_CURLY_BRACES EXPRESSION_STOP SEMICOLON CLOSING_CURLY_BRACES
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACES")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACES")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACES")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CAST_STOP")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SEMICOLON")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACES")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);

    // void main() { SIZEOF ( VOID ); }
    // VOID IDENTIFIER OPENING_BRACES CLOSING_BRACES OPENING_CURLY_BRACES SIZEOF OPENING_BRACES VOID CLOSING_BRACES SEMICOLON CLOSING_CURLY_BRACES
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACES")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACES")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACES")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SIZEOF")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACES")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACES")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SEMICOLON")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACES")));
    // provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);

    // void main() { IDENTIFIER = IDENTIFIER; }
    // VOID IDENTIFIER OPENING_BRACES CLOSING_BRACES OPENING_CURLY_BRACES IDENTIFIER EQUALS_SIGN IDENTIFIER SEMICOLON CLOSING_CURLY_BRACES
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("VOID")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_BRACES")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_BRACES")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("OPENING_CURLY_BRACES")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("EQUALS_SIGN")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("IDENTIFIER")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("SEMICOLON")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Terminal(String::from("CLOSING_CURLY_BRACES")));
    provide_input(&mut parser, &grammar_state_hashmap, &mut step, &RuleElement::Closure);
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
        println!("Step {}", *step);
        consumed = parser.consume(rule_element.clone(), &grammar_state_hashmap);
        *step = *step + 1;
    }

    *step
}