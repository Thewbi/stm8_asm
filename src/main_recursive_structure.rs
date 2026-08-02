// filename: main_recursive_structure.rs

#[derive(Debug)]
pub enum AstNodeType {
    Operator,
    Value,
}

pub struct AstNode {
    pub lhs: Option<Box<AstNode>>,
    pub rhs: Option<Box<AstNode>>,
    pub node_type: AstNodeType,
    pub string_val: String,
}

impl AstNode {

    pub fn traverse(&self) {
        // Process current node...
        println!("{}", self.string_val);

        // Recurse safely using .as_ref() to borrow the contents of the Option
        if let Some(left_node) = self.lhs.as_ref() {
            left_node.traverse();
        }

        if let Some(right_node) = self.rhs.as_ref() {
            right_node.traverse();
        }
    }

    // Helper function to create a leaf node (a node with no children)
    pub fn leaf(node_type: AstNodeType, val: &str) -> Self {
        AstNode {
            lhs: None,
            rhs: None,
            node_type,
            string_val: val.to_string(),
        }
    }

    // Helper function to create a node with children
    pub fn branch(node_type: AstNodeType, val: &str, left: AstNode, right: AstNode) -> Self {
        AstNode {
            lhs: Some(Box::new(left)),
            rhs: Some(Box::new(right)),
            node_type,
            string_val: val.to_string(),
        }
    }

}

fn main() {

    //      [ * ]           <- Root (1)
    //     /     \
    //  [ + ]   [ - ]       <- Children (2)
    //  /   \   /   \
    // [1]  [2] [3] [4]     <- Grandchildren (4)

    // --- Step 1: Build the 4 Grandchildren (Leaf Nodes) ---
    // Represents an expression like: (1 + 2) * (3 - 4)
    let g1 = AstNode::leaf(AstNodeType::Value, "1");
    let g2 = AstNode::leaf(AstNodeType::Value, "2");
    let g3 = AstNode::leaf(AstNodeType::Value, "3");
    let g4 = AstNode::leaf(AstNodeType::Value, "4");

    // --- Step 2: Build the 2 Children (Sub-branches) ---
    let left_child = AstNode::branch(AstNodeType::Operator, "+", g1, g2);
    let right_child = AstNode::branch(AstNodeType::Operator, "-", g3, g4);

    // --- Step 3: Build the Root Node (Total 7 Nodes) ---
    let root = AstNode::branch(AstNodeType::Operator, "*", left_child, right_child);

    // --- Step 4: Traverse the tree multiple times ---
    println!("--- First Traversal ---");
    root.traverse(); // Borrows `root`

    println!("\n--- Second Traversal ---");
    root.traverse(); // Borrows `root` again, proving it wasn't consumed!

}