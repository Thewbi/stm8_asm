use crate::example_grammars::common::create_rule;
use crate::Rule;
use crate::RuleElement;

pub fn produce_grammar_c_full_if_else_4(grammar_rules: &mut Vec::<Rule<String>>) -> (Rule<String>, crate::RuleElement<String>) {

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
    // let augmented_start_symbol = RuleElement::NonTerminal(String::from("function_definition'"));
    // let augmented_start_symbol = RuleElement::NonTerminal(String::from("declaration'"));
    let augmented_start_symbol = RuleElement::NonTerminal(String::from("translation_unit'"));

    let treat_nonterminal_lowercase: bool = true;

    //
    // add augmentation start rule
    //

    // create_rule(grammar_rules, String::from("additive_expression' -> additive_expression"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("expression_statement' -> expression_statement"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("expression' -> expression"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("statement' -> statement"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("declaration_or_statement_list' -> declaration_or_statement_list"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("compound_statement' -> compound_statement"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("selection_statement' -> selection_statement"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("function_definition' -> function_definition"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("declaration' -> declaration"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("translation_unit' -> translation_unit"), treat_nonterminal_lowercase);

    //
    // add all rules here
    //

    // primary_expression
    //     : IDENTIFIER
    //     | CONSTANT
    //     | STRING_LITERAL
    //     | '(' expression ')'
    //     ;
    create_rule(grammar_rules, String::from("primary_expression -> IDENTIFIER"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("primary_expression -> HEX_NUMERIC"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("primary_expression -> NUMERIC"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("primary_expression -> FLOAT_NUMERIC"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("primary_expression -> STRING_LITERAL"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("primary_expression -> OPENING_BRACKET expression CLOSING_BRACKET"), treat_nonterminal_lowercase);
    
    // postfix_expression
    //     : primary_expression
    //     | postfix_expression '[' expression ']'
    //     | postfix_expression '(' ')'
    //     | postfix_expression '(' argument_expression_list ')'
    //     | postfix_expression '.' IDENTIFIER
    //     | postfix_expression PTR_OP IDENTIFIER
    //     | postfix_expression INC_OP
    //     | postfix_expression DEC_OP
    //     ;
    create_rule(grammar_rules, String::from("postfix_expression -> primary_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("postfix_expression -> postfix_expression OPENING_ANGULAR_BRACKET expression CLOSING_ANGULAR_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("postfix_expression -> postfix_expression OPENING_BRACKET CLOSING_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("postfix_expression -> postfix_expression OPENING_BRACKET argument_expression_list CLOSING_BRACKET "), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("postfix_expression -> postfix_expression DOT IDENTIFIER"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("postfix_expression -> postfix_expression PTR_OP IDENTIFIER"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("postfix_expression -> postfix_expression INC_OP"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("postfix_expression -> postfix_expression DEC_OP"), treat_nonterminal_lowercase);

    // argument_expression_list
    //     : assignment_expression
    //     | argument_expression_list ',' assignment_expression
    //     ;
    create_rule(grammar_rules, String::from("argument_expression_list -> assignment_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("argument_expression_list -> argument_expression_list COMMA assignment_expression"), treat_nonterminal_lowercase);

    // unary_expression
    //     : postfix_expression
    //     | INC_OP unary_expression
    //     | DEC_OP unary_expression
    //     | unary_operator cast_expression
    //     | SIZEOF unary_expression
    //     | SIZEOF '(' type_name ')'
    //     ;
    create_rule(grammar_rules, String::from("unary_expression -> postfix_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("unary_expression -> INC_OP unary_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("unary_expression -> DEC_OP unary_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("unary_expression -> unary_operator cast_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("unary_expression -> SIZEOF unary_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("unary_expression -> SIZEOF OPENING_BRACKET type_name CLOSING_BRACKET"), treat_nonterminal_lowercase);

    // unary_operator
    //     : '&'
    //     | '*'
    //     | '+'
    //     | '-'
    //     | '~'
    //     | '!'
    //     ;
    create_rule(grammar_rules, String::from("unary_operator -> AMPERSAND"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("unary_operator -> ASTERISK"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("unary_operator -> PLUS"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("unary_operator -> MINUS"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("unary_operator -> TILDE"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("unary_operator -> EXCLAMATION_MARK"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("cast_expression -> OPENING_BRACKET type_name CLOSING_BRACKET cast_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("cast_expression -> unary_expression"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("multiplicative_expression -> cast_expression ASTERISK multiplicative_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("multiplicative_expression -> cast_expression SLASH multiplicative_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("multiplicative_expression -> cast_expression PERCENT multiplicative_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("multiplicative_expression -> cast_expression"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("additive_expression -> multiplicative_expression PLUS additive_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("additive_expression -> multiplicative_expression MINUS additive_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("additive_expression -> multiplicative_expression"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("shift_expression -> additive_expression LEFT_OP shift_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("shift_expression -> additive_expression RIGHT_OP shift_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("shift_expression -> additive_expression"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("relational_expression -> shift_expression LT relational_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("relational_expression -> shift_expression GT relational_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("relational_expression -> shift_expression LE_OP relational_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("relational_expression -> shift_expression GE_OP relational_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("relational_expression -> shift_expression"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("equality_expression -> relational_expression EQ_OP equality_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("equality_expression -> relational_expression NE_OP equality_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("equality_expression -> relational_expression"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("and_expression -> equality_expression AMPERSAND and_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("and_expression -> equality_expression"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("exclusive_or_expression -> and_expression CIRCUMFLEX exclusive_or_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("exclusive_or_expression -> and_expression"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("inclusive_or_expression -> exclusive_or_expression OR inclusive_or_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("inclusive_or_expression -> exclusive_or_expression"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("logical_and_expression -> inclusive_or_expression AND_OP logical_and_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("logical_and_expression -> inclusive_or_expression"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("logical_or_expression -> logical_and_expression OR_OP logical_or_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("logical_or_expression -> logical_and_expression"), treat_nonterminal_lowercase);
    
    // conditional_expression
	//      : logical_or_expression
	//      | logical_or_expression '?' expression ':' conditional_expression
	//      ;
    create_rule(grammar_rules, String::from("conditional_expression -> logical_or_expression QUESTION_MARK expression COLON conditional_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("conditional_expression -> logical_or_expression"), treat_nonterminal_lowercase);

    // assignment_expression
    //     : conditional_expression
    //     | unary_expression assignment_operator assignment_expression
    //     ;
    create_rule(grammar_rules, String::from("assignment_expression -> unary_expression assignment_operator assignment_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("assignment_expression -> conditional_expression"), treat_nonterminal_lowercase);

    // assignment_operator
    //     : '='
    //     | MUL_ASSIGN
    //     | DIV_ASSIGN
    //     | MOD_ASSIGN
    //     | ADD_ASSIGN
    //     | SUB_ASSIGN
    //     | LEFT_ASSIGN
    //     | RIGHT_ASSIGN
    //     | AND_ASSIGN
    //     | XOR_ASSIGN
    //     | OR_ASSIGN
    //     ;
    create_rule(grammar_rules, String::from("assignment_operator -> EQUALS_SIGN"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("assignment_operator -> MUL_ASSIGN"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("assignment_operator -> DIV_ASSIGN"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("assignment_operator -> MOD_ASSIGN"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("assignment_operator -> ADD_ASSIGN"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("assignment_operator -> SUB_ASSIGN"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("assignment_operator -> LEFT_ASSIGN"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("assignment_operator -> RIGHT_ASSIGN"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("assignment_operator -> AND_ASSIGN"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("assignment_operator -> XOR_ASSIGN"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("assignment_operator -> OR_ASSIGN"), treat_nonterminal_lowercase);
    
    // expression
    //     : assignment_expression
    //     | expression ',' assignment_expression
    //     ;
    create_rule(grammar_rules, String::from("expression -> assignment_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("expression -> expression COMMA assignment_expression"), treat_nonterminal_lowercase);

    // constant_expression
	//      : conditional_expression
	//      ;
    create_rule(grammar_rules, String::from("constant_expression -> conditional_expression"), treat_nonterminal_lowercase);

    // declaration
	//      : declaration_specifiers ';'
	//      | declaration_specifiers init_declarator_list ';'
	//      ;
    create_rule(grammar_rules, String::from("declaration -> declaration_specifiers init_declarator_list SEMICOLON"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("declaration -> declaration_specifiers SEMICOLON"), treat_nonterminal_lowercase);

    // declaration_specifiers
	//      : storage_class_specifier
	//      | storage_class_specifier declaration_specifiers
	//      | type_specifier
	//      | type_specifier declaration_specifiers
	//      | type_qualifier
	//      | type_qualifier declaration_specifiers
	//      ;
    create_rule(grammar_rules, String::from("declaration_specifiers -> storage_class_specifier declaration_specifiers"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("declaration_specifiers -> storage_class_specifier"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("declaration_specifiers -> type_specifier declaration_specifiers"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("declaration_specifiers -> type_specifier"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("declaration_specifiers -> type_qualifier declaration_specifiers"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("declaration_specifiers -> type_qualifier"), treat_nonterminal_lowercase);

    // init_declarator_list
	//      : init_declarator
	//      | init_declarator_list ',' init_declarator
	//      ;
    create_rule(grammar_rules, String::from("init_declarator_list -> init_declarator COMMA init_declarator_list"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("init_declarator_list -> init_declarator"), treat_nonterminal_lowercase);

    // init_declarator
	//      : declarator
	//      | declarator '=' initializer
	//      ;
    create_rule(grammar_rules, String::from("init_declarator -> declarator EQUALS_SIGN initializer"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("init_declarator -> declarator"), treat_nonterminal_lowercase);

    // storage_class_specifier
    //     : TYPEDEF
    //     | EXTERN
    //     | STATIC
    //     | AUTO
    //     | REGISTER
    //     ;
    create_rule(grammar_rules, String::from("storage_class_specifier -> TYPEDEF"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("storage_class_specifier -> EXTERN"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("storage_class_specifier -> STATIC"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("storage_class_specifier -> AUTO"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("storage_class_specifier -> REGISTER"), treat_nonterminal_lowercase);

    // type_specifier
    //     : VOID
    //     | CHAR
    //     | SHORT
    //     | INT
    //     | LONG
    //     | FLOAT
    //     | DOUBLE
    //     | SIGNED
    //     | UNSIGNED
    //     | struct_or_union_specifier
    //     | enum_specifier
    //     | TYPE_NAME
    //     ;
    create_rule(grammar_rules, String::from("type_specifier -> VOID"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("type_specifier -> CHAR"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("type_specifier -> SHORT"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("type_specifier -> INT"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("type_specifier -> LONG"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("type_specifier -> FLOAT"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("type_specifier -> DOUBLE"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("type_specifier -> SIGNED"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("type_specifier -> UNSIGNED"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("type_specifier -> struct_or_union_specifier"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("type_specifier -> enum_specifier"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("type_specifier -> TYPE_NAME"), treat_nonterminal_lowercase);

    // struct_or_union_specifier
    //     : struct_or_union IDENTIFIER '{' struct_declaration_list '}'
    //     | struct_or_union '{' struct_declaration_list '}'
    //     | struct_or_union IDENTIFIER
    //     ;
    create_rule(grammar_rules, String::from("struct_or_union_specifier -> struct_or_union IDENTIFIER OPENING_CURLY_BRACKET struct_declaration_list CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("struct_or_union_specifier -> struct_or_union OPENING_CURLY_BRACKET struct_declaration_list CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("struct_or_union_specifier -> struct_or_union IDENTIFIER"), treat_nonterminal_lowercase);

    // struct_or_union
    //     : STRUCT
    //     | UNION
    //     ;
    create_rule(grammar_rules, String::from("struct_or_union -> STRUCT"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("struct_or_union -> UNION"), treat_nonterminal_lowercase);

    // struct_declaration_list
    //     : struct_declaration
    //     | struct_declaration_list struct_declaration
    //     ;
    create_rule(grammar_rules, String::from("struct_declaration_list -> struct_declaration_list struct_declaration"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("struct_declaration_list -> struct_declaration"), treat_nonterminal_lowercase);

    // struct_declaration
    //     : specifier_qualifier_list struct_declarator_list ';'
    //     ;
    create_rule(grammar_rules, String::from("struct_declaration -> specifier_qualifier_list struct_declarator_list SEMICOLON"), treat_nonterminal_lowercase);

    // specifier_qualifier_list
    //     : type_specifier specifier_qualifier_list
    //     | type_specifier
    //     | type_qualifier specifier_qualifier_list
    //     | type_qualifier
    //     ;
    create_rule(grammar_rules, String::from("specifier_qualifier_list -> type_specifier specifier_qualifier_list"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("specifier_qualifier_list -> type_specifier"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("specifier_qualifier_list -> type_qualifier specifier_qualifier_list"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("specifier_qualifier_list -> type_qualifier"), treat_nonterminal_lowercase);

    // struct_declarator_list
    //     : struct_declarator
    //     | struct_declarator_list ',' struct_declarator
    //     ;
    create_rule(grammar_rules, String::from("struct_declarator_list -> struct_declarator"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("struct_declarator_list -> struct_declarator_list COMMA struct_declarator"), treat_nonterminal_lowercase);

    // struct_declarator
    //     : declarator
    //     | ':' constant_expression
    //     | declarator ':' constant_expression
    //     ;
    create_rule(grammar_rules, String::from("struct_declarator -> declarator"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("struct_declarator -> COLON constant_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("struct_declarator -> declarator COLON constant_expression"), treat_nonterminal_lowercase);

    // enum_specifier
    //     : ENUM '{' enumerator_list '}'
    //     | ENUM IDENTIFIER '{' enumerator_list '}'
    //     | ENUM IDENTIFIER
    //     ;
    create_rule(grammar_rules, String::from("enum_specifier -> ENUM OPENING_CURLY_BRACKET enumerator_list CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("enum_specifier -> ENUM IDENTIFIER OPENING_CURLY_BRACKET enumerator_list CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("enum_specifier -> ENUM IDENTIFIER"), treat_nonterminal_lowercase);

    // enumerator_list
    //     : enumerator
    //     | enumerator_list ',' enumerator
    //     ;
    create_rule(grammar_rules, String::from("enumerator_list -> enumerator"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("enumerator_list -> enumerator_list COMMA enumerator"), treat_nonterminal_lowercase);

    // enumerator
    //     : IDENTIFIER
    //     | IDENTIFIER '=' constant_expression
    //     ;
    create_rule(grammar_rules, String::from("enumerator -> IDENTIFIER"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("enumerator -> IDENTIFIER EQUALS_SIGN constant_expression"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("type_qualifier -> CONST"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("type_qualifier -> VOLATILE"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("declarator -> pointer direct_declarator"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("declarator -> direct_declarator"), treat_nonterminal_lowercase);

    // direct_declarator
	// : IDENTIFIER
	// | '(' declarator ')'
	// | direct_declarator '[' constant_expression ']'
	// | direct_declarator '[' ']'
	// | direct_declarator '(' parameter_type_list ')'
	// | direct_declarator '(' identifier_list ')'
	// | direct_declarator '(' ')'
	// ;
    create_rule(grammar_rules, String::from("direct_declarator -> IDENTIFIER"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("direct_declarator -> OPENING_BRACKET declarator CLOSING_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("direct_declarator -> direct_declarator OPENING_ANGULAR_BRACKET constant_expression CLOSING_ANGULAR_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("direct_declarator -> direct_declarator OPENING_ANGULAR_BRACKET CLOSING_ANGULAR_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("direct_declarator -> direct_declarator OPENING_BRACKET parameter_type_list CLOSING_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("direct_declarator -> direct_declarator OPENING_BRACKET identifier_list CLOSING_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("direct_declarator -> direct_declarator OPENING_BRACKET CLOSING_BRACKET"), treat_nonterminal_lowercase);

    // pointer
    //     : '*'
    //     | '*' type_qualifier_list
    //     | '*' pointer
    //     | '*' type_qualifier_list pointer
    //     ;
    create_rule(grammar_rules, String::from("pointer -> ASTERISK"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("pointer -> ASTERISK type_qualifier_list"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("pointer -> ASTERISK pointer"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("pointer -> ASTERISK type_qualifier_list pointer"), treat_nonterminal_lowercase);

    // type_qualifier_list
    //     : type_qualifier
    //     | type_qualifier_list type_qualifier
    //     ;
    create_rule(grammar_rules, String::from("type_qualifier_list -> type_qualifier"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("type_qualifier_list -> type_qualifier_list type_qualifier"), treat_nonterminal_lowercase);

    // parameter_type_list
    //     : parameter_list
    //     | parameter_list ',' ELLIPSIS
    //     ;
    create_rule(grammar_rules, String::from("parameter_type_list -> parameter_list"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("parameter_type_list -> parameter_list COMMA ELLIPSIS"), treat_nonterminal_lowercase);

    // parameter_list
    //     : parameter_declaration
    //     | parameter_list ',' parameter_declaration
    //     ;
    create_rule(grammar_rules, String::from("parameter_list -> parameter_declaration"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("parameter_list -> parameter_list COMMA parameter_declaration"), treat_nonterminal_lowercase);

    // parameter_declaration
    //     : declaration_specifiers declarator
    //     | declaration_specifiers abstract_declarator
    //     | declaration_specifiers
    //     ;
    create_rule(grammar_rules, String::from("parameter_declaration -> declaration_specifiers declarator"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("parameter_declaration -> declaration_specifiers abstract_declarator"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("parameter_declaration -> declaration_specifiers"), treat_nonterminal_lowercase);

    // identifier_list
    //     : IDENTIFIER
    //     | identifier_list ',' IDENTIFIER
    //     ;
    create_rule(grammar_rules, String::from("identifier_list -> IDENTIFIER"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("identifier_list -> identifier_list COMMA IDENTIFIER"), treat_nonterminal_lowercase);

    // type_name
    //     : specifier_qualifier_list
    //     | specifier_qualifier_list abstract_declarator
    //     ;
    create_rule(grammar_rules, String::from("type_name -> specifier_qualifier_list"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("type_name -> specifier_qualifier_list abstract_declarator"), treat_nonterminal_lowercase);

    // abstract_declarator
    //     : pointer
    //     | direct_abstract_declarator
    //     | pointer direct_abstract_declarator
    //     ;
    create_rule(grammar_rules, String::from("abstract_declarator -> pointer"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("abstract_declarator -> direct_abstract_declarator"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("abstract_declarator -> pointer direct_abstract_declarator"), treat_nonterminal_lowercase);


    // direct_abstract_declarator
    //     : '(' abstract_declarator ')'
    //     | '[' ']'
    //     | '[' constant_expression ']'
    //     | direct_abstract_declarator '[' ']'
    //     | direct_abstract_declarator '[' constant_expression ']'
    //     | '(' ')'
    //     | '(' parameter_type_list ')'
    //     | direct_abstract_declarator '(' ')'
    //     | direct_abstract_declarator '(' parameter_type_list ')'
    //     ;
    create_rule(grammar_rules, String::from("direct_abstract_declarator -> OPENING_BRACKET abstract_declarator CLOSING_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("direct_abstract_declarator -> OPENING_ANGULAR_BRACKET CLOSING_ANGULAR_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("direct_abstract_declarator -> OPENING_ANGULAR_BRACKET constant_expression CLOSING_ANGULAR_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("direct_abstract_declarator -> direct_abstract_declarator OPENING_ANGULAR_BRACKET CLOSING_ANGULAR_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("direct_abstract_declarator -> direct_abstract_declarator OPENING_ANGULAR_BRACKET constant_expression CLOSING_ANGULAR_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("direct_abstract_declarator -> OPENING_BRACKET CLOSING_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("direct_abstract_declarator -> OPENING_BRACKET parameter_type_list CLOSING_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("direct_abstract_declarator -> direct_abstract_declarator OPENING_BRACKET CLOSING_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("direct_abstract_declarator -> direct_abstract_declarator OPENING_BRACKET parameter_type_list CLOSING_BRACKET"), treat_nonterminal_lowercase);

    // initializer
	//      : assignment_expression
	//      | '{' initializer_list '}'
	//      | '{' initializer_list ',' '}'
	//      ;
    create_rule(grammar_rules, String::from("initializer -> assignment_expression"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("initializer -> OPENING_CURLY_BRACKET initializer_list CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("initializer -> OPENING_CURLY_BRACKET initializer_list COMMA CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);

    // initializer_list
	//      : initializer
	//      | initializer_list ',' initializer
	//      ;
    create_rule(grammar_rules, String::from("initializer_list -> initializer"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("initializer_list -> initializer COMMA initializer_list"), treat_nonterminal_lowercase);
    
    // statement
	//      : labeled_statement
	//      | compound_statement
	//      | expression_statement
	//      | selection_statement
	//      | iteration_statement
	//      | jump_statement
	//      ;
    create_rule(grammar_rules, String::from("statement -> labeled_statement"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("statement -> compound_statement"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("statement -> expression_statement"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("statement -> selection_statement"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("statement -> iteration_statement"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("statement -> jump_statement"), treat_nonterminal_lowercase);

    // labeled_statement
	//      : IDENTIFIER ':' statement
	//      | CASE constant_expression ':' statement
	//      | DEFAULT ':' statement
	//      ;
    create_rule(grammar_rules, String::from("labeled_statement -> IDENTIFIER COLON statement"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("labeled_statement -> CASE constant_expression COLON statement"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("labeled_statement -> DEFAULT COLON statement"), treat_nonterminal_lowercase);

    // compound_statement
	//      : '{' '}'
	//      | '{' statement_list '}'
	//      | '{' declaration_list '}'
	//      | '{' declaration_list statement_list '}'
	//      ;
    // create_rule(grammar_rules, String::from("compound_statement -> OPENING_CURLY_BRACKET statement_list CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("compound_statement -> OPENING_CURLY_BRACKET CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("compound_statement -> OPENING_CURLY_BRACKET statement CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);
    // These two rules are all you need!
    create_rule(grammar_rules, String::from("compound_statement -> OPENING_CURLY_BRACKET declaration_or_statement_list CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("compound_statement -> OPENING_CURLY_BRACKET CLOSING_CURLY_BRACKET"), treat_nonterminal_lowercase);

    // Leave this disabled as long as you have declaration_or_statement_list
    //
    // declaration_list
    //     : declaration
    //     | declaration_list declaration
    //     ;
    //
    // statement_list
	//      : statement
	//      | statement_list statement
	//      ;
    //
    // create_rule(grammar_rules, String::from("statement_list -> statement statement_list"), treat_nonterminal_lowercase);
    // create_rule(grammar_rules, String::from("statement_list -> statement"), treat_nonterminal_lowercase);

    create_rule(grammar_rules, String::from("declaration_or_statement_list -> declaration declaration_or_statement_list"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("declaration_or_statement_list -> declaration"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("declaration_or_statement_list -> statement declaration_or_statement_list"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("declaration_or_statement_list -> statement"), treat_nonterminal_lowercase);

    // expression_statement
	//      : ';'
	//      | expression ';'
	//      ;
    create_rule(grammar_rules, String::from("expression_statement -> expression SEMICOLON"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("expression_statement -> SEMICOLON"), treat_nonterminal_lowercase);

    // selection_statement
    //      : IF '(' expression ')' statement
    //      | IF '(' expression ')' statement ELSE statement
    //      | SWITCH '(' expression ')' statement
    //      ;
    create_rule(grammar_rules, String::from("selection_statement -> IF OPENING_BRACKET expression CLOSING_BRACKET statement"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("selection_statement -> IF OPENING_BRACKET expression CLOSING_BRACKET statement ELSE statement"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("selection_statement -> SWITCH OPENING_BRACKET expression CLOSING_BRACKET statement"), treat_nonterminal_lowercase);

    // iteration_statement
	//      : WHILE '(' expression ')' statement
	//      | DO statement WHILE '(' expression ')' ';'
	//      | FOR '(' expression_statement expression_statement ')' statement
	//      | FOR '(' expression_statement expression_statement expression ')' statement
    //      ;
    create_rule(grammar_rules, String::from("iteration_statement -> WHILE OPENING_BRACKET expression CLOSING_BRACKET statement"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("iteration_statement -> DO statement WHILE OPENING_BRACKET expression CLOSING_BRACKET SEMICOLON"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("iteration_statement -> FOR OPENING_BRACKET expression_statement expression_statement CLOSING_BRACKET statement"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("iteration_statement -> FOR OPENING_BRACKET expression_statement expression_statement expression CLOSING_BRACKET statement"), treat_nonterminal_lowercase);

    // jump_statement
	//      : GOTO IDENTIFIER ';'
	//      | CONTINUE ';'
	//      | BREAK ';'
	//      | RETURN ';'
	//      | RETURN expression ';'
	//      ;
    create_rule(grammar_rules, String::from("jump_statement -> GOTO IDENTIFIER"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("jump_statement -> CONTINUE"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("jump_statement -> BREAK"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("jump_statement -> RETURN expression SEMICOLON"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("jump_statement -> RETURN SEMICOLON"), treat_nonterminal_lowercase);

    // translation_unit
	//      : external_declaration
	//      | translation_unit external_declaration
	//      ;
    create_rule(grammar_rules, String::from("translation_unit -> translation_unit external_declaration"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("translation_unit -> external_declaration"), treat_nonterminal_lowercase);

    // external_declaration
	//      : function_definition
	//      | declaration
	//      ;
    create_rule(grammar_rules, String::from("external_declaration -> function_definition"), treat_nonterminal_lowercase);
    create_rule(grammar_rules, String::from("external_declaration -> declaration"), treat_nonterminal_lowercase);

    // function_definition
    //     : declaration_specifiers declarator declaration_list compound_statement
    //     | declaration_specifiers declarator compound_statement
    //     | declarator declaration_list compound_statement
    //     | declarator compound_statement
    //     ;
    create_rule(grammar_rules, String::from("function_definition -> declaration_specifiers declarator compound_statement"), treat_nonterminal_lowercase);

    // the first rule defined by definition is the start rule.
    // By definition, the start rule receives the EOI symbol
    let mut rule_1 = grammar_rules.first().unwrap().clone();
    rule_1.lookahead.push(RuleElement::Closure);

    (rule_1, augmented_start_symbol)
}