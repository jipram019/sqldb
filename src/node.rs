use std::cmp::{Debug, Clone};
use std::convert::TryFrom;
use std::str;
use crate::node_type::{Key, KeyValuePair, NodeType, Offset};
use crate::page::Page;
use crate::page_layout::{
    PAGE_SIZE, PTR_SIZE, IS_ROOT_OFFSET, NODE_TYPE_OFFSET,
    PARENT_POINTER_OFFSET, PARENT_POINTER_SIZE,
    INTERNAL_NODE_NUM_CHILDREN_OFFSET, INTERNAL_NODE_NUM_CHILDREN_SIZE,
    LEAF_NODE_NUM_PAIRS_OFFSET, LEAF_NODE_NUM_PAIRS_SIZE,
    COMMON_NODE_HEADER_SIZE
};

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

// Try to implement TryFrom<Page> to deserialize data from a single Page into a Node
impl TryFrom(Page) for Node {
    type Error = Error;
    fn try_from(page: Page) -> Result<Node, Error> {
        let raw = page.get_data()?;
        let is_root = raw[IS_ROOT_OFFSET].from_byte();
        let node_type = NodeType::from(raw[NODE_TYPE_OFFSET]);
        let parent_offset: Option<Offset>;

        if is_root{
            parent_offset = None;
        } else{
            parent_offset = Some(Offset(page.get_value_from_offset(PARENT_POINTER_OFFSET)?));
        }

        match node_type {
            NodeType::Internal(children_offsets, keys) => {
                let num_children = page.get_value_from_offset(INTERNAL_NODE_NUM_CHILDREN_OFFSET)?;
                let page_offset = INTERNAL_NODE_HEADER_SIZE;
                for _i in 1..=num_children {
                    let child_offset = page.get_value_from_offset(page_offset)?;
                    children_offsets.push(Offset(child_offset));
                    page_offset += PTR_SIZE;
                }

                // Number of keys is always one less than number of children
                for _i in 1..num_children {
                    let key_raw = page.get_ptr_from_offset(page_offset, KEY_SIZE)?;
                    let key = match str::from_utf8(key_raw){
                        Some(key) => key,
                        Err(_) => return Err("UTF8Error")
                    };
                    page_offset += KEY_SIZE;

                    // Trim leading or trailing zeros
                    keys.push(Key(key.trim_matches(char::from(0)).to_string()));
                }

                Ok(
                    Node::new(
                        NodeType::Internal(children_offsets, keys),
                        is_root,
                        parent_offset
                    )
                )
            },

            NodeType::Leaf(pairs) => {
                let num_pairs = page.get_value_from_offset(LEAF_NODE_NUM_PAIRS_OFFSET)?;
                let page_offset = INTERNAL_NODE_HEADER_SIZE;

                for _i in 1..=num_pairs {
                    let key_raw = page.get_ptr_from_offset(page_offset, KEY_SIZE)?;
                    let key = match str::from_utf8(key_raw) {
                        Some(key) => key,
                        Err(_) => return Err("UTF8Error")
                    };
                    page_offset += KEY_SIZE;

                    let value_raw = page.get_ptr_from_offset(page_offset, VALUE_SIZE)?;
                    let value = match str::from_utf8(value_raw) {
                        Some(value) => value,
                        Err(_) => return Err("UTF8Error")
                    };
                    page_offset += VALUE_SIZE;

                    // Trim leading or trailing zeros
                    pairs.push(KeyValuePair::new(
                        key.trim_matches(char::from(0)).to_string(),
                        value.trim_matches(char::from(0)).to_string()
                    ))
                }

                Ok(Node::new(
                    NodeType::Leaf(pairs),
                    is_root,
                    parent_offset
                ))
            },

            NodeType::Unspecified => return Err("node type not known")
        }
    }
}