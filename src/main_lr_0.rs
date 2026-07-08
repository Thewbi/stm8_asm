// filename: main_lr_0.rs

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RuleElement<T> {
    NonTerminal(T),
    Terminal(T),
    Epsilon,
    Dot,
    Unknown,
    AcceptingStateTransition,
    Unused
}

#[derive(Clone, Eq, Hash)]
pub struct Rule<T> {
    id: usize,
    dot_idx: usize,
    lhs: RuleElement<T>,
    rhs: Vec::<RuleElement<T>>,
}

impl<T: Debug> Rule<T> {
    pub fn new(id: usize) -> Self {
        Rule {
            id: id,
            dot_idx: 0,
            lhs: RuleElement::<T>::Unknown,
            rhs: Vec::<RuleElement<T>>::new(),
        }
    }
}

impl<T: std::cmp::PartialEq> PartialEq<Rule<T>> for Rule<T> {

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
        match &self.lhs {
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
                write!(f, "$!$!$").expect("Write failed!");
            }
            RuleElement::Unknown => {
                write!(f, "UNKNOWN").expect("Write failed!");
            }
            RuleElement::Unused => {
                // nop, do not display unused
            }
        }

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
                    write!(f, "$!$!$").expect("Write failed!");
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

        Ok(())
    }
}

impl<T: Display> fmt::Display for Rule<T> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        // LHS
        match &self.lhs {
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
                write!(f, "$!$!$").expect("Write failed!");
            }
            RuleElement::Unknown => {
                write!(f, "UNKNOWN").expect("Write failed!");
            }
            RuleElement::Unused => {
                // nop, do not display unused
            }
        }

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
                    write!(f, "$!$!$").expect("Write failed!");
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

        Ok(())
    }
}

#[derive(Clone)]
pub struct GrammarState<T: Debug> {
    pub id: usize,
    pub current_rule: Rule<T>,
    pub identification_rules: Vec::<Rule<T>>,
    pub rules: Vec::<Rule<T>>,
}

impl<T: Clone + Debug + Display + std::cmp::PartialEq> GrammarState<T> {

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
    pub fn unfold_grammar_state(&mut self, grammar_rules: &Vec::<Rule<T>>) {

        let mut d_set = Vec::<Rule<T>>::new();
        d_set.append(&mut self.identification_rules.clone());

        let mut done: bool = d_set.is_empty();
        while !done {

            let current_rule: Rule<T> = d_set.pop().expect("Need at least one rule!");

            // // DEBUG
            // println!("current_rule {}", current_rule);

            if current_rule.dot_idx >= current_rule.rhs.len() {
                done = d_set.is_empty();
                continue;
            }

            // if the dot is points to a non-terminal, extend the rule set
            match &current_rule.rhs[current_rule.dot_idx] {

                RuleElement::NonTerminal(non_terminal) => {

                    // DEBUG
                    // println!("non_terminal {}", non_terminal);

                    // find all rules that have a LHS == the non-terminal and add them into the d_set
                    for non_terminal_rule in grammar_rules {

                        if non_terminal_rule.lhs == RuleElement::<T>::NonTerminal(non_terminal.clone()) {

                            if self.rules.contains(non_terminal_rule) {
                                continue;
                            }

                            // DEBUG
                            // println!("non_terminal Rule: {}", non_terminal_rule);

                            d_set.push(non_terminal_rule.clone());
                            self.rules.push(non_terminal_rule.clone());
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
            write!(f, "  {}\n", rule).expect("Write failed!");
        }

        write!(f, "  ============\n").expect("Write failed!");

        // rules
        for rule in &self.rules {
            write!(f, "  {}\n", rule).expect("Write failed!");
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

    // DEBUG
    println!("CREATED RULE: {:?}", rule);

    grammar_rules.push(rule);
}

// https://stackoverflow.com/questions/32935808/generate-sequential-ids-for-each-instance-of-a-struct
static RULE_COUNTER: AtomicUsize = AtomicUsize::new(0);
static STATE_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn main() {

    println!("start");

    println!("All Rules:");

    let mut grammar_rules = Vec::<Rule<String>>::new();

/*
    // // https://staff.polito.it/silvano.rivoira/LingTrad/ParsingTechniques/ParsingTechniques.pdf
    // S'' -> S'
    // S' -> S #
    // S -> E
    // E -> E - T
    // E -> T
    // T -> n
    // T -> ( E )

    let start_symbol = RuleElement::NonTerminal(String::from("S'"));

    let treat_nonterminal_lowercase = false;
    create_rule(&mut grammar_rules, String::from("S'' -> S'"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S' -> S #"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> E"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("E -> E - T"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("E -> T"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("T -> n"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("T -> ( E )"), treat_nonterminal_lowercase);

    let rule_1 = grammar_rules.first().unwrap().clone();
*/


/**/
    // DragonBook 2nd Edition, page 255, Example 4.48. Figure 4.39
    // Reproduced on page 271

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

    // has to be the start symbol of the non-augmented (= original) grammar!
    let start_symbol = RuleElement::NonTerminal(String::from("S"));

    let treat_nonterminal_lowercase = false;
    create_rule(&mut grammar_rules, String::from("S' -> S"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> L = R"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> R"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("L -> * R"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("L -> id"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("R -> L"), treat_nonterminal_lowercase);

    let rule_1 = grammar_rules.first().unwrap().clone();

    

/*
    // https://cyberzhg.github.io/toolbox/lr0

    // this has to be the start symbol of the original unaugmented grammar
    let start_symbol = RuleElement::NonTerminal(String::from("translation_unit"));

    // add augmentation start rule
    // translation_unit' -> translation_unit
    let mut rule_1: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_1.lhs = RuleElement::NonTerminal(String::from("translation_unit'"));
    rule_1.rhs.push(RuleElement::NonTerminal(String::from("translation_unit")));
    println!("{:?}", rule_1);
    grammar_rules.push(rule_1.clone());

    let treat_nonterminal_lowercase: bool = true;

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
    create_rule(&mut grammar_rules, String::from("direct_declarator -> ( declarator )"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("direct_declarator -> ( )"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("compound_statement -> { declaration_or_statement_list }"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("compound_statement -> { }"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("declaration_or_statement_list -> declaration declaration_or_statement_list"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("declaration_or_statement_list -> declaration"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("declaration_or_statement_list -> statement declaration_or_statement_list"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("declaration_or_statement_list -> statement"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("declaration -> declaration_specifiers init_declarator_list SEMICOLON"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("declaration -> declaration_specifiers SEMICOLON"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("init_declarator_list -> init_declarator COMMA init_declarator_list"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("init_declarator_list -> init_declarator"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("init_declarator -> declarator EQUALS_SIGN initializer"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("init_declarator -> declarator"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("statement -> expression_statement"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("expression_statement -> expression SEMICOLON"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("expression_statement -> SEMICOLON"), treat_nonterminal_lowercase);

    //expression -> SUPER
    create_rule(&mut grammar_rules, String::from("expression -> assignment_expression"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("assignment_expression -> unary_expression assignment_operator assignment_expression"), treat_nonterminal_lowercase);
    // this rule causes deep-dive with loop
    create_rule(&mut grammar_rules, String::from("assignment_expression -> conditional_expression"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("unary_expression -> postfix_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_expression -> INC_OP unary_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_expression -> DEC_OP unary_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_expression -> unary_operator cast_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_expression -> SIZEOF unary_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_expression -> SIZEOF ( type_name )"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("unary_operator -> AMPERSAND"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_operator -> ASTERISK"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_operator -> PLUS"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_operator -> MINUS"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_operator -> TILDE"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("unary_operator -> EXCLAMATION_MARK"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("type_name -> specifier_qualifier_list"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("specifier_qualifier_list -> type_specifier"), treat_nonterminal_lowercase);

    create_rule(&mut grammar_rules, String::from("cast_expression -> OPENING_BRACES type_name CLOSING_BRACES cast_expression"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("cast_expression -> unary_expression"), treat_nonterminal_lowercase);

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
// ----------------------------------------------------------------------------------------------------------------
*/

/*
    // https://cyberzhg.github.io/toolbox/lr0
    // 
    // L -> S
    // L -> S L
    // S -> id = E
    // E -> int

    let start_symbol = RuleElement::NonTerminal(String::from("L"));

    // L' -> L
    let mut rule_1: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_1.lhs = RuleElement::NonTerminal(String::from("L'"));
    rule_1.rhs.push(RuleElement::NonTerminal(String::from("L")));
    println!("{:?}", rule_1);
    grammar_rules.push(rule_1.clone());

    // L -> S
    let mut rule_2: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_2.lhs = RuleElement::NonTerminal(String::from("L"));
    rule_2.rhs.push(RuleElement::NonTerminal(String::from("S")));
    println!("{:?}", rule_2);
    grammar_rules.push(rule_2.clone());

    // L -> S L
    let mut rule_3: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_3.lhs = RuleElement::NonTerminal(String::from("L"));
    rule_3.rhs.push(RuleElement::NonTerminal(String::from("S")));
    rule_3.rhs.push(RuleElement::NonTerminal(String::from("L")));
    println!("{:?}", rule_3);
    grammar_rules.push(rule_3.clone());

    // S -> id = E
    let mut rule_4: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_4.lhs = RuleElement::NonTerminal(String::from("S"));
    rule_4.rhs.push(RuleElement::Terminal(String::from("id")));
    rule_4.rhs.push(RuleElement::Terminal(String::from("=")));
    rule_4.rhs.push(RuleElement::NonTerminal(String::from("E")));
    println!("{:?}", rule_4);
    grammar_rules.push(rule_4.clone());

    // E -> int
    let mut rule_5: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_5.lhs = RuleElement::NonTerminal(String::from("E"));
    rule_5.rhs.push(RuleElement::NonTerminal(String::from("int")));
    println!("{:?}", rule_5);
    grammar_rules.push(rule_5.clone());
*/

/*
    // https://cyberzhg.github.io/toolbox/lr0
    // 
    // A -> B A B
    // A -> B
    // B -> 1 C C
    // C -> 0
    // C -> ϵ

    let start_symbol = RuleElement::NonTerminal(String::from("A"));

    // A' -> A
    let mut rule_1: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_1.lhs = RuleElement::NonTerminal(String::from("A'"));
    rule_1.rhs.push(RuleElement::NonTerminal(String::from("A")));
    println!("{:?}", rule_1);
    grammar_rules.push(rule_1.clone());

    // A -> B A B
    let mut rule_2: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_2.lhs = RuleElement::NonTerminal(String::from("A"));
    rule_2.rhs.push(RuleElement::NonTerminal(String::from("B")));
    rule_2.rhs.push(RuleElement::NonTerminal(String::from("A")));
    rule_2.rhs.push(RuleElement::NonTerminal(String::from("B")));
    println!("{:?}", rule_2);
    grammar_rules.push(rule_2.clone());

    // A -> B
    let mut rule_3: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_3.lhs = RuleElement::NonTerminal(String::from("A"));
    rule_3.rhs.push(RuleElement::NonTerminal(String::from("B")));
    println!("{:?}", rule_3);
    grammar_rules.push(rule_3.clone());

    // B -> 1 C C
    let mut rule_4: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_4.lhs = RuleElement::NonTerminal(String::from("B"));
    rule_4.rhs.push(RuleElement::NonTerminal(String::from("1")));
    rule_4.rhs.push(RuleElement::NonTerminal(String::from("C")));
    rule_4.rhs.push(RuleElement::NonTerminal(String::from("C")));
    println!("{:?}", rule_4);
    grammar_rules.push(rule_4.clone());

    // C -> 0
    let mut rule_5: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_5.lhs = RuleElement::NonTerminal(String::from("C"));
    rule_5.rhs.push(RuleElement::NonTerminal(String::from("0")));
    println!("{:?}", rule_5);
    grammar_rules.push(rule_5.clone());

    // C -> ϵ
    let mut rule_6: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_6.lhs = RuleElement::NonTerminal(String::from("C"));
    rule_6.rhs.push(RuleElement::Epsilon);
    println!("{:?}", rule_6);
    grammar_rules.push(rule_6.clone());
*/

/*
    // https://cyberzhg.github.io/toolbox/lr0
    // 
    // E -> E + T | T
    // T -> T * F | F
    // F -> ( E ) | id
    //
    // Annotated
    //
    // E -> E + T
    // E -> T
    // T -> T * F
    // T -> F
    // F -> ( E )
    // F -> id

    // has to be the start symbol of the non-augmented (= original) grammar!
    let start_symbol = RuleElement::NonTerminal(String::from("E"));

    let treat_nonterminal_lowercase = false;
    create_rule(&mut grammar_rules, String::from("E' -> E"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("E -> E + T"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("E -> T"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("T -> T * F"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("T -> F"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("F -> ( E )"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("F -> id"), treat_nonterminal_lowercase);

    let rule_1 = grammar_rules.first().unwrap().clone();
*/

/*
    // https://cyberzhg.github.io/toolbox/lr0
    // 
    // E -> E + T | T
    // T -> T * F | F
    // F -> ( E ) | id
    //
    // Annotated
    //
    // E -> E + T
    // E -> T
    // T -> T * F
    // T -> F
    // F -> ( E )
    // F -> id

    let start_symbol = RuleElement::NonTerminal(String::from("E"));

    // E' -> E
    let mut rule_1: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_1.lhs = RuleElement::NonTerminal(String::from("E'"));
    rule_1.rhs.push(RuleElement::NonTerminal(String::from("E")));
    println!("{:?}", rule_1);
    grammar_rules.push(rule_1.clone());

    // E -> E + T
    let mut rule_2: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_2.lhs = RuleElement::NonTerminal(String::from("E"));
    rule_2.rhs.push(RuleElement::NonTerminal(String::from("E")));
    rule_2.rhs.push(RuleElement::Terminal(String::from("+")));
    rule_2.rhs.push(RuleElement::NonTerminal(String::from("T")));
    println!("{:?}", rule_2);
    grammar_rules.push(rule_2.clone());

    // E -> T
    let mut rule_3: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_3.lhs = RuleElement::NonTerminal(String::from("E"));
    rule_3.rhs.push(RuleElement::NonTerminal(String::from("T")));
    println!("{:?}", rule_3);
    grammar_rules.push(rule_3.clone());

    // T -> T * F
    let mut rule_4: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_4.lhs = RuleElement::NonTerminal(String::from("T"));
    rule_4.rhs.push(RuleElement::NonTerminal(String::from("T")));
    rule_4.rhs.push(RuleElement::Terminal(String::from("*")));
    rule_4.rhs.push(RuleElement::NonTerminal(String::from("F")));
    println!("{:?}", rule_4);
    grammar_rules.push(rule_4.clone());

    // T -> F
    let mut rule_5: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_5.lhs = RuleElement::NonTerminal(String::from("T"));
    rule_5.rhs.push(RuleElement::NonTerminal(String::from("F")));
    println!("{:?}", rule_5);
    grammar_rules.push(rule_5.clone());

    // F -> ( E )
    let mut rule_6: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_6.lhs = RuleElement::NonTerminal(String::from("F"));
    rule_6.rhs.push(RuleElement::Terminal(String::from("(")));
    rule_6.rhs.push(RuleElement::NonTerminal(String::from("E")));
    rule_6.rhs.push(RuleElement::Terminal(String::from(")")));
    println!("{:?}", rule_6);
    grammar_rules.push(rule_6.clone());

    // F -> id
    let mut rule_7: Rule<String> = Rule::new(RULE_COUNTER.fetch_add(1, Ordering::SeqCst));
    rule_7.lhs = RuleElement::NonTerminal(String::from("F"));
    rule_7.rhs.push(RuleElement::Terminal(String::from("id")));
    println!("{:?}", rule_7);
    grammar_rules.push(rule_7.clone());
*/

    let mut found_start_state: bool = false;
    let mut start_state_id: usize = 0;
    let mut found_final_state: bool = false;
    let mut final_state_id: usize = 0;

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
            grammar_state.unfold_grammar_state(&grammar_rules);

            // // DEBUG
            // println!("\n");
            // println!("----------------------------------------");
            // println!("{:?}", grammar_state);
            // println!("========================================");
        }

        // clone current state
        let curr_state = grammar_state_hashmap[&current_grammar_state_id].clone();

        // collect all rules without removing them from the current state
        let mut all_rules = Vec::new();
        all_rules.append(&mut curr_state.identification_rules.clone());
        all_rules.append(&mut curr_state.rules.clone());

        // iterate over all rules and collect all rules that are activated by the same symbol
        while all_rules.len() > 0 {

            // remove rules that are completely processed (dot-marker is after last symbol)
            let consumed_rules = all_rules.extract_if(.., |r| r.dot_idx >= r.rhs.len()).collect::<Vec<_>>();

            // check for the end state
            // The end state has the dot marker after the start symbol
            if consumed_rules.len() == 1 && let Some(last_symbol) = consumed_rules[0].rhs.last() {

                if *last_symbol == start_symbol {

                    if found_final_state {
                        panic!("DFA cannot have two end states!");
                    }

                    found_final_state = true;
                    final_state_id = current_grammar_state_id;

                    // TODO output transition
                    println!("{:?} -{:?}-> {:?}", &current_grammar_state_id, "$!$!$", std::usize::MAX);
                }
            }

            if all_rules.len() == 0 {
                continue;
            }

            // get activated symbol from first rule
            let current_symbol = all_rules[0].rhs[all_rules[0].dot_idx].clone();

            // extract other rules that have the same activated symbol
            let mut rules_for_symbol = all_rules.extract_if(.., |r| r.dot_idx < r.rhs.len() && r.rhs[r.dot_idx] == current_symbol).collect::<Vec<_>>();

            if rules_for_symbol.len() == 0 {
                continue;
            }

            // if it is an epsilon, do nothing because that node will not transition to another node at all
            match current_symbol {
                RuleElement::Epsilon => { continue; }
                _ => {}
            }
            
            // // DEBUG - output the symbol and the rules activated by it
            // println!("{:?} {:?}", &current_symbol, &rules_for_symbol);
            // println!("");

            // TODO
            // iterate over each rule in rules_for_symbol
            //      - advance the dot in the collected rules
            //      - look for states globally in grammar_state_hashmap, that have all the collected, 
            //        modified rules in their identifying set at the same time! VERY IMPORTANT
            //          - if no such state exists yet, create one
            //              - insert newly created state into e_set
            //              - insert newly created state into global set
            //          - if such a state exists, build transition to it

            // remove depleted rules
            let _ = rules_for_symbol.extract_if(.., |r| r.dot_idx >= r.rhs.len()).collect::<Vec<_>>();
            for rule in &mut rules_for_symbol {
                rule.dot_idx = rule.dot_idx + 1;
            }

            // DEBUG - output the symbol and the rules activated by it
            // println!("{:?} {:?}", &current_symbol, &rules_for_symbol);

            if rules_for_symbol.len() == 0 {
                continue;
            }

            let mut state_contained_already = false;
            let mut state_id: usize = 0;

            // this is an example for the global map
            // TODO: we do not want this huge for loop over all global states! This is trash! 
            // Needs to be constant time and not O(n)
            for (loop_state_id, loop_state) in &grammar_state_hashmap {

                // a state is identified via the (all rules in) identification rules set
                if loop_state.identification_rules == rules_for_symbol {

                    // state found
                    state_contained_already = true;
                    state_id = *loop_state_id;

                    // only a single state can be found
                    break;
                }
            }

            if state_contained_already {

                // TODO output transition (to already existing state)
                println!("{:?} -{:?}-> {:?}", &current_grammar_state_id, &current_symbol, &state_id);

            } else {

                // state not contained, build state, insert into e_set, insert transition

                let mut new_grammar_state: GrammarState<String> = GrammarState::new(STATE_COUNTER.fetch_add(1, Ordering::SeqCst));

                // DEBUG
                // println!("Created new state: {:?}", new_grammar_state.id);

                new_grammar_state.identification_rules.append(&mut rules_for_symbol);

                e_set.insert(0, new_grammar_state.id);

                // TODO output transition (to new state)
                println!("{:?} -{:?}-> {:?}", &current_grammar_state_id, &current_symbol, &new_grammar_state.id);

                grammar_state_hashmap.insert(new_grammar_state.id, new_grammar_state);
            }
        }

        done = e_set.is_empty();
    }
    
    // DEBUG
    // rust iterate over hashmap
    // https://stackoverflow.com/questions/45724517/how-to-iterate-through-a-hashmap-print-the-key-value-and-remove-the-value-in-ru
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

    println!("end");
}