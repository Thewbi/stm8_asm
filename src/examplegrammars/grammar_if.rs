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