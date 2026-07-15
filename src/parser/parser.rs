use std::collections::HashMap;
use std::collections::BTreeMap;

use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;

use crate::GrammarState;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseTableCell<T> {

    // ACTION-Part
    Shift(T),
    Reduce(T),
    Accept,

    // GOTO-Part
    Goto(T),
}

impl<T: Display> fmt::Debug for ParseTableCell<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match &self {

            // ACTION-Part
            ParseTableCell::Shift(target) => {
                write!(f, "Shift {}", target).expect("Write failed!");
            },
            ParseTableCell::Reduce(target) => {
                write!(f, "Reduce {}", target).expect("Write failed!");
            },
            ParseTableCell::Accept=> {
                write!(f, "Accept").expect("Write failed!");
            },

            // GOTO-Part
            ParseTableCell::Goto(target) => {
                write!(f, "Goto {}", target).expect("Write failed!");
            },
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum ParseStackElement<T: std::fmt::Display> {
    RuleElement(RuleElement<T>),
    StateId(usize),
}

pub struct Parser<T> {
    pub parse_table: HashMap::<usize, HashMap::<RuleElement<T>, ParseTableCell<usize>>>,
    pub stack: Vec::<ParseStackElement<String>>,
}



impl Parser<String> {

    pub fn new(parse_table_param: HashMap::<usize, HashMap::<RuleElement<String>, ParseTableCell<usize>>>) -> Self {

        let mut p = Parser {
            parse_table: parse_table_param,
            stack: Vec::<ParseStackElement<String>>::new(),
        };

        let pse = ParseStackElement::<String>::StateId(0);
        p.stack.push(pse);

        p
    }

    pub fn consume(&mut self, input: RuleElement<String>, grammar_state_hashmap: &BTreeMap<usize, GrammarState<String>>) -> bool {

        let debug = false;

        if debug {
            println!("[Parser::consume] stack {:?}", self.stack);
        }

        let parse_stack_element = self.stack.pop().unwrap();
        self.stack.push(parse_stack_element.clone());

        match &parse_stack_element {

            ParseStackElement::RuleElement(rule_element) => {
                if debug {
                    println!("[Parser::consume] RuleElement {:?}", rule_element);
                }

                let stack_len = self.stack.len();

                if debug {
                    println!("[Parser::consume] stack_len {:?}", stack_len);
                }

                let stack_content = &self.stack[stack_len - 2];

                match &stack_content {

                    ParseStackElement::StateId(current_state_id) => {

                        if debug {
                            println!("[Parser::consume] StateId: {}", current_state_id);
                        }

                        let parse_table_row = self.parse_table.get(&current_state_id).unwrap();

                        // DEBUG
                        // let contains_key: bool = parse_table_row.contains_key(&rule_element);
                        // println!("State: {}, parse_table_row: {:?}, input: {:?}, contains_key: {:?}", current_state_id, parse_table_row, input, contains_key);

                        let idk = parse_table_row.get(&rule_element).unwrap();
                        match idk {

                            ParseTableCell::Goto(next_state_id) => {
                                if debug {
                                    println!("[Parser::consume] GOTO: {:?}", *next_state_id);
                                }
                                let pse = ParseStackElement::<String>::StateId(*next_state_id);
                                self.stack.push(pse);
                            }

                            _ => {
                                panic!("[Parser::consume] test");
                            }
                        }
                    }

                    ParseStackElement::RuleElement(rule_element) => {
                        panic!("[Parser::consume] RuleElement");
                    }
                }

                false
            }

            ParseStackElement::StateId(current_state_id) => {
                // println!("[Parser::consume] StateId: {}", current_state_id);

                let parse_table_row = self.parse_table.get(&current_state_id).unwrap();

                // DEBUG
                let state = grammar_state_hashmap.get(&current_state_id).unwrap();

                let contains_key: bool = parse_table_row.contains_key(&input);

                if debug {
                    println!("[Parser::consume] StateId: {}", current_state_id);
                    println!("[Parser::consume] State: {:?}", state);
                    println!("[Parser::consume] parse_table_row: {:?}", parse_table_row);
                    println!("[Parser::consume] Input: {:?}", input);
                    println!("[Parser::consume] Contains Key: {:?}", contains_key);
                }

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

                    let stack_top_element = self.stack.pop().unwrap();
                        self.stack.push(stack_top_element.clone());

                    if debug {
                        println!("stack_top_element: {:?}", stack_top_element);
                    }

                    match &stack_top_element {

                        ParseStackElement::StateId(current_state_id) => {
                            panic!("[Parser::consume] StateId: {}", current_state_id);
                        }

                        ParseStackElement::RuleElement(rule_element) => {
                            if debug {
                                println!("[Parser::consume] RuleElement");
                            }

                            let idk = parse_table_row.get(&rule_element).unwrap();
                            match idk {

                                ParseTableCell::Goto(state_id) => {
                                    if debug {
                                        println!("[Parser::consume] GOTO: {:?}", *state_id);
                                    }
                                    let pse = ParseStackElement::<String>::StateId(*state_id);
                                    self.stack.push(pse);

                                    false
                                }

                                ParseTableCell::Shift(state_id) => {
                                    if debug {
                                        println!("[Parser::consume] SHIFT: {:?}", *state_id);
                                    }
                                    let pse = ParseStackElement::<String>::StateId(*state_id);
                                    self.stack.push(pse);

                                    false
                                }

                                _ => {
                                    panic!("[Parser::consume] test {:?}", idk);
                                }
                            }
                        }
                    }

                } else {
                    
                    let parser_step = parse_table_row.get(&input).expect("Parse Table is broken!");
                    match parser_step {

                        ParseTableCell::Shift(next_state_id) => {
                            if debug {
                                println!("[Parser::consume] shift {}", next_state_id);
                            }
                            self.stack.push(ParseStackElement::<String>::RuleElement(input));
                            self.stack.push(ParseStackElement::<String>::StateId(*next_state_id));

                            true
                        }

                        ParseTableCell::Reduce(rule_id) => {
                            // println!("[Parser::consume] reducing rule_id: {}", rule_id);

                            let state = grammar_state_hashmap.get(&current_state_id).unwrap();

                            // DEBUG
                            // println!("[Parser::consume] reduce State: {:?}, rule_id: {:?}", state, rule_id);

                            // TODO: only using rules (and not identification_rules) right now
                            //let rule = state.unwrap().rules.into_iter().filter(|r| r.id == *rule_id).collect::<Vec<_>>().first();

                            let mut found_rule = Rule::<String>::new(0);

                            let mut found = false;
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

                            if !found {
                                panic!("[Parser::consume] Rule not found!");
                            } else {
                                if debug {
                                    println!("[Parser::consume] rule: {:?}", found_rule);
                                }

                                print!("[Parser::consume()] REDUCING RULE: ");
                                found_rule.print_rule_simple();

                                for rhs in found_rule.rhs {
                                    self.stack.pop();
                                    self.stack.pop();
                                }

                                self.stack.push(ParseStackElement::<String>::RuleElement(found_rule.lhs));
                            }

                            false
                        }

                        ParseTableCell::Accept => {
                            println!("[Parser::consume] ACCEPT !!!!");
                            true
                        }

                        _ => {
                            panic!("[Parser::consume] NIY!");
                        }

                    }
                }
            }
        }
    }
}

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
    pub id: usize,
    pub dot_idx: usize,
    pub lhs: RuleElement<T>,
    pub rhs: Vec::<RuleElement<T>>,
    pub lookahead: Vec::<RuleElement<T>>,
    pub channels: Vec::<usize>,
}

impl<T: Debug + std::fmt::Display> Rule<T> {

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

    pub fn print_rule_simple(&self) {

        // LHS
        print!("{:?}", &self.lhs);

        print!(" -> ");

        // RHS
        let mut index: usize = 0;
        for symbol in &self.rhs {

            match &symbol {
                RuleElement::NonTerminal(str_val) => {
                    if index > 0 {
                        print!(" ");
                    }
                    print!("{str_val}");
                    index = index + 1;
                }
                RuleElement::Terminal(str_val) => {
                    if index > 0 {
                        print!(" ");
                    }
                    print!("{str_val}");
                    index = index + 1;
                }
                RuleElement::Epsilon => {
                    if index > 0 {
                        print!(" ");
                    }
                    print!("ϵ");
                    index = index + 1;
                }
                RuleElement::Dot => {
                    if index > 0 {
                        print!(" ");
                    }
                    print!(".");
                    index = index + 1;
                }
                RuleElement::AcceptingStateTransition => {
                    if index > 0 {
                        print!(" ");
                    }
                    print!("$");
                    index = index + 1;
                }
                RuleElement::Closure => {
                    if index > 0 {
                        print!(" ");
                    }
                    print!("#");
                    index = index + 1;
                }
                RuleElement::Unknown => {
                    if index > 0 {
                        print!(" ");
                    }
                    print!("UNKNOWN");
                    index = index + 1;
                }
                RuleElement::Unused => {
                    // nop, do not display unused
                }
            }
        }

        println!("");
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

        if self.rhs.len() != other.rhs.len() {
            return false;
        }

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

        // RHS
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

pub struct Transition<T>(pub usize, pub RuleElement<T>);