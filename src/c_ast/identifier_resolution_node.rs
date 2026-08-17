use crate::AstNodeType;

#[derive(Debug)]
pub struct IdentifierResolutionNode {
    pub node_type: AstNodeType,
    pub string_val: String,
}

impl IdentifierResolutionNode {

    pub fn new() -> IdentifierResolutionNode {
        IdentifierResolutionNode {
            node_type: AstNodeType::Unknown,
            string_val: String::new(),
        }
    }

}