use crate::BTreeMap;

use crate::parser::rule::Rule;
use crate::parser::rule::RuleElement;

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
pub fn compute_first_original(grammar_rules: &Vec::<Rule<String>>, nullable: &BTreeMap::<RuleElement::<String>, bool>, first: &mut BTreeMap::<RuleElement::<String>, Vec::<RuleElement::<String>>>) {

    let mut change_detected = true;
    while change_detected {

        // println!("Change detected == false");
        change_detected = false;

        for rule in grammar_rules.iter() {

            // DEBUG
            // println!("{:?}", rule);

            for r in rule.rhs.iter() {

                match &r {

                    RuleElement::Terminal(str_val) => {
                        //println!("Terminal in first position found: {:?}", str_val);

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

                            // if the non-terminal is nullable, proceed with the next symbol, 
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

    // DEBUG output FIRST()
    println!("");
    println!("FIRST() *************************************************************************");
    for (key, value) in first.clone().into_iter() {
        println!("{:?} / {:?}", key, value);
    }
    println!("*********************************************************************************");
}