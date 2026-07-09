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