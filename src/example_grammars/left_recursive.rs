use crate::example_grammars::common::create_rule;
use crate::Rule;
use crate::RuleElement;

pub fn produce_grammar_left_recursive(grammar_rules: &mut Vec::<Rule<String>>) -> (Rule<String>, crate::RuleElement<String>) {

    // VALID input
    // void
    // void void 
    // void void void void void

    let treat_nonterminal_lowercase: bool = true;

    // augmented rule
    create_rule(grammar_rules, String::from("statement_list' -> statement_list"), treat_nonterminal_lowercase);

    // // real start symbol
    // let start_symbol = RuleElement::NonTerminal(String::from("statement_list"));

    // augmented start symbol
    let augmented_start_symbol = RuleElement::NonTerminal(String::from("statement_list'"));

    create_rule(grammar_rules, String::from("statement_list -> statement statement_list"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("statement_list -> statement"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("statement -> VOID"), treat_nonterminal_lowercase);

    // the first rule defined by definition is the start rule.
    // By definition, the start rule receives the EOI symbol
    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);

    //(rule_1, start_symbol)
    (rule_1, augmented_start_symbol)
}