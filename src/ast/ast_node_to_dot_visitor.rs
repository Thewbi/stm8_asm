/*
use crate::c_ast::ast_node::AstNode;

pub struct AstNodeToDotVisitor {
    pub string_buffer: String,
}

impl AstNodeToDotVisitor {

    pub fn new() -> AstNodeToDotVisitor {
        AstNodeToDotVisitor {
            string_buffer: String::from(""),
        }
    }

    pub fn visit(&mut self, ast_node: &AstNode) {
        self.string_buffer.push_str(format!("  {:?} [label=\"{}\"]\n", ast_node.id, ast_node.string_val).as_str());

        // LHS
        if let Some(lhs) = ast_node.lhs.as_ref() {
            self.string_buffer.push_str(format!("  {:?} -> {:?}\n", ast_node.id, lhs.id).as_str());
            if let Some(parent_id) = lhs.parent_id {
                self.string_buffer.push_str(format!("  {:?} -> {:?}\n", lhs.id, parent_id).as_str());
            }
            self.visit(lhs);
        }

        // RHS
        if let Some(rhs) = ast_node.rhs.as_ref() {
            self.string_buffer.push_str(format!("  {:?} -> {:?}\n", ast_node.id, rhs.id).as_str());
            if let Some(parent_id) = rhs.parent_id {
                self.string_buffer.push_str(format!("  {:?} -> {:?}\n", rhs.id, parent_id).as_str());
            }
            self.visit(rhs);
        }
    }

}
     */