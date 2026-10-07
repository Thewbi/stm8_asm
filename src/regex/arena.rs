use std::fmt;
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct NodeId {
    pub index: usize,
}

pub struct Node<T> {
    pub left: Option<NodeId>,
    pub right: Option<NodeId>,
    pub parent: Option<NodeId>,
    pub data: T, // payload
}

impl<T> Node<T> {
    pub fn new(data_param: T) -> Node<T> {
        Node {
            left: None,
            right: None,
            parent: None,
            data: data_param,
        }
    }
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

        // get the next free index
        let next_index = self.nodes.len();

        // push the node into the arena
        self.nodes.push(Node::new(data));

        // return the node identifier
        NodeId { index: next_index }
    }

    pub fn change_payload(&mut self, node_id: &NodeId, data: T) {
        self.nodes[node_id.index].data = data;
    }

    pub fn get_payload(&self, node_id: &NodeId) -> &T {
        &self.nodes[node_id.index].data
    }

    pub fn get_payload_by_index(&mut self, index: usize) -> &T {
        &self.nodes[index].data
    }

    pub fn get_left_id(&mut self, parent_node_id: &NodeId) -> Option<&NodeId> {
        let parent_node: &Node<T> = &self.nodes[parent_node_id.index];
        parent_node.left.as_ref()
    }

    pub fn get_right_id(&self, parent_node_id: &NodeId) -> Option<&NodeId> {
        let parent_node: &Node<T> = &self.nodes[parent_node_id.index];
        parent_node.right.as_ref()
    }

    pub fn has_left(&self, parent_node_id: &NodeId) -> bool {
        let parent_node: &Node<T> = &self.nodes[parent_node_id.index];
        parent_node.left.is_some()
    }

    pub fn has_right(&self, parent_node_id: &NodeId) -> bool {
        let parent_node: &Node<T> = &self.nodes[parent_node_id.index];
        parent_node.right.is_some()
    }

    pub fn has_parent(&self, parent_node_id: &NodeId) -> bool {
        let parent_node: &Node<T> = &self.nodes[parent_node_id.index];
        parent_node.parent.is_some()
    }

    /**
     * First create a new node from the data parameter then insert that node
     * into the LHS of the parent node.
     */
    pub fn add_left(&mut self, parent_node_id: &NodeId, data: T) -> NodeId {
        let new_node_id: NodeId = self.new_node(data);

        let child_node: &mut Node<T> = &mut self.nodes[new_node_id.index];
        child_node.parent = Some(*parent_node_id);

        let parent_node: &mut Node<T> = &mut self.nodes[parent_node_id.index];
        parent_node.left = Some(new_node_id);

        new_node_id.clone()
    }

    /**
     * First create a new node from the data parameter then insert that node
     * into the RHS of the parent node.
     */
    pub fn add_right(&mut self, parent_node_id: &NodeId, data: T) -> NodeId {
        let new_node_id: NodeId = self.new_node(data);

        let child_node: &mut Node<T> = &mut self.nodes[new_node_id.index];
        child_node.parent = Some(*parent_node_id);

        let parent_node: &mut Node<T> = &mut self.nodes[parent_node_id.index];
        parent_node.right = Some(new_node_id);

        new_node_id.clone()
    }

    /**
     * Insert node on the LHS of parent.
     */
    pub fn insert_left(&mut self, parent_node_id: &NodeId, left_node_id: NodeId) {
        let parent_node: &mut Node<T> = &mut self.nodes[parent_node_id.index];
        parent_node.left = Some(left_node_id);

        let child_node: &mut Node<T> = &mut self.nodes[left_node_id.index];
        child_node.parent = Some(*parent_node_id);
    }

    /**
     * Insert node on the RHS of parent.
     */
    pub fn insert_right(&mut self, parent_node_id: &NodeId, right_node_id: NodeId) {
        let parent_node: &mut Node<T> = &mut self.nodes[parent_node_id.index];
        parent_node.right = Some(right_node_id);

        let child_node: &mut Node<T> = &mut self.nodes[right_node_id.index];
        child_node.parent = Some(*parent_node_id);
    }

    pub fn insert_repeat_node_into_node(&mut self, node_id: &NodeId, regex_building_block: T) {

        // get the next free index
        let next_index = self.nodes.len();

        // push the node into the arena
        self.nodes.push(Node::new(regex_building_block));

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

/**
 * DEBUG function to print the arena to the console
 */
pub fn recurse_arena<T>(arena: &Arena<T>, parent_node_id: &NodeId)
where T:std::fmt::Debug,
{
    let parent_node: &Node<T> = &arena.nodes[parent_node_id.index];

    // output node (Prefix)
    //println!("{:?}", parent_node.data);

    // output children
    match &parent_node.left {
        Some(_) => {
            recurse_arena(arena, parent_node.left.as_ref().unwrap());
        }
        None => {
        }
    }

    // output node (Infix)
    println!("{:?}", parent_node.data);

    match &parent_node.right {
        Some(_) => {
            recurse_arena(arena, parent_node.right.as_ref().unwrap());
        }
        None => {
        }
    }

    // output node (Postfix)
    // println!("{:?}", parent_node.data);
}

/**
 * DEBUG function to print the arena to the console
 */
pub fn recurse_arena_dot<T>(arena: &Arena<T>,
    parent_node_id: &NodeId,
    string_buffer: &mut String)
where T:std::fmt::Debug, T:std::fmt::Display
{
    let parent_node: &Node<T> = &arena.nodes[parent_node_id.index];

    // output node
    println!("{:?}", parent_node.data);
    string_buffer.push_str(format!("{} [label=\"{} {}\"]\n",
        parent_node_id.index,
        parent_node_id.index,
        &parent_node.data).as_str());

    // output children
    match &parent_node.left {
        Some(_) => {
            let left_id = parent_node.left.as_ref().unwrap();
            recurse_arena_dot(arena, left_id, string_buffer);
            string_buffer.push_str(format!("{} -> {}\n", parent_node_id.index, left_id.index).as_str());
        }
        None => {
        }
    }
    match &parent_node.right {
        Some(_) => {
            let right_id = parent_node.right.as_ref().unwrap();
            recurse_arena_dot(arena, right_id, string_buffer);
            string_buffer.push_str(format!("{} -> {}\n", parent_node_id.index, right_id.index).as_str());
        }
        None => {
        }
    }
}