use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::str::Chars;
use std::vec::IntoIter;

use crate::EpsilonNfa;
use crate::c_ast::ast_node::AstNode;
use crate::example_lexers::c_lexer::IDENTIFIER_TOKEN_ID;
use crate::example_lexers::c_lexer::NEWLINE_TOKEN_ID;
use crate::example_lexers::c_lexer::WHITESPACE_TOKEN_ID;
use crate::parser::parser::ParserTrait;
use crate::regex::enfa::transition_dfa;
use crate::State;
use crate::RegexBuildingBlock;
use crate::Parser;
use crate::DebugNode;
use crate::Rule;
use crate::RuleElement;

use crate::parser::grammar_state::GrammarState;

// This class is a driver rather than real implementation of lexing and parsing.
// Lexing is performed by a DFA. Parsing is performed by the parser struct.
//
// This Lexer contains DFA which accepts all token regular expressions.
// It also contains the parser.
//
// The Lexer feeds input characters to the DFA.
// Should the DFA enter a TrapState (which means a token has been accepted),
// then a token variable is created from the DFA's inner state.
// The created token variable is then forwarded to the parser.
//
// The parser keeps reducing the grammar rules until the start symbol is reduced
// (which means the input applicatio is syntactically sound) or it will refuse input
// by panicing!
//
// The DFA is implemented in regex/enfa.rs as a NFA which is actually a DFA. NFA's can be
// used to represent DFA by just not having more than a single target state for a
// alphabet symbol. You will not find an explicit DFA structure in the code base.
//
// The parser is implemented in parser/parser.rs
//
// The README.md documents the lexer and parser generation and implementation.
pub struct Lexer {
    pub string_data_iterator: IntoIter<char>,
    pub dfa: EpsilonNfa::<State, RegexBuildingBlock>,
    pub current_state_id: usize,
    pub token_string_buffer: String,
    pub lexer_debug: bool,
    pub lexer_token_debug: bool,
    buffered_character_option: Option<char>,
    done: bool,
}

impl Lexer {

    pub fn new(
        string_data_param: String,
        dfa_param: EpsilonNfa::<State, RegexBuildingBlock>,
        lexer_debug_param: bool,
        lexer_token_debug_param: bool)
    -> Self
    {
        // // DEBUG
        // println!("{}", string_data_param);
        // if string_data_param.ends_with('\n') {
        //     println!("test");
        // }

        // https://users.rust-lang.org/t/having-an-iterator-as-a-struct-field/86570/2
        // Iterators in rust always have a lifetime parameter and it is hard
        // to store an iterator as a member because the iterator might outlife the
        // memory it iterates over which is unsafe and rust will not allow it.
        // One way to keep an iterator as a member is to also own the data iterated over!
        // Therefor convert the string into a vector of characters and make that
        // vector a part of the struct aka. owned.
        let string_data = string_data_param.chars().collect::<Vec<_>>();
        let lexer = Lexer {
            string_data_iterator: string_data.into_iter(),
            current_state_id: dfa_param.start_state_id,
            dfa: dfa_param,
            token_string_buffer: String::new(),
            lexer_debug: lexer_debug_param,
            lexer_token_debug: lexer_token_debug_param,
            buffered_character_option: None,
            done: false,
        };

        lexer
    }

    pub fn consume_character(&mut self,
        current_character: char,
        step: &mut usize,
        parser: &mut Parser::<String>,
        rule_map: &BTreeMap<usize, Rule<String>>,
        debug_node_string_buffer: &mut String,
        debug_node_stack: &mut Vec::<DebugNode>,
        file: &String,
        line: usize,
        node_map: &mut Box<HashMap::<usize, AstNode>>
    ) -> usize {

        // TODO: write line and file into the token before passing it to the parser
        // so that the parser has line and file information

        // check if there is a valid transition for the next character
        // greedily consume it and do not directly feed a half finished token to the parser

        // // DEBUG
        // if self.lexer_debug {
        //     println!("[LEXER.TRAP_STATE] Lookahead character is: '{}'", lookahead_character);
        // }

        // // DEBUG
        // println!("[LEXER.consume_character()] Character is: '{}'", current_character);

        let mut next_state_id = self.current_state_id;

        let mut char_consumed = false;
        while !char_consumed {

            // // DEBUG
            // if self.lexer_debug {
            //     println!("[LEXER] State; '{}', Input: '{}', lookahead: '{}'",
            //         self.current_state_id, current_character, lookahead_character);
            // }

            //
            // try to transition the large lexer DFA to produce a token for the input.
            //
            // If the input has no valid transition, the DFA transitions into a trap state.
            // This means that the lexer has identified a token.
            //

            next_state_id = transition_dfa(&mut self.dfa,
                self.current_state_id, &RegexBuildingBlock::CharacterLiteral(current_character));

            // DEBUG
            if self.lexer_debug {
                println!("[LEXER] From State: '{}', To State: '{}'", self.current_state_id, next_state_id);
            }

            //
            // Next, check where the DFA has transitioned to
            //

            if self.dfa.is_trap_state(next_state_id) {

                // DEBUG
                if self.lexer_debug {
                    println!("[LEXER.TRAP_STATE] Emitting '{}', Token-Id: {}, Token-Name: {} | File: {:?}, Line: {:?}",
                        self.token_string_buffer,
                        self.dfa.states[&self.current_state_id].token_id,
                        self.dfa.states[&self.current_state_id].token_name,
                        file,
                        line);
                    println!("");
                }

                // create a Token / Terminal
                let terminal = RuleElement::Terminal(self.dfa.states[&self.current_state_id].token_name.clone());

                // DEBUG - this outputs the string and the token generated from the string
                // This is a good starting point for debugging
                if self.lexer_token_debug {
                    println!("[LEXER.TRAP_STATE] {:?} ---> {:?} | File: {:?}, Line: {:?}",
                        self.token_string_buffer,
                        terminal,
                        file,
                        line);
                }

                match self.dfa.states[&self.current_state_id].token_id {

                    WHITESPACE_TOKEN_ID => {
                        // ignore NEWLINE and WHITESPACE

                        // DEBUG
                        if self.lexer_debug {
                            println!("[LEXER.TRAP_STATE] NOT Passing token to parser: {:?}, {:?}", self.token_string_buffer, terminal);
                        }
                    }

                    NEWLINE_TOKEN_ID => {
                        // ignore NEWLINE and WHITESPACE

                        // DEBUG
                        if self.lexer_debug {
                            println!("[LEXER.TRAP_STATE] NOT Passing token to parser: {:?}, {:?}", self.token_string_buffer, terminal);
                        }
                    }

                    IDENTIFIER_TOKEN_ID => {

                        // DEBUG
                        if self.lexer_debug {
                            println!("[LEXER.TRAP_STATE] Passing token to parser: {:?}, {:?}", self.token_string_buffer, terminal);
                        }

                        // handle typedef
                        // turn an identifier into a TYPE_NAME if the identifier matches a user-defined type

                        if parser.defined_types.contains(&self.token_string_buffer) {

                            // pass token to the parser
                            parser.provide_input(
                                rule_map,
                                step,
                                &RuleElement::Terminal(String::from("TYPE_NAME")),
                                &self.token_string_buffer,
                                debug_node_string_buffer,
                                debug_node_stack,
                                node_map
                            );

                        } else {

                            // pass token to the parser
                            parser.provide_input(
                                rule_map,
                                step,
                                &terminal,
                                &self.token_string_buffer,
                                debug_node_string_buffer,
                                debug_node_stack,
                                node_map
                            );

                        }
                    }

                    _ => {

                        // DEBUG
                        if self.lexer_debug {
                            println!("[LEXER.TRAP_STATE] Passing token to parser: {:?}, {:?}", self.token_string_buffer, terminal);
                        }

                        if rule_map.len() > 0 {
                            // pass token to the parser
                            parser.provide_input(
                                rule_map,
                                step,
                                &terminal,
                                &self.token_string_buffer,
                                debug_node_string_buffer,
                                debug_node_stack,
                                node_map
                            );
                        } else {
                            println!("[WARN] No rules supplied! Not calling parser!");
                            *step = *step + 1;
                        }
                    }
                }

                // reset the lexer's DFA back to the start state and
                // try to accept the symbol again which was read from input already
                char_consumed = false;
                self.current_state_id = self.dfa.start_state_id;
                self.token_string_buffer.clear();

            } else if self.dfa.is_end_state(next_state_id) {

                //
                // if the state is normal or an end state, just consume the character
                //

                self.token_string_buffer.push(current_character);

                char_consumed = true;

                // DEBUG
                if self.lexer_debug {
                    println!("[LEXER] Consumed '{}', Token-Id: {}, Token-Name: {} | File: {:?}, Line: {:?}",
                        self.token_string_buffer,
                        self.dfa.states[&next_state_id].token_id,
                        self.dfa.states[&next_state_id].token_name,
                        file,
                        line
                    );
                }

            } else {

                //
                // if the state is normal or an end state, just consume the character
                //

                // DEBUG
                // println!("STATE '{}' NOT END STATE!", current_state_id);

                self.token_string_buffer.push(current_character);

                char_consumed = true;

                // DEBUG
                if self.lexer_debug {
                    println!("[LEXER] Consumed '{}', Token-Id: {}, Token-Name: {} | File: {:?}, Line: {:?}",
                        self.token_string_buffer,
                        self.dfa.states[&next_state_id].token_id,
                        self.dfa.states[&next_state_id].token_name,
                        file,
                        line
                    );
                }
            }
        }

        self.current_state_id = next_state_id;

        next_state_id
    }

    pub fn parser_provide_input(&mut self,
        parser: &mut Parser::<String>,
        step: &mut usize,
        rule_map: &BTreeMap<usize, Rule<String>>,
        rule_element: &RuleElement<String>,
        debug_node_string_buffer: &mut String,
        debug_node_stack: &mut Vec::<DebugNode>,
        node_map: &mut Box<HashMap::<usize, AstNode>>
    ) {
        // pass token to the parser
        parser.provide_input(
            &rule_map,
            step,
            rule_element,
            &self.token_string_buffer,
            debug_node_string_buffer,
            debug_node_stack,
            node_map
        );
    }

    // fn next(&mut self) -> Option<Token> {

    //     match self.string_data_iterator {
    //         Option::None => {
    //             self.string_data_iterator = Some(self.string_data.chars());
    //         }
    //         _ => {
    //         }
    //     }

    //     panic!();
    // }
}

pub struct Token {
    pub token_id: usize,
    pub text: String,
    pub terminal: RuleElement<String>,
    pub weight: usize,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token").field("token_id", &self.token_id).field("text", &self.text).field("terminal", &self.terminal).finish()
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        // write!(f, "{{\n").expect("Write failed!");
        // write!(f, "  TokenId: '{}',\n", self.token_id).expect("Write failed!");
        // write!(f, "  Text: '{}',\n", self.text).expect("Write failed!");
        // write!(f, "  Type: {:?}\n", self.terminal).expect("Write failed!");
        // write!(f, "}}").expect("Write failed!");

        write!(f, "{}", self.text).expect("Write failed!");

        Ok(())
    }
}

impl Iterator for Lexer {

    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {

        while !self.done {

            let current_character;

            // if a character is buffered, use that character instead
            // of pulling in the next character from the file
            match self.buffered_character_option {

                Some(buffered_character) => {
                    // // DEBUG
                    // println!("buffered_character: {:?}", buffered_character);
                    // if buffered_character == '\n' {
                    //     println!("newline");
                    // }

                    current_character = buffered_character;
                    self.buffered_character_option = None;
                }

                _ => {
                    // pull in the next character from the file
                    let current_character_option = self.string_data_iterator.next();
                    match current_character_option {
                        Some(current_character_value) => {
                            // // DEBUG
                            // println!("current_character_value: {:?}", current_character_value);
                            // if current_character_value == '\n' {
                            //     println!("newline");
                            // }

                            current_character = current_character_value;
                        }
                        _ => {
                            // push out the remaining data if the buffer has data left
                            if self.token_string_buffer.len() > 0 {
                                // add a dummy character which will terminate the last token
                                current_character = '\n';
                                self.done = true;
                            } else {
                                self.done = true;
                                continue;
                            }
                        }
                    }
                }
            }

            // // DEBUG
            // println!("current_character: {:?}", current_character);
            // if current_character == '\n' {
            //     println!("newline");
            // }

            let mut next_state_id = self.current_state_id;

            let mut char_consumed = false;
            while !char_consumed {

                // // DEBUG
                // if self.lexer_debug {
                //     println!("[LEXER] State; '{}', Input: '{}', lookahead: '{}'",
                //         self.current_state_id, current_character, lookahead_character);
                // }

                //
                // try to transition the large lexer DFA to produce a token for the input.
                //
                // If the input has no valid transition, the DFA transitions into a trap state.
                // This means that the lexer has identified a token.
                //

                next_state_id = transition_dfa(&mut self.dfa,
                    self.current_state_id,
                    &RegexBuildingBlock::CharacterLiteral(current_character));

                // DEBUG
                if self.lexer_debug {
                    println!("[LEXER] From State: '{}', To State: '{}'", self.current_state_id, next_state_id);
                }

                //
                // Next, check where the DFA has transitioned to
                //

                if self.dfa.is_trap_state(next_state_id) {

                    // DEBUG
                    if self.lexer_debug {
                        println!("[LEXER.TRAP_STATE] Emitting '{}', Token-Id: {},
                            Token-Name: {}", // | File: {:?}, Line: {:?}",
                            self.token_string_buffer,
                            self.dfa.states[&self.current_state_id].token_id,
                            self.dfa.states[&self.current_state_id].token_name,
                            // file,
                            // line
                        );
                        println!("");
                    }

                    // create a Token / Terminal
                    let terminal = RuleElement::Terminal(
                        self.dfa.states[&self.current_state_id].token_name.clone());

                    // DEBUG - this outputs the string and the token generated from the string
                    // This is a good starting point for debugging
                    if self.lexer_token_debug {
                        println!("[LEXER.TRAP_STATE] {:?} ---> {:?}", // | File: {:?}, Line: {:?}",
                            self.token_string_buffer,
                            terminal,
                            // file,
                            // line
                        );
                    }

                    self.buffered_character_option = Some(current_character);

                    // create token
                    let token = Token {
                        token_id: self.dfa.states[&self.current_state_id].token_id,
                        text: self.token_string_buffer.clone(),
                        terminal: terminal.clone(),
                        weight: 0usize,
                    };

                    char_consumed = false;

                    // reset the lexer's DFA back to the start state and
                    // try to accept the symbol again which was read from input already
                    self.current_state_id = self.dfa.start_state_id;
                    self.token_string_buffer.clear();

                    // return token
                    return Some(token);

                } else if self.dfa.is_end_state(next_state_id) {

                    //
                    // if the state is normal or an end state, just consume the character
                    //

                    self.token_string_buffer.push(current_character);

                    char_consumed = true;

                    // DEBUG
                    if self.lexer_debug {
                        println!("[LEXER] Consumed '{}', Token-Id: {}, Token-Name: {}",
                            // | File: {:?}, Line: {:?}",
                            self.token_string_buffer,
                            self.dfa.states[&next_state_id].token_id,
                            self.dfa.states[&next_state_id].token_name,
                            // file,
                            // line
                        );
                    }

                } else {

                    //
                    // if the state is normal or an end state, just consume the character
                    //

                    // DEBUG
                    // println!("STATE '{}' NOT END STATE!", current_state_id);

                    self.token_string_buffer.push(current_character);

                    char_consumed = true;

                    // DEBUG
                    if self.lexer_debug {
                        println!("[LEXER] Consumed '{}', Token-Id: {}, Token-Name: {}",
                            // | File: {:?}, Line: {:?}",
                            self.token_string_buffer,
                            self.dfa.states[&next_state_id].token_id,
                            self.dfa.states[&next_state_id].token_name,
                            // file,
                            // line
                        );
                    }
                }
            }

            self.current_state_id = next_state_id;
        }
        return None;
    }
}





/*
                    match self.dfa.states[&self.current_state_id].token_id {

                        NEWLINE_TOKEN_ID | WHITESPACE_TOKEN_ID => {
                            // ignore NEWLINE and WHITESPACE

                            // DEBUG
                            if self.lexer_debug {
                                println!("[LEXER.TRAP_STATE] NOT Passing token to parser: {:?}, {:?}", self.token_string_buffer, terminal);
                            }
                        }

                        // IDENTIFIER_TOKEN_ID => {

                        //     // DEBUG
                        //     if self.lexer_debug {
                        //         println!("[LEXER.TRAP_STATE] Passing token to parser: {:?}, {:?}", self.token_string_buffer, terminal);
                        //     }

                        //     // handle typedef
                        //     // turn an identifier into a TYPE_NAME if the identifier matches a user-defined type

                        //     if parser.defined_types.contains(&self.token_string_buffer) {

                        //         // pass token to the parser
                        //         parser.provide_input(
                        //             rule_map,
                        //             step,
                        //             &RuleElement::Terminal(String::from("TYPE_NAME")),
                        //             &self.token_string_buffer,
                        //             debug_node_string_buffer,
                        //             debug_node_stack,
                        //             node_map
                        //         );

                        //     } else {

                        //         // pass token to the parser
                        //         parser.provide_input(
                        //             rule_map,
                        //             step,
                        //             &terminal,
                        //             &self.token_string_buffer,
                        //             debug_node_string_buffer,
                        //             debug_node_stack,
                        //             node_map
                        //         );

                        //     }

                        // }

                        _ => {

                            // DEBUG
                            if self.lexer_debug {
                                println!("[LEXER.TRAP_STATE] Passing token to parser: {:?}, {:?}", self.token_string_buffer, terminal);
                            }

                            // panic!();

                            if rule_map.len() > 0 {
                                // pass token to the parser
                                parser.provide_input(
                                    rule_map,
                                    step,
                                    &terminal,
                                    &self.token_string_buffer,
                                    debug_node_string_buffer,
                                    debug_node_stack,
                                    node_map
                                );
                            } else {
                                println!("[WARN] No rules supplied! Not calling parser!");
                                *step = *step + 1;
                            }

                        }
                    }
*/