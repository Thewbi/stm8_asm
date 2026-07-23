use crate::example_grammars::common::create_rule;
use crate::Rule;
use crate::RuleElement;

// The problem with this grammar is that it contains the empty string which is not supported by every tool
// used during validation

pub fn produce_grammar_1(grammar_rules: &mut Vec::<Rule<String>>) -> (Rule<String>, crate::RuleElement<String>) {

    // https://www.geeksforgeeks.org/compiler-design/first-set-in-syntax-analysis/

    // Production Rules of Grammar
    // S -> ACB | Cbb | Ba
    // A -> da | BC
    // B -> g | ε
    // C -> h | ε

    // FIRST sets
    // FIRST(S) = FIRST(ACB) U FIRST(Cbb) U FIRST(Ba) = { d, g, h, b, a,  ε}
    // FIRST(A) = { d } U FIRST(BC) = { d, g, h,  ε }
    // FIRST(B) = { g ,  ε }
    // FIRST(C) = { h ,  ε }

    // https://cyberzhg.github.io/toolbox/lr0
    // https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html (Do not add a rule S' -> S into the webapp)
    // https://jsmachines.sourceforge.net/machines/lalr1.html (add a rule S' -> S into the webapp)

    // VALID-INPUT:
    // d a h g #

    // // this has to be the start symbol of the original unaugmented grammar
    // let start_symbol = RuleElement::NonTerminal(String::from("S"));

    // augmented start symbol
    let augmented_start_symbol = RuleElement::NonTerminal(String::from("S'"));

    let treat_nonterminal_lowercase: bool = false;

    // add augmentation start rule
    create_rule(grammar_rules, String::from("S' -> S"), treat_nonterminal_lowercase);

    // S -> ACB | Cbb | Ba
    create_rule(grammar_rules, String::from("S -> A C B"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("S -> C b b"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("S -> B a"), treat_nonterminal_lowercase);

    // A -> da | BC
    create_rule(grammar_rules, String::from("A -> d a"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("A -> B C"), treat_nonterminal_lowercase);

    // B -> g | ε
    create_rule(grammar_rules, String::from("B -> g"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("B -> $$_EPSILON_$$"), treat_nonterminal_lowercase);

    // C -> h | ε
    create_rule(grammar_rules, String::from("C -> h"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("C -> $$_EPSILON_$$"), treat_nonterminal_lowercase);

    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);

    (rule_1, augmented_start_symbol)
}



// Production Rules of Grammar
    // S -> ACB | Cbb | Ba
    // A -> da | BC
    // B -> g | ε
    // C -> h | ε

    // FIRST sets
    // FIRST(S) = FIRST(ACB) U FIRST(Cbb) U FIRST(Ba) = { d, g, h, b, a,  ε}
    // FIRST(A) = { d } U FIRST(BC) = { d, g, h,  ε }
    // FIRST(B) = { g ,  ε }
    // FIRST(C) = { h ,  ε }

    // println!("Test");

    // let first = BTreeMap::new();

    // let terminal_a = RuleElement::<String>::Terminal(String::from("a"));
    // let terminal_b = RuleElement::<String>::Terminal(String::from("b"));

    // let mut first_A = Vec::<RuleElement::<String>>::new();
    // first_A.push(terminal_a);
    // first_A.push(terminal_b);

    // let non_terminal_A = RuleElement::<String>::NonTerminal(String::from("A"));

    // let first = BTreeMap::new();
    // first.insert(non_terminal_A.clone(), first_A);