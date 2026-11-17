use std::sync::atomic::Ordering;

use crate::parser::parser::{DEBUG_NODE_COUNTER, DebugNode, Parser};

use super::parser_handler;

impl Parser<String> {

    // removes the current node from the stack and replaces it by a new node,
    // inserting a transition line and a line for the new node.
    pub fn node_to_node(&mut self,
        label: &str,
        rule_id: usize,
        string_buffer: &mut String,
        debug_node_stack: &mut Vec::<DebugNode>) -> usize
    {
        // leave this function empty for a compact Tree which could be an AST.
        // Uncomment the code for a full parse tree that contains all production rules!

        if !self.collapse_nodes {

            // create new node id
            // create new node with node id and label
            let debug_node_id = DEBUG_NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
            let debug_node = DebugNode::new(debug_node_id, String::from(label));
            // print new node into string buffer. e.g.    0 [label="test"]
            string_buffer.push_str(format!("{} [label=\"{} Rule:{} {}\"]\n", debug_node_id, debug_node_id, rule_id, String::from(label)).as_str());

            // take old node from stack
            let old_debug_node = debug_node_stack.pop().unwrap();
            // print transition from old node id to new node id into string buffer. e.g. 0 -> 1 [label="Symbol(h)"];
            string_buffer.push_str(format!(" {:?} -> {:?}\n", debug_node_id, old_debug_node.id).as_str());

            // push new node to stack
            debug_node_stack.push(debug_node);

            return debug_node_id;
        }

        0usize
    }

}