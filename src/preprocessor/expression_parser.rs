#![allow(
dead_code,
unused_imports,
unused_must_use,
unused_variables,
unused_assignments,
non_snake_case,
non_camel_case_types
)]

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;

use crate::example_lexers::preprocessor_lexer::PP_AND_OP_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_GE_OP_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_EQ_OP_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_CLOSING_BRACKET_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_OPENING_BRACKET_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_OR_OP_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_SEMICOLON_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_COMMA_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_COLON_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_DEFINE_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_WHITESPACE_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_IDENTIFIER_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_NUMERIC_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_NEWLINE_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_STRING_LITERAL_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_QUESTION_MARK_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_HASHTAG_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_BACKSLASH_TOKEN_ID;

use crate::regex::arena::recurse_arena;
use crate::regex::arena::{Arena, NodeId, recurse_arena_dot};

use crate::lexer::lexer::{Lexer, Token};

enum Operation {
    REPARENT,
    ADD_RIGHT,
    ADD_LEFT,
    CLOSE_BRACKETS,
    NO_ACTION
}

// the expression parser is used to parse well-formed text to an AST for evaluation
// Well-formed text appears in all PPI that need to be evaluated such as #if, #elif
// Note that the #define PPI only has well-formed text in the macro_interface part
// but the macro_definition can be free-form text of any kind and does not have to
// be well-formed
pub struct ExpressionParser {
    pub arena: Arena<Token>,
    pub ptr_node_id: NodeId,
}

impl ExpressionParser {

    pub fn new() -> ExpressionParser {
        ExpressionParser {
            arena: Arena::new(),
            ptr_node_id: NodeId { index: usize::MAX },
        }
    }

    pub fn process_token(&mut self, mut token: Token) {

        // DEBUG
        // println!("[ExpressionParser::process_token()] Token: {}", token);

        //
        // STEP 1 - Assign weight to token
        //

        match token.token_id {
            PP_WHITESPACE_TOKEN_ID => {
                token.weight = 0;
            }
            PP_NEWLINE_TOKEN_ID => {
                token.weight = 0;
            }
            PP_IDENTIFIER_TOKEN_ID => {
                // token.weight = 11111;
                token.weight = 99999;
            }
            PP_AND_OP_TOKEN_ID => { // AND_OP
                token.weight = 180;
            }
            PP_OR_OP_TOKEN_ID => { // OR_OP
                token.weight = 180;
            }
            PP_GE_OP_TOKEN_ID => {
                token.weight = 180;
            }
            PP_EQ_OP_TOKEN_ID => {
                token.weight = 179;
            }
            PP_SEMICOLON_TOKEN_ID => {
                token.weight = 182;
            }
            PP_COMMA_TOKEN_ID => { // COMMA
                token.weight = 182;
            }
            PP_COLON_TOKEN_ID => {
                token.weight = 182;
            }
            28 => { // OPENING_BRACKET
                token.weight = 11111;
                //token.weight = 99999;
            }
            29 => { // CLOSING_BRACKET
                token.weight = 11111;
                //token.weight = 99999;
            }
            34 => { // EXCLAMATION_MARK !
                token.weight = 430;
            }
            36 => { // MINUS
                token.weight = 410;
            }
            37 => { // PLUS
                token.weight = 400;
            }
            38 => { // ASTERISK
                token.weight = 420;
            }
            PP_QUESTION_MARK_TOKEN_ID => {
                token.weight = 420;
            }
            PP_HASHTAG_TOKEN_ID => {
                token.weight = 420;
            }
            PP_BACKSLASH_TOKEN_ID => {
                token.weight = 420;
            }
            100 => { // DEFINED (Preprocessor Function (PPF))
                token.weight = 499;
            }
            200 => { // DEFINE (Preprocessor Instruction (PPI))
                token.weight = 500;
            }
            PP_STRING_LITERAL_TOKEN_ID => {
                token.weight = 999;
            }
            PP_NUMERIC_TOKEN_ID => {
                token.weight = 998;
            }
            _ => {
                panic!("Cannot assign weight to token.token_id: {} token.text: {}", token.token_id, token.text);
            }
        }

        //
        // Step 2 - Insert token into the AST
        //

        // check src\example_lexers\preprocessor_lexer.rs for the token ids
        match token.token_id {
            //
            // WHITESPACE and NEWLINE is ignored
            //
            PP_WHITESPACE_TOKEN_ID => {
            }
            PP_NEWLINE_TOKEN_ID => {
            }
            PP_DEFINE_TOKEN_ID => {
                // DEFINE
                // feed the rest of the line into the #DEFINE handler
            }
            _ => {
                // println!("Token: {:?}", token);

                if self.arena.is_empty() {

                    self.ptr_node_id.index = self.arena.new_node(token).index;

                } else {

                    let ( operation, insert_node_id ) = self.insert_into_pp_ast(&self.ptr_node_id, &token);
                    match operation {

                        Operation::REPARENT => {

                            // TODO: assumption, reparent at root!
                            if self.arena.has_parent(&insert_node_id) {

                                // create a new node for the inserted token
                                let new_node_id = self.arena.new_node(token);
                                let parent_node_id = self.arena.nodes[insert_node_id.index].parent.unwrap();

                                // TODO: Assumes right child
                                // TODO: check if left or right!

                                // will also update the parent link
                                self.arena.insert_right(&parent_node_id, new_node_id);

                                // copy parent pointer from child
                                //arena.nodes[insert_node_id.index].parent = arena.nodes[insert_node_id.index].parent;

                                // will also update the parent link
                                self.arena.insert_left(&new_node_id, insert_node_id);

                                // change reparented node's parent
                                // arena.nodes[insert_node_id.index].parent = Some(new_node_id.clone());

                            } else {
                                // new parent
                                self.ptr_node_id.index = self.arena.new_node(token).index;
                                self.arena.insert_left(&self.ptr_node_id, insert_node_id);
                            }
                        }

                        Operation::ADD_LEFT => {
                            self.arena.add_left(&insert_node_id, token);
                        }

                        Operation::ADD_RIGHT => {
                            let token_id = token.token_id;
                            let new_node_id = self.arena.add_right(&insert_node_id, token);

                            // TODO: if the current token is a '(', then
                            // set the current ptr to this new node!
                            if token_id == 28 { // OPENING_BRACKET
                                self.ptr_node_id.index = new_node_id.index;
                            }
                        }

                        Operation::CLOSE_BRACKETS => {
                            // println!("CLOSE_BRACKETS");

                            let mut current_node = &mut self.arena.nodes[self.ptr_node_id.index];
                            current_node.data.token_id = 80usize; // 80 for close bracke-pair "()"
                            current_node.data.text = String::from("()");

                            // walk up the tree to the next open bracket or to the root node
                            loop {
                                if let Some(parent_token) = current_node.parent {
                                    // the node has a parent

                                    // update pointer
                                    self.ptr_node_id.index = parent_token.index;

                                    // retrieve the new pointed node
                                    current_node = &mut self.arena.nodes[self.ptr_node_id.index];

                                    // if the new node is an opening bracket, abort
                                    if current_node.data.token_id == 28 { // OPENING_BRACKET
                                        break;
                                    }

                                } else {
                                    // the node has no parent
                                    break;
                                }
                            }
                        }

                        Operation::NO_ACTION => {
                        }
                    }
                }

                // DEBUG - print the arena to the console after a new node has been inserted
                //recurse_arena(&arena, &ptr_node_id);
            }
        }
    }

    fn insert_into_pp_ast(&self,
        node_id: &NodeId,
        token: &Token)
        -> ( Operation, NodeId )
    {
        loop {

            let payload: &Token = self.arena.get_payload(&node_id);
            match payload.token_id {

                PP_OPENING_BRACKET_TOKEN_ID => { // OPENING_BRACKET

                    // ) received
                    if token.token_id == PP_CLOSING_BRACKET_TOKEN_ID {
                        return ( Operation::CLOSE_BRACKETS, node_id.clone() );
                    }

                    // sends every other node to the right

                    // sink into the right node
                    let has_right = self.arena.has_right(&node_id);
                    if has_right {
                        let node_id = self.arena.get_right_id(&node_id).unwrap();
                        return self.insert_into_pp_ast(node_id, token);
                    } else {
                        return ( Operation::ADD_RIGHT, node_id.clone() );
                    }
                }

                // 200 - DEFINE
                _ => {

                    if payload.weight > token.weight {

                        // existing root node is heavier

                        return ( Operation::REPARENT, node_id.clone() );

                    } else if payload.weight < token.weight {

                        // existing node is lighter.
                        // The new node sinks into the existing node

                        // TODO: sink into the left node???

                        // normally a node sinks into the left node, unless the
                        // parent node is a defined or a '(' they immediately
                        // send nodes into the right child!

                        // sink into the right node
                        let has_right = self.arena.has_right(&node_id);
                        if has_right {
                            let node_id = self.arena.get_right_id(&node_id).unwrap();
                            return self.insert_into_pp_ast(node_id, token);
                        } else {
                            return ( Operation::ADD_RIGHT, node_id.clone() );
                        }

                    } else {

                        // same weight

                        let existing_node_id = &self.arena.nodes[node_id.index];
                        match existing_node_id.data.token_id {

                            28 => { // OPENING_BRACKET
                                if token.token_id == 29 { // CLOSING_BRACKET
                                    // println!("test");

                                    // Do not insert a new node but change the current node
                                    // existing_node_id.data.text = String::from("()");

                                    return ( Operation::CLOSE_BRACKETS, node_id.clone() );

                                    // TODO: move the ptr back up the tree until the last
                                    // open bracket is found or until the root node is found

                                    // TODO: close the open bracket because it has been matched
                                    // by a closing bracket
                                }
                            }
                            _ => {
                                // reparent
                                return ( Operation::REPARENT, node_id.clone() );
                            }
                        }
                    }
                }

                _ => {
                    panic!();
                }
            }
        }
    }

    pub fn print_console(&self) {
        recurse_arena(&self.arena, &self.ptr_node_id);
    }

    // prints the internal area as a dot graphviz tree
    // TODO: extract this to a general method an place this method next to the arena maybe?
    pub fn print_dot(&self) {

        let mut ast_string_buffer = String::from("");

        // serialize the AST into .dot graphviz format
        ast_string_buffer.push_str("digraph {\n");
        recurse_arena_dot(&self.arena, &self.ptr_node_id, &mut ast_string_buffer);
        ast_string_buffer.push_str("}");

        // 1. Create or overwrite the file
        let file = File::create("preprocessor_tree.dot").expect("Create file failed!");

        // 2. Wrap the file in a BufWriter
        let mut writer = BufWriter::new(file);

        // 3. Write data
        write!(writer, "{}", ast_string_buffer);

        // 4. Explicitly flush the remaining data to disk
        writer.flush().expect("flush failed!");
    }

    pub fn reset(&mut self) {
        self.arena.reset();
        self.ptr_node_id = NodeId { index: 0 };
    }
}