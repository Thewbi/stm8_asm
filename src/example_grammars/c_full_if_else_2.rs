use crate::Rule;
use crate::RuleElement;

use crate::example_grammars::common::create_rule;

pub fn produce_grammar_c_full_if_else_2(grammar_rules: &mut Vec::<Rule<String>>) -> (Rule<String>, crate::RuleElement<String>) {

    // https://cyberzhg.github.io/toolbox/lr0
    // https://cyberzhg.github.io/parsing-toys/tools/cfg_lalr1.html (Do not add augmented start rule!)
    // https://jsmachines.sourceforge.net/machines/lalr1.html (Add augmented start rule!)

    // https://www.lysator.liu.se/c/ANSI-C-grammar-y.html

    // VALID INPUT:
    
    // void main () {}
    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET CLOSING_CURLY_BRACKET

    // void main ( EXPRESSION_STOP SEMICOLON )
    // VOID IDENTIFIER OPENING_BRACKET CLOSING_BRACKET OPENING_CURLY_BRACKET EXPRESSION_STOP SEMICOLON CLOSING_CURLY_BRACKET

    // this has to be the start symbol of the original unaugmented grammar

    // ORIG
    //let start_symbol = RuleElement::NonTerminal(String::from("translation_unit"));

    // DEBUG
    // let start_symbol = RuleElement::NonTerminal(String::from("additive_expression"));
    // let start_symbol = RuleElement::NonTerminal(String::from("expression_statement"));
    //let start_symbol = RuleElement::NonTerminal(String::from("expression"));
    // let start_symbol = RuleElement::NonTerminal(String::from("statement"));

    //
    // add augmented start symbol
    //

    // let augmented_start_symbol = RuleElement::NonTerminal(String::from("statement'"));
    // let augmented_start_symbol = RuleElement::NonTerminal(String::from("declaration_or_statement_list'"));
    // let augmented_start_symbol = RuleElement::NonTerminal(String::from("compound_statement'"));
    // let augmented_start_symbol = RuleElement::NonTerminal(String::from("selection_statement'"));
    let augmented_start_symbol = RuleElement::NonTerminal(String::from("function_definition'"));
    // let augmented_start_symbol = RuleElement::NonTerminal(String::from("declaration'"));

    let treat_nonterminal_lowercase: bool = true;

    //
    // add augmentation start rule
    //
    
    // ORIG
    //create_rule(grammar_rules, String::from("translation_unit' -> translation_unit"), treat_nonterminal_lowercase);
    
    // DEBUG
    // create_rule(grammar_rules, String::from("additive_expression' -> additive_expression"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("expression_statement' -> expression_statement"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("expression' -> expression"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("statement' -> statement"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("declaration_or_statement_list' -> declaration_or_statement_list"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("compound_statement' -> compound_statement"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("selection_statement' -> selection_statement"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("function_definition' -> function_definition"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("declaration' -> declaration"), treat_nonterminal_lowercase);


//     // ORIG
// //     create_rule(grammar_rules, String::from("translation_unit -> function_definition"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("function_definition -> declaration_specifiers declarator compound_statement"), treat_nonterminal_lowercase);

    // create_rule(grammar_rules, String::from("declaration_specifiers -> type_specifier declaration_specifiers"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("declaration_specifiers -> type_specifier"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("declaration_specifiers -> type_qualifier declaration_specifiers"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("declaration_specifiers -> type_qualifier"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("type_specifier -> VOID"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("type_specifier -> INT"), treat_nonterminal_lowercase);

    // create_rule(grammar_rules, String::from("type_qualifier -> CONST"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("type_qualifier -> VOLATILE"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("declarator -> direct_declarator"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("direct_declarator -> IDENTIFIER direct_declarator"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("direct_declarator -> IDENTIFIER"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("direct_declarator -> OPENING_BRACKET declarator CLOSING_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("direct_declarator -> OPENING_BRACKET CLOSING_BRACKET"), treat_nonterminal_lowercase);

    //     // compound_statement
// 	// : '{' '}'
// 	// | '{' statement_list '}'
// 	// | '{' declaration_list '}'
// 	// | '{' declaration_list statement_list '}'
// 	// ;
    // create_rule(grammar_rules, String::from("compound_statement -> OPENING_CURLY_BRACKET statement_list CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("compound_statement -> OPENING_CURLY_BRACKET CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("compound_statement -> OPENING_CURLY_BRACKET declaration_or_statement_list CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("compound_statement -> OPENING_CURLY_BRACKET CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("compound_statement -> OPENING_CURLY_BRACKET statement CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);

    // create_rule(grammar_rules, String::from("declaration_or_statement_list -> declaration declaration_or_statement_list"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("declaration_or_statement_list -> declaration"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("declaration_or_statement_list -> statement declaration_or_statement_list"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("declaration_or_statement_list -> statement"), treat_nonterminal_lowercase);

// //     // DEBUG
// //     create_rule(grammar_rules, String::from("declaration -> DECLARATION_STOP"), treat_nonterminal_lowercase);

    // create_rule(grammar_rules, String::from("declaration -> declaration_specifiers init_declarator_list SEMICOLON"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("declaration -> declaration_specifiers SEMICOLON"), treat_nonterminal_lowercase);

    // create_rule(grammar_rules, String::from("init_declarator_list -> init_declarator COMMA init_declarator_list"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("init_declarator_list -> init_declarator"), treat_nonterminal_lowercase);

    // create_rule(grammar_rules, String::from("init_declarator -> declarator EQUALS_SIGN initializer"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("init_declarator -> declarator"), treat_nonterminal_lowercase);

    // create_rule(grammar_rules, String::from("initializer -> assignment_expression"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("initializer -> OPENING_CURLY_BRACKET initializer_list CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("initializer -> OPENING_CURLY_BRACKET initializer_list COMMA CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);

    // create_rule(grammar_rules, String::from("initializer_list -> initializer"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("initializer_list -> initializer COMMA initializer_list"), treat_nonterminal_lowercase);
// // 

// //     // statement
// // 	// : labeled_statement
// // 	// | compound_statement
// // 	// | expression_statement
// // 	// | selection_statement
// // 	// | iteration_statement
// // 	// | jump_statement
// // 	// ;
    create_rule(grammar_rules, String::from("statement -> compound_statement"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("statement -> expression_statement"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("statement -> selection_statement"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("statement -> jump_statement"), treat_nonterminal_lowercase);
// //     // DEBUG
    // create_rule(grammar_rules, String::from("statement -> STATEMENT_STOP"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("statement -> VOID"), treat_nonterminal_lowercase);
// //     // DEBUG
// //     // create_rule(grammar_rules, String::from("statement -> STATEMENT_STOP"), treat_nonterminal_lowercase);


    
//     // statement_list
// 	// : statement
// 	// | statement_list statement
// 	// ;
    // create_rule(grammar_rules, String::from("statement_list -> statement statement_list"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("statement_list -> statement"), treat_nonterminal_lowercase);

//     create_rule(grammar_rules, String::from("expression_statement -> expression SEMICOLON"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("expression_statement -> SEMICOLON"), treat_nonterminal_lowercase);

    //     // selection_statement
    // 	// : IF '(' expression ')' statement
    // 	// | IF '(' expression ')' statement ELSE statement
    // 	// | SWITCH '(' expression ')' statement
    // 	// ;
    // create_rule(grammar_rules, String::from("selection_statement -> IF OPENING_BRACKET expression CLOSING_BRACKET statement"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("selection_statement -> IF OPENING_BRACKET expression CLOSING_BRACKET statement ELSE statement"), treat_nonterminal_lowercase);

// // //     // ORIG 
//     create_rule(grammar_rules, String::from("expression -> assignment_expression"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("expression -> expression COMMA assignment_expression"), treat_nonterminal_lowercase);
//     // DEBUG
//     // create_rule(grammar_rules, String::from("expression -> EXPRESSION_STOP"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("expression -> VOID"), treat_nonterminal_lowercase);

// // //     // ORIG - this rule causes deep-dive with loop
// //     // create_rule(grammar_rules, String::from("assignment_expression -> unary_expression assignment_operator assignment_expression"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("assignment_expression -> conditional_expression"), treat_nonterminal_lowercase);
// // //     // 
// // //     //DEBUG  
// // //     //
// // //     //create_rule(grammar_rules, String::from("assignment_expression -> ASSIGN_STOP"), treat_nonterminal_lowercase);
// // // 
//     create_rule(grammar_rules, String::from("unary_expression -> postfix_expression"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("unary_expression -> INC_OP unary_expression"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("unary_expression -> DEC_OP unary_expression"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("unary_expression -> unary_operator cast_expression"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("unary_expression -> SIZEOF unary_expression"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("unary_expression -> SIZEOF OPENING_BRACKET type_name CLOSING_BRACKET"), treat_nonterminal_lowercase);

// // //     create_rule(grammar_rules, String::from("unary_operator -> AMPERSAND"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("unary_operator -> ASTERISK"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("unary_operator -> PLUS"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("unary_operator -> MINUS"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("unary_operator -> TILDE"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("unary_operator -> EXCLAMATION_MARK"), treat_nonterminal_lowercase);

// // //     create_rule(grammar_rules, String::from("type_name -> specifier_qualifier_list"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("specifier_qualifier_list -> type_specifier"), treat_nonterminal_lowercase);
// // //     // ORIG
// // //     create_rule(grammar_rules, String::from("cast_expression -> OPENING_BRACKET type_name CLOSING_BRACKET cast_expression"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("cast_expression -> unary_expression"), treat_nonterminal_lowercase);
// // //     // // DEBUG
// // //     // create_rule(grammar_rules, String::from("cast_expression -> CAST_STOP"), treat_nonterminal_lowercase);

// // //     // DEBUG
// // //     //create_rule(grammar_rules, String::from("postfix_expression -> END_POSTFIX"), treat_nonterminal_lowercase);
// // // 
// // //     create_rule(grammar_rules, String::from("postfix_expression -> primary_expression postfix_expression_list"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("postfix_expression -> primary_expression"), treat_nonterminal_lowercase);

// // //     create_rule(grammar_rules, String::from("postfix_expression_list -> INC_OP postfix_expression_list"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("postfix_expression_list -> INC_OP"), treat_nonterminal_lowercase);

//     // create_rule(grammar_rules, String::from("primary_expression -> IDENTIFIER"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("primary_expression -> HEX_NUMERIC"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("primary_expression -> NUMERIC"), treat_nonterminal_lowercase);

// // //     create_rule(grammar_rules, String::from("assignment_operator -> EQUALS_SIGN"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("assignment_operator -> MUL_ASSIGN"), treat_nonterminal_lowercase);

// // //     // -----------------------------------------------------------

// //     create_rule(grammar_rules, String::from("conditional_expression -> logical_or_expression QUESTION_MARK expression COLON conditional_expression"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("conditional_expression -> logical_or_expression"), treat_nonterminal_lowercase);

// //     // create_rule(grammar_rules, String::from("logical_or_expression -> logical_and_expression OR_OP logical_or_expression"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("logical_or_expression -> logical_and_expression"), treat_nonterminal_lowercase);

// //     // create_rule(grammar_rules, String::from("logical_and_expression -> inclusive_or_expression AND_OP logical_and_expression"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("logical_and_expression -> inclusive_or_expression"), treat_nonterminal_lowercase);

// //     // create_rule(grammar_rules, String::from("inclusive_or_expression -> exclusive_or_expression BIN_OR_OP inclusive_or_expression"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("inclusive_or_expression -> exclusive_or_expression"), treat_nonterminal_lowercase);

// //     // create_rule(grammar_rules, String::from("exclusive_or_expression -> and_expression CIRCUMFLEX exclusive_or_expression"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("exclusive_or_expression -> and_expression"), treat_nonterminal_lowercase);

// //     // create_rule(grammar_rules, String::from("and_expression -> equality_expression AMPERSAND and_expression"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("and_expression -> equality_expression"), treat_nonterminal_lowercase);

// //     // create_rule(grammar_rules, String::from("equality_expression -> relational_expression EQ_OP equality_expression"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("equality_expression -> relational_expression NE_OP equality_expression"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("equality_expression -> relational_expression"), treat_nonterminal_lowercase);

//     create_rule(grammar_rules, String::from("relational_expression -> shift_expression LT relational_expression"), treat_nonterminal_lowercase);
// //     // create_rule(grammar_rules, String::from("relational_expression -> shift_expression GT relational_expression"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("relational_expression -> shift_expression LTE relational_expression"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("relational_expression -> shift_expression GTE relational_expression"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("relational_expression -> shift_expression"), treat_nonterminal_lowercase);

// // //     create_rule(grammar_rules, String::from("shift_expression -> additive_expression LEFT_OP shift_expression"), treat_nonterminal_lowercase);
// // //     create_rule(grammar_rules, String::from("shift_expression -> additive_expression RIGHT_OP shift_expression"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("shift_expression -> additive_expression"), treat_nonterminal_lowercase);

// //     // causes trouble
// //     create_rule(grammar_rules, String::from("additive_expression -> multiplicative_expression PLUS additive_expression"), treat_nonterminal_lowercase);
// //     create_rule(grammar_rules, String::from("additive_expression -> multiplicative_expression MINUS additive_expression"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("additive_expression -> multiplicative_expression"), treat_nonterminal_lowercase);

// //     // create_rule(grammar_rules, String::from("multiplicative_expression -> cast_expression ASTERISK multiplicative_expression"), treat_nonterminal_lowercase);
// //     // create_rule(grammar_rules, String::from("multiplicative_expression -> cast_expression SLASH multiplicative_expression"), treat_nonterminal_lowercase);
// //     create_rule(grammar_rules, String::from("multiplicative_expression -> cast_expression PERCENT multiplicative_expression"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("multiplicative_expression -> cast_expression"), treat_nonterminal_lowercase);
// // // 

// // //     // jump_statement
// // // 	// : GOTO IDENTIFIER ';'
// // // 	// | CONTINUE ';'
// // // 	// | BREAK ';'
// // // 	// | RETURN ';'
// // // 	// | RETURN expression ';'
// // // 	// ;

// //     // ORIG
//     // create_rule(grammar_rules, String::from("jump_statement -> RETURN expression SEMICOLON"), treat_nonterminal_lowercase);
//     create_rule(grammar_rules, String::from("jump_statement -> RETURN SEMICOLON"), treat_nonterminal_lowercase);

    // the first rule defined by definition is the start rule.
    // By definition, the start rule receives the EOI symbol
    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);

    //(rule_1, start_symbol)
    (rule_1, augmented_start_symbol)
}