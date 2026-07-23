use std::collections::HashSet;

use crate::parser::rule::Rule;
use crate::parser::rule::RuleElement;

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