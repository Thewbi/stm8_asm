use std::fmt;
use std::fmt::Debug;

use crate::regex::regex_building_block::RegexBuildingBlock;

#[derive(Clone, Copy)]
pub struct NodeId {
    pub index: usize,
}

pub struct Node<T> {
    pub left: Option<NodeId>,
    pub right: Option<NodeId>,
    pub data: T, // payload
}

pub struct Arena<T> {
    pub nodes: Vec<Node<T>>,
}

impl<T> Arena<T> {

    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.nodes.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.len() == 0
    }

    pub fn new_node(&mut self, data: T) -> NodeId {

        // Get the next free index
        let next_index = self.nodes.len();

        // Push the node into the arena
        self.nodes.push(Node {
            left: None,
            right: None,
            data: data,
        });

        // Return the node identifier
        NodeId { index: next_index }
    }

    pub fn change_payload(&mut self, node_id: &NodeId, data: T) {
        self.nodes[node_id.index].data = data;
    }

    pub fn get_payload(&mut self, node_id: &NodeId) -> &T {
        &self.nodes[node_id.index].data
    }

    pub fn get_payload_by_index(&mut self, index: usize) -> &T {
        &self.nodes[index].data
    }

    pub fn get_left_id(&mut self, parent_node_id: &NodeId) -> Option<&NodeId> {
        let parent_node: &Node<T> = &self.nodes[parent_node_id.index];
        parent_node.left.as_ref()
    }

    pub fn get_right_id(&mut self, parent_node_id: &NodeId) -> Option<&NodeId> {
        let parent_node: &Node<T> = &self.nodes[parent_node_id.index];
        parent_node.right.as_ref()
    }

    pub fn add_left(&mut self, parent_node_id: &NodeId, data: T) {
        let new_node_id: NodeId = self.new_node(data);
        let parent_node: &mut Node<T> = &mut self.nodes[parent_node_id.index];
        parent_node.left = Some(new_node_id);
    }

    pub fn add_right(&mut self, parent_node_id: &NodeId, data: T) -> NodeId {
        let new_node_id: NodeId = self.new_node(data);
        let parent_node: &mut Node<T> = &mut self.nodes[parent_node_id.index];
        parent_node.right = Some(new_node_id);
        new_node_id.clone()
    }

    pub fn insert_left(&mut self, parent_node_id: &NodeId, left_node_id: NodeId) {
        let parent_node: &mut Node<T> = &mut self.nodes[parent_node_id.index];
        parent_node.left = Some(left_node_id);
    }

    pub fn insert_right(&mut self, parent_node_id: &NodeId, right_node_id: NodeId) {
        let parent_node: &mut Node<T> = &mut self.nodes[parent_node_id.index];
        parent_node.right = Some(right_node_id);
    }

    pub fn insert_repeat_node_into_node(&mut self, node_id: &NodeId, regex_building_block: T) {

        // Get the next free index
        let next_index = self.nodes.len();

        // Push the node into the arena
        self.nodes.push(Node {
            left: None,
            right: None,
            data: regex_building_block,
        });

        let old_right_option = self.nodes[node_id.index].right;

        self.nodes[node_id.index].right = Some ( NodeId { index: next_index } );
        
        match old_right_option {
            Some(old_right_node_id) => {
                self.nodes[next_index].left = Some ( NodeId { index: old_right_node_id.index } );
            }
            _ => {

            }
        }
    }
}

pub fn recurse_arena<T>(arena: &Arena<T>, parent_node_id: &NodeId) 
where T:std::fmt::Debug,
{
    let parent_node: &Node<T> = &arena.nodes[parent_node_id.index];
    match &parent_node.left {
        Some(_) => {
            recurse_arena(arena, parent_node.left.as_ref().unwrap());
        }
        None => {
        }
    }
    match &parent_node.right {
        Some(_) => {
            recurse_arena(arena, parent_node.right.as_ref().unwrap());
        }
        None => {
        }
    }
    print!("{:?}", parent_node.data);
}

pub fn recurse_arena_postfix(arena: &Arena<RegexBuildingBlock>, parent_node_id: &NodeId, string_buffer: &mut String) {
    let parent_node: &Node<RegexBuildingBlock> = &arena.nodes[parent_node_id.index];
    match &parent_node.left {
        Some(_) => {
            recurse_arena_postfix(arena, parent_node.left.as_ref().unwrap(), string_buffer);
        }
        None => {
        }
    }
    match &parent_node.right {
        Some(_) => {
            recurse_arena_postfix(arena, parent_node.right.as_ref().unwrap(), string_buffer);
        }
        None => {
        }
    }

    // DEBUG
    print!("{:?}", parent_node.data);

    match parent_node.data {

        // unescaped for processing, the special characters have to be escaped again for output
        RegexBuildingBlock::CharacterLiteral(c) => {
            match c {
                '|' => { string_buffer.push_str("\\|"); }
                '+' => { string_buffer.push_str("\\+"); }
                '-' => { string_buffer.push_str("\\-"); }
                '*' => { string_buffer.push_str("\\*"); }
                '^' => { string_buffer.push_str("\\^"); }
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