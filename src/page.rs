use crate::page_layout::{PAGE_SIZE, PTR_SIZE};

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