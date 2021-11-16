use std::mem::size_of;

pub const PTR_SIZE : usize = size_of::<usize>();

// Set page size to 4kb
pub const PAGE_SIZE: usize = 4096;

// Common node header layout (10 bytes in total)
pub const IS_ROOT_OFFSET: usize = 0;
pub const IS_ROOT_SIZE: usize = 1;
pub const NODE_TYPE_OFFSET: usize = 1;
pub const NODE_TYPE_SIZE: usize = 1;
pub const PARENT_POINTER_OFFSET: usize = 2;
pub const PARENT_POINTER_SIZE: PTR_SIZE;
pub const COMMON_NODE_HEADER_SIZE :usize = IS_ROOT_SIZE + NODE_TYPE_SIZE + PARENT_POINTER_SIZE;

// Leaf node header layout
pub const LEAF_NODE_NUM_PAIRS_OFFSET: usize = COMMON_NODE_HEADER_SIZE;
pub const LEAF_NODE_NUM_PAIRS_SIZE: usize = PTR_SIZE;
pub const LEAF_NODE_HEADER_SIZE: usize = COMMON_NODE_HEADER_SIZE + LEAF_NODE_NUM_PAIRS_SIZE;

// Internal node header layout
pub const INTERNAL_NODE_NUM_CHILDREN_OFFSET: usize = COMMON_NODE_HEADER_SIZE;
pub const INTERNAL_NODE_NUM_CHILDREN_SIZE: usize = PTR_SIZE;
pub const INTERNAL_NODE_HEADER_SIZE: usize = COMMON_NODE_HEADER_SIZE + INTERNLA_NODE_NUM_CHILDREN_SIZE;

// Set key and value size to 10 bytes
pub const KEY_SIZE: usize = 10;
pub const VALUE_SIZE: usize = 10;

