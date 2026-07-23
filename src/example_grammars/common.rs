use crate::Rule;
use crate::RuleElement;

use crate::RULE_COUNTER;
use std::{
    sync::atomic::{AtomicUsize, Ordering}
};

pub fn create_rule(grammar_rules: &mut Vec::<Rule<String>>, rule_as_string: String, treat_nonterminal_lowercase: bool) {

    // split by a single whitespace!!! This is bad because the user might mess this up easily!
    // BETTER split by arbitrary whitespace!
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