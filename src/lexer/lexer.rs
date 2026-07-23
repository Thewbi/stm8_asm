use std::collections::BTreeMap;

use crate::EpsilonNfa;
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
}

impl Lexer {

    pub fn new(dfa_param: EpsilonNfa::<State, RegexBuildingBlock>) -> Self {

        // let mut lexer = Lexer {
        let lexer = Lexer {
            current_state_id: dfa_param.start_state_id,
            dfa: dfa_param,
            token_string_buffer: String::new(),
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
        // grammar_state_hashmap: &BTreeMap<usize, GrammarState<String>>,
        rule_map: &BTreeMap<usize, Rule<String>>,
        debug_node_string_buffer: &mut String,
        debug_node_stack: &mut Vec::<DebugNode>) -> usize {

        // let lexer_debug = true;
        let lexer_debug = false;

        // check if there is a valid transition for the next character
        // greedily consume it and do not directly feed a half finished token to the parser
        if lexer_debug {
            println!("[LEXER.TRAP_STATE] Lookahead character is: '{}'", lookahead_character);
        }

        let mut next_state_id = self.current_state_id;

        let mut char_consumed = false;
        while !char_consumed {

            if lexer_debug {
                println!("[LEXER] State; '{}', Input: '{}', lookahead: '{}'", 
                    self.current_state_id, current_character, lookahead_character);
            }

            //
            // try to transition the large lexer DFA to produce a token for the input.
            // If the input has no valid transition, the DFA transitions into a trap state.
            //

            next_state_id = transition_dfa(&mut self.dfa, 
                self.current_state_id, &RegexBuildingBlock::CharacterLiteral(current_character));

            if lexer_debug {
                println!("[LEXER] From State: '{}', To State: '{}'", self.current_state_id, next_state_id);
            }

            //
            // Next, check where the DFA has transitioned to
            //

            if self.dfa.is_trap_state(next_state_id) {

                if lexer_debug {
                    println!("[LEXER.TRAP_STATE] Emitting '{}', Token-Id: {}, Token-Name: {}", 
                        self.token_string_buffer, 
                        self.dfa.states[&self.current_state_id].token_id, 
                        self.dfa.states[&self.current_state_id].token_name);
                    println!("");
                }

                // create a Token / Terminal
                let terminal = RuleElement::Terminal(self.dfa.states[&self.current_state_id].token_name.clone());

                if lexer_debug {
                    println!("[LEXER.TRAP_STATE] {:?} ---> {:?}", self.token_string_buffer, terminal);
                }

                match self.dfa.states[&self.current_state_id].token_id {
                    
                    NEWLINE_TOKEN_ID | WHITESPACE_TOKEN_ID => {
                        // ignore NEWLINE and WHITESPACE
                        if lexer_debug {
                            println!("[LEXER.TRAP_STATE] NOT Passing token to parser: {:?}, {:?}", self.token_string_buffer, terminal);
                        }
                    }

                    IDENTIFIER_TOKEN_ID => {

                        if lexer_debug {
                            println!("[LEXER.TRAP_STATE] Passing token to parser: {:?}, {:?}", self.token_string_buffer, terminal);
                        }

                        // turn an identifier into a TYPE_NAME if the identifier matches a user-defined type
                        
                        if parser.defined_types.contains(&self.token_string_buffer) {

                            // pass token to the lexer
                            parser.provide_input(
                                // grammar_state_hashmap,
                                rule_map,
                                step, 
                                &RuleElement::Terminal(String::from("TYPE_NAME")),
                                &self.token_string_buffer,
                                debug_node_string_buffer,
                                debug_node_stack);

                        } else {

                            // pass token to the lexer
                            parser.provide_input(
                                // grammar_state_hashmap,
                                rule_map,
                                step, 
                                &terminal,
                                &self.token_string_buffer,
                                debug_node_string_buffer,
                                debug_node_stack);

                        }
                    }

                    _ => {

                        if lexer_debug {
                            println!("[LEXER.TRAP_STATE] Passing token to parser: {:?}, {:?}", self.token_string_buffer, terminal);
                        }

                        // pass token to the lexer
                        parser.provide_input(
                            // grammar_state_hashmap,
                            rule_map,
                            step, 
                            &terminal,
                            &self.token_string_buffer,
                            debug_node_string_buffer,
                            debug_node_stack);
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

                // DEBUG
                if lexer_debug {
                    println!("[LEXER] Emitting '{}', Token-Id: {}, Token-Name: {}", 
                        self.token_string_buffer, 
                        self.dfa.states[&next_state_id].token_id, 
                        self.dfa.states[&next_state_id].token_name);
                }

                self.token_string_buffer.push(current_character);

                char_consumed = true;

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
        // grammar_state_hashmap: &BTreeMap<usize, GrammarState<String>>,
        rule_map: &BTreeMap<usize, Rule<String>>,
        rule_element: &RuleElement<String>,
        debug_node_string_buffer: &mut String,
        debug_node_stack: &mut Vec::<DebugNode>) {

        parser.provide_input(
            //&grammar_state_hashmap,
            &rule_map,
            step, 
            rule_element,
            &self.token_string_buffer, 
            debug_node_string_buffer, 
            debug_node_stack);
    }

}