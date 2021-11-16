use std::cmp::{Ord, Ordering, Eq, PartialOrd};
use std::convert::From;
use std::convert::TryFrom;
use crate::page_layout::{PTR_SIZE};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Offset(pub usize);

// Try to convert from BigEndian bytes array into an Offset representation of its BigEndian integer
impl TryFrom([u8; PTR_SIZE]) for Offset {
    type Error = Error;

    fn try_from(arr: [u8; PTR_SIZE]) -> Result<Self, Self::Error> {
        Ok(Offset(usize::from_be_bytes(arr)))
    }
}
// A NodeType represents different node type inside a node
#[derive(Eq, Clone, Debug, PartialEq)]
pub enum NodeType {
    // Internal node contains a vector of pointers to their children and a vector of keys
    Internal(Vec<Offset>, Vec<Key>),

    // Leaf node contains a vector of key value pairs
    Leaf(Vec<KeyValuePair>),

    Unspecified
}

// Convert a byte into a NodeType
impl From(u8) for NodeType {
    fn from(node: u8) -> NodeType {
        match node {
            0x01 => NodeType::Internal(Vec<Offset>::new(), Vec<Key>::new()),
            0x02 => NodeType::Leaf(Vec<KeyValuePair>::new()),
            _ => NodeType::Unspecified,
        }
    }
}

// Convert a NodeType into a byte
impl From(&NodeType) for u8 {
    fn from(node_type: &NodeType) -> u8 {
        match node_type {
            NodeType::Internal(_, _) => 0x01,
            NodeType::Leaf(_) => 0x02,
            NodeType::Unspecified => 0x03
        }
    }
}

#[derive(Clone, Eq, Debug, Ord, PartialOrd, PartialEq)]
pub struct Key (pub String);

#[derive(Clone, Eq, Debug)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String
}

impl Ord for KeyValuePair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key);
    }
}

impl PartialOrd for KeyValuePair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other));
    }
}

impl PartialEq for KeyValuePair {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

impl KeyValuePair {
    pub fn new(key: String, value: String) -> KeyValuePair {
        KeyValuePair {key, value}
    }
}