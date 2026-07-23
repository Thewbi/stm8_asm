use crate::parser::rule::Rule;
use crate::parser::rule::RuleElement;

pub fn print_rules(mut temp_rules: Vec::<Rule<String>>, rule_1: &Rule<String>) {

    println!("");
    println!("https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html  (Do not add augmented start rule)");
    println!("https://jsmachines.sourceforge.net/machines/lalr1.html               (add augmented start rule)");
    println!("");
    println!("All Rules:");

    let mut unlocked_rules_lhs = Vec::new();
    unlocked_rules_lhs.push(rule_1.lhs.clone());

    let mut printed_rules = Vec::<Rule<String>>::new();

    // DEBUG - print all rules
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
}