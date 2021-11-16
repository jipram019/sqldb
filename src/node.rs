use std::cmp::{Debug, Clone};

#[derive(Debug, Clone)]
pub struct Node {
    node_type: NodeType,
    is_root: bool,
    parent_offset: Option<Offset>
}

impl Node {
    pub fn new(node_type: NodeType, is_root: bool, parent_offset: Option<Offset>) -> Node {
        Node {
            node_type,
            is_root,
            parent_offset
        }
    }
}