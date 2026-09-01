use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::EpsilonNfa;
use crate::c_ast::ast_node::AstNode;
use crate::regex::enfa::transition_dfa;
use crate::State;
use crate::RegexBuildingBlock;
use crate::Parser;
use crate::DebugNode;
use crate::Rule;
use crate::RuleElement;

use crate::parser::grammar_state::GrammarState;

pub const WHITESPACE_TOKEN_ID: usize = 46;
pub const NEWLINE_TOKEN_ID: usize = 47;
pub const IDENTIFIER_TOKEN_ID: usize = 500;

pub struct Lexer {
    pub dfa: EpsilonNfa::<State, RegexBuildingBlock>,
    pub current_state_id: usize,
    pub token_string_buffer: String,
    pub lexer_debug: bool,
    pub lexer_token_debug: bool,
}

impl Lexer {

    pub fn new(dfa_param: EpsilonNfa::<State, RegexBuildingBlock>,
        lexer_debug_param: bool, lexer_token_debug_param: bool) -> Self {

        let lexer = Lexer {
            current_state_id: dfa_param.start_state_id,
            dfa: dfa_param,
            token_string_buffer: String::new(),
            lexer_debug: lexer_debug_param,
            lexer_token_debug: lexer_token_debug_param,
        };

        lexer
    }

    // TODO: the lookahead character is not used at all!
    // Remove it! It makes the parser loop more complicated
    pub fn consume_character(&mut self,
        current_character: char,
        lookahead_character: char,
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
        if self.lexer_debug {
            println!("[LEXER.TRAP_STATE] Lookahead character is: '{}'", lookahead_character);
        }

        let mut next_state_id = self.current_state_id;

        let mut char_consumed = false;
        while !char_consumed {

            // DEBUG
            if self.lexer_debug {
                println!("[LEXER] State; '{}', Input: '{}', lookahead: '{}'",
                    self.current_state_id, current_character, lookahead_character);
            }

            //
            // try to transition the large lexer DFA to produce a token for the input.
            // If the input has no valid transition, the DFA transitions into a trap state.
            //

            next_state_id = transition_dfa(&mut self.dfa,
                self.current_state_id, &RegexBuildingBlock::CharacterLiteral(current_character));

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

                    NEWLINE_TOKEN_ID | WHITESPACE_TOKEN_ID => {
                        // ignore NEWLINE and WHITESPACE
                        if self.lexer_debug {
                            println!("[LEXER.TRAP_STATE] NOT Passing token to parser: {:?}, {:?}", self.token_string_buffer, terminal);
                        }
                    }

                    IDENTIFIER_TOKEN_ID => {

                        if self.lexer_debug {
                            println!("[LEXER.TRAP_STATE] Passing token to parser: {:?}, {:?}", self.token_string_buffer, terminal);
                        }

                        // turn an identifier into a TYPE_NAME if the identifier matches a user-defined type

                        if parser.defined_types.contains(&self.token_string_buffer) {

                            // pass token to the lexer
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

                            // pass token to the lexer
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
                            // pass token to the lexer
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

                self.token_string_buffer.push(current_character);

                char_consumed = true;

                //
                // if the state is normal or an end state, just consume the character
                //

                // DEBUG
                if self.lexer_debug {
                    println!("[LEXER] Emitting '{}', Token-Id: {}, Token-Name: {} | File: {:?}, Line: {:?}",
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

}