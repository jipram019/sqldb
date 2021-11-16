use crate::node_type::NodeType;
use crate::page_layout::{
    PAGE_SIZE, PTR_SIZE, IS_ROOT_OFFSET, NODE_TYPE_OFFSET,
    PARENT_POINTER_OFFSET, PARENT_POINTER_SIZE,
    INTERNAL_NODE_NUM_CHILDREN_OFFSET, INTERNAL_NODE_NUM_CHILDREN_SIZE,
    LEAF_NODE_NUM_PAIRS_OFFSET, LEAF_NODE_NUM_PAIRS_SIZE,
    COMMON_NODE_HEADER_SIZE
};

// Value is a wrapper of value in a page
pub struct Value(usize);

// Try to convert from slices of bytes into an integer representation of its BigEndian bytes array
impl TryFrom(&[u8]) for Value {
    type Error = Error;

    fn try_from(arr: &[u8]) -> Result<Self, Self::Error> {
        let truncated_arr : [u8; PTR_SIZE] = [0x00; PTR_SIZE];
        for (i, byte) in arr.iter().enumerate() {
            truncated_arr[i] = *byte;
        }
        Ok(Value(usize::from_be_bytes(truncated_arr)))
    }
}

// Page is a representation of single page of memory
pub struct Page {
    data: Box<[u8; PAGE_SIZE]>
}

impl Page {
    pub fn new(data: [u8; PAGE_SIZE]) -> Page {
        Page {
            data: Box::new(data),
        }
    }

    // Return the data underlying array
    pub fn get_data(&self) -> [u8; PAGE_SIZE] {
        *self.data
    }

    // Fetches a pointer from specified offset with specified size
    pub fn get_ptr_from_offset(&self, offset: usize, size: usize) -> &[u8] {
        &self.data[offset..offset+size]
    }

    pub fn get_value_from_offset(&self, offset: usize) -> Result<usize, Error> {
        let bytes = &self.data[offset..offset+PTR_SIZE];
        Value(res) = Value::try_from(bytes)?;
        Ok(res)
    }

    pub fn write_value_at_offset(&self, value: usize, offset: usize) -> Result<(), Error> {
        let bytes = value.to_be_bytes();
        self.data[offset..offset+PTR_SIZE].clone_from_slice(&bytes);
        Ok(())
    }

    pub fn write_bytes_at_offset(&self, bytes: &[u8], offset: usize) -> Result<(), Error> {
        self.data[offset..offset+PTR_SIZE].clone_from_slice(bytes);
        Ok(())
    }
}

// Implement TryFrom<Box<Node>> for easier serialization from Node into on disk formatted Page
impl TryFrom(&Node) for Page {
    type Error = Error;

    fn try_from(node: &Node) -> Result<Page, Error> {
        let mut data : [u8; PAGE_SIZE] = [0x00, PAGE_SIZE];
        data[IS_ROOT_OFFSET] = node.is_root.to_byte();
        data[NODE_TYPE_OFFSET] = u8::from(&node.node_type);

        if !node.is_root {
            match node.parent_offset {
                Some(Offset(parent_offset)) => {
                    data[PARENT_POINTER_OFFSET..PARENT_POINTER_OFFSET+PARENT_POINTER_SIZE]
                    .clone_from_slice(&parent_offset.to_be_bytes())
                },
                None => return Err("Error on parent_offset")
            }
        }

        match node.node_type {
            NodeType::Internal(children_offsets, keys) => {
                data[INTERNAL_NODE_NUM_CHILDREN_OFFSET..INTERNAL_NODE_NUM_CHILDREN_OFFSET+INTERNAL_NODE_NUM_CHILDREN_SIZE]
                .clone_from_slice(children_offsets.len().to_be_bytes());

                let mut page_offset = COMMON_NODE_HEADER_SIZE;

                for Offset(child_offset) in children_offsets {
                    data[page_offset..page_offset+PTR_SIZE].clone_from_slice(&child_offset.to_be_bytes());
                    page_offset += PTR_SIZE;
                }

                for Key(key) in keys {
                    let key_bytes = key.as_bytes();
                    let raw_key: [u8; KEY_SIZE] = [0x00; PTR_SIZE];
                    if key_bytes > KEY_SIZE {
                        return Err("Exceeded key len");
                    }
                    for (i, byte) in key_bytes.iter().enumerate(){
                        raw_key[i] = *byte;
                    }
                    data[page_offset..page_offset+KEY_SIZE].clone_from_slice(&raw_key);
                    page_offset += KEY_SIZE;
                }
            },
            NodeType::Leaf(pairs) => {
                data[LEAF_NODE_NUM_PAIRS_OFFSET..LEAF_NODE_NUM_PAIRS_OFFSET+LEAF_NODE_NUM_PAIRS_SIZE]
                .clone_from_slice(pairs.len().to_be_bytes());

                let mut page_offset = COMMON_NODE_HEADER_SIZE;

                for pair in pairs {
                    let key_bytes = pair.key.as_bytes();
                    let raw_key: [u8; KEY_SIZE] = [0x00; KEY_SIZE];
                    if key_bytes > KEY_SIZE {
                        return Err("Exceeded key size");
                    }
                    for (i, byte) in key_bytes.iter().enumerate(){
                        raw_key[i] = *byte;
                    }
                    data[page_offset..page_offset+KEY_SIZE].clone_from_slice(&raw_key);
                    page_offset += KEY_SIZE;

                    let value_bytes = pair.value.as_bytes();
                    let raw_value: [u8; VALUE_SIZE] = [0x00; VALUE_SIZE];
                    if value_bytes > VALUE_SIZE {
                        return Err("Exceeded value size");
                    }
                    for (i, byte) in value_bytes.iter().enumerate() {
                        raw_value[i] = *byte;
                    }
                    data[page_offset..page_offset+VALUE_SIZE].clone_from_slice(&raw_value);
                    page_offset += VALUE_SIZE;
                }
            },
            NodeType::Unspecified => return Err("Unspecified node type")
        }
    }
}