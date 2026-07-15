/*
    // https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html#eyJncmFtbWFyIjoiUyAtPiBhIEEgY1xuUyAtPiBhIEIgZFxuUyAtPiBCIGNcbkEgLT4gelxuQiAtPiB6IiwiaW5wdXQiOiIifQ==

    // S' -> S
    // S -> a A c
    // S -> a B d
    // S -> B c
    // A -> z
    // B -> z

    // VALID-INPUT
    // a z d #

    // this must be the start symbol of the original grammar
    let start_symbol = RuleElement::NonTerminal(String::from("S"));

    let treat_nonterminal_lowercase = false;
    create_rule(&mut grammar_rules, String::from("S' -> S"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> a A c"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> a B d"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("S -> B c"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("A -> z"), treat_nonterminal_lowercase);
    create_rule(&mut grammar_rules, String::from("B -> z"), treat_nonterminal_lowercase);

    // the first rule per definition has the closure symbol as a spontaneous symbol
    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);
*/

use crate::create_rule;
use crate::Rule;
use crate::RuleElement;

pub fn produce_grammar_4(grammar_rules: &mut Vec::<Rule<String>>) -> (Rule<String>, crate::RuleElement<String>) {

    //
    // This is an example grammar for a grammar that needs lookahead propagation iteration!
    //
    // https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html (do not add augmented start rule)
    // https://jsmachines.sourceforge.net/machines/lalr1.html (add augmented start rule)
    //
 
    // https://stackoverflow.com/questions/77577494/is-this-grammar-lalr1

    // VALID-INPUT: a c b #

    // this must be the start symbol of the original grammar
    // let start_symbol = RuleElement::NonTerminal(String::from("S"));

    // augmented start symbol
    let augmented_start_symbol = RuleElement::NonTerminal(String::from("S'"));

    let treat_nonterminal_lowercase = false;

    create_rule(grammar_rules, String::from("S' -> S"), treat_nonterminal_lowercase);

    // S -> a A c
    // S -> a B d
    // S -> B c
    // A -> z
    // B -> z
    create_rule(grammar_rules, String::from("S -> a A c"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("S -> a B d"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("S -> B c"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("A -> z"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("B -> z"), treat_nonterminal_lowercase);

    // the first rule per definition has the closure symbol as a spontaneous symbol
    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);
    
    (rule_1, augmented_start_symbol)

}