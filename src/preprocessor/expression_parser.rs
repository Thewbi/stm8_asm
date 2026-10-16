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
use crate::example_lexers::preprocessor_lexer::PP_LT_OP_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_NE_OP_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_OPENING_BRACKET_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_CLOSING_BRACKET_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_CLOSED_BRACKET_PAIR_TOKEN_ID;
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
use crate::example_lexers::preprocessor_lexer::PP_PRAGMA_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_ELLIPSIS_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_GREATER_THAN_TOKEN_ID;
use crate::example_lexers::preprocessor_lexer::PP_CONCATENATION_TOKEN_ID;

use crate::regex::arena::recurse_arena;
use crate::regex::arena::{Arena, NodeId, recurse_arena_dot};

use crate::lexer::lexer::Token;

enum Operation {
    REPARENT,
    ADD_RIGHT,
    ADD_LEFT,
    CLOSE_BRACKETS,
    CONCATENATE,
    CLOSE_BRACKETS_UPWARDS,
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
    pub ignore_input_due_to_single_line_comment: bool,
    bracket_balance: usize,
}

impl ExpressionParser {

    pub fn new() -> ExpressionParser {
        ExpressionParser {
            arena: Arena::new(),
            ptr_node_id: NodeId { index: usize::MAX },
            ignore_input_due_to_single_line_comment: false,
            bracket_balance: 0,
        }
    }

    pub fn is_tree_balanced(&mut self) -> bool {
        return self.bracket_balance == 0
    }

    pub fn process_token(&mut self, mut token: Token) {

        // DEBUG
        // println!("[ExpressionParser::process_token()] Token: {}", token);

        if self.ignore_input_due_to_single_line_comment {
            return;
        }

        //
        // STEP 1 - Assign weight to token
        //

        match token.token_id {

            // whitespace
            PP_WHITESPACE_TOKEN_ID => {
                token.weight = 0;
            }
            PP_NEWLINE_TOKEN_ID => {
                token.weight = 0;
                self.ignore_input_due_to_single_line_comment = false;
            }

            // string literal / identifier
            PP_IDENTIFIER_TOKEN_ID => {
                token.weight = 99999;
            }
            PP_STRING_LITERAL_TOKEN_ID => {
                token.weight = 999;
            }
            PP_NUMERIC_TOKEN_ID => {
                token.weight = 998;
            }

            // logical operators
            PP_AND_OP_TOKEN_ID => {
                token.weight = 180;
            }
            PP_OR_OP_TOKEN_ID => {
                token.weight = 180;
            }
            PP_GE_OP_TOKEN_ID => {
                token.weight = 180;
            }
            PP_GREATER_THAN_TOKEN_ID => {
                token.weight = 180;
            }
            PP_EQ_OP_TOKEN_ID => {
                token.weight = 179;
            }
            PP_NE_OP_TOKEN_ID => {
                token.weight = 190;
            }
            PP_LT_OP_TOKEN_ID => {
                token.weight = 190;
            }

            PP_SEMICOLON_TOKEN_ID => {
                token.weight = 182;
            }
            PP_COMMA_TOKEN_ID => {
                token.weight = 182;
            }
            PP_COLON_TOKEN_ID => {
                token.weight = 182;
            }

            // Brackets
            PP_OPENING_BRACKET_TOKEN_ID => {
                token.weight = 11111;
            }
            PP_CLOSING_BRACKET_TOKEN_ID => {
                token.weight = 11111;
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

            PP_ELLIPSIS_TOKEN_ID => {
                token.weight = 997;
            }

            _ => {
                panic!("Cannot assign weight to token.token_id: {} token.text: {}, file: {}, line: {}", token.token_id, token.text, token.file, token.line);
            }
        }

        //
        // Step 2 - Insert token into the AST
        //

        // check src\example_lexers\preprocessor_lexer.rs for the token ids
        match token.token_id {
            //
            // WHITESPACE, NEWLINE are ignored
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

                let token_text = token.text.clone();

                if token_text == "(" {
                    self.bracket_balance = self.bracket_balance + 1;
                    // println!("[bracket_balance]: {:?}", self.bracket_balance);
                }
                if token.text == ")" {
                    self.bracket_balance = self.bracket_balance - 1;
                    // println!("[bracket_balance]: {:?}", self.bracket_balance);
                }

                if self.arena.is_empty() {

                    self.ptr_node_id.index = self.arena.new_node(token).index;

                } else {

                    let ( operation, insert_node_id ) = self.insert_into_pp_ast(&self.ptr_node_id, &token);
                    match operation {

                        Operation::REPARENT => {

                            if self.arena.has_parent(&insert_node_id) {

                                // create a new node for the inserted token
                                let new_node_id = self.arena.new_node(token);
                                let parent_node_id = self.arena.nodes[insert_node_id.index].parent.unwrap();

                                // TODO: Assumes right child
                                // TODO: check if left or right!

                                // insert new node into old parent
                                // will also update the parent link
                                self.arena.insert_right(&parent_node_id, new_node_id);

                                // insert old node into new node
                                // will also update the parent link
                                self.arena.insert_left(&new_node_id, insert_node_id);

                                // change reparented node's parent
                                // arena.nodes[insert_node_id.index].parent = Some(new_node_id.clone());

                                // move the current pointer to the opening bracket so that
                                // all subsequent inserts go into this node
                                if token_text == "(" {
                                    self.ptr_node_id = new_node_id;
                                }

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
                            if token_id == PP_OPENING_BRACKET_TOKEN_ID { // OPENING_BRACKET
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
                                    if current_node.data.token_id == PP_OPENING_BRACKET_TOKEN_ID { // OPENING_BRACKET
                                        break;
                                    }

                                } else {
                                    // the node has no parent
                                    break;
                                }
                            }
                        }

                        Operation::CLOSE_BRACKETS_UPWARDS => {
                            // println!("CLOSE_BRACKETS_UPWARDS");

                            let mut current_node = &mut self.arena.nodes[insert_node_id.index];

                            // walk up the tree to the next open bracket or to the root node
                            loop {
                                if let Some(parent_token) = current_node.parent {
                                    // the node has a parent

                                    // update pointer
                                    self.ptr_node_id.index = parent_token.index;

                                    // retrieve the new pointed node
                                    current_node = &mut self.arena.nodes[self.ptr_node_id.index];

                                    // if the new node is an opening bracket, abort
                                    if current_node.data.token_id == PP_OPENING_BRACKET_TOKEN_ID { // OPENING_BRACKET

                                        // let mut current_node = &mut self.arena.nodes[self.ptr_node_id.index];
                                        current_node.data.token_id = 80usize; // 80 for close bracke-pair "()"
                                        current_node.data.text = String::from("()");

                                        break;
                                    }

                                } else {
                                    // the node has no parent
                                    break;
                                }
                            }


                        }

                        Operation::CONCATENATE => {
                            if self.arena.has_parent(&insert_node_id) {

                                // let parent_node = &self.arena.nodes[insert_node_id.index];
                                let parent_node_id = self.arena.nodes[insert_node_id.index].parent.unwrap();

                                // new node for concatenation
                                let mut concatenate_token: Token = Token::new();
                                // concatenate_token.text = String::from("#");
                                concatenate_token.token_id = PP_CONCATENATION_TOKEN_ID;
                                let concatenate_token_id = self.arena.new_node(concatenate_token);

                                self.arena.insert_right(&parent_node_id, concatenate_token_id);

                                // insert the old node into the left of the new token
                                // will also update the parent link
                                self.arena.insert_left(&concatenate_token_id, insert_node_id);



                                // new node for the inserted token
                                let new_node_id = self.arena.new_node(token);
                                self.arena.insert_right(&concatenate_token_id, new_node_id);

                                // self.ptr_node_id.index = new_node_id.index;

                                // aim the pointer at the new concatenation node
                                self.ptr_node_id.index = concatenate_token_id.index;

                            } else {

                                // new parent
                                let mut concatenate_token: Token = Token::new();
                                // concatenate_token.text = String::from("#");
                                concatenate_token.token_id = PP_CONCATENATION_TOKEN_ID;

                                let concat_node_id = self.arena.new_node(concatenate_token);

                                // insert the old node into the left of the token
                                // will also update the parent link
                                self.arena.insert_left(&concat_node_id, insert_node_id);

                                // new node for the inserted token
                                let new_node_id = self.arena.new_node(token);
                                self.arena.insert_right(&concat_node_id, new_node_id);

                                // self.ptr_node_id.index = new_node_id.index;

                                // aim the pointer at the new concatenation node
                                self.ptr_node_id.index = concat_node_id.index;
                                // self.ptr_node_id.index = self.arena.new_node(concatenate_token).index;
                            }
                        }

                        Operation::NO_ACTION => {
                        }
                    }
                }

                // DEBUG - print the arena to the console after a new node has been inserted
                //recurse_arena(&arena, &ptr_node_id);

                // if token_text == ")" {
                //     self.bracket_balance = self.bracket_balance - 1;
                //     println!("[bracket_balance]: {:?}", self.bracket_balance);
                // }
            }
        }
    }

    /// Given a token to insert and a node_id to insert into, will
    /// determine the child node to insert into by diving into the tree
    /// and return an operation to perform.
    ///
    /// Will not execute the insert immediately!
    fn insert_into_pp_ast(&self,
        // the node to insert into (Most likely the node that the insert pointer points to)
        node_id: &NodeId,
        // the new object to insert
        token: &Token) -> ( Operation, NodeId )
    {

        if token.text == ")" {
            println!("test");
        }

        loop {

            let payload: &Token = self.arena.get_payload(&node_id);
            match payload.token_id {

                PP_OPENING_BRACKET_TOKEN_ID => {

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

                PP_CLOSING_BRACKET_TOKEN_ID => {
                    // println!("test");
                }

                PP_CLOSED_BRACKET_PAIR_TOKEN_ID => {
                    if token.token_id == PP_IDENTIFIER_TOKEN_ID {
                        return ( Operation::CONCATENATE, node_id.clone() );
                    } else {
                        // let rhs_node_id = self.arena.get_right_id(&node_id).unwrap();
                        // return self.insert_into_pp_ast(rhs_node_id, token);
                        return ( Operation::REPARENT, node_id.clone() );
                    }
                }

                PP_CONCATENATION_TOKEN_ID => {
                    if token.token_id == PP_IDENTIFIER_TOKEN_ID {
                        return ( Operation::CONCATENATE, node_id.clone() );
                    } else if token.token_id == PP_CLOSING_BRACKET_TOKEN_ID {
                        return ( Operation::CLOSE_BRACKETS_UPWARDS, node_id.clone() );
                    } else {
                        let rhs_node_id = self.arena.get_right_id(&node_id).unwrap();
                        return self.insert_into_pp_ast(rhs_node_id, token);
                    }
                }

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

                        // check if an identifier meets an identifier
                        if existing_node_id.data.token_id == PP_IDENTIFIER_TOKEN_ID {
                            if token.token_id == PP_IDENTIFIER_TOKEN_ID {
                                return ( Operation::CONCATENATE, node_id.clone() );
                            }
                        }

                        match existing_node_id.data.token_id {

                            PP_OPENING_BRACKET_TOKEN_ID => { // OPENING_BRACKET
                                if token.token_id == PP_CLOSING_BRACKET_TOKEN_ID { // CLOSING_BRACKET
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
            }
        }
    }

    pub fn print_console(&self) {
        let mut string_buffer: String = String::new();
        recurse_arena(&self.arena, &self.ptr_node_id, &mut string_buffer);
        println!("{}", string_buffer);
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
        let file = File::create("dot/preprocessor_tree.dot").expect("Create file failed!");

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
        self.bracket_balance = 0;
    }
}