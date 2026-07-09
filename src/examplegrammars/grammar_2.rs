use crate::create_rule;
use crate::Rule;
use crate::RuleElement;

pub fn produce_grammar_2(grammar_rules: &mut Vec::<Rule<String>>) -> (Rule<String>, crate::RuleElement<String>) {

    // S' -> S
    // S -> A A
    // A -> a A
    // A -> b

    // Valid input: a b a b

    // this must be the start symbol of the original grammar
    // let start_symbol = RuleElement::NonTerminal(String::from("S"));

    // augmented start symbol
    let augmented_start_symbol = RuleElement::NonTerminal(String::from("S'"));

    let treat_nonterminal_lowercase = false;

    create_rule(grammar_rules, String::from("S' -> S"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("S -> A A"), treat_nonterminal_lowercase);
    
    create_rule(grammar_rules, String::from("A -> a A"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("A -> b"), treat_nonterminal_lowercase);

    // the first rule per definition has the closure symbol as a spontaneous symbol
    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);
    
    (rule_1, augmented_start_symbol)

}