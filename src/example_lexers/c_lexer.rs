use std::collections::HashSet;

use crate::regex::enfa::Fragment;
use crate::regex::enfa::FragmentStack;
use crate::regex::enfa::enfa_copy;
use crate::regex::enfa::enfa_to_dfa;

use crate::EpsilonNfa;
use crate::State;
use crate::RegexBuildingBlock;
use crate::InfixPostfixConverter;
use crate::regex::enfa::recurse_postfix_build_fragment_stack;
use crate::Input;
use crate::example_lexers::common::add_token_definition;

use crate::IDENTIFIER_TOKEN_ID;
use crate::WHITESPACE_TOKEN_ID;
use crate::NEWLINE_TOKEN_ID;

pub fn produce_c_lexer() -> EpsilonNfa::<State, RegexBuildingBlock> {

    //
    // Phase 0 - 
    //

    let mut combined_fragment = Fragment::new(RegexBuildingBlock::Or);

    //
    // Pre-Build alphabet
    //

    // complete alphabet has to be known in advance
    let mut alphabet = HashSet::<RegexBuildingBlock>::new();

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('a'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('A'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('b'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('B'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('c'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('C'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('d'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('D'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('e'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('E'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('f'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('F'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('g'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('G'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('h'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('H'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('i'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('I'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('j'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('J'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('k'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('K'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('l'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('L'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('m'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('M'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('n'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('N'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('o'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('O'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('p'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('P'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('q'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('Q'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('r'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('R'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('s'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('S'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('t'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('T'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('u'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('U'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('v'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('V'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('w'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('W'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('x'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('X'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('y'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('Y'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('z'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('Z'));

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('0'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('1'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('2'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('3'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('4'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('5'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('6'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('7'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('8'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('9'));

    alphabet.insert(RegexBuildingBlock::CharacterLiteral(' ')); // WHITESPACE, SPACE

    alphabet.insert(RegexBuildingBlock::CharacterLiteral('_'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('<'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('>'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('{'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('}'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('('));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral(')'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('['));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral(']'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('+'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('-'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('*'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('/'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('%'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('&'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('^')); // Used in regex as NOT operator
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('|'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('!'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral(';'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral(','));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('~'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('?'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('.'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('='));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('"'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral(':'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('\\'));

    // "\n" | "\r\n" | "\r"
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('\n'));
    alphabet.insert(RegexBuildingBlock::CharacterLiteral('\r'));

    //
    // Phase 1 - build all regexes
    //

    //
    // identifier

    // provide a regex in infix notation and let the converter produce a postfix notation
    // The result is stored within the state of the converter instance, this is why the converter can be reset
    let mut converter = InfixPostfixConverter::new();
    //converter.infix_to_postfix("(_|a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z)(_|a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z)+");
    converter.infix_to_postfix("(_|a|A|b|B|c|C|d|D|e|E|f|F|g|G|h|H|i|I|j|J|k|K|l|L|m|M|n|N|o|O|p|P|q|Q|r|R|s|S|t|T|u|U|v|V|w|W|x|X|y|Y|z|Z)|(_|a|A|b|B|c|C|d|D|e|E|f|F|g|G|h|H|i|I|j|J|k|K|l|L|m|M|n|N|o|O|p|P|q|Q|r|R|s|S|t|T|u|U|v|V|w|W|x|X|y|Y|z|Z)(_|a|A|b|B|c|C|d|D|e|E|f|F|g|G|h|H|i|I|j|J|k|K|l|L|m|M|n|N|o|O|p|P|q|Q|r|R|s|S|t|T|u|U|v|V|w|W|x|X|y|Y|z|Z|0|1|2|3|4|5|6|7|8|9)+");
    
    // next, from the regex-items in the postfix notation, construct a eNFA
    // This function will go through the infix character by character and extend a eNFA as it goes.
    // Once done, the eNFA will accept all input described by the regex infix notation
    let mut fragment_stack_identifier = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_identifier, &mut alphabet);
    // reset the converter
    converter.reset();
    // the top fragment on the fragment stack contains the root of the eNFA
    let mut fragment_identifier = fragment_stack_identifier.stack.pop().unwrap();
    // assign a token id to eNFA so it will assign that token id to all token it accepts
    fragment_identifier.enfa.states.get_mut(&fragment_identifier.end_id).unwrap().token_id = IDENTIFIER_TOKEN_ID;
    fragment_identifier.enfa.states.get_mut(&fragment_identifier.end_id).unwrap().token_name = String::from("IDENTIFIER");
    // insert into LEXER
    let (start_id_identifier, end_id_identifier) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_identifier.enfa, fragment_identifier.end_id);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_identifier);
    
    // DEBUG dump the graph to .dot format for viewing using https://dreampuf.github.io/GraphvizOnline
    //enfa_to_dot_directed_graph(&mut fragment_identifier.enfa, "fragment_identifier_automaton.dot");

    //
    // Float Numeric - {D}*"."{D}+({E})?{FS}?
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "(0|1|2|3|4|5|6|7|8|9)*.(0|1|2|3|4|5|6|7|8|9)+((e|E)(\\+|\\-)?(0|1|2|3|4|5|6|7|8|9)+)?(f|F|l|L)?", "FLOAT_NUMERIC", 601);

    //
    // IS			(u|U|l|L)*
    // H			[a-fA-F0-9]
    // Hex Numeric - 0[xX]{H}+{IS}?
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "0(x|X)(0|1|2|3|4|5|6|7|8|9|a|A|b|B|c|C|d|D|e|E|f|F)+(u|U|l|L)?", "HEX_NUMERIC", 602);

    //
    // numeric (token-id: 600)
    converter.infix_to_postfix("(0|1|2|3|4|5|6|7|8|9)+");
    let mut fragment_stack_numeric = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_numeric, &mut alphabet);
    converter.reset();
    let mut fragment_numeric = fragment_stack_numeric.stack.pop().unwrap();
    fragment_numeric.enfa.states.get_mut(&fragment_numeric.end_id).unwrap().token_id = 600;
    fragment_numeric.enfa.states.get_mut(&fragment_numeric.end_id).unwrap().token_name = String::from("NUMERIC");
    // insert into LEXER
    let (start_id_numeric, end_id_numeric) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_numeric.enfa, fragment_numeric.end_id);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_numeric);
    
    //
    // string_literal (token-id: 610)
    //converter.infix_to_postfix("\"(a|A|b|B|c|C|d|D)+\"");
    //converter.infix_to_postfix("\"^[\",\"]+\"");
    // converter.infix_to_postfix("^a");
    // converter.infix_to_postfix("^(a)");
    // converter.infix_to_postfix("^(\")");
    // converter.infix_to_postfix("//^(a)");
    // converter.infix_to_postfix("\"^(\")\"");
    converter.infix_to_postfix("\"^\"");
    let mut fragment_stack_string_literal = FragmentStack::new();
    recurse_postfix_build_fragment_stack(&converter.arena, &converter.root_node_id, &mut fragment_stack_string_literal, &mut alphabet);
    converter.reset();
    let mut fragment_string_literal = fragment_stack_string_literal.stack.pop().unwrap();
    fragment_string_literal.enfa.states.get_mut(&fragment_string_literal.end_id).unwrap().token_id = 610;
    fragment_string_literal.enfa.states.get_mut(&fragment_string_literal.end_id).unwrap().token_name = String::from("STRING_LITERAL");
    // // DEBUG
    // enfa_to_dot_directed_graph(&mut fragment_string_literal.enfa, "string_literal_enfa_automaton.dot");
    // insert into LEXER
    let (start_id_string_literal, end_id_string_literal) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_string_literal.enfa, fragment_string_literal.end_id);
    combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_string_literal);

    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_2);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_3);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_4);
    // // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_5);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_6);
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_7);

    // DEBUG - print to dot file format for debugging with https://dreampuf.github.io/GraphvizOnline
    //enfa_to_dot_directed_graph(&mut combined_fragment.enfa, "enfa_automaton.dot");

    //
    // define operators
    //

    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "...", "ELLIPSIS", 0);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ">>=", "RIGHT_ASSIGN", 1);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "<<=", "LEFT_ASSIGN", 2);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\+=", "ADD_ASSIGN", 3); // used in Regex as Repeat(1, std::usize::MAX)
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\-=", "SUB_ASSIGN", 4); // used in Regex to build character classes
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\*=", "MUL_ASSIGN", 5); // used in Regex as Repeat(0, std::usize::MAX)
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "/=", "DIV_ASSIGN", 6);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "%=", "MOD_ASSIGN", 7);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "&=", "AND_ASSIGN", 8);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\^=", "XOR_ASSIGN", 9); // used in Regex as NEGATION operator
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\|=", "OR_ASSIGN", 10); // used in Regex as OR operator
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ">>", "RIGHT_OP", 11);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "<<", "LEFT_OP", 12);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\+\\+", "INC_OP", 13); // used in Regex as Repeat(1, std::usize::MAX)
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\-\\-", "DEC_OP", 14); // used in Regex to build character classes
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\->", "PTR_OP", 15);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "&&", "AND_OP", 16);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\|\\|", "OR_OP", 17); // used in Regex as OR operator
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "<=", "LE_OP", 18);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ">=", "GE_OP", 19);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "==", "EQ_OP", 20);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\!=", "NE_OP", 21); // ???
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ";", "SEMICOLON", 22);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\{", "OPENING_CURLY_BRACKET", 23);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\}", "CLOSING_CURLY_BRACKET", 24);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ",", "COMMA", 25);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ":", "COLON", 26);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "=", "EQUALS_SIGN", 27);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\(", "OPENING_BRACKET", 28); // used in Regex to build blocks
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\)", "CLOSING_BRACKET", 29); // used in Regex to build blocks
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\[", "OPENING_ANGULAR_BRACKET", 30); // used in Regex to build character classes
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\]", "CLOSING_ANGULAR_BRACKET", 31); // used in Regex to build character classes
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ".", "DOT", 32);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "&", "AMPERSAND", 33);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\!", "EXCLAMATION_MARK", 34); // ???
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "~", "TILDE", 35);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\-", "MINUS", 36);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\+", "PLUS", 37); // used in Regex as Repeat(1, std::usize::MAX)
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\*", "ASTERISK", 38); // used in Regex as Repeat(0, std::usize::MAX)
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "/", "SLASH", 39);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "%", "PERCENT", 40);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "<", "LT", 41);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, ">", "GT", 42);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\^", "CIRCUMFLEX", 43); // used in Regex as NEGATION operator
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\|", "OR", 44); // used in Regex as OR operator
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\\?", "QUESTION_MARK", 45); // used in Regex as Repeat(0, 1)

    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, " ", "WHITESPACE", WHITESPACE_TOKEN_ID);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "\n|\r\n|\r", "NEWLINE", NEWLINE_TOKEN_ID);

    // //
    // // Whitespace
    // // ' ' (toke-id: 46)
    // let mut fragment_stack_whitespace = FragmentStack::new();
    // add_character_literal(&mut fragment_stack_whitespace, RegexBuildingBlock::CharacterLiteral(' '), &mut alphabet);
    // // the top fragment on the fragment stack contains the root of the eNFA
    // let mut fragment_whitespace = fragment_stack_whitespace.stack.pop().unwrap();
    // fragment_whitespace.enfa.states.get_mut(&fragment_whitespace.end_id).unwrap().token_id = WHITESPACE_TOKEN_ID;
    // fragment_whitespace.enfa.states.get_mut(&fragment_whitespace.end_id).unwrap().token_name = String::from("WHITESPACE");
    // // Add to lexer
    // let (start_id_whitespace, end_id_whitespace) = enfa_copy(&mut combined_fragment.enfa, &mut fragment_whitespace.enfa, fragment_whitespace.end_id);
    // // add epsilon transitions to all the individual keyword eNFAs
    // combined_fragment.enfa.add_transition(combined_fragment.start_id, Input::Epsilon, start_id_whitespace);

    //
    // define keywords last so they have precedence over identifiers!
    //

    // auto        break       case        char 
    // const       continue    default     do 
    // double      else        enum        extern 
    // float       for         goto        if 
    // int         long        register    return 
    // short       signed      sizeof      static 
    // struct      switch      typedef     union 
    // unsigned    void        volatile    while

    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "auto", "AUTO", 100);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "break", "BREAK", 101);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "case", "CASE", 102);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "char", "CHAR", 103);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "const", "CONST", 104);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "continue", "CONTINUE", 105); // continue  // 105  
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "default", "DEFAULT", 106); // default     
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "do", "DO", 107); // do  
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "double", "DOUBLE", 108); // double      
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "else", "ELSE", 109);        
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "enum", "ENUM", 110); // enum      // 110  
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "extern", "EXTERN", 111); // extern 
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "float", "FLOAT", 112); // float       
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "for", "FOR", 113); // for         
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "goto", "GOTO", 114); // goto        
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "if", "IF", 115);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "int", "INT", 116);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "long", "LONG", 117);  // long        
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "register", "REGISTER", 118); // register   
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "return", "RETURN", 119);
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "short", "SHORT", 120); // short   // 120    
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "signed", "SIGNED", 121); // signed      
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "sizeof", "SIZEOF", 122); // sizeof      
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "static", "STATIC", 123); // static 
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "struct", "STRUCT", 124); // struct      
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "switch", "SWITCH", 125); // switch      // 125
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "typedef", "TYPEDEF", 126); // typedef     
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "union", "UNION", 127); // union 
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "unsigned", "UNSIGNED", 128); // unsigned    
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "void", "VOID", 129);    
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "volatile", "VOLATILE", 130); // volatile    
    add_token_definition(&mut converter, &mut combined_fragment, &mut alphabet, "while", "WHILE", 131); // while

    //
    // Phase 3 - Convert eNFA to DFA
    //

    let dfa = enfa_to_dfa(&mut combined_fragment.enfa, &mut alphabet);

    // DEBUG - print to dot file format for debugging with https://dreampuf.github.io/GraphvizOnline
    // enfa_to_dot_directed_graph(&mut dfa, "dfa_automaton.dot");

    dfa
}