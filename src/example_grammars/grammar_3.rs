use crate::example_grammars::common::create_rule;
use crate::Rule;
use crate::RuleElement;

pub fn produce_grammar_3(grammar_rules: &mut Vec::<Rule<String>>) -> (Rule<String>, crate::RuleElement<String>) {

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

    create_rule(grammar_rules, String::from("S -> a S b"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("S -> c"), treat_nonterminal_lowercase);

    // the first rule per definition has the closure symbol as a spontaneous symbol
    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);
    
    (rule_1, augmented_start_symbol)

}