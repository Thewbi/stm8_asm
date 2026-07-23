use std::collections::BTreeMap;

use crate::parser::rule::Rule;
use crate::parser::rule::RuleElement;

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
    println!("NULLABLE 000000000000000000000000000000000000000000000000000000000000000000000000");
    for (key, value) in nullable.clone().into_iter() {
        println!("{:?} / {:?}", key, value);
    }
    println!("000000000000000000000000000000000000000000000000000000000000000000000000000000000");

    // println!("Test");
}