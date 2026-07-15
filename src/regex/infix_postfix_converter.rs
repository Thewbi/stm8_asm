#![allow(
dead_code,
unused_imports,
unused_must_use,
unused_variables,
unused_assignments
)]

use std::fmt;
use std::fmt::Debug;

use crate::regex::arena::Arena;
use crate::regex::regex_building_block::RegexBuildingBlock;
use crate::regex::arena::Node;
use crate::regex::arena::NodeId;

pub fn recurse_postfix(arena: &Arena<RegexBuildingBlock>, parent_node_id: &NodeId, string_buffer: &mut String) {
    let parent_node: &Node<RegexBuildingBlock> = &arena.nodes[parent_node_id.index];
    match &parent_node.left {
        Some(_) => {
            recurse_postfix(arena, parent_node.left.as_ref().unwrap(), string_buffer);
        }
        None => {
        }
    }
    match &parent_node.right {
        Some(_) => {
            recurse_postfix(arena, parent_node.right.as_ref().unwrap(), string_buffer);
        }
        None => {
        }
    }

    // // DEBUG
    // print!("{:?}", parent_node.data);

    match parent_node.data {

        // unescaped for processing, the special characters have to be escaped again for output
        RegexBuildingBlock::CharacterLiteral(c) => {
            match c {
                '|' => { string_buffer.push_str("\\|"); }
                '+' => { string_buffer.push_str("\\+"); }
                '-' => { string_buffer.push_str("\\-"); }
                '*' => { string_buffer.push_str("\\*"); }
                '^' => { string_buffer.push_str("\\^"); }
                '(' => { string_buffer.push_str("\\("); }
                ')' => { string_buffer.push_str("\\)"); }
                _ => { string_buffer.push_str(format!("{:?}", parent_node.data).as_str()); }
            }
        }
        _ => {
            // output to string buffer
            string_buffer.push_str(format!("{:?}", parent_node.data).as_str());
        }
    }
    
    // output to string buffer
    //string_buffer.push_str(format!("{:?}", parent_node.data).as_str());
}

pub fn descend_into_concatenation_right_side(arena: &Arena<RegexBuildingBlock>, start_node_id: &NodeId) -> NodeId {
    let mut node_id = start_node_id.clone();
    loop {
        let node: &Node<RegexBuildingBlock> = &arena.nodes[node_id.index];
        match node.right {
            Some(right_node_id) => {
                let right_child_node: &Node<RegexBuildingBlock> = &arena.nodes[right_node_id.index];
                match right_child_node.data {
                    RegexBuildingBlock::Concatenation => {
                        node_id = right_node_id.clone();
                    }
                    _ => {
                        break;
                    }
                }
            }
            None => {
                break;
            }
        }
    }

    node_id
}

pub fn replace_parent_of_right_child(arena: &mut Arena<RegexBuildingBlock>, node_id: &NodeId, regex_building_block: RegexBuildingBlock) {

    // get the next free index
    let next_index = arena.nodes.len();

    // push the node into the arena
    arena.nodes.push(Node {
        left: None,
        right: None,
        data: regex_building_block,
    });

    // get node_id of right child
    let right_child_node_id = arena.nodes[node_id.index].right.unwrap().index;

    // insert new right child
    // arena.nodes[node_id.index].right.unwrap().index = next_index;
    arena.nodes[node_id.index].right = Some ( NodeId { index: next_index } );

    // insert old child into left side of new child
    arena.nodes[next_index].left = Some ( NodeId { index: right_child_node_id } );
}

// add a new concatenation node as right child into node_id and place the old right child and the regex_building_block as left and right children into the concatenation
pub fn concat_right_side(arena: &mut Arena<RegexBuildingBlock>, node_id: &NodeId, regex_building_block: RegexBuildingBlock) -> usize {

    // get the next free index
    let next_index = arena.nodes.len();

    // push the node into the arena
    arena.nodes.push(Node {
        // parent: None,
        left: None,
        right: None,
        data: regex_building_block,
    });

    // get the next free index
    let concat_index = arena.nodes.len();

    // push the node into the arena
    arena.nodes.push(Node {
        // parent: None,
        left: None,
        right: None,
        data: RegexBuildingBlock::Concatenation,
    });

    // save right index
    let old_right_option = arena.nodes[node_id.index].right;

    // insert the new concat into the right side
    arena.nodes[node_id.index].right = Some ( NodeId { index: concat_index } );
    
    // insert old right side into left side of the new concat
    match old_right_option {
        Some(old_right_node_id) => {
            arena.nodes[concat_index].left = Some ( NodeId { index: old_right_node_id.index } );
        }
        _ => {

        }
    }

    // insert new building block into the right side of the new concat
    arena.nodes[concat_index].right = Some ( NodeId { index: next_index } );

    next_index
}

// build a new concatenation node and return it's id as the first touple element
// make that new concatenation the parent of the old node (left-child) and the specified RegexBuildingBlock, right side
// return the id of the new node for the regex building block as the second touple element
pub fn new_concat_root(arena: &mut Arena<RegexBuildingBlock>, node_id: &NodeId, regex_building_block: RegexBuildingBlock) -> (usize, usize) {

    // get the next free index
    let next_index = arena.nodes.len();

    // push the node into the arena
    arena.nodes.push(Node {
        left: None,
        right: None,
        data: RegexBuildingBlock::Concatenation,
    });

    // get the next free index
    let regex_bb_index = arena.nodes.len();

    // push the node into the arena
    arena.nodes.push(Node {
        left: None,
        right: None,
        data: regex_building_block,
    });

    arena.nodes[next_index].left = Some ( NodeId { index: node_id.index } );
    arena.nodes[next_index].right = Some ( NodeId { index: regex_bb_index } );

    (next_index, regex_bb_index)
}

pub struct InfixPostfixConverter {
    pub arena: Arena<RegexBuildingBlock>,
    pub root_node_id: NodeId,
    character_class_mode: bool,
    repeat_mode: bool,
    character_start_option: Option<char>,
    character_end_option: Option<char>,
    left: bool,
    pub root_index: usize,
    pub root_stack: Vec<NodeId>,
    escaped_sequence: bool,
}

impl InfixPostfixConverter {

    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            root_node_id: NodeId {
                index: 0
            },
            character_class_mode: false,
            repeat_mode: false,
            character_start_option: None,
            character_end_option: None,
            left: true,
            root_index: 0,
            root_stack: vec![NodeId { index: 0 }; 10], // for large expressions, this stack will be too small. Adjust it.
            escaped_sequence: false,
        }
    }

    pub fn process_literal_character(&mut self, c: char) {

        // if in bracket_mode build up the character class operator with start and end node
        if self.character_class_mode { // e.g. [a-z]
            if !c.is_alphanumeric() {
                panic!("Range border in repeat operator is not a numeric character!");
            }
            if self.character_start_option.is_none() {
                self.character_start_option = Some(c);
            } else {
                self.character_end_option = Some(c);
            }
        } else if self.repeat_mode { // e.g {3}, {2,4}
            if !c.is_numeric() && c != ',' {
                panic!("Range border in repeat operator is not a numeric character!");
            }
            if self.character_start_option.is_none() {
                self.character_start_option = Some(c);
                self.character_end_option = Some(c);
            } else {
                self.character_end_option = Some(c);
            }
        } else {
            let character_literal = RegexBuildingBlock::CharacterLiteral(c);
            if self.arena.is_empty() {
                self.root_node_id = self.arena.new_node(character_literal);
            } else {
    
                let mut inserted: bool = false;
                let mut update_root_node_id: bool = true;
    
                let mut last_node_id: NodeId = self.root_node_id.clone();
                let mut node_id: NodeId = self.root_node_id.clone();
    
                while !inserted {
                
                    let root_value = self.arena.get_payload(&node_id);
                    match root_value {

                        RegexBuildingBlock::Not => {
                            let node_id_option = self.arena.get_right_id(&node_id);
    
                            match node_id_option {
                                Some(right_child_node_id) => {
                                    
                                    // self.left = false;
                                    // last_node_id = node_id.clone();
                                    // node_id = right_child_node_id.clone();
                                    // update_root_node_id = false;
                                    // inserted = false;

                                    // concatenate instead of descend
                                    let res = new_concat_root(&mut self.arena, &node_id, character_literal);
                                    self.root_stack[self.root_index].index = res.0;

                                    // self.root_index = self.root_index + 1;
                                    // self.root_stack[self.root_index].index = res.1;

                                    inserted = true;

                                    // panic!("test");
                                }
                                None => {
                                    self.arena.add_right(&node_id, character_literal);
    
                                    inserted = true;
                                }
                            }
                        }
    
                        RegexBuildingBlock::OpeningBraces => {
                            
                            let node_id_option = self.arena.get_right_id(&node_id);
    
                            match node_id_option {
                                Some(right_child_node_id) => {
                                    
                                    self.left = false;
    
                                    last_node_id = node_id.clone();
    
                                    node_id = right_child_node_id.clone();
    
                                    update_root_node_id = false;
                                    inserted = false;
                                }
                                None => {
                                    self.arena.add_right(&node_id, character_literal);
    
                                    inserted = true;
                                }
                            }
                        }
    
                        RegexBuildingBlock::ClosingBraces => {}
    
                        RegexBuildingBlock::ClosedBraces => {
                            // println!("ClosedBraces");
    
                            let concat_node_id: NodeId = self.arena.new_node(RegexBuildingBlock::Concatenation);
                            
                            // insert old node into the left side of new node
                            self.arena.insert_left(&concat_node_id, node_id.clone());
    
                            // insert new literal into the right side
                            self.arena.add_right(&concat_node_id, character_literal);
    
                            // new node becomes root node
                            if update_root_node_id {
                                self.root_node_id.index = concat_node_id.index;
                                self.root_stack[self.root_index].index = concat_node_id.index;
                            } else {
                                self.arena.insert_right(&last_node_id, concat_node_id.clone());
                            }
    
                            inserted = true;
                        }
    
                        RegexBuildingBlock::Concatenation => {
                            let new_root_node_id: NodeId = self.arena.new_node(RegexBuildingBlock::Concatenation);
                            // insert old node into the left side of new node
                            self.arena.insert_left(&new_root_node_id, node_id.clone());
    
                            // insert new literal into the right side
                            self.arena.add_right(&new_root_node_id, character_literal);
    
                            // new node becomes root node
                            if update_root_node_id {
                                self.root_node_id.index = new_root_node_id.index;
                                self.root_stack[self.root_index].index = new_root_node_id.index;
                            } else {
                                self.arena.insert_right(&last_node_id, new_root_node_id.clone());
                            }
    
                            inserted = true;
                        }
    
                        RegexBuildingBlock::CharacterLiteral(_) | RegexBuildingBlock::CharacterClass(_, _) => {
    
                            let concatenation_node_id: NodeId = self.arena.new_node(RegexBuildingBlock::Concatenation);
                            // insert old node into the left side of new node
                            self.arena.insert_left(&concatenation_node_id, node_id.clone());
    
                            // insert new literal into the right side
                            self.arena.add_right(&concatenation_node_id, character_literal);
                            
                            if update_root_node_id {
                                // new node becomes root node
                                self.root_node_id.index = concatenation_node_id.index;
                                self.root_stack[self.root_index].index = concatenation_node_id.index;
                            } else {
                                if self.left {
                                    self.arena.insert_left(&last_node_id, concatenation_node_id.clone());
                                } else {
                                    self.arena.insert_right(&last_node_id, concatenation_node_id.clone());
                                }
                            }
    
                            inserted = true;
                        }
    
                        RegexBuildingBlock::Repeat(_min, _max) => {
    
                            let new_root_node_id: NodeId = self.arena.new_node(RegexBuildingBlock::Concatenation);
                            // insert old node into the left side of new node
                            self.arena.insert_left(&new_root_node_id, node_id.clone());
    
                            // insert new literal into the right side
                            self.arena.add_right(&new_root_node_id, character_literal);
    
                            if node_id.index == self.root_node_id.index {
                                // new node becomes root node
                                self.root_node_id.index = new_root_node_id.index;
                                self.root_stack[self.root_index].index = new_root_node_id.index;
                            } else {
                                self.arena.insert_right(&last_node_id, new_root_node_id.clone());
                            }
    
                            inserted = true;
                        }
    
                        RegexBuildingBlock::Or => {
    
                            let node_id_option = self.arena.get_right_id(&node_id);
                            match node_id_option {
    
                                Some(right_child_node_id) => {
                                    
                                    self.left = false;
    
                                    last_node_id = node_id.clone();
    
                                    node_id = right_child_node_id.clone();
    
                                    update_root_node_id = false;
                                    inserted = false;
                                }
                                None => {
                                    // insert new literal into the right side
                                    self.arena.add_right(&node_id, character_literal);
    
                                    inserted = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn infix_to_postfix(&mut self, regex_infix: &str) -> String {

        // println!("{:?}", regex_infix);

        let mut chars = regex_infix.chars().fuse();
        
        // let mut copy = chars.clone();
        // while let Some(c) = copy.next() {
        //     println!("{:?}", c);
        // }

        while let Some(c) = chars.next() {

            //
            // Escaped sequences
            // 

            if c == '\\' {
                self.escaped_sequence = true;
                continue;
            }

            self.root_node_id = self.root_stack[self.root_index].clone();

/*
            // DEBUG
            println!("");
            println!("{}", c);
            if !self.arena.is_empty() {
                println!("root id: {:?}", self.root_node_id.index);
                for (pos, e) in self.arena.nodes.iter().enumerate() {
                    println!("Node {}, value: {:?}", pos, e.data);
                    match &e.left {
                        Some(_) => {
                            println!("    Left {}", e.left.as_ref().unwrap().index);
                        }
                        None => {
                        }
                    }
                    match &e.right {
                        Some(_) => {
                            println!("    Right {}", e.right.as_ref().unwrap().index);
                        }
                        None => {
                        }
                    }
                }
            }
*/

            // process escaped character sequence
            if self.escaped_sequence {

                self.escaped_sequence = false;

                match c {
                    'n' => { self.process_literal_character('\n'); }
                    'r' => { self.process_literal_character('\r'); }
                    't' => { self.process_literal_character('\t'); }
                    '|' => { self.process_literal_character('|'); }
                    '-' => { self.process_literal_character('-'); }
                    '+' => { self.process_literal_character('+'); }
                    '*' => { self.process_literal_character('*'); }
                    '^' => { self.process_literal_character('^'); }
                    //'s' => { self.process_literal_character(r"\s"); }
                    '(' => { self.process_literal_character('('); }
                    ')' => { self.process_literal_character(')'); }
                    '{' => { self.process_literal_character('{'); }
                    '}' => { self.process_literal_character('}'); }
                    '[' => { self.process_literal_character('['); }
                    ']' => { self.process_literal_character(']'); }
                    '!' => { self.process_literal_character('!'); }
                    '?' => { self.process_literal_character('?'); }
                    _ => { panic!("[infix_to_postfix] unhandled character sequence: {}", c); }
                }

                continue;
            }

            match c {

                // '!' => { panic!(); }

                '^' => {

                    if self.arena.is_empty() {

                        self.root_node_id = self.arena.new_node(RegexBuildingBlock::Not);
                        self.root_stack[self.root_index] = self.root_node_id.clone();

                    } else {

                        let mut inserted: bool = false;
                        let update_root_node_id: bool = true;
            
                        let last_node_id: NodeId = self.root_node_id.clone();
                        let node_id: NodeId = self.root_node_id.clone();
            
                        while !inserted {
                        
                            let root_value = self.arena.get_payload(&node_id);
                            match root_value {

                                RegexBuildingBlock::CharacterLiteral(_) | RegexBuildingBlock::CharacterClass(_, _) | RegexBuildingBlock::Concatenation => {

                                    // applied at root
                                    if node_id.index == self.root_node_id.index {

                                        let res = new_concat_root(&mut self.arena, &node_id, RegexBuildingBlock::Not);

                                        self.root_stack[self.root_index].index = res.0;

                                        // not becomes the new root temporarily so that new symbols are inserted into it instead of at the root
                                        self.root_index = self.root_index + 1;
                                        self.root_stack[self.root_index].index = res.1;

                                        inserted = true;

                                    } else {

                                        let id = concat_right_side(&mut self.arena, &last_node_id, RegexBuildingBlock::Not);

                                        // not becomes the new root
                                        self.root_index = self.root_index + 1;
                                        self.root_stack[self.root_index].index = id;

                                        inserted = true;

                                    }
                                }

                                _ => { panic!("test: {} meets {:?}. Only allowed: character_class and concatenation and CharacterLiteral", c, root_value); }

                            }

                        }
                        
                    }
                }

                // # not allowed in regex format
                '#' => { panic!(); }

                '+' | '*' | '?' => {

                    let mut min:u8 = 0;
                    let mut max:u8 = std::u8::MAX;
                    if c == '+' {
                        min = 1;
                    }
                    if c == '?' {
                        min = 0;
                        max = 1;
                    }
                    let repeat = RegexBuildingBlock::Repeat(min, max);

                    if self.arena.is_empty() {
                        panic!("illegal");
                    } else {

                        let mut inserted: bool = false;
                        let mut update_root_node_id: bool = true;

                        let mut last_node_id: NodeId = self.root_node_id.clone();
                        let mut node_id: NodeId = self.root_node_id.clone();

                        while !inserted {

                            let root_value = self.arena.get_payload(&node_id);
                            match root_value {

                                RegexBuildingBlock::Not => {
                                    // applied at root
                                    if node_id.index == self.root_node_id.index {

                                        let res = new_concat_root(&mut self.arena, &node_id, RegexBuildingBlock::Not);

                                        self.root_stack[self.root_index].index = res.0;

                                        // not becomes the new root
                                        self.root_index = self.root_index + 1;
                                        self.root_stack[self.root_index].index = res.1;

                                        inserted = true;

                                    } else {

                                        let id = concat_right_side(&mut self.arena, &last_node_id, RegexBuildingBlock::Not);

                                        // not becomes the new root
                                        self.root_index = self.root_index + 1;
                                        self.root_stack[self.root_index].index = id;

                                        inserted = true;

                                    }
                                }

                                RegexBuildingBlock::Concatenation => {

                                    node_id = descend_into_concatenation_right_side(&self.arena, &node_id).clone();

                                    // repeat is applied at the root of the tree
                                    if node_id.index == self.root_node_id.index {

                                        self.arena.change_payload(&node_id, repeat);

                                        let concatenation_node_id: NodeId = self.arena.new_node(RegexBuildingBlock::Concatenation);

                                        // insert old node into the left side of new node
                                        self.arena.insert_left(&concatenation_node_id, node_id.clone());

                                        // new node becomes root node
                                        if update_root_node_id {
                                            self.root_node_id.index = concatenation_node_id.index;
                                            self.root_stack[self.root_index].index = concatenation_node_id.index;
                                        }

                                        inserted = true;

                                    } else {

                                        // repeat is applied in the middle of the tree
                                        self.arena.insert_repeat_node_into_node(&node_id, repeat.clone());

                                        inserted = true;
                                    }
                                }

                                RegexBuildingBlock::OpeningBraces => {
                                    let node_id_option = self.arena.get_right_id(&node_id);

                                    match node_id_option {
                                        Some(right_child_node_id) => {
                                            last_node_id = node_id.clone();
                                            node_id = right_child_node_id.clone();

                                            update_root_node_id = false;
                                            inserted = false;
                                        }
                                        None => {
                                            panic!("test");
                                        }
                                    }
                                }

                                RegexBuildingBlock::ClosedBraces | RegexBuildingBlock::CharacterLiteral(_) | RegexBuildingBlock::CharacterClass(_, _) => {

                                    // repeat is applied at the root of the tree
                                    if node_id.index == self.root_node_id.index {

                                        let new_root_node_id: NodeId = self.arena.new_node(repeat);

                                        // insert old node into the left side of new node
                                        self.arena.insert_left(&new_root_node_id, node_id.clone());

                                        // new node becomes root node
                                        if update_root_node_id {
                                            self.root_node_id.index = new_root_node_id.index;
                                            self.root_stack[self.root_index].index = new_root_node_id.index;
                                        }

                                        inserted = true;

                                    } else {
                                        // repeat is applied in the middle of the tree
                                        self.arena.insert_repeat_node_into_node(&last_node_id, repeat.clone());

                                        inserted = true;
                                    }
                                }

                                RegexBuildingBlock::Or => {
                                    let node_id_option = self.arena.get_right_id(&node_id);
                                    match node_id_option {

                                        Some(right_child_node_id) => {
                                            
                                            self.left = false;

                                            last_node_id = node_id.clone();

                                            node_id = right_child_node_id.clone();

                                            update_root_node_id = false;
                                            inserted = false;
                                        }
                                        None => {
                                            panic!("error");
                                        }
                                    }
                                }

                                _ => { panic!("test: {} meets {:?}", c, root_value); }
                            }
                        }
                    }
                }
                '{' => {
                    if self.character_class_mode || self.repeat_mode {
                        panic!("Illegal Syntax! Nested brackets!");
                    }
                    self.repeat_mode = true;
                }
                '}' => {
                    if !self.repeat_mode {
                        panic!("Illegal Syntax! '}}' used without opening brackets!");
                    }

                    if self.character_end_option == None {
                        self.character_end_option = self.character_start_option;
                    }

                    let repeat = RegexBuildingBlock::Repeat(
                        (self.character_start_option.unwrap() as u8 - 0x30) as u8, 
                        (self.character_end_option.unwrap() as u8 - 0x30) as u8
                    );

                    if self.arena.is_empty() {
                        self.root_node_id = self.arena.new_node(repeat);
                    } else {
                        let root_value = self.arena.get_payload(&self.root_node_id);
                        match root_value {

                            RegexBuildingBlock::Concatenation => {

                                self.arena.change_payload(&self.root_node_id, repeat);

                                let new_root_node_id: NodeId = self.arena.new_node(RegexBuildingBlock::Concatenation);

                                // insert old node into the left side of new node
                                self.arena.insert_left(&new_root_node_id, self.root_node_id.clone());

                                // new node becomes root node
                                self.root_node_id.index = new_root_node_id.index;
                                self.root_stack[self.root_index].index = new_root_node_id.index;
                            }

                            RegexBuildingBlock::CharacterLiteral(_) | RegexBuildingBlock::CharacterClass(_, _) => {

                                let new_root_node_id: NodeId = self.arena.new_node(repeat);
                                // insert old node into the left side of new node
                                self.arena.insert_left(&new_root_node_id, self.root_node_id.clone());

                                // new node becomes root node
                                self.root_node_id.index = new_root_node_id.index;
                                self.root_stack[self.root_index].index = new_root_node_id.index;

                            }

                            _ => { panic!("NIY"); }
                        }
                    }

                    self.character_start_option = None;
                    self.character_end_option = None;
                    self.repeat_mode = false;
                }

                //
                // Character Class
                //

                '-' => {
                    // // process the minus/dash as a literal character, if it has been escaped
                    // // otherwise it is part of a character class and will not be processed in isolation
                    // if self.escaped_sequence {
                    //     self.escaped_sequence = false;
                    //     self.process_literal_character(c);
                    // }
                }
                '[' => {
                    if self.character_class_mode || self.repeat_mode {
                        panic!("Illegal Syntax! Nested brackets!");
                    }
                    self.character_class_mode = true;
                }
                ']' => { 
                    if !self.character_class_mode {
                        panic!("Illegal Syntax! ']' used without opening brackets!");
                    }

                    let character_class = RegexBuildingBlock::CharacterClass(self.character_start_option.unwrap(), self.character_end_option.unwrap());
                    
                    if self.arena.is_empty() {
                        self.root_node_id = self.arena.new_node(character_class);
                    } else {
                        let root_value = self.arena.get_payload(&self.root_node_id);
                        match root_value {

                            RegexBuildingBlock::Not => {
                                let node_id_option = self.arena.get_right_id(&self.root_node_id);
        
                                match node_id_option {
                                    Some(right_child_node_id) => {
                                        
                                        // self.left = false;
                                        // last_node_id = node_id.clone();
                                        // node_id = right_child_node_id.clone();
                                        // update_root_node_id = false;
                                        // inserted = false;
    
                                        // concatenate instead of descend
                                        let res = new_concat_root(&mut self.arena, &self.root_node_id, character_class);
                                        self.root_stack[self.root_index].index = res.0;
    
                                        // ascend from not to old root
                                        if self.root_index > 0 {
                                            self.root_index = self.root_index - 1;
                                        }

                                        //self.root_index = self.root_index + 1;
                                        //self.root_stack[self.root_index].index = res.index;
        
                                        //inserted = true;
    
                                        // panic!("test");
                                    }
                                    None => {
                                        let res = self.arena.add_right(&self.root_node_id, character_class);

                                        // ascend from not to old root
                                        if self.root_index > 0 {
                                            self.root_index = self.root_index - 1;
                                        }

                                        // self.root_index = self.root_index + 1;
                                        // self.root_stack[self.root_index].index = res.index;
        
                                        //inserted = true;
                                    }
                                }
                            }

                            RegexBuildingBlock::CharacterLiteral(_) | RegexBuildingBlock::CharacterClass(_, _) | RegexBuildingBlock::Concatenation => {

                                let new_root_node_id: NodeId = self.arena.new_node(RegexBuildingBlock::Concatenation);
                                // insert old node into the left side of new node
                                self.arena.insert_left(&new_root_node_id, self.root_node_id.clone());

                                // insert new literal into the right side
                                self.arena.add_right(&new_root_node_id, character_class);

                                // new node becomes root node
                                self.root_node_id.index = new_root_node_id.index;
                                self.root_stack[self.root_index].index = new_root_node_id.index;
                            }

                            RegexBuildingBlock::Repeat(_min, _max) => {

                                let new_root_node_id: NodeId = self.arena.new_node(RegexBuildingBlock::Concatenation);

                                // insert old node into the left side of new node
                                self.arena.insert_left(&new_root_node_id, self.root_node_id.clone());

                                // insert new literal into the right side
                                self.arena.add_right(&new_root_node_id, character_class);

                                // new node becomes root node
                                self.root_node_id.index = new_root_node_id.index;
                                self.root_stack[self.root_index].index = new_root_node_id.index;
                            }

                            RegexBuildingBlock::OpeningBraces => {

                                let mut inserted: bool = false;
                                let mut update_root_node_id: bool = true;

                                let mut last_node_id: NodeId = self.root_node_id.clone();
                                let mut node_id: NodeId = self.root_node_id.clone();

                                while !inserted {
                                    let node_id_option = self.arena.get_right_id(&node_id);
        
                                    match node_id_option {

                                        Some(right_child_node_id) => {
                                            
                                            self.left = false;

                                            // update the pointers to the current tree node, move down one level
                                            last_node_id = node_id.clone();
                                            node_id = right_child_node_id.clone();
            
                                            // keep descending into the tree
                                            update_root_node_id = false;
                                            inserted = false;
                                        }
                                        None => {
                                            self.arena.add_right(&node_id, character_class);
            
                                            inserted = true;
                                        }
                                    }
                                }
                            }

                            RegexBuildingBlock::Or => {
                                // println!("test");

                                let mut inserted: bool = false;
                                let mut update_root_node_id: bool = true;

                                let mut last_node_id: NodeId = self.root_node_id.clone();
                                let mut node_id: NodeId = self.root_node_id.clone();

                                while !inserted {
                                    let node_id_option = self.arena.get_right_id(&node_id);
        
                                    match node_id_option {

                                        Some(right_child_node_id) => {
                                            
                                            self.left = false;

                                            // update the pointers to the current tree node, move down one level
                                            last_node_id = node_id.clone();
                                            node_id = right_child_node_id.clone();
            
                                            // keep descending into the tree
                                            update_root_node_id = false;
                                            inserted = false;
                                        }
                                        None => {
                                            self.arena.add_right(&node_id, character_class);
            
                                            inserted = true;
                                        }
                                    }
                                }
                            }

                            _ => { 
                                panic!("NIY"); 
                            }
                        }
                    }

                    self.character_start_option = None;
                    self.character_end_option = None;
                    self.character_class_mode = false;
                }

                //
                // braces
                //

                '(' => {

                    if self.arena.is_empty() {

                        self.root_node_id = self.arena.new_node(RegexBuildingBlock::OpeningBraces);
                        self.root_stack[self.root_index] = self.root_node_id.clone();

                    } else {

                        let mut inserted: bool = false;
                        let mut update_root_node_id: bool = true;

                        let mut last_node_id: NodeId = self.root_node_id.clone();
                        let mut node_id: NodeId = self.root_node_id.clone();

                        while !inserted {

                            let root_value = self.arena.get_payload(&node_id);
                            match root_value {

                                RegexBuildingBlock::Not => {
                                    let node_id_option = self.arena.get_right_id(&node_id);
            
                                    match node_id_option {
                                        Some(right_child_node_id) => {
                                            
                                            // self.left = false;
                                            // last_node_id = node_id.clone();
                                            // node_id = right_child_node_id.clone();
                                            // update_root_node_id = false;
                                            // inserted = false;
        
                                            // concatenate instead of descend
                                            let res = new_concat_root(&mut self.arena, &node_id, RegexBuildingBlock::OpeningBraces);
                                            self.root_stack[self.root_index].index = res.0;
        
                                            self.root_index = self.root_index + 1;
                                            self.root_stack[self.root_index].index = res.1;
        
                                            inserted = true;
        
                                            // panic!("test");
                                        }
                                        None => {
                                            let res = self.arena.add_right(&node_id, RegexBuildingBlock::OpeningBraces);

                                            self.root_index = self.root_index + 1;
                                            self.root_stack[self.root_index].index = res.index;
            
                                            inserted = true;
                                        }
                                    }
                                }

                                RegexBuildingBlock::OpeningBraces => {
                                    let node_id_option = self.arena.get_right_id(&node_id);

                                    match node_id_option {
                                        Some(left_child_node_id) => {
                                            last_node_id = node_id.clone();
                                            node_id = left_child_node_id.clone();

                                            update_root_node_id = false;
                                            inserted = false;
                                        }
                                        None => {
                                            self.arena.add_right(&node_id, RegexBuildingBlock::OpeningBraces);

                                            inserted = true;
                                        }
                                    }
                                }

                                RegexBuildingBlock::ClosedBraces => {

                                    // opening brace meets closed braces: () <- (
                                    // (a)(b)

                                    // applied at root
                                    if node_id.index == self.root_node_id.index {

                                        let res = new_concat_root(&mut self.arena, &node_id, RegexBuildingBlock::OpeningBraces);

                                        self.root_stack[self.root_index].index = res.0;

                                        self.root_index = self.root_index + 1;
                                        self.root_stack[self.root_index].index = res.1;

                                        inserted = true;

                                    } else {

                                        let id = concat_right_side(&mut self.arena, &last_node_id, RegexBuildingBlock::OpeningBraces);

                                        self.root_index = self.root_index + 1;
                                        self.root_stack[self.root_index].index = id;

                                        inserted = true;

                                    }
                                }

                                RegexBuildingBlock::CharacterLiteral(_c) => {

                                    let concat_node_id: NodeId = self.arena.new_node(RegexBuildingBlock::Concatenation);
                                    // insert old node into the left side of new node
                                    self.arena.insert_left(&concat_node_id, node_id.clone());

                                    // insert new literal into the right side
                                    let right_node_id = self.arena.add_right(&concat_node_id, RegexBuildingBlock::OpeningBraces);

                                    // new node becomes root node
                                    if update_root_node_id {
                                        self.root_node_id.index = concat_node_id.index;
                                        self.root_stack[self.root_index].index = concat_node_id.index;
                                    } else {
                                        self.arena.insert_right(&last_node_id, concat_node_id.clone());
                                    }

                                    self.root_index = self.root_index + 1;
                                    self.root_stack[self.root_index] = right_node_id.clone();

                                    inserted = true;
                                }

                                RegexBuildingBlock::Or => {
                                    let node_id_option = self.arena.get_right_id(&node_id);
                                    match node_id_option {

                                        Some(right_child_node_id) => {
                                            
                                            self.left = false;

                                            last_node_id = node_id.clone();

                                            node_id = right_child_node_id.clone();

                                            update_root_node_id = false;
                                            inserted = false;
                                        }
                                        None => {
                                            // insert new literal into the right side
                                            let child_node_id = self.arena.add_right(&node_id, RegexBuildingBlock::OpeningBraces);

                                            self.root_index = self.root_index + 1;
                                            self.root_stack[self.root_index] = child_node_id.clone();

                                            inserted = true;
                                        }
                                    }
                                }

                                _ => {

                                    // CHANGE
                                    // let new_root_node_id: NodeId = arena.new_node(RegexBuildingBlock::OpeningBraces);
                                    // // insert old node into the left side of new node
                                    // arena.insert_left(&new_root_node_id, root_node_id.clone());
                                    // // new node becomes root node
                                    // root_node_id.index = new_root_node_id.index;

                                    let concat_node_id: NodeId = self.arena.new_node(RegexBuildingBlock::Concatenation);
                                    // insert old node into the left side of new node
                                    self.arena.insert_left(&concat_node_id, node_id.clone());

                                    // insert new literal into the right side
                                    let opening_braces_id = self.arena.add_right(&concat_node_id, RegexBuildingBlock::OpeningBraces);

                                    // new node becomes root node
                                    if update_root_node_id {
                                        self.root_node_id.index = concat_node_id.index;
                                        self.root_stack[self.root_index].index = concat_node_id.index;
                                    } else {
                                        self.arena.insert_right(&last_node_id, concat_node_id.clone());
                                    }

                                    inserted = true;

                                    self.root_index = self.root_index + 1;
                                    self.root_stack[self.root_index] = opening_braces_id.clone();
                                }
                            }
                        }
                    }
                }

                ')' => {
                    if self.arena.is_empty() {
                        panic!("invalid!");
                    } else {

                        let mut inserted: bool = false;
                        let mut node_id: NodeId = self.root_node_id.clone();

                        while !inserted {

                            let root_value = self.arena.get_payload(&node_id);
                            match root_value {

                                RegexBuildingBlock::Concatenation => {

                                    let node_id_option = self.arena.get_right_id(&node_id);
                                    match node_id_option {
                                        Some(right_child_node_id) => {
                                            node_id = right_child_node_id.clone();

                                            inserted = false;
                                        }
                                        None => {
                                            panic!("test");
                                        }
                                    }
                                }

                                RegexBuildingBlock::OpeningBraces => {

                                    self.arena.change_payload(&node_id, RegexBuildingBlock::ClosedBraces);
                                    inserted = true;

                                    // ascend out if braces and over NOT operators
                                    while self.root_index > 0 {
                                        self.root_index = self.root_index - 1;
                                        match self.arena.get_payload(&self.root_stack[self.root_index]) {
                                            RegexBuildingBlock::Not => { }
                                            _ => { break; }
                                        }
                                    }
                                }

                                RegexBuildingBlock::Or => {
                                    let node_id_option = self.arena.get_right_id(&node_id);
                                    match node_id_option {
                                        Some(right_child_node_id) => {
                                            node_id = right_child_node_id.clone();

                                            inserted = false;
                                        }
                                        None => {
                                            panic!("test");
                                        }
                                    }
                                }

                                _ => { 
                                    panic!("invalid"); 
                                }
                            }
                        }
                    }
                }

                '|' => {

                    let mut inserted: bool = false;

                    let mut last_node_id: NodeId = self.root_node_id.clone();
                    let mut node_id: NodeId = self.root_node_id.clone();

                    while !inserted {

                        let root_value = self.arena.get_payload(&node_id);
                        match root_value {

                            RegexBuildingBlock::Repeat(_min, _max) => {

                                let new_root_node_id: NodeId = self.arena.new_node(RegexBuildingBlock::Or);
                                // insert old node into the left side of new node
                                self.arena.insert_left(&new_root_node_id, self.root_node_id.clone());

                                // new node becomes root node
                                self.root_node_id.index = new_root_node_id.index;
                                self.root_stack[self.root_index].index = new_root_node_id.index;

                                inserted = true;
                            }

                            RegexBuildingBlock::CharacterLiteral(_) | RegexBuildingBlock::CharacterClass(_, _) | RegexBuildingBlock::Concatenation => {

                                // OR is applied at the root of the tree
                                if node_id.index == self.root_node_id.index {

                                    let new_root_node_id: NodeId = self.arena.new_node(RegexBuildingBlock::Or);

                                    // insert old node into the left side of new node
                                    self.arena.insert_left(&new_root_node_id, node_id.clone());

                                    // new node becomes root node
                                    self.root_node_id.index = new_root_node_id.index;
                                    self.root_stack[self.root_index].index = new_root_node_id.index;

                                    inserted = true;
                                } else {
                                    replace_parent_of_right_child(&mut self.arena, &last_node_id, RegexBuildingBlock::Or);
                                    inserted = true;
                                }
                            }

                            RegexBuildingBlock::Or => {

                                // OR is applied at the root of the tree
                                if node_id.index == self.root_node_id.index {

                                    let new_root_node_id: NodeId = self.arena.new_node(RegexBuildingBlock::Or);

                                    // insert old node into the left side of new node
                                    self.arena.insert_left(&new_root_node_id, node_id.clone());

                                    // new node becomes root node
                                    self.root_node_id.index = new_root_node_id.index;
                                    self.root_stack[self.root_index].index = new_root_node_id.index;

                                    inserted = true;
                                } else {
                                    replace_parent_of_right_child(&mut self.arena, &last_node_id, RegexBuildingBlock::Or);
                                    inserted = true;
                                }
                            }

                            RegexBuildingBlock::OpeningBraces => {
                                let node_id_option = self.arena.get_right_id(&node_id);

                                match node_id_option {
                                    Some(left_child_node_id) => {
                                        last_node_id = node_id.clone();
                                        node_id = left_child_node_id.clone();

                                        // update_root_node_id = false;
                                        inserted = false;
                                    }
                                    None => {
                                        self.arena.add_right(&node_id, RegexBuildingBlock::OpeningBraces);

                                        inserted = true;
                                    }
                                }
                            }

                            RegexBuildingBlock::ClosedBraces => {
                                // OR is applied at the root of the tree
                                if node_id.index == self.root_node_id.index {

                                    let new_root_node_id: NodeId = self.arena.new_node(RegexBuildingBlock::Or);

                                    // insert old node into the left side of new node
                                    self.arena.insert_left(&new_root_node_id, node_id.clone());

                                    // new node becomes root node
                                    self.root_node_id.index = new_root_node_id.index;
                                    self.root_stack[self.root_index].index = new_root_node_id.index;

                                    inserted = true;
                                } else {
                                    panic!("test");
                                }
                            }

                            _ => {
                                panic!("test");
                            }
                        }
                    }
                }

                _ => {
                    self.process_literal_character(c);
                }
            }
        }

/*
        // DEBUG
        self.root_node_id = self.root_stack[self.root_index].clone();
        println!("");
        if !self.arena.is_empty() {
            println!("root id: {:?}", self.root_node_id.index);
            for (pos, e) in self.arena.nodes.iter().enumerate() {
                println!("Node {}, value: {:?}", pos, e.data);
                match &e.left {
                    Some(_) => {
                        println!("    Left {}", e.left.as_ref().unwrap().index);
                    }
                    None => {
                    }
                }
                match &e.right {
                    Some(_) => {
                        println!("    Right {}", e.right.as_ref().unwrap().index);
                    }
                    None => {
                    }
                }
            }
        }
*/

        let mut string_buffer = String::from("");

        //self.root_node_id = self.root_stack[self.root_index].clone();
        self.root_node_id = self.root_stack[0].clone();

        recurse_postfix(&self.arena, &self.root_node_id, &mut string_buffer);
        // println!("");

        string_buffer
    }

    pub fn reset(&mut self) {
        
        // reset
        self.root_node_id.index = 0;
        self.character_class_mode = false;
        self.character_start_option = None;
        self.character_end_option = None;
        self.left = true;
        self.root_index = 0;
        self.root_stack[self.root_index].index = 0;
        self.escaped_sequence = false;
        self.arena.reset();
    }

}