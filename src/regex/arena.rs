use std::fmt;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub enum VisitMode {
    VisitLeft,
    VisitRight,
    BackToParent,
}

#[derive(Clone, Copy, Debug)]
pub struct NodeId {
    pub index: usize,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub left: Option<NodeId>,
    pub right: Option<NodeId>,
    pub parent: Option<NodeId>,
    pub data: T, // payload
    pub visit_mode: VisitMode,
}

impl<T> Node<T> {
    pub fn new(data_param: T) -> Node<T> {
        Node {
            left: None,
            right: None,
            parent: None,
            data: data_param,
            visit_mode: VisitMode::VisitLeft,
        }
    }

    pub fn is_leaf(&self) -> bool {
        return self.left.is_none() && self.right.is_none()
    }
}

pub struct Arena<T> {
    pub nodes: Vec<Node<T>>,
    pub iterator_current_node_id: usize,
}

impl<T> Arena<T> {

    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            iterator_current_node_id: 0,
        }
    }

    pub fn prepare_for_iteration(&mut self, root_node_id: usize) {

        // need to know at which node to start the iteration
        self.iterator_current_node_id = root_node_id;

        // reset the VisitMode for all nodes to VisitLeft which
        // is the start mode out of all the possible modes
        // { VisitLeft, VisitRight, BackToParent }

        self.set_visit_mode_for_all_nodes(root_node_id, VisitMode::VisitLeft);
    }

    pub fn set_visit_mode_for_all_nodes(&mut self, node_id: usize, visit_mode: VisitMode) {

        let mut has_left:bool = false;
        let mut has_right:bool = false;
        let mut left_id:usize = 0;
        let mut right_id:usize = 0;

        // retrieve the node for this level or recursion
        let parent_node: &mut Node<T> = &mut self.nodes[node_id];

        // set the visit mode into the node
        parent_node.visit_mode = visit_mode;

        if let Some(left_id_temp) = &parent_node.left {
            left_id = left_id_temp.index;
            has_left = true;
        }
        if let Some(right_id_temp) = &parent_node.right {
            right_id = right_id_temp.index;
            has_right = true;
        }

        // update the node's left and right child
        // if let Some(left_id) = &parent_node.left {
        if has_left {
            self.set_visit_mode_for_all_nodes(left_id, visit_mode);
        }
        // if let Some(right_id) = &parent_node.right {
        if has_right {
            self.set_visit_mode_for_all_nodes(right_id, visit_mode);
        }
    }

    pub fn reset(&mut self) {
        self.nodes.clear();
        self.iterator_current_node_id = 0;
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
     * into the LHS of the parent node and assign the payload (data).
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
     * into the RHS of the parent node and assign the payload (data).
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

    pub fn remove_left(&mut self, parent_node_id: &NodeId) {
        let parent_node: &mut Node<T> = &mut self.nodes[parent_node_id.index];

        parent_node.left = None;
    }

    pub fn remove_right(&mut self, parent_node_id: &NodeId) {
        let parent_node: &mut Node<T> = &mut self.nodes[parent_node_id.index];

        parent_node.right = None;
    }
}

// iterate over all nodes. Nodes are returned in in-order
// Each node has optional left and right children.
// In-order means the node itself is output after visiting
// the left and before visiting the right child.
// In this order, outputting the AST yields the original String
// the AST was parsed from.
//
// NOTE: This iterator returns copies of nodes! It does not
// return the original nodes! The copied, returned nodes
// do not contain the original left and right children!
// The left and right pointers are None-options!
impl<T> Iterator for Arena<T>
where T: Clone
{
    type Item = Node<T>;

    fn next(&mut self) -> Option<Self::Item> {

        // DEBUG
        // println!("next: {}", self.iterator_current_node_id);

        // perform in-order iteration of all nodes
        loop {

            let current_node = &mut self.nodes[self.iterator_current_node_id];

            let mut has_left:bool = false;
            let mut has_right:bool = false;
            let mut left_id:usize = 0;
            let mut right_id:usize = 0;

            if let Some(left_id_temp) = &current_node.left {
                left_id = left_id_temp.index;
                has_left = true;
            }
            if let Some(right_id_temp) = &current_node.right {
                right_id = right_id_temp.index;
                has_right = true;
            }

            match current_node.visit_mode {
                VisitMode::VisitLeft => {
                    current_node.visit_mode = VisitMode::VisitRight;
                    if has_left {
                        self.iterator_current_node_id = left_id;
                    }
                }
                VisitMode::VisitRight => {
                    current_node.visit_mode = VisitMode::BackToParent;
                    if has_right {
                        self.iterator_current_node_id = right_id;
                    }
                    // output this node
                    let node = Node::new(current_node.data.clone());
                    return Some(node);
                }
                VisitMode::BackToParent => {
                    // reset
                    current_node.visit_mode = VisitMode::VisitLeft;
                    if let Some(parent_node_id) = current_node.parent {
                        self.iterator_current_node_id = parent_node_id.index;
                    } else {
                        return None;
                    }
                }
            }
        }

        return None;
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

    // LHS - output children
    if let Some(left_id) = &parent_node.left {
        recurse_arena(arena, left_id);
    }
    // match &parent_node.left {
    //     Some(_) => {
    //         recurse_arena(arena, parent_node.left.as_ref().unwrap());
    //     }
    //     None => {
    //     }
    // }

    // output node (Infix)
    println!("{:?}", parent_node.data);

    // RHS - output children
    if let Some(right_id) = &parent_node.right {
        recurse_arena(arena, right_id);
    }
    // match &parent_node.right {
    //     Some(_) => {
    //         recurse_arena(arena, parent_node.right.as_ref().unwrap());
    //     }
    //     None => {
    //     }
    // }

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
    // println!("{:?}", parent_node.data);
    string_buffer.push_str(format!("{} [label=\"{} {}\"]\n",
        parent_node_id.index,
        parent_node_id.index,
        &parent_node.data).as_str());

    // LHS - output left child
    if let Some(left_id) = &parent_node.left {
        recurse_arena_dot(arena, left_id, string_buffer);
        string_buffer.push_str(format!("{} -> {}\n", parent_node_id.index, left_id.index).as_str());
    }

    // RHS - output right child
    if let Some(right_id) = &parent_node.right {
        recurse_arena_dot(arena, right_id, string_buffer);
        string_buffer.push_str(format!("{} -> {}\n", parent_node_id.index, right_id.index).as_str());
    }
}