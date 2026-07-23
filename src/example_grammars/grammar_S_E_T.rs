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